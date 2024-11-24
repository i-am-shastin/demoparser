use crate::definitions::DemoParserError;
use crate::definitions::INNER_BUF_DEFAULT_LEN;
use crate::definitions::OUTER_BUF_DEFAULT_LEN;
use crate::first_pass::frameparser::FrameParser;
use crate::first_pass::prop_controller::*;
use crate::first_pass::read_bits::Bitreader;
use crate::second_pass::collect_data::ProjectileRecord;
use crate::second_pass::game_events::GameEvent;
use crate::second_pass::parser_settings::SecondPassParser;
use crate::second_pass::parser_settings::*;
use crate::second_pass::variants::InputHistory;
use crate::second_pass::variants::PropColumn;
use crate::second_pass::variants::Variant;
use ahash::AHashMap;
use ahash::AHashSet;
use csgoproto::message_type::NetMessageType;
use csgoproto::message_type::NetMessageType::*;
use csgoproto::CDemoPacket;
use csgoproto::CsvcMsgServerInfo;
use csgoproto::CsvcMsgUserCommands;
use csgoproto::CsvcMsgVoiceData;
use csgoproto::CsgoUserCmdPb;
use csgoproto::CnetMsgTick;
use csgoproto::EDemoCommands::*;
use prost::Message;

#[derive(Debug)]
pub struct SecondPassOutput {
    pub prop_data_per_player: AHashMap<u64, AHashMap<u32, PropColumn>>,
    pub prop_data: AHashMap<u32, PropColumn>,
    pub game_events_counter: AHashSet<String>,
    pub game_events: Vec<GameEvent>,
    pub item_drops: Vec<EconItem>,
    pub player_md: Vec<PlayerEndMetaData>,
    pub projectiles: Vec<ProjectileRecord>,
    pub ptr: usize,
    pub skins: Vec<EconItem>,
    pub voice_data: Vec<CsvcMsgVoiceData>,
}

impl<'a> SecondPassParser<'a> {
    pub fn start(&mut self, demo_bytes: &'a [u8]) -> Result<(), DemoParserError> {
        let upper_bound = self.demo_chunk.map_or_else(|| demo_bytes.len(), |chunk| chunk.end);

        // re-use these to avoid allocation
        let mut buf = vec![0_u8; INNER_BUF_DEFAULT_LEN];
        let mut buf2 = vec![0_u8; OUTER_BUF_DEFAULT_LEN];
        loop {
            let frame = FrameParser::read_frame(demo_bytes, &mut self.ptr)?;
            self.tick = frame.tick;
            if frame.demo_cmd == DemStop {
                break;
            }

            // Skip reading/decompressing frame if we have nothing to do with it
            if !matches!(frame.demo_cmd, DemPacket | DemSignonPacket | DemFullPacket) {
                continue;
            }

            match frame.demo_cmd {
                DemPacket | DemSignonPacket => {
                    let bytes = frame.get_bytes(&mut buf, demo_bytes)?;
                    self.parse_packet(bytes, &mut buf2)?
                },
                DemFullPacket => self.parse_full_packet(frame.starts_at, &mut buf2)?,
                _ => {},
            };

            if self.ptr >= upper_bound {
                // Parsed to the end of chunk or entire demo
                break;
            }
        }
        Ok(())
    }

    pub fn create_output(self) -> SecondPassOutput {
        SecondPassOutput {
            prop_data_per_player: self.prop_data_per_player,
            prop_data: self.prop_data,
            game_events_counter: self.game_events_counter,
            game_events: self.game_events,
            item_drops: self.item_drops,
            player_md: self.player_end_data,
            projectiles: self.projectile_records,
            ptr: self.ptr,
            skins: self.skins,
            voice_data: self.voice_data,
        }
    }

    #[inline(always)]
    fn parse_packet(&mut self, bytes: &[u8], buf: &mut Vec<u8>) -> Result<(), DemoParserError> {
        let msg = CDemoPacket::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        let mut bitreader = Bitreader::new(msg.data());
        self.parse_packet_from_bitreader(&mut bitreader, buf, false)
    }

    fn parse_packet_from_bitreader(
        &mut self,
        bitreader: &mut Bitreader,
        buf: &mut Vec<u8>,
        is_fullpacket: bool,
    ) -> Result<(), DemoParserError> {
        let mut wrong_order_events = vec![];

        while bitreader.bits_remaining().unwrap_or(0) > 8 {
            let msg_type = bitreader.read_u_bit_var()? as i32;
            let size = bitreader.read_varint()? as usize;
            if buf.len() < size {
                buf.resize(size, 0)
            }
            bitreader.read_n_bytes_mut(size, buf)?;
            let msg_bytes = &buf[..size];

            match NetMessageType::from(msg_type) {
                svc_PacketEntities => {
                    if self.settings.parse_ents {
                        self.parse_packet_ents(msg_bytes, is_fullpacket)?;
                    }
                    if !is_fullpacket {
                        self.collect_entities();
                    }
                    Ok(())
                },
                svc_ServerInfo => self.parse_server_info(msg_bytes),
                svc_VoiceData => self.parse_voice_data(msg_bytes),
                // This method is quite expensive so call it only if needed.
                svc_UserCmds if self.parse_usercmd => self.parse_user_cmd(msg_bytes),

                net_Tick => self.parse_net_tick(msg_bytes),
                net_SetConVar => self.create_custom_event_parse_convars(msg_bytes),

                CS_UM_SendPlayerItemDrops => self.parse_item_drops(msg_bytes),
                CS_UM_EndOfMatchAllPlayersData => self.parse_player_end_msg(msg_bytes),
                CS_UM_PlayerStatsUpdate => self.parse_player_stats_update(msg_bytes),
                CS_UM_ServerRankUpdate => self.create_custom_event_rank_update(msg_bytes),

                UM_SayText2 => self.create_custom_event_chat_message(msg_bytes),
                UM_SayText => self.create_custom_event_server_message(msg_bytes),

                GE_Source1LegacyGameEvent if self.has_wanted_events => self.parse_game_event(msg_bytes, &mut wrong_order_events),
                _ => Ok(()),
            }?
        }

        if !wrong_order_events.is_empty() {
            self.resolve_wrong_order_event(wrong_order_events)?;
        }
        Ok(())
    }

