use crate::definitions::DemoParserError;
use crate::definitions::HEADER_ENDS_AT_BYTE;
use crate::definitions::INNER_BUF_DEFAULT_LEN;
use crate::first_pass::fallbackbytes::GAME_EVENT_LIST_FALLBACK_BYTES;
use crate::first_pass::frameparser::Frame;
use crate::first_pass::frameparser::FrameParser;
use crate::first_pass::parser_settings::FirstPassParser;
use crate::first_pass::prop_controller::PropController;
use crate::first_pass::read_bits::Bitreader;
use crate::first_pass::sendtables::Serializer;
use crate::first_pass::stringtables::StringTable;
use crate::first_pass::stringtables::UserInfo;
use crate::second_pass::decoder::QfMapper;
use ahash::AHashMap;
use csgoproto::message_type::NetMessageType;
use csgoproto::message_type::NetMessageType::*;
use csgoproto::CDemoFullPacket;
use csgoproto::CDemoPacket;
use csgoproto::CDemoSendTables;
use csgoproto::EDemoCommands::*;
use csgoproto::CDemoClassInfo;
use csgoproto::CDemoFileHeader;
use csgoproto::csvc_msg_game_event_list::DescriptorT;
use csgoproto::CsvcMsgGameEventList;
use nohash::IntMap;
use nohash::IntSet;
use prost::Message;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct FirstPassOutput {
    pub added_temp_props: Vec<String>,
    pub baselines: AHashMap<u32, Vec<u8>>,
    pub fullpackets: IntMap<usize, Option<CDemoPacket>>,
    pub ge_list: AHashMap<i32, DescriptorT>,
    pub header: AHashMap<String, String>,
    pub prop_controller: PropController,
    pub qfmap: QfMapper,
    pub serializer_by_cls_id: Vec<Serializer>,
    pub string_tables: Vec<StringTable>,
    pub stringtable_players: BTreeMap<i32, UserInfo>,
    pub wanted_players: IntSet<u64>,
    pub wanted_ticks: IntSet<i32>,
}

impl<'a> FirstPassParser<'a> {
    pub fn parse_demo(&mut self, demo_bytes: &'a [u8]) -> Result<(), DemoParserError> {
        self.handle_short_header(demo_bytes)?;

        let mut reuseable_buffer = vec![0_u8; INNER_BUF_DEFAULT_LEN];
        let mut ptr = HEADER_ENDS_AT_BYTE;
        let max_ptr_value = demo_bytes.len();

        // Loop that goes through the entire file
        while ptr < max_ptr_value {
            let frame = FrameParser::read_frame(demo_bytes, &mut ptr)?;
            if frame.demo_cmd == DemStop {
                break;
            }
            // Skip reading/decompressing frame if we have nothing to do with it
            if !matches!(frame.demo_cmd, DemSendTables | DemFileHeader | DemClassInfo | DemSignonPacket | DemFullPacket) {
                continue;
            }

            let bytes = frame.get_bytes(&mut reuseable_buffer, demo_bytes)?;
            match frame.demo_cmd {
                DemSendTables => self.parse_sendtable_bytes(bytes)?,
                DemFileHeader => self.parse_header(bytes)?,
                DemClassInfo => self.parse_class_info(bytes)?,
                DemSignonPacket => self.parse_packet(bytes)?,
                DemFullPacket => self.parse_full_packet(bytes, &frame)?,
                _ => {}
            };
        }
        self.fallback_if_first_pass_missing_data()
    }

    fn parse_sendtable_bytes(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        self.sendtable_message = CDemoSendTables::decode(bytes).map_err(|_| DemoParserError::MalformedMessage).map(Some)?;
        Ok(())
    }

    fn parse_fallback_event_list(&mut self) -> Result<(), DemoParserError> {
        let event_list = CsvcMsgGameEventList::decode(GAME_EVENT_LIST_FALLBACK_BYTES).map_err(|_| DemoParserError::MalformedMessage)?;
        for event_desc in event_list.descriptors {
            self.ge_list.insert(event_desc.eventid(), event_desc);
        }
        Ok(())
    }

    pub fn create_output(self) -> Result<FirstPassOutput, DemoParserError> {
        let serializer_by_cls_id = self.serializer_by_cls_id.ok_or_else(|| DemoParserError::ClassMapperNotFoundFirstPass)?;
        Ok(FirstPassOutput {
            header: self.header,
            fullpackets: self.fullpackets,
            baselines: self.baselines,
            prop_controller: self.prop_controller,
            serializer_by_cls_id,
            qfmap: self.qf_mapper,
            ge_list: self.ge_list,
            wanted_players: IntSet::from_iter(self.settings.wanted_players.iter().cloned()),
            wanted_ticks: IntSet::from_iter(self.settings.wanted_ticks.iter().cloned()),
            string_tables: self.string_tables,
            stringtable_players: self.stringtable_players,
            added_temp_props: self.added_temp_props,
        })
    }

    fn fallback_if_first_pass_missing_data(&mut self) -> Result<(), DemoParserError> {
        if self.ge_list.is_empty() {
            self.parse_fallback_event_list()?;
        }
        Ok(())
    }

    // Message that should come before first game event
    fn parse_game_event_list(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let event_list = CsvcMsgGameEventList::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        self.ge_list = event_list.descriptors.into_iter().map(|d| (d.eventid(), d)).collect();
        Ok(())
    }

