#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use napi::bindgen_prelude::*;
use napi::Either;
use napi::JsBigInt;
use napi::JsUnknown;
use parser::exports;
use parser::second_pass::parser_settings::create_huffman_lookup_table;
use parser::second_pass::variants::to_serde_output;
use parser::second_pass::variants::BytesVariant;
use parser::second_pass::variants::Variant;
use serde_json::Value;
use std::collections::HashMap;
use std::result::Result;

#[napi]
#[derive(Clone)]
pub struct JsVariant(Variant);

impl FromNapiValue for JsVariant {
    unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> napi::Result<Self> {
        let js_unknown = JsUnknown::from_napi_value(env, napi_val)?;
        let js_unknown_type = js_unknown.get_type().map_err(|_| to_napi_err("Unsupported type for Variant"))?;

        match js_unknown_type {
            ValueType::Boolean => {
                let js_bool = js_unknown.coerce_to_bool().map_err(|_| to_napi_err("Unsupported Boolean for Variant"))?;
                Ok(JsVariant(Variant::Bool(js_bool.get_value()?)))
            },
            ValueType::String => {
                let js_string = js_unknown.coerce_to_string().map_err(|_| to_napi_err("Unsupported String for Variant"))?;
                Ok(JsVariant(Variant::String(js_string.into_utf8()?.into_owned()?)))
            },
            ValueType::Number => {
                let js_number = js_unknown.coerce_to_number().map_err(|_| to_napi_err("Unsupported Number for Variant"))?;
                let val_f64 = js_number.get_double()?;
                if val_f64.fract() != 0.0 {
                    return Ok(JsVariant(Variant::F32(val_f64 as f32)));
                }
                if val_f64 >= u8::MIN as f64 && val_f64 <= u8::MAX as f64 {
                    return Ok(JsVariant(Variant::U8(val_f64 as u8)));
                }
                if let Ok(val_i32) = js_number.get_int32() {
                    if val_i32 >= i16::MIN as i32 && val_i32 <= i16::MAX as i32 {
                        return Ok(JsVariant(Variant::I16(val_i32 as i16)));
                    }
                    return Ok(JsVariant(Variant::I32(val_i32)));
                }
                if let Ok(val_u32) = js_number.get_uint32() {
                   return Ok(JsVariant(Variant::U32(val_u32)))
                }
                Err(to_napi_err("Unsupported number type"))
            },
            ValueType::BigInt => {
                let js_bigint = js_unknown.cast::<JsBigInt>();
                let js_bigint_u64 = js_bigint.get_u64();
                match js_bigint_u64 {
                    Ok((val, true)) => Ok(JsVariant(Variant::U64(val))),
                    _ => Err(to_napi_err("Unsupported BigInt for Variant")),
                }
            },
            _ => Err(to_napi_err("Unsupported JS type"))
        }
    }
}

#[napi]
pub struct WantedPropState {
    pub prop: String,
    pub state: JsVariant,
}

