use crate::first_pass::prop_controller::PropController;
use crate::first_pass::sendtables::Serializer;
use crate::first_pass::stringtables::StringTable;
use crate::first_pass::stringtables::UserInfo;
use crate::second_pass::decoder::QfMapper;
use crate::second_pass::variants::Variant;
use ahash::AHashMap;
use csgoproto::CDemoPacket;
use csgoproto::CDemoSendTables;
use csgoproto::csvc_msg_game_event_list::DescriptorT;
use nohash::IntMap;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ParserInputs {
    pub only_convars: bool,
    pub only_header: bool,
    pub order_by_steamid: bool,
    pub parse_ents: bool,
    pub parse_projectiles: bool,
    pub real_name_to_og_name: AHashMap<String, String>,
    pub wanted_events: Vec<String>,
    pub wanted_other_props: Vec<String>,
    pub wanted_player_props: Vec<String>,
    pub wanted_players: Vec<u64>,
    pub wanted_prop_states: AHashMap<String, Variant>,
    pub wanted_ticks: Vec<i32>,
}

pub struct FirstPassParser<'a> {
    pub added_temp_props: Vec<String>,
    pub baselines: AHashMap<u32, Vec<u8>>,
    pub fullpackets: IntMap<usize, Option<CDemoPacket>>,
    pub ge_list: AHashMap<i32, DescriptorT>,
    pub header: AHashMap<String, String>,
    pub prop_controller: PropController,
    pub qf_mapper: QfMapper,
    pub sendtable_message: Option<CDemoSendTables>,
    pub serializer_by_cls_id: Option<Vec<Serializer>>,
    pub settings: &'a ParserInputs,
    pub string_tables: Vec<StringTable>,
    pub stringtable_players: BTreeMap<i32, UserInfo>,
    pub wanted_player_props: Vec<String>,
}

impl<'a> FirstPassParser<'a> {
    pub fn new(settings: &'a ParserInputs) -> Self {
        let mut wanted_player_props = settings.wanted_player_props.clone();
        let mut added_temp_props = Vec::new();

        let needs_velocity = needs_velocity(&wanted_player_props);
        if needs_velocity {
            for prop in ["X", "Y", "Z"].map(String::from) {
                if !wanted_player_props.contains(&prop) {
                    added_temp_props.push(prop);
                }
            }
            wanted_player_props.extend(added_temp_props.clone());
        }

        FirstPassParser {
            added_temp_props,
            baselines: AHashMap::default(),
            fullpackets: IntMap::default(),
            ge_list: AHashMap::default(),
            header: AHashMap::default(),
            prop_controller: PropController::new(
                wanted_player_props.clone(),
                settings.wanted_other_props.clone(),
                settings.wanted_prop_states.clone(),
                settings.real_name_to_og_name.clone(),
                needs_velocity,
            ),
            qf_mapper: QfMapper {
                idx: 0,
                map: AHashMap::default(),
            },
            sendtable_message: None,
            serializer_by_cls_id: None,
            settings,
            string_tables: vec![],
            stringtable_players: BTreeMap::default(),
            wanted_player_props,
        }
    }
}

pub fn needs_velocity(props: &[String]) -> bool {
    props.iter().any(|p| p.contains("velo"))
}