    fn parse_user_cmd(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        // We simply inject the values into the entities as if they came from packet_ents like any other val.
        // TODO: throw instead of silent return when decoding fails?
        let msg = match CsvcMsgUserCommands::decode(bytes) {
            Ok(m) => m,
            _ => return Ok(()),
        };
    
        for cmd in msg.commands {
            let user_cmd = match CsgoUserCmdPb::decode(cmd.data()) {
                Ok(m) => m,
                _ => return Ok(()),
            };
            let Some(base) = user_cmd.base else { continue };
            let entity_id = base.pawn_entity_handle() & 0x7FF;
            let Some(Some(ent)) = self.entities.get_mut(entity_id as usize) else { continue };

            let input_history = user_cmd.input_history
                .into_iter()
                .map(|input| {
                    InputHistory {
                        player_tick_count: input.player_tick_count(),
                        player_tick_fraction: input.player_tick_fraction(),
                        render_tick_count: input.render_tick_count(),
                        render_tick_fraction: input.render_tick_fraction(),
                        x: input.view_angles.and_then(|v| v.x),
                        y: input.view_angles.and_then(|v| v.y),
                        z: input.view_angles.and_then(|v| v.z),
                    }
                })
                .collect();
            ent.props.insert(USERCMD_INPUT_HISTORY_BASEID, Variant::InputHistory(input_history));
            ent.props.insert(USERCMD_LEFTMOVE, Variant::F32(base.leftmove()));
            ent.props.insert(USERCMD_FORWARDMOVE, Variant::F32(base.forwardmove()));
            ent.props.insert(USERCMD_IMPULSE, Variant::I32(base.impulse()));
            ent.props.insert(USERCMD_MOUSE_DX, Variant::I32(base.mousedx()));
            ent.props.insert(USERCMD_MOUSE_DY, Variant::I32(base.mousedy()));
            ent.props.insert(USERCMD_CONSUMED_SERVER_ANGLE_CHANGES, Variant::U32(base.consumed_server_angle_changes()));

            if let Some(viewangles) = base.viewangles {
                ent.props.insert(USERCMD_VIEWANGLE_X, Variant::F32(viewangles.x()));
                ent.props.insert(USERCMD_VIEWANGLE_Y, Variant::F32(viewangles.y()));
                ent.props.insert(USERCMD_VIEWANGLE_Z, Variant::F32(viewangles.z()));
            }
            if let Some(buttons_pb) = base.buttons_pb {
                ent.props.insert(USERCMD_BUTTONSTATE_1, Variant::U64(buttons_pb.buttonstate1()));
                ent.props.insert(USERCMD_BUTTONSTATE_2, Variant::U64(buttons_pb.buttonstate2()));
                ent.props.insert(USERCMD_BUTTONSTATE_3, Variant::U64(buttons_pb.buttonstate3()));
            }
        }
        Ok(())
    }

    fn parse_voice_data(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let data = CsvcMsgVoiceData::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        self.voice_data.push(data);
        Ok(())
    }

    fn parse_game_event(&mut self, bytes: &[u8], wrong_order_events: &mut Vec<GameEvent>) -> Result<(), DemoParserError> {
        self.parse_event(bytes).map(|e| {
            if let Some(event) = e {
                wrong_order_events.push(event)
            }
        })
    }

    fn parse_net_tick(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let message = CnetMsgTick::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        self.net_tick = message.tick() as f32;
        Ok(())
    }

    fn parse_full_packet(&mut self, offset: usize, buf: &mut Vec<u8>) -> Result<(), DemoParserError> {
        let packet = self.fullpackets.get(&offset).ok_or_else(|| DemoParserError::MalformedMessage)?;
        match packet {
            Some(packet) => {
                let mut bitreader = Bitreader::new(packet.data());
                self.parse_packet_from_bitreader(&mut bitreader, buf, true)
            }
            None => Ok(()),
        }
    }

    fn parse_server_info(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let _server_info = CsvcMsgServerInfo::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        // let class_count = server_info.max_classes() as f32;
        // self.cls_bits = Some((class_count + 1.).log2().ceil() as u32);
        Ok(())
    }

    #[allow(dead_code)]
    fn parse_user_command_cmd(&mut self, _bytes: &[u8]) -> Result<(), DemoParserError> {
        // Only in pov demos. Maybe implement sometime. Includes buttons etc.
        Ok(())
    }
}
