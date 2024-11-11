use std::collections::HashMap;
use std::fs::File;

use ahash::AHashMap;
use memmap2::MmapOptions;
use crate::definitions::DemoParserError;
use crate::first_pass::parser_settings::ParserInputs;
use crate::maps::FRIENDLY_NAMES_MAPPING;
use crate::parse_demo::DemoOutput;
use crate::parse_demo::Parser;
use crate::parse_demo::ParsingMode;
use crate::second_pass::collect_data::ProjectileRecord;
use crate::second_pass::game_events::GameEvent;
use crate::second_pass::parser_settings::EconItem;
use crate::second_pass::parser_settings::PlayerEndMetaData;
use crate::second_pass::variants::BytesVariant;
use crate::second_pass::variants::Variant;
#[cfg(feature = "voice")]
use crate::second_pass::voice_data::convert_voice_data_to_wav;

pub fn create_mmap(path: &String) -> Result<BytesVariant, DemoParserError> {
    let file = File::open(path).map_err(|e| DemoParserError::FileNotFound(format!("{e}")))?;
    unsafe { MmapOptions::new().map(&file) }.map_err(|e| DemoParserError::FileNotFound(format!("{e}"))).map(BytesVariant::Mmap)
}

#[inline(always)]
fn parse_demo(bytes: &BytesVariant, parser: &mut Parser) -> Result<DemoOutput, DemoParserError> {
    match bytes {
        BytesVariant::Mmap(m) => parser.parse_demo(m),
        BytesVariant::Vec(v) => parser.parse_demo(v),
    }
}

fn get_friendly_name(name: &String) -> Result<String, DemoParserError> {
    FRIENDLY_NAMES_MAPPING.get(name).map(|s| s.to_string()).ok_or_else(|| DemoParserError::UnknownPropName(name.to_string()))
}

fn rm_user_friendly_names(names: &[String]) -> Result<Vec<String>, DemoParserError> {
    names.iter().map(get_friendly_name).collect()
}

fn rm_map_user_friendly_names(names_hm: &AHashMap<String, Variant>) -> Result<AHashMap<String, Variant>, DemoParserError> {
    names_hm
        .iter()
        .map(|(name, variant)| {
            Ok((get_friendly_name(name)?, variant.clone()))
        })
        .collect()
}

