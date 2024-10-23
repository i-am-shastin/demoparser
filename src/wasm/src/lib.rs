use parser::first_pass::parser_settings::rm_user_friendly_names;
use parser::first_pass::parser_settings::ParserInputs;
use parser::parse_demo::Parser;
use parser::parse_demo::ParsingMode;
use parser::second_pass::parser_settings::create_huffman_lookup_table;
use parser::second_pass::variants::soa_to_aos;
use parser::second_pass::variants::OutputSerdeHelperStruct;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::result::Result;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[cfg(feature = "threads")]
pub use wasm_bindgen_rayon::init_thread_pool;

const PARSING_MODE: ParsingMode = if cfg!(feature = "threads") {
    ParsingMode::ForceRayonThreaded
} else {
    ParsingMode::ForceSingleThreaded
};

fn to_js_error<T>(e: T) -> JsError
where
    T: std::fmt::Display
{
    JsError::new(&format!("{}", e))
}

#[wasm_bindgen(js_name = parseEvent)]
pub fn parse_event(
    file: Vec<u8>,
    event_name: Option<String>,
    wanted_player_props: Option<Vec<String>>,
    wanted_other_props: Option<Vec<String>>,
) -> Result<JsValue, JsError> {
    let player_props = wanted_player_props.unwrap_or_default();
    let other_props = wanted_other_props.unwrap_or_default();
    let real_names_player = rm_user_friendly_names(&player_props).map_err(to_js_error)?;
    let real_other_props = rm_user_friendly_names(&other_props).map_err(to_js_error)?;

    let mut real_name_to_og_name = HashMap::default();
    for (real_name, user_friendly_name) in real_names_player.iter().zip(&player_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }
    for (real_name, user_friendly_name) in real_other_props.iter().zip(&other_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }

    let arc_huf = Arc::new(create_huffman_lookup_table());
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table: &arc_huf,
        only_convars: false,
        only_header: false,
        order_by_steamid: false,
        parse_ents: true,
        parse_projectiles: false,
        real_name_to_og_name: real_name_to_og_name.into(),
        wanted_events: vec![event_name.unwrap_or("none".to_string())],
        wanted_other_props: real_other_props,
        wanted_player_props: real_names_player,
        wanted_players: vec![],
        wanted_prop_states: HashMap::default().into(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, PARSING_MODE);
    let output = parser.parse_demo(&file).map_err(to_js_error)?;

    serde_wasm_bindgen::to_value(&output.game_events).map_err(to_js_error)
}

#[wasm_bindgen(js_name = parseEvents)]
pub fn parse_events(
    file: Vec<u8>,
    event_names: Option<Vec<String>>,
    wanted_player_props: Option<Vec<String>>,
    wanted_other_props: Option<Vec<String>>,
) -> Result<JsValue, JsError> {
    let event_names = event_names.unwrap_or_default();
    let player_props = wanted_player_props.unwrap_or_default();
    let other_props = wanted_other_props.unwrap_or_default();
    let real_names_player = rm_user_friendly_names(&player_props).map_err(to_js_error)?;
    let real_other_props = rm_user_friendly_names(&other_props).map_err(to_js_error)?;

    let mut real_name_to_og_name = HashMap::default();
    for (real_name, user_friendly_name) in real_names_player.iter().zip(&player_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }
    for (real_name, user_friendly_name) in real_other_props.iter().zip(&other_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }

    let arc_huf = Arc::new(create_huffman_lookup_table());
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table: &arc_huf,
        only_convars: false,
        only_header: false,
        order_by_steamid: false,
        parse_ents: true,
        parse_projectiles: false,
        real_name_to_og_name: real_name_to_og_name.into(),
        wanted_events: event_names,
        wanted_other_props: real_other_props,
        wanted_player_props: real_names_player,
        wanted_players: vec![],
        wanted_prop_states: HashMap::default().into(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, PARSING_MODE);
    let output = parser.parse_demo(&file).map_err(to_js_error)?;

    serde_wasm_bindgen::to_value(&output.game_events).map_err(to_js_error)
}

