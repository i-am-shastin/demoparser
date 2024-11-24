use crate::definitions::DemoParserError;
use crate::definitions::HEADER_ENDS_AT_BYTE;
use crate::first_pass::frameparser::DemoChunk;
use crate::first_pass::parser::FirstPassOutput;
use crate::first_pass::parser_settings::ParserInputs;
use crate::first_pass::prop_controller::PropController;
use crate::first_pass::sendtables::Serializer;
use crate::first_pass::stringtables::UserInfo;
use crate::second_pass::collect_data::ProjectileRecord;
use crate::second_pass::decoder::QfMapper;
use crate::second_pass::entities::Entity;
use crate::second_pass::entities::PlayerMetaData;
use crate::second_pass::game_events::GameEvent;
use crate::second_pass::path_ops::FieldPath;
use crate::second_pass::variants::PropColumn;
use crate::serde_helper::as_string;
use ahash::AHashMap;
use ahash::AHashSet;
use csgoproto::csvc_msg_game_event_list::DescriptorT;
use csgoproto::CDemoPacket;
use csgoproto::CsvcMsgVoiceData;
use nohash::IntMap;
use nohash::IntSet;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;

const DEFAULT_MAX_ENTITY_ID: usize = 1024;

pub struct SecondPassParser<'a> {
    pub baselines: &'a AHashMap<u32, Vec<u8>>,
    pub c4_entity_id: Option<i32>,
    pub demo_chunk: Option<DemoChunk>,
    pub entities: Vec<Option<Entity>>,
    pub game_events_counter: AHashSet<String>,
    pub game_events: Vec<GameEvent>,
    pub ge_list: &'a AHashMap<i32, DescriptorT>,
    pub net_tick: f32,
    pub paths: Vec<FieldPath>,
    pub players: BTreeMap<i32, PlayerMetaData>,
    pub projectiles: BTreeSet<i32>,
    pub prop_controller: &'a PropController,
    pub ptr: usize,
    pub qf_mapper: &'a QfMapper,
    pub rules_entity_id: Option<i32>,
    pub serializer_by_cls_id: &'a Vec<Serializer>,
    pub stringtable_players: &'a BTreeMap<i32, UserInfo>,
    pub fullpackets: &'a IntMap<usize, Option<CDemoPacket>>,
    pub teams: Teams,
    pub tick: i32,
    // Output from parsing
    pub prop_data_per_player: AHashMap<u64, AHashMap<u32, PropColumn>>,
    pub item_drops: Vec<EconItem>,
    pub prop_data: AHashMap<u32, PropColumn>,
    pub player_end_data: Vec<PlayerEndMetaData>,
    pub projectile_records: Vec<ProjectileRecord>,
    pub skins: Vec<EconItem>,
    pub voice_data: Vec<CsvcMsgVoiceData>,
    pub wanted_players: &'a IntSet<u64>,
    pub wanted_ticks: &'a IntSet<i32>,
    // Settings
    pub is_debug_mode: bool,
    pub parse_usercmd: bool,
    pub settings: &'a ParserInputs,
    pub has_wanted_events: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Teams {
    pub team1_entid: Option<i32>,
    pub team2_entid: Option<i32>,
    pub team3_entid: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct EconItem {
    pub account_id: Option<u32>,
    pub item_id: Option<u64>,
    pub def_index: Option<u32>,
    pub paint_index: Option<u32>,
    pub rarity: Option<u32>,
    pub quality: Option<u32>,
    pub paint_wear: Option<u32>,
    pub paint_seed: Option<u32>,
    pub quest_id: Option<u32>,
    pub dropreason: Option<u32>,
    pub custom_name: Option<String>,
    pub inventory: Option<u32>,
    pub ent_idx: Option<i32>,
    #[serde(serialize_with = "as_string")]
    pub steamid: Option<u64>,
    pub item_name: Option<String>,
    pub skin_name: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PlayerEndMetaData {
    #[serde(serialize_with = "as_string")]
    pub steamid: Option<u64>,
    pub name: Option<String>,
    pub team_number: Option<i32>,
}

impl<'a> SecondPassParser<'a> {
    pub fn new(
        settings: &'a ParserInputs,
        first_pass_output: &'a FirstPassOutput,
        demo_chunk: Option<DemoChunk>,
    ) -> Result<Self, DemoParserError> {
        let args: Vec<String> = env::args().collect();
        let debug = if args.len() > 2 { args[2] == "true" } else { false };

        let ptr = demo_chunk.map(|c| c.start).unwrap_or(HEADER_ENDS_AT_BYTE);

        Ok(SecondPassParser {
            settings,
            parse_usercmd: contains_usercmd_prop(&settings.wanted_player_props),
            demo_chunk,
            prop_data_per_player: AHashMap::default(),
            voice_data: vec![],
            paths: vec![
                FieldPath {
                    last: 0,
                    path: [0, 0, 0, 0, 0, 0, 0],
                };
                8192
            ],
            net_tick: 0.0,
            c4_entity_id: None,
            stringtable_players: &first_pass_output.stringtable_players,
            is_debug_mode: debug,
            projectile_records: vec![],
            prop_controller: &first_pass_output.prop_controller,
            qf_mapper: &first_pass_output.qfmap,
            ptr,
            ge_list: &first_pass_output.ge_list,
            serializer_by_cls_id: &first_pass_output.serializer_by_cls_id,
            entities: vec![None; DEFAULT_MAX_ENTITY_ID],
            tick: -99999,
            players: BTreeMap::default(),
            prop_data: AHashMap::default(),
            game_events: vec![],
            projectiles: BTreeSet::default(),
            baselines: &first_pass_output.baselines,
            fullpackets: &first_pass_output.fullpackets,
            teams: Teams::default(),
            game_events_counter: AHashSet::default(),
            rules_entity_id: None,
            item_drops: vec![],
            skins: vec![],
            player_end_data: vec![],
            has_wanted_events: !settings.wanted_events.is_empty(),
            wanted_players: &first_pass_output.wanted_players,
            wanted_ticks: &first_pass_output.wanted_ticks,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct SpecialIDs {
    pub teamnum: Option<u32>,
    pub player_name: Option<u32>,
    pub steamid: Option<u32>,
    pub player_pawn: Option<u32>,

    pub player_team_pointer: Option<u32>,
    pub weapon_owner_pointer: Option<u32>,
    pub team_team_num: Option<u32>,

    pub cell_x_player: Option<u32>,
    pub cell_y_player: Option<u32>,
    pub cell_z_player: Option<u32>,

    pub cell_x_offset_player: Option<u32>,
    pub cell_y_offset_player: Option<u32>,
    pub cell_z_offset_player: Option<u32>,
    pub active_weapon: Option<u32>,
    pub item_def: Option<u32>,

    pub cell_x_offset_grenade: Option<u32>,
    pub cell_y_offset_grenade: Option<u32>,
    pub cell_z_offset_grenade: Option<u32>,

    pub cell_x_grenade: Option<u32>,
    pub cell_y_grenade: Option<u32>,
    pub cell_z_grenade: Option<u32>,

    pub grenade_owner_id: Option<u32>,
    pub buttons: Option<u32>,
    pub eye_angles: Option<u32>,

    pub orig_own_low: Option<u32>,
    pub orig_own_high: Option<u32>,
    pub life_state: Option<u32>,

    pub h_owner_entity: Option<u32>,
    pub agent_skin_idx: Option<u32>,
    pub total_rounds_played: Option<u32>,

    pub round_win_reason: Option<u32>,
    pub round_start_count: Option<u32>,
    pub round_end_count: Option<u32>,
    pub match_end_count: Option<u32>,

    pub is_incendiary_grenade: Option<u32>,
    pub sellback_entry_def_idx: Option<u32>,
    pub sellback_entry_n_cost: Option<u32>,
    pub sellback_entry_prev_armor: Option<u32>,
    pub sellback_entry_prev_helmet: Option<u32>,
    pub sellback_entry_h_item: Option<u32>,

    pub weapon_purchase_count: Option<u32>,
    pub in_buy_zone: Option<u32>,
    pub custom_name: Option<u32>,

    pub is_airborne: Option<u32>,
}

fn contains_usercmd_prop(names: &[String]) -> bool {
    names.iter().any(|name| name.contains("usercmd"))
}