pub fn parse_player_skins(bytes: &BytesVariant, huffman_lookup_table: &Vec<(u8, u8)>, parsing_mode: ParsingMode) -> Result<Vec<EconItem>, DemoParserError> {
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: true,
        parse_projectiles: false,
        real_name_to_og_name: AHashMap::default(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    Ok(output.skins)
}

pub fn parse_grenades(bytes: &BytesVariant, huffman_lookup_table: &Vec<(u8, u8)>, parsing_mode: ParsingMode) -> Result<Vec<ProjectileRecord>, DemoParserError> {
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: true,
        parse_projectiles: true,
        real_name_to_og_name: AHashMap::default(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    Ok(output.projectiles)
}

pub fn parse_header(bytes: &BytesVariant, huffman_lookup_table: &Vec<(u8, u8)>, parsing_mode: ParsingMode) -> Result<HashMap<String, String, ahash::RandomState>, DemoParserError> {
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: false,
        parse_projectiles: false,
        real_name_to_og_name: AHashMap::default(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    Ok(output.header.unwrap_or_default().into())
}

pub fn list_game_events(bytes: &BytesVariant, huffman_lookup_table: &Vec<(u8, u8)>, parsing_mode: ParsingMode) -> Result<Vec<String>, DemoParserError> {
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: false,
        order_by_steamid: false,
        parse_ents: false,
        parse_projectiles: false,
        real_name_to_og_name: AHashMap::default(),
        wanted_events: vec!["all".to_string()],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    Ok(Vec::from_iter(output.game_events_counter))
}

pub fn parse_event(
    bytes: &BytesVariant,
    huffman_lookup_table: &Vec<(u8, u8)>,
    parsing_mode: ParsingMode,
    event_name: String,
    player_props: Option<Vec<String>>,
    other_props: Option<Vec<String>>
) -> Result<Vec<GameEvent>, DemoParserError> {
    let player_props = player_props.unwrap_or_default();
    let other_props = other_props.unwrap_or_default();
    let real_names_player = rm_user_friendly_names(&player_props)?;
    let real_other_props = rm_user_friendly_names(&other_props)?;

    let mut real_name_to_og_name = AHashMap::with_capacity(real_names_player.len() + real_other_props.len());
    for (real_name, user_friendly_name) in real_names_player.iter().zip(&player_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }
    for (real_name, user_friendly_name) in real_other_props.iter().zip(&other_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }

    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: true,
        parse_projectiles: false,
        real_name_to_og_name,
        wanted_events: vec![event_name],
        wanted_other_props: real_other_props,
        wanted_player_props: real_names_player,
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    Ok(output.game_events)
}

pub fn parse_events(
    bytes: &BytesVariant,
    huffman_lookup_table: &Vec<(u8, u8)>,
    parsing_mode: ParsingMode,
    wanted_events: Vec<String>,
    player_props: Option<Vec<String>>,
    other_props: Option<Vec<String>>,
) -> Result<Vec<GameEvent>, DemoParserError> {
    let player_props = player_props.unwrap_or_default();
    let other_props = other_props.unwrap_or_default();
    let real_names_player = rm_user_friendly_names(&player_props)?;
    let real_other_props = rm_user_friendly_names(&other_props)?;

    let mut real_name_to_og_name = AHashMap::with_capacity(real_names_player.len() + real_other_props.len());
    for (real_name, user_friendly_name) in real_names_player.iter().zip(&player_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }
    for (real_name, user_friendly_name) in real_other_props.iter().zip(&other_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }

    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: true,
        parse_projectiles: false,
        real_name_to_og_name,
        wanted_events,
        wanted_other_props: real_other_props,
        wanted_player_props: real_names_player,
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    Ok(output.game_events)
}

pub fn parse_player_info(bytes: &BytesVariant, huffman_lookup_table: &Vec<(u8, u8)>, parsing_mode: ParsingMode) -> Result<Vec<PlayerEndMetaData>, DemoParserError> {
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: false,
        parse_projectiles: false,
        real_name_to_og_name: AHashMap::default(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    Ok(output.player_md)
}

#[cfg(feature = "voice")]
pub fn parse_voice(bytes: &BytesVariant, parsing_mode: ParsingMode) -> Result<Vec<(String, Vec<u8>)>, DemoParserError> {
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table: &vec![],
        only_convars: false,
        only_header: false,
        order_by_steamid: false,
        parse_ents: false,
        parse_projectiles: false,
        real_name_to_og_name: AHashMap::default(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    convert_voice_data_to_wav(output.voice_data)
}

pub fn parse_item_drops(bytes: &BytesVariant, huffman_lookup_table: &Vec<(u8, u8)>, parsing_mode: ParsingMode) -> Result<Vec<EconItem>, DemoParserError> {
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: false,
        parse_projectiles: false,
        real_name_to_og_name: AHashMap::default(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, parsing_mode);
    let output = parse_demo(bytes, &mut parser)?;

    Ok(output.item_drops)
}

#[allow(clippy::too_many_arguments)]
pub fn parse_ticks(
    bytes: &BytesVariant,
    huffman_lookup_table: &Vec<(u8, u8)>,
    parsing_mode: ParsingMode,
    wanted_props: Vec<String>,
    wanted_ticks: Option<Vec<i32>>,
    wanted_players: Option<Vec<u64>>,
    wanted_prop_states: AHashMap<String, Variant>,
    order_by_steamid: bool,
) -> Result<DemoOutput, DemoParserError> {
    let real_names = rm_user_friendly_names(&wanted_props)?;
    let real_wanted_prop_states = rm_map_user_friendly_names(&wanted_prop_states)?;

    let mut real_name_to_og_name = AHashMap::with_capacity(real_names.len() + real_wanted_prop_states.len());
    for (real_name, user_friendly_name) in real_names.iter().zip(&wanted_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }
    for (real_name, user_friendly_name) in real_wanted_prop_states.keys().zip(wanted_prop_states.keys()) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }

    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table,
        only_convars: false,
        only_header: false,
        order_by_steamid,
        parse_ents: true,
        parse_projectiles: false,
        real_name_to_og_name,
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: real_names,
        wanted_players: wanted_players.unwrap_or_default(),
        wanted_prop_states: real_wanted_prop_states,
        wanted_ticks: wanted_ticks.unwrap_or_default(),
    };
    let mut parser = Parser::new(settings, parsing_mode);
    parse_demo(bytes, &mut parser)
}