    fn parse_full_packet(&mut self, bytes: &[u8], frame: &Frame) -> Result<(), DemoParserError> {
        let full_packet = CDemoFullPacket::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        self.fullpackets.insert(frame.starts_at, full_packet.packet);

        if let Some(string_table) = full_packet.string_table {
            for table in &string_table.tables {
                match table.table_name() {
                    "instancebaseline" => table.items.iter().for_each(|i| {
                        let k = i.str().parse::<u32>().unwrap_or(u32::MAX);
                        self.baselines.insert(k, i.data().to_vec());
                    }),
                    "userinfo" => table.items.iter().for_each(|i| {
                        if let Ok(player) = UserInfo::from_bytes(i.data()) {
                            if player.steamid != 0 {
                                self.stringtable_players.insert(player.userid, player);
                            }
                        }
                    }),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn parse_packet(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let msg = CDemoPacket::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        let mut bitreader = Bitreader::new(msg.data());

        while bitreader.bits_remaining().unwrap_or(0) > 8 {
            let msg_type = bitreader.read_u_bit_var()? as i32;
            let size = bitreader.read_varint()?;
            let msg_bytes = bitreader.read_n_bytes(size as usize)?;

            match NetMessageType::from(msg_type) {
                GE_Source1LegacyGameEventList => self.parse_game_event_list(&msg_bytes),
                svc_CreateStringTable => self.parse_create_stringtable(&msg_bytes),
                svc_UpdateStringTable => self.update_string_table(&msg_bytes),
                svc_ClearAllStringTables => self.clear_stringtables(),
                _ => Ok(()),
            }?
        }
        Ok(())
    }

    fn clear_stringtables(&mut self) -> Result<(), DemoParserError> {
        self.string_tables.truncate(0);
        Ok(())
    }

    fn parse_header(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let header = CDemoFileHeader::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        self.header.insert("demo_file_stamp".to_string(), header.demo_file_stamp.to_string());
        self.header.insert("demo_version_guid".to_string(), header.demo_version_guid().to_string());
        self.header.insert("network_protocol".to_string(), header.network_protocol().to_string());
        self.header.insert("server_name".to_string(), header.server_name().to_string());
        self.header.insert("client_name".to_string(), header.client_name().to_string());
        self.header.insert("map_name".to_string(), header.map_name().to_string());
        self.header.insert("game_directory".to_string(), header.game_directory().to_string());
        self.header.insert("fullpackets_version".to_string(), header.fullpackets_version().to_string());
        self.header.insert("allow_clientside_entities".to_string(),header.allow_clientside_entities().to_string());
        self.header.insert("allow_clientside_particles".to_string(),header.allow_clientside_particles().to_string());
        self.header.insert("addons".to_string(), header.addons().to_string());
        self.header.insert("build_num".to_string(), header.build_num().to_string());
        self.header.insert("game".to_string(), header.game().to_string());
        self.header.insert("server_start_tick".to_string(), header.server_start_tick().to_string());
        self.header.insert("demo_version_name".to_string(), header.demo_version_name().to_string());
        Ok(())
    }

    fn handle_short_header(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let file_len = bytes.len();
        if file_len < HEADER_ENDS_AT_BYTE {
            return Err(DemoParserError::OutOfBytesError);
        }
        if let Ok(magic) = std::str::from_utf8(&bytes[..8]) {
            match magic {
                "PBDEMS2\0" => {}
                "HL2DEMO\0" => return Err(DemoParserError::Source1DemoError),
                _ => return Err(DemoParserError::UnknownFile)
            }
        };
        // hmmmm not sure where the 18 comes from if the header is only 16?
        // can be used to check that file ends early
        let file_length_expected = match bytes[8..12].try_into() {
            Err(_) => return Err(DemoParserError::OutOfBytesError),
            Ok(arr) => u32::from_le_bytes(arr) + 18,
        };
        let missing_percentage = 100.0 - (file_len as f32 / file_length_expected as f32 * 100.0);
        if missing_percentage > 10.0 {
            return Err(
                DemoParserError::DemoEndsEarly(format!(
                    "Demo ends early. Expected length: {}, actual length: {}. Missing: {:.2}%",
                    file_length_expected,
                    file_len,
                    100.0 - (file_len as f32 / file_length_expected as f32 * 100.0),
                ))
            );
        }
        // seems to be byte offset to where DEM_END command happens. After that comes Spawngroups and fileinfo. odd...
        let _no_clue_what_this_is = match bytes[8..12].try_into() {
            Err(_) => return Err(DemoParserError::OutOfBytesError),
            Ok(arr) => i32::from_le_bytes(arr),
        };
        Ok(())
    }

    fn parse_class_info(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let msg = CDemoClassInfo::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;

        let mut serializers = self.parse_sendtable()?;
        let mut serializer_by_cls_id = vec![Serializer::default(); msg.classes.len()];
        for class_t in msg.classes {
            let cls_id = class_t.class_id() as usize;
            let network_name = class_t.network_name();

            if let Some(serializer) = serializers.remove(network_name) {
                serializer_by_cls_id[cls_id] = serializer;
            }
        }
        self.serializer_by_cls_id = Some(serializer_by_cls_id);
        Ok(())
    }
}