#[wasm_bindgen(js_name = listGameEvents)]
pub fn list_game_events(file: Vec<u8>) -> Result<JsValue, JsError> {
    let arc_huf = Arc::new(create_huffman_lookup_table());
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table: &arc_huf,
        only_convars: false,
        only_header: false,
        order_by_steamid: false,
        parse_ents: false,
        parse_projectiles: false,
        real_name_to_og_name: HashMap::default().into(),
        wanted_events: vec!["all".to_string()],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: HashMap::default().into(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, PARSING_MODE);
    let output = parser.parse_demo(&file).map_err(to_js_error)?;

    let game_events = Vec::from_iter(output.game_events_counter.iter());
    serde_wasm_bindgen::to_value(&game_events).map_err(to_js_error)
}

#[wasm_bindgen(js_name = parseTicks)]
pub fn parse_ticks(
    file: Vec<u8>,
    wanted_props: Option<Vec<String>>,
    wanted_ticks: Option<Vec<i32>>,
    wanted_players: Option<Vec<String>>,
    struct_of_arrays: Option<bool>,
) -> Result<JsValue, JsError> {
    let wanted_props = wanted_props.unwrap_or_default();
    let wanted_players_u64 = wanted_players.map_or_else(Vec::new, |v| v.iter().map(|x| x.parse::<u64>().unwrap_or(0)).collect());
    let real_names = rm_user_friendly_names(&wanted_props).map_err(to_js_error)?;

    let mut real_name_to_og_name = HashMap::default();
    for (real_name, user_friendly_name) in real_names.iter().zip(&wanted_props) {
        real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
    }

    let arc_huf = Arc::new(create_huffman_lookup_table());
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table: &arc_huf,
        only_convars: false,
        only_header: false,
        order_by_steamid: false,
        parse_ents: true,
        parse_projectiles: false,
        real_name_to_og_name: real_name_to_og_name.into(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: real_names.clone(),
        wanted_players: wanted_players_u64,
        wanted_prop_states: HashMap::default().into(),
        wanted_ticks: wanted_ticks.unwrap_or_default(),
    };
    let mut parser = Parser::new(settings, PARSING_MODE);
    let output = parser.parse_demo(&file).map_err(to_js_error)?;

    let mut prop_infos = output.prop_controller.prop_infos.clone();
    prop_infos.sort_by_key(|x| x.prop_name.clone());

    let helper = OutputSerdeHelperStruct {
        prop_infos,
        inner: output.df,
    };

    if matches!(struct_of_arrays, Some(true)) {
        serde_wasm_bindgen::to_value(&helper).map_err(to_js_error)
    } else {
        let aos_result = soa_to_aos(helper);
        serde_wasm_bindgen::to_value(&aos_result).map_err(to_js_error)
    }
}

#[wasm_bindgen(js_name = parseGrenades)]
pub fn parse_grenades(file: Vec<u8>) -> Result<JsValue, JsError> {
    let arc_huf = Arc::new(create_huffman_lookup_table());
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table: &arc_huf,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: true,
        parse_projectiles: true,
        real_name_to_og_name: HashMap::default().into(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: HashMap::default().into(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, PARSING_MODE);
    let output = parser.parse_demo(&file).map_err(to_js_error)?;

    let projectiles = Vec::from_iter(output.projectiles.iter());
    serde_wasm_bindgen::to_value(&projectiles).map_err(to_js_error)
}

#[wasm_bindgen(js_name = parseHeader)]
pub fn parse_header(file: Vec<u8>) -> Result<JsValue, JsError> {
    let arc_huf = Arc::new(create_huffman_lookup_table());
    let settings = ParserInputs {
        count_props: false,
        huffman_lookup_table: &arc_huf,
        only_convars: false,
        only_header: true,
        order_by_steamid: false,
        parse_ents: false,
        parse_projectiles: true,
        real_name_to_og_name: HashMap::default().into(),
        wanted_events: vec![],
        wanted_other_props: vec![],
        wanted_player_props: vec![],
        wanted_players: vec![],
        wanted_prop_states: HashMap::default().into(),
        wanted_ticks: vec![],
    };
    let mut parser = Parser::new(settings, PARSING_MODE);
    let output = parser.parse_demo(&file).map_err(to_js_error)?;

    let header: HashMap<String, String, _> = output.header.unwrap_or_default().into();
    serde_wasm_bindgen::to_value(&header).map_err(to_js_error)
}