impl FromNapiValue for WantedPropState {
    unsafe fn from_napi_value(
        env: sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> napi::Result<Self> {
        let obj: Object = Object::from_napi_value(env, napi_val)?;

        let prop: String = obj.get_named_property("prop")?;
        let state: JsVariant = obj.get_named_property("state")?;

        Ok(WantedPropState { prop, state })
    }
}

fn to_napi_err<T>(e: T) -> Error
where
    T: std::fmt::Display
{
    Error::new(Status::InvalidArg, format!("{e}"))
}

#[napi]
pub fn parse_voice(path_or_buf: Either<String, Buffer>) -> napi::Result<HashMap<String, Vec<u8>>> {
    let bytes = resolve_byte_type(path_or_buf)?;

    let voice_data_wav = exports::parse_voice(&bytes, parser::parse_demo::ParsingMode::Normal).map_err(to_napi_err)?;
    Ok(HashMap::from_iter(voice_data_wav))
}

#[napi]
pub fn list_game_events(path_or_buf: Either<String, Buffer>) -> napi::Result<Value> {
    let bytes = resolve_byte_type(path_or_buf)?;
    let huf = create_huffman_lookup_table();

    let game_events = exports::list_game_events(&bytes, &huf, parser::parse_demo::ParsingMode::Normal).map_err(to_napi_err)?;
    serde_json::to_value(game_events).map_err(to_napi_err)
}

#[napi]
pub fn parse_grenades(path_or_buf: Either<String, Buffer>) -> napi::Result<Value> {
    let bytes = resolve_byte_type(path_or_buf)?;
    let huf = create_huffman_lookup_table();
    
    let projectiles = exports::parse_grenades(&bytes, &huf, parser::parse_demo::ParsingMode::Normal).map_err(to_napi_err)?;
    serde_json::to_value(&projectiles).map_err(to_napi_err)
}

#[napi]
pub fn parse_header(path_or_buf: Either<String, Buffer>) -> napi::Result<Value> {
    let bytes = resolve_byte_type(path_or_buf)?;
    let huf = create_huffman_lookup_table();

    let header = exports::parse_header(&bytes, &huf, parser::parse_demo::ParsingMode::Normal).map_err(to_napi_err)?;
    serde_json::to_value(&header).map_err(to_napi_err)
}

#[napi]
pub fn parse_event(
    path_or_buf: Either<String, Buffer>,
    event_name: String,
    player_props: Option<Vec<String>>,
    other_props: Option<Vec<String>>,
) -> napi::Result<Value> {
    let bytes = resolve_byte_type(path_or_buf)?;
    let huf = create_huffman_lookup_table();
    
    let game_events = exports::parse_event(&bytes, &huf, parser::parse_demo::ParsingMode::Normal, event_name, player_props, other_props).map_err(to_napi_err)?;
    serde_json::to_value(&game_events).map_err(to_napi_err)
}

#[napi]
pub fn parse_events(
    path_or_buf: Either<String, Buffer>,
    event_names: Vec<String>,
    player_props: Option<Vec<String>>,
    other_props: Option<Vec<String>>,
) -> napi::Result<Value> {
    let bytes = resolve_byte_type(path_or_buf)?;
    let huf = create_huffman_lookup_table();

    let game_events = exports::parse_events(&bytes, &huf, parser::parse_demo::ParsingMode::Normal, event_names, player_props, other_props).map_err(to_napi_err)?;
    serde_json::to_value(&game_events).map_err(to_napi_err)
}

#[napi]
pub fn parse_ticks(
    path_or_buf: Either<String, Buffer>,
    wanted_props: Vec<String>,
    wanted_ticks: Option<Vec<i32>>,
    wanted_players: Option<Vec<String>>,
    struct_of_arrays: Option<bool>,
    order_by_steamid: Option<bool>,
    prop_states: Option<Vec<WantedPropState>>,
) -> napi::Result<Value> {
    let wanted_players_u64 = Some(wanted_players.map_or_else(Vec::new, |v| v.iter().map(|x| x.parse::<u64>().unwrap_or(0)).collect()));
    let wanted_prop_states = prop_states.unwrap_or_default().into_iter().map(|prop| (prop.prop, prop.state.0)).collect();
    let order_by_steamid = matches!(order_by_steamid, Some(true));

    let bytes = resolve_byte_type(path_or_buf)?;
    let huf = create_huffman_lookup_table();

    let output = exports::parse_ticks(
        &bytes,
        &huf,
        parser::parse_demo::ParsingMode::Normal,
        wanted_props,
        wanted_ticks,
        wanted_players_u64,
        wanted_prop_states,
        order_by_steamid
    ).map_err(to_napi_err)?;

    let serde_output = to_serde_output(output, order_by_steamid, matches!(struct_of_arrays, Some(true)));
    serde_json::to_value(&serde_output).map_err(to_napi_err)
}

#[napi]
pub fn parse_player_info(path_or_buf: Either<String, Buffer>) -> napi::Result<Value> {
    let bytes = resolve_byte_type(path_or_buf)?;
    let huf = create_huffman_lookup_table();

    let player_md = exports::parse_player_info(&bytes, &huf, parser::parse_demo::ParsingMode::Normal).map_err(to_napi_err)?;
    serde_json::to_value(&player_md).map_err(to_napi_err)
}

#[napi]
pub fn parse_player_skins(path_or_buf: Either<String, Buffer>) -> napi::Result<Value> {
    let bytes = resolve_byte_type(path_or_buf)?;
    let huf = create_huffman_lookup_table();
    
    let skins = exports::parse_player_skins(&bytes, &huf, parser::parse_demo::ParsingMode::Normal).map_err(to_napi_err)?;
    serde_json::to_value(&skins).map_err(to_napi_err)
}

fn resolve_byte_type(path_or_buf: Either<String, Buffer>) -> Result<BytesVariant, napi::Error> {
    match path_or_buf {
        Either::A(path) => exports::create_mmap(&path).map_err(to_napi_err),
        Either::B(buf) => Ok(Vec::from(buf).into()),
    }
}