use crate::first_pass::prop_controller::PropInfo;
use crate::parse_demo::DemoOutput;
use crate::second_pass::variants::{PropColumn, VarVec, Variant};
use ahash::AHashMap;
use std::collections::HashMap;
use itertools::Itertools;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct OutputSerdeHelperStruct {
    pub prop_infos: Vec<PropInfo>,
    pub inner: AHashMap<u32, PropColumn>,
}

#[derive(Serialize)]
pub enum SerdeOutput {
    PerPlayer(HashMap<u64, OutputSerdeHelperStruct>),
    StructOfArrays(OutputSerdeHelperStruct),
    ArrayOfStructs(Vec<HashMap<String, Option<Variant>>>),
}

pub fn to_serde_output(demo_output: DemoOutput, order_by_steamid: bool, struct_of_arrays: bool) -> SerdeOutput {
    let mut prop_infos = demo_output.prop_controller.prop_infos;
    prop_infos.sort_by_key(|x| x.prop_name.clone());

    if order_by_steamid {
        let per_player_hashmap: HashMap<u64, OutputSerdeHelperStruct> = demo_output.df_per_player
            .into_iter()
            .map(|(player_id, value)| {
                let helper = OutputSerdeHelperStruct {
                    prop_infos: prop_infos.clone(),
                    inner: value,
                };
                (player_id, helper)
            })
            .collect();
        SerdeOutput::PerPlayer(per_player_hashmap)
    } else {
        let helper = OutputSerdeHelperStruct {
            prop_infos,
            inner: demo_output.df,
        };

        if struct_of_arrays {
            SerdeOutput::StructOfArrays(helper)
        } else {
            SerdeOutput::ArrayOfStructs(soa_to_aos(helper))
        }
    }
}

pub fn soa_to_aos(soa: OutputSerdeHelperStruct) -> Vec<HashMap<String, Option<Variant>>> {
    let mut total_rows = 0;
    for v in soa.inner.values() {
        total_rows = v.len();
    }
    let mut v = Vec::with_capacity(total_rows);
    for idx in 0..total_rows {
        let mut hm: HashMap<String, Option<Variant>> = HashMap::with_capacity(soa.prop_infos.len());
        for prop_info in &soa.prop_infos {
            if !soa.inner.contains_key(&prop_info.id) {
                continue;
            }
            let variant = match &soa.inner[&prop_info.id].data {
                None => continue,
                Some(VarVec::F32(val)) => val.get(idx).and_then(|x| x.as_ref().map(|f| Variant::F32(*f))),
                Some(VarVec::I32(val)) => val.get(idx).and_then(|x| x.as_ref().map(|f| Variant::I32(*f))),
                Some(VarVec::String(val)) => val.get(idx).and_then(|x| x.as_ref().map(|f| Variant::String(f.to_string()))),
                Some(VarVec::U64(val)) => val.get(idx).and_then(|x| x.as_ref().map(|f| Variant::String(f.to_string()))),
                Some(VarVec::Bool(val)) => val.get(idx).and_then(|x| x.as_ref().map(|f| Variant::Bool(*f))),
                Some(VarVec::U32(val)) => val.get(idx).and_then(|x| x.as_ref().map(|f| Variant::U32(*f))),
                Some(VarVec::StringVec(val)) => val.get(idx).map(|f| Variant::StringVec(f.clone())),
                Some(VarVec::U64Vec(val)) => val.get(idx).map(|f| Variant::U64Vec(f.clone())),
                Some(VarVec::U32Vec(val)) => val.get(idx).map(|f| Variant::U32Vec(f.clone())),
                Some(VarVec::XYVec(val)) => val.get(idx).and_then(|x| x.as_ref().map(|f| Variant::XYVec(*f))),
                Some(VarVec::XYZVec(val)) => val.get(idx).and_then(|x| x.as_ref().map(|f| Variant::XYZVec(*f))),
                Some(VarVec::Stickers(val)) => val.get(idx).map(|f| Variant::Stickers(f.clone())),
                Some(VarVec::InputHistory(val)) => val.get(idx).map(|f| Variant::InputHistory(f.clone())),
            };
            hm.insert(prop_info.prop_friendly_name.clone(), variant);
        }
        v.push(hm);
    }
    v
}

impl Serialize for OutputSerdeHelperStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.prop_infos.len()))?;

        for prop_info in &self.prop_infos {
            if !self.inner.contains_key(&prop_info.id) {
                continue;
            }
            match &self.inner[&prop_info.id].data {
                None => {},
                Some(VarVec::F32(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::I32(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::String(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::Bool(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::U32(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::StringVec(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::U32Vec(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::XYVec(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::XYZVec(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::Stickers(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::InputHistory(val)) => map.serialize_entry(&prop_info.prop_friendly_name, val)?,
                Some(VarVec::U64(val)) => {
                    let as_str = val
                        .iter()
                        .map(|x| x.as_ref().map(|u| u.to_string()))
                        .collect_vec();
                    map.serialize_entry(&prop_info.prop_friendly_name, &as_str)?;
                },
                Some(VarVec::U64Vec(val)) => {
                    let string_sid = val
                        .iter()
                        .map(|v| v.iter().map(|s| s.to_string()).collect_vec())
                        .collect_vec();
                    map.serialize_entry(&prop_info.prop_friendly_name, &string_sid)?;
                }
            }
        }
        map.end()
    }
}

pub fn as_string<S>(v: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(id) = v.as_ref() {
        serializer.serialize_str(&id.to_string())
    } else {
        serializer.serialize_none()
    }
}
