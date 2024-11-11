use parser::exports;
use parser::parse_demo::ParsingMode;
use parser::second_pass::parser_settings::create_huffman_lookup_table;
use parser::serde_helper::to_serde_output;
use parser::second_pass::variants::Variant;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
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

#[wasm_bindgen]
#[derive(Deserialize)]
pub struct WantedPropState {
    prop: String,
    #[serde(deserialize_with = "from_js")]
    state: Variant,
}

fn from_js<'de, D>(deserializer: D) -> Result<Variant, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    struct VariantVisitor;

    impl<'de> Visitor<'de> for VariantVisitor {
        type Value = Variant;
    
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Variant type of bool/string/number/bigint")
        }
    
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: Error { Ok(Variant::Bool(v)) }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error { Ok(Variant::String(v.to_owned())) }
        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: Error { Ok(Variant::F32(v)) }
        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: Error { Ok(Variant::I16(v)) }
        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: Error { Ok(Variant::I32(v)) }
        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: Error { Ok(Variant::U32(v)) }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: Error { Ok(Variant::U64(v)) }
        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: Error { Ok(Variant::U8(v)) }
    }

    deserializer.deserialize_any(VariantVisitor)
}

fn to_js_error<T>(e: T) -> JsError
where
    T: std::fmt::Display
{
    JsError::new(&format!("{e}"))
}

#[wasm_bindgen(js_name = parseEvent)]
pub fn parse_event(
    file: Vec<u8>,
    event_name: String,
    player_props: Option<Vec<String>>,
    other_props: Option<Vec<String>>,
) -> Result<JsValue, JsError> {
    let arc_huf = Arc::new(create_huffman_lookup_table());
    let game_events = exports::parse_event(&file.into(), &arc_huf, PARSING_MODE, event_name, player_props, other_props).map_err(to_js_error)?;

    serde_wasm_bindgen::to_value(&game_events).map_err(to_js_error)
}

#[wasm_bindgen(js_name = parseEvents)]
pub fn parse_events(
    file: Vec<u8>,
    event_names: Vec<String>,
    player_props: Option<Vec<String>>,
    other_props: Option<Vec<String>>,
) -> Result<JsValue, JsError> {
    let arc_huf = Arc::new(create_huffman_lookup_table());
    let game_events = exports::parse_events(&file.into(), &arc_huf, PARSING_MODE, event_names, player_props, other_props).map_err(to_js_error)?;

    serde_wasm_bindgen::to_value(&game_events).map_err(to_js_error)
}

#[wasm_bindgen(js_name = parseTicks)]
pub fn parse_ticks(
    file: Vec<u8>,
    wanted_props: Vec<String>,
    wanted_ticks: Option<Vec<i32>>,
    wanted_players: Option<Vec<String>>,
    struct_of_arrays: Option<bool>,
    order_by_steamid: Option<bool>,
    prop_states: Option<Vec<WantedPropState>>,
) -> Result<JsValue, JsError> {
    let wanted_players_u64 = Some(wanted_players.map_or_else(Vec::new, |v| v.iter().map(|x| x.parse::<u64>().unwrap_or(0)).collect()));
    let wanted_prop_states = HashMap::from_iter(prop_states.unwrap_or_default().into_iter().map(|prop| (prop.prop, prop.state)));
    let order_by_steamid = matches!(order_by_steamid, Some(true));

    let arc_huf = Arc::new(create_huffman_lookup_table());
    let output = exports::parse_ticks(
        &file.into(),
        &arc_huf,
        PARSING_MODE,
        wanted_props,
        wanted_ticks,
        wanted_players_u64,
        wanted_prop_states.into(),
        order_by_steamid
    ).map_err(to_js_error)?;

    let serde_output = to_serde_output(output, order_by_steamid, matches!(struct_of_arrays, Some(true)));
    serde_wasm_bindgen::to_value(&serde_output).map_err(to_js_error)
}

#[wasm_bindgen(js_name = listGameEvents)]
pub fn list_game_events(file: Vec<u8>) -> Result<JsValue, JsError> {
    let arc_huf = Arc::new(create_huffman_lookup_table());

    let game_events = exports::list_game_events(&file.into(), &arc_huf, PARSING_MODE).map_err(to_js_error)?;
    serde_wasm_bindgen::to_value(&game_events).map_err(to_js_error)
}

#[wasm_bindgen(js_name = parseGrenades)]
pub fn parse_grenades(file: Vec<u8>) -> Result<JsValue, JsError> {
    let arc_huf = Arc::new(create_huffman_lookup_table());
    let projectiles = exports::parse_grenades(&file.into(), &arc_huf, PARSING_MODE).map_err(to_js_error)?;

    serde_wasm_bindgen::to_value(&projectiles).map_err(to_js_error)
}

#[wasm_bindgen(js_name = parseHeader)]
pub fn parse_header(file: Vec<u8>) -> Result<JsValue, JsError> {
    let arc_huf = Arc::new(create_huffman_lookup_table());
    let header = exports::parse_header(&file.into(), &arc_huf, PARSING_MODE).map_err(to_js_error)?;

    serde_wasm_bindgen::to_value(&header).map_err(to_js_error)
}
