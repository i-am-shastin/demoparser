use ahash::AHashMap;
use itertools::Itertools;
use parser::exports;
use parser::definitions::DemoParserError;
use parser::parse_demo::ParsingMode;
use parser::second_pass::game_events::EventField;
use parser::second_pass::game_events::GameEvent;
use parser::second_pass::variants::BytesVariant;
use parser::second_pass::variants::VarVec;
use parser::second_pass::variants::Variant;
use polars::prelude::ArrayRef;
use polars::prelude::ArrowField;
use polars::prelude::NamedFrom;
use polars::series::Series;
use polars_arrow::array::{
    Array, BooleanArray, Float32Array, Int32Array, UInt32Array, UInt64Array, Utf8Array,
};
use polars_arrow::ffi;
use pyo3::exceptions::PyValueError;
use pyo3::ffi::Py_uintptr_t;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::PyBytes;
use pyo3::types::PyDict;
use pyo3::types::PyList;
use pyo3::Python;
use pyo3::{PyAny, PyObject, PyResult};

use pyo3::create_exception;
create_exception!(DemoParser, Exception, pyo3::exceptions::PyException);

struct PyVariant(Variant);

impl<'source> FromPyObject<'source> for PyVariant {
    fn extract_bound(obj: &pyo3::Bound<'source, PyAny>) -> PyResult<Self> {
        if let Ok(val) = obj.extract::<bool>() {
            Ok(PyVariant(Variant::Bool(val)))
        } else if let Ok(val) = obj.extract::<String>() {
            Ok(PyVariant(Variant::String(val)))
        } else if let Ok(val) = obj.extract::<u8>() {
            Ok(PyVariant(Variant::U8(val)))
        } else if let Ok(val) = obj.extract::<i16>() {
            Ok(PyVariant(Variant::I16(val)))
        } else if let Ok(val) = obj.extract::<i32>() {
            Ok(PyVariant(Variant::I32(val)))
        } else if let Ok(val) = obj.extract::<u32>() {
            Ok(PyVariant(Variant::U32(val)))
        } else if let Ok(val) = obj.extract::<u64>() {
            Ok(PyVariant(Variant::U64(val)))
        } else if let Ok(val) = obj.extract::<f32>() {
            Ok(PyVariant(Variant::F32(val)))
        } else {
            Err(PyValueError::new_err("Unsupported type for Variant"))
        }
    }
}

#[derive(FromPyObject)]
struct WantedPropState {
    prop: String,
    state: PyVariant,
}

fn to_py_err<T>(e: T) -> PyErr
where
    T: std::fmt::Display
{
    Exception::new_err(format!("{e}"))
}

#[pyclass]
struct DemoParser {
    bytes: BytesVariant,
}

#[pymethods]
impl DemoParser {
    #[new]
    pub fn py_new(demo_path: String) -> PyResult<Self> {
        let bytes = exports::create_mmap(&demo_path).map_err(|e| Exception::new_err(format!("{e}. File name: {demo_path}")))?;
        Ok(Self { bytes })
    }

    /// Parses header message (different from the first 16 bytes of the file)
    /// Should have the following fields:
    ///
    /// "addons", "server_name", "demo_file_stamp", "network_protocol",
    /// "map_name", "fullpackets_version", "allow_clientside_entities",
    /// "allow_clientside_particles", "demo_version_name", "demo_version_guid",
    /// "client_name", "game_directory"
    pub fn parse_header(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let header = exports::parse_header(&self.bytes, ParsingMode::Normal).map_err(to_py_err)?;
        Ok(header.to_object(py))
    }

    /// Returns the names of game events present in the demo
    pub fn list_game_events(&self, _py: Python<'_>) -> PyResult<Py<PyAny>> {
        let game_events = exports::list_game_events(&self.bytes, ParsingMode::Normal).map_err(to_py_err)?;
        Ok(Python::with_gil(|py| game_events.to_object(py)))
    }

    /// Returns all coordinates of all grenades along with info about thrower.
    ///
    /// Example:
    ///          X           Y       Z  tick  thrower_steamid grenade_type
    /// 0 -388.875  1295.46875 -5120.0   982              NaN    HeGrenade
    /// 1 -388.875  1295.46875 -5120.0   983              NaN    HeGrenade
    /// 2 -388.875  1295.46875 -5120.0   983              NaN    HeGrenade
    pub fn parse_grenades(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let projectiles = exports::parse_grenades(&self.bytes, ParsingMode::Normal).map_err(to_py_err)?;

        let entity_id: Vec<Option<i32>> = projectiles.iter().map(|s| s.entity_id).collect();
        let xs: Vec<Option<f32>> = projectiles.iter().map(|s| s.x).collect();
        let ys: Vec<Option<f32>> = projectiles.iter().map(|s| s.y).collect();
        let zs: Vec<Option<f32>> = projectiles.iter().map(|s| s.z).collect();

        let ticks: Vec<Option<i32>> = projectiles.iter().map(|s| s.tick).collect();
        let steamid: Vec<Option<u64>> = projectiles.iter().map(|s| s.steamid).collect();
        let name: Vec<Option<String>> = projectiles.iter().map(|s| s.name.clone()).collect();
        let grenade_type: Vec<Option<String>> = projectiles.iter().map(|s| s.grenade_type.clone()).collect();

        // SoA form
        let xs = arr_to_py(Box::new(Float32Array::from(xs))).unwrap();
        let ys = arr_to_py(Box::new(Float32Array::from(ys))).unwrap();
        let zs = arr_to_py(Box::new(Float32Array::from(zs))).unwrap();
        // Actually not sure about Z coordinate. Leave out for now.
        let ticks = arr_to_py(Box::new(Int32Array::from(ticks))).unwrap();
        let grenade_type = arr_to_py(Box::new(Utf8Array::<i32>::from(grenade_type))).unwrap();
        let name = arr_to_py(Box::new(Utf8Array::<i32>::from(name))).unwrap();
        let steamids = arr_to_py(Box::new(UInt64Array::from(steamid))).unwrap();
        let entity_ids = arr_to_py(Box::new(Int32Array::from(entity_id))).unwrap();

        let polars = py.import_bound("polars")?;
        let all_series_py = [xs, ys, zs, ticks, steamids, name, grenade_type, entity_ids].to_object(py);
        Python::with_gil(|py| {
            let df = polars.call_method1("DataFrame", (all_series_py,))?;
            // Set column names
            let column_names = [
                "X",
                "Y",
                "Z",
                "tick",
                "thrower_steamid",
                "name",
                "grenade_type",
                "entity_id",
            ];
            df.setattr("columns", column_names.to_object(py)).unwrap();
            // Call to_pandas with use_pyarrow_extension_array = true
            let kwargs = vec![("use_pyarrow_extension_array", true)].into_py_dict_bound(py);
            let pandas_df = df.call_method("to_pandas", (), Some(&kwargs)).unwrap();
            Ok(pandas_df.to_object(py))
        })
    }

    pub fn parse_player_info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let player_md = exports::parse_player_info(&self.bytes, ParsingMode::Normal).map_err(to_py_err)?;

        let steamids: Vec<Option<u64>> = player_md.iter().map(|p| p.steamid).collect();
        let team_numbers: Vec<Option<i32>> = player_md.iter().map(|p| p.team_number).collect();
        let names: Vec<Option<String>> = player_md.iter().map(|p| p.name.clone()).collect();

        // SoA form
        let steamid = rust_series_to_py_series(&Series::new("Steamid", steamids))?;
        let team_number = arr_to_py(Box::new(Int32Array::from(team_numbers)))?;
        let name = rust_series_to_py_series(&Series::new("param2", names))?;

        let polars = py.import_bound("polars")?;
        let all_series_py = [steamid, name, team_number].to_object(py);
        Python::with_gil(|py| {
            let df = polars.call_method1("DataFrame", (all_series_py,))?;
            // Set column names
            let column_names = ["steamid", "name", "team_number"];
            df.setattr("columns", column_names.to_object(py))?;
            // Call to_pandas with use_pyarrow_extension_array = true
            let kwargs = vec![("use_pyarrow_extension_array", true)].into_py_dict_bound(py);
            let pandas_df = df.call_method("to_pandas", (), Some(&kwargs))?;
            Ok(pandas_df.to_object(py))
        })
    }

    pub fn parse_item_drops(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let item_drops = exports::parse_item_drops(&self.bytes, ParsingMode::Normal).map_err(to_py_err)?;

        let def_index: Vec<Option<u32>> = item_drops.iter().map(|x| x.def_index).collect();
        let account_id: Vec<Option<u32>> = item_drops.iter().map(|x| x.account_id).collect();
        let dropreason: Vec<Option<u32>> = item_drops.iter().map(|x| x.dropreason).collect();
        let inventory: Vec<Option<u32>> = item_drops.iter().map(|x| x.inventory).collect();
        let item_id: Vec<Option<u64>> = item_drops.iter().map(|x| x.item_id).collect();
        let paint_index: Vec<Option<u32>> = item_drops.iter().map(|x| x.paint_index).collect();
        let paint_seed: Vec<Option<u32>> = item_drops.iter().map(|x| x.paint_seed).collect();
        let paint_wear: Vec<Option<u32>> = item_drops.iter().map(|x| x.paint_wear).collect();
        let custom_name: Vec<Option<String>> = item_drops.iter().map(|x| x.custom_name.clone()).collect();

        // SoA form
        let account_id = arr_to_py(Box::new(UInt32Array::from(account_id)))?;
        let def_index = arr_to_py(Box::new(UInt32Array::from(def_index)))?;
        let dropreason = arr_to_py(Box::new(UInt32Array::from(dropreason)))?;
        let inventory = arr_to_py(Box::new(UInt32Array::from(inventory)))?;
        let item_id = arr_to_py(Box::new(UInt64Array::from(item_id)))?;
        let paint_index = arr_to_py(Box::new(UInt32Array::from(paint_index)))?;
        let paint_seed = arr_to_py(Box::new(UInt32Array::from(paint_seed)))?;
        let paint_wear = arr_to_py(Box::new(UInt32Array::from(paint_wear)))?;
        let custom_name = rust_series_to_py_series(&Series::new("custom_name", custom_name))?;

        let polars = py.import_bound("polars")?;
        let all_series_py = [
            account_id,
            def_index,
            dropreason,
            inventory,
            item_id,
            paint_index,
            paint_seed,
            paint_wear,
            custom_name,
        ]
        .to_object(py);
        Python::with_gil(|py| {
            let df = polars.call_method1("DataFrame", (all_series_py,))?;
            // Set column names
            let column_names = [
                "account_id",
                "def_index",
                "dropreason",
                "inventory",
                "item_id",
                "paint_index",
                "paint_seed",
                "paint_wear",
                "custom_name",
            ];
            df.setattr("columns", column_names.to_object(py))?;
            // Call to_pandas with use_pyarrow_extension_array = true
            let kwargs = vec![("use_pyarrow_extension_array", true)].into_py_dict_bound(py);
            let pandas_df = df.call_method("to_pandas", (), Some(&kwargs))?;
            Ok(pandas_df.to_object(py))
        })
    }

    pub fn parse_skins(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let skins = exports::parse_player_skins(&self.bytes, ParsingMode::Normal).map_err(to_py_err)?;

        let def_idx_vec: Vec<Option<u32>> = skins.iter().map(|s| s.def_index).collect();
        let item_id: Vec<Option<u64>> = skins.iter().map(|s| s.item_id).collect();
        let paint_index: Vec<Option<u32>> = skins.iter().map(|s| s.paint_index).collect();
        let paint_seed: Vec<Option<u32>> = skins.iter().map(|s| s.paint_seed).collect();
        let paint_wear: Vec<Option<u32>> = skins.iter().map(|s| s.paint_wear).collect();
        let steamid: Vec<Option<u64>> = skins.iter().map(|s| s.steamid).collect();
        let custom_name: Vec<Option<String>> = skins.iter().map(|s| s.custom_name.clone()).collect();

        let def_index = arr_to_py(Box::new(UInt32Array::from(def_idx_vec)))?;
        let item_id = arr_to_py(Box::new(UInt64Array::from(item_id)))?;
        let paint_index = arr_to_py(Box::new(UInt32Array::from(paint_index)))?;
        let paint_seed = arr_to_py(Box::new(UInt32Array::from(paint_seed)))?;
        let paint_wear = arr_to_py(Box::new(UInt32Array::from(paint_wear)))?;
        let steamid = arr_to_py(Box::new(UInt64Array::from(steamid)))?;
        let custom_name = rust_series_to_py_series(&Series::new("custom_name", custom_name))?;

        let polars = py.import_bound("polars")?;
        let all_series_py = [
            def_index,
            item_id,
            paint_index,
            paint_seed,
            paint_wear,
            custom_name,
            steamid,
        ]
        .to_object(py);
        Python::with_gil(|py| {
            let df = polars.call_method1("DataFrame", (all_series_py,))?;
            // Set column names
            let column_names = [
                "def_index",
                "item_id",
                "paint_index",
                "paint_seed",
                "paint_wear",
                "custom_name",
                "steamid",
            ];
            df.setattr("columns", column_names.to_object(py))?;
            // Call to_pandas with use_pyarrow_extension_array = true
            let kwargs = vec![("use_pyarrow_extension_array", true)].into_py_dict_bound(py);
            let pandas_df = df.call_method("to_pandas", (), Some(&kwargs))?;
            Ok(pandas_df.to_object(py))
        })
    }

    #[pyo3(signature = (event_name, *, player=None, other=None))]
    pub fn parse_event(
        &self,
        py: Python<'_>,
        event_name: String,
        player: Option<Vec<String>>,
        other: Option<Vec<String>>,
    ) -> PyResult<Py<PyAny>> {
        let game_events = exports::parse_event(
            &self.bytes,
            ParsingMode::Normal,
            event_name,
            player,
            other
        ).map_err(to_py_err)?;
        series_from_event(game_events, py).or_else(|_| Ok(PyList::empty_bound(py).into()))
    }

    #[pyo3(signature = (event_name, *, player=None, other=None))]
    pub fn parse_events(
        &self,
        py: Python<'_>,
        event_name: Vec<String>,
        player: Option<Vec<String>>,
        other: Option<Vec<String>>,
    ) -> PyResult<Py<PyAny>> {
        let game_events = exports::parse_events(
            &self.bytes,
            ParsingMode::Normal,
            event_name,
            player,
            other
        ).map_err(to_py_err)?;
        series_from_multiple_events(&game_events, py).map_err(to_py_err)
    }

    #[cfg(feature = "voice")]
    pub fn parse_voice(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let voice_data = exports::parse_voice(&self.bytes, ParsingMode::Normal).map_err(to_py_err)?;

        let mut voice_hashmap = AHashMap::default();
        for (steamid, bytes) in voice_data {
            let py_bytes = PyBytes::new_bound(py, &bytes);
            voice_hashmap.insert(steamid, py_bytes);
        }
        Ok(voice_hashmap.to_object(py))
    }

    #[pyo3(signature = (wanted_props, *, players=None, ticks=None, prop_states=None))]
    pub fn parse_ticks(
        &self,
        py: Python,
        wanted_props: Vec<String>,
        players: Option<Vec<u64>>,
        ticks: Option<Vec<i32>>,
        prop_states: Option<Vec<WantedPropState>>,
    ) -> PyResult<PyObject> {
        let wanted_prop_states = prop_states.unwrap_or_default().into_iter().map(|prop| (prop.prop, prop.state.0)).collect();

        let output = exports::parse_ticks(
            &self.bytes,
            ParsingMode::Normal,
            wanted_props,
            ticks,
            players,
            wanted_prop_states,
            false
        ).map_err(to_py_err)?;

        let mut all_series = vec![];
        let mut all_pyobjects = vec![];
        let prop_infos = output.prop_controller.prop_infos;
        let mut df_column_names_arrow = vec![];
        let mut df_column_names_py = vec![];

        for prop_info in prop_infos {
            if !output.df.contains_key(&prop_info.id) {
                continue;
            }
            match &output.df[&prop_info.id].data {
                Some(VarVec::F32(data)) => {
                    df_column_names_arrow.push(prop_info.prop_friendly_name);
                    all_series.push(arr_to_py(Box::new(Float32Array::from(data)))?);
                }
                Some(VarVec::I32(data)) => {
                    df_column_names_arrow.push(prop_info.prop_friendly_name);
                    all_series.push(arr_to_py(Box::new(Int32Array::from(data)))?);
                }
                Some(VarVec::U64(data)) => {
                    df_column_names_arrow.push(prop_info.prop_friendly_name);
                    all_series.push(arr_to_py(Box::new(UInt64Array::from(data)))?);
                }
                Some(VarVec::U32(data)) => {
                    df_column_names_arrow.push(prop_info.prop_friendly_name);
                    all_series.push(arr_to_py(Box::new(UInt32Array::from(data)))?);
                }
                Some(VarVec::Bool(data)) => {
                    df_column_names_arrow.push(prop_info.prop_friendly_name);
                    all_series.push(arr_to_py(Box::new(BooleanArray::from(data)))?);
                }
                Some(VarVec::String(data)) => {
                    df_column_names_arrow.push(prop_info.prop_friendly_name.clone());
                    let s = Series::new(&prop_info.prop_friendly_name.clone(), data);
                    let py_series = rust_series_to_py_series(&s)?;
                    all_series.push(py_series);
                }
                Some(VarVec::StringVec(data)) => {
                    df_column_names_py.push(prop_info.prop_friendly_name);
                    all_pyobjects.push(data.to_object(py));
                }
                Some(VarVec::U64Vec(data)) => {
                    df_column_names_py.push(prop_info.prop_friendly_name);
                    all_pyobjects.push(data.to_object(py));
                }
                Some(VarVec::XYZVec(data)) => {
                    df_column_names_py.push(prop_info.prop_friendly_name);
                    all_pyobjects.push(data.to_object(py));
                }
                Some(VarVec::U32Vec(data)) => {
                    df_column_names_py.push(prop_info.prop_friendly_name);
                    all_pyobjects.push(data.to_object(py));
                }

                Some(VarVec::Stickers(data)) => {
                    let mut dicts = vec![];
                    for weapon in data {
                        let mut v = vec![];
                        for sticker in weapon {
                            let dict = PyDict::new_bound(py);
                            dict.set_item("id", sticker.id.to_object(py))?;
                            dict.set_item("name", sticker.name.to_object(py))?;
                            dict.set_item("wear", sticker.wear.to_object(py))?;
                            dict.set_item("x", sticker.x.to_object(py))?;
                            dict.set_item("y", sticker.y.to_object(py))?;
                            v.push(dict);
                        }
                        dicts.push(v);
                    }
                    df_column_names_py.push(prop_info.prop_friendly_name);
                    all_pyobjects.push(dicts.to_object(py));
                }

                Some(VarVec::InputHistory(data)) => {
                    let mut dicts = vec![];
                    for input in data {
                        let mut v = vec![];
                        for sticker in input {
                            let dict = PyDict::new_bound(py);
                            dict.set_item("x", sticker.x.to_object(py))?;
                            dict.set_item("y", sticker.y.to_object(py))?;
                            dict.set_item("z", sticker.z.to_object(py))?;
                            dict.set_item("render_tick_count", sticker.render_tick_count.to_object(py))?;
                            dict.set_item("render_tick_fraction", sticker.render_tick_fraction.to_object(py))?;
                            dict.set_item("player_tick_count", sticker.player_tick_count.to_object(py))?;
                            dict.set_item("player_tick_fraction", sticker.player_tick_fraction.to_object(py))?;
                            v.push(dict);
                        }
                        dicts.push(v);
                    }
                    df_column_names_py.push(prop_info.prop_friendly_name);
                    all_pyobjects.push(dicts.to_object(py));
                }
                _ => {}
            }
        }
        Python::with_gil(|py| {
            let polars = py.import_bound("polars")?;
            let all_series_py = all_series.to_object(py);
            let df = polars.call_method1("DataFrame", (all_series_py,))?;
            df.setattr("columns", df_column_names_arrow.to_object(py))?;
            let pandas_df = df.call_method0("to_pandas")?;
            for (pyobj, col_name) in all_pyobjects.iter().zip(&df_column_names_py) {
                pandas_df.call_method1("insert", (0, col_name, pyobj))?;
            }
            df_column_names_arrow.extend(df_column_names_py);
            df_column_names_arrow.sort();
            let kwargs = vec![("axis", 1)].into_py_dict_bound(py);
            let args = (df_column_names_arrow,);
            pandas_df.call_method("reindex", args, Some(&kwargs))?;
            Ok(pandas_df.to_object(py))
        })
    }
}

/// <https://github.com/pola-rs/polars/blob/master/examples/python_rust_compiled_function/src/ffi.rs>
pub(crate) fn to_py_array(
    py: Python,
    pyarrow: &Bound<PyModule>,
    array: ArrayRef,
) -> PyResult<PyObject> {
    let schema = Box::new(ffi::export_field_to_c(&ArrowField::new(
        "",
        array.data_type().clone(),
        true,
    )));
    let array = Box::new(ffi::export_array_to_c(array));

    let schema_ptr: *const ffi::ArrowSchema = &*schema;
    let array_ptr: *const ffi::ArrowArray = &*array;

    let array = pyarrow.getattr("Array")?.call_method1(
        "_import_from_c",
        (array_ptr as Py_uintptr_t, schema_ptr as Py_uintptr_t),
    )?;

    Ok(array.to_object(py))
}

/// <https://github.com/pola-rs/polars/blob/master/examples/python_rust_compiled_function/src/ffi.rs>
pub fn rust_series_to_py_series(series: &Series) -> PyResult<PyObject> {
    // ensure we have a single chunk
    let series = series.rechunk();
    let array = series.to_arrow(0, false);

    Python::with_gil(|py| {
        // import pyarrow
        let pyarrow = py.import_bound("pyarrow")?;

        // pyarrow array
        let pyarrow_array = to_py_array(py, &pyarrow, array)?;

        // import polars
        let polars = py.import_bound("polars")?;
        let out = polars.call_method1("from_arrow", (pyarrow_array,))?;
        Ok(out.to_object(py))
    })
}

/// <https://github.com/pola-rs/polars/blob/master/examples/python_rust_compiled_function/src/ffi.rs>
pub fn arr_to_py(array: Box<dyn Array>) -> PyResult<PyObject> {
    //let series = series.rechunk();
    //let array = series.to_arrow(0);
    Python::with_gil(|py| {
        let pyarrow = py.import_bound("pyarrow")?;
        let pyarrow_array = to_py_array(py, &pyarrow, array)?;
        let polars = py.import_bound("polars")?;
        let out = polars.call_method1("from_arrow", (pyarrow_array,))?;
        Ok(out.to_object(py))
    })
}

pub fn series_from_multiple_events(
    events: &[GameEvent],
    py: Python,
) -> Result<Py<PyAny>, DemoParserError> {
    let per_ge = events.iter().into_group_map_by(|x| x.name.clone());
    let mut vv = vec![];
    for (k, v) in per_ge {
        let pairs: Vec<EventField> = v.iter().flat_map(|x| x.fields.clone()).collect();
        let per_key_name = pairs.into_iter().into_group_map_by(|x| x.name.clone());

        let mut series_columns = vec![];
        let mut py_columns = vec![];
        let mut rows = 0;

        for (name, vals) in per_key_name {
            match column_from_pairs(vals, &name, py)? {
                DataFrameColumn::PyAny(p) => py_columns.push((p, name)),
                DataFrameColumn::Series(s) => {
                    rows = s.len().max(rows);
                    series_columns.push((s, name));
                }
            };
        }
        if rows == 0 {
            continue;
        }

        let mut series_col_names = series_columns
            .iter()
            .map(|(_, name)| name.to_string())
            .collect_vec();
        let series_columns = series_columns
            .iter()
            .map(|(ser, _)| rust_series_to_py_series(ser).unwrap())
            .collect_vec();
        let py_col_names = py_columns
            .iter()
            .map(|(_, name)| name.to_string())
            .collect_vec();

        let dfp = Python::with_gil(|py| {
            let polars = py.import_bound("polars").unwrap();
            let all_series_py = series_columns.to_object(py);
            let df = polars.call_method1("DataFrame", (all_series_py,)).unwrap();
            df.setattr("columns", series_col_names.to_object(py)).unwrap();
            let pandas_df = df.call_method0("to_pandas").unwrap();

            for (pyobj, col_name) in py_columns {
                pandas_df.call_method1("insert", (0, col_name, pyobj)).unwrap();
            }

            series_col_names.extend(py_col_names);
            series_col_names.sort();

            let kwargs = vec![("axis", 1)].into_py_dict_bound(py);
            let args = (series_col_names,);
            let df = pandas_df.call_method("reindex", args, Some(&kwargs)).unwrap();
            df.to_object(py)
        });
        vv.push((k, dfp));
    }
    Ok(vv.to_object(py))
}

pub enum DataFrameColumn {
    Series(Series),
    PyAny(pyo3::Py<PyAny>),
}

fn series_from_event(events: Vec<GameEvent>, py: Python) -> Result<Py<PyAny>, DemoParserError> {
    let pairs = events.into_iter().flat_map(|x| x.fields).collect_vec();
    let per_key_name = pairs.into_iter().into_group_map_by(|x| x.name.clone());

    let mut series_columns = vec![];
    let mut py_columns = vec![];
    let mut rows = 0;

    for (name, vals) in per_key_name {
        match column_from_pairs(vals, &name, py)? {
            DataFrameColumn::PyAny(p) => py_columns.push((p, name)),
            DataFrameColumn::Series(s) => {
                rows = s.len().max(rows);
                series_columns.push((s, name));
            }
        };
    }
    if rows == 0 {
        return Err(DemoParserError::NoEvents);
    }

    let mut series_col_names = series_columns
        .iter()
        .map(|(_, name)| name.to_string())
        .collect_vec();
    let series_columns = series_columns
        .iter()
        .map(|(ser, _)| rust_series_to_py_series(ser).unwrap())
        .collect_vec();
    let py_col_names = py_columns
        .iter()
        .map(|(_, name)| name.to_string())
        .collect_vec();

    let dfp = Python::with_gil(|py| {
        let polars = py.import_bound("polars").unwrap();
        let all_series_py = series_columns.to_object(py);
        let df = polars.call_method1("DataFrame", (all_series_py,)).unwrap();
        df.setattr("columns", series_col_names.to_object(py)).unwrap();
        let pandas_df = df.call_method0("to_pandas").unwrap();
        for (pyobj, col_name) in py_columns {
            pandas_df.call_method1("insert", (0, col_name, pyobj)).unwrap();
        }
        series_col_names.extend(py_col_names);
        series_col_names.sort();
        let kwargs = vec![("axis", 1)].into_py_dict_bound(py);
        let args = (series_col_names,);
        let df = pandas_df.call_method("reindex", args, Some(&kwargs)).unwrap();
        df.to_object(py)
    });
    Ok(dfp)
}

fn to_f32_series(pairs: Vec<EventField>, name: &str) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::F32(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::Series(Series::new(name, v))
}

fn to_u32_series(pairs: Vec<EventField>, name: &str) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::U32(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::Series(Series::new(name, v))
}

fn to_i32_series(pairs: Vec<EventField>, name: &str) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::I32(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::Series(Series::new(name, v))
}

fn to_u64_series(pairs: Vec<EventField>, name: &str) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::U64(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::Series(Series::new(name, v))
}

fn to_py_string_col(pairs: Vec<EventField>, _name: &str, py: Python) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::StringVec(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::PyAny(v.to_object(py))
}

fn to_py_u64_col(pairs: Vec<EventField>, _name: &str, py: Python) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::U64Vec(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::PyAny(v.to_object(py))
}

fn to_py_u32_col(pairs: Vec<EventField>, _name: &str, py: Python) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::U32Vec(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::PyAny(v.to_object(py))
}

fn to_string_series(pairs: Vec<EventField>, name: &str) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::String(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::Series(Series::new(name, v))
}

fn to_bool_series(pairs: Vec<EventField>, name: &str) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            if let Some(Variant::Bool(val)) = pair.data {
                Some(val)
            } else {
                None
            }
        })
        .collect_vec();
    DataFrameColumn::Series(Series::new(name, v))
}

fn to_py_sticker_col(pairs: Vec<EventField>, _name: &str, py: Python) -> DataFrameColumn {
    let v = pairs
        .into_iter()
        .map(|pair| {
            let Some(Variant::Stickers(weapon)) = &pair.data else { return vec![] };
            weapon
                .iter()
                .map(|sticker| {
                    let dict = PyDict::new_bound(py);
                    let _ = dict.set_item("id", sticker.id.to_object(py));
                    let _ = dict.set_item("name", sticker.name.to_object(py));
                    let _ = dict.set_item("wear", sticker.wear.to_object(py));
                    let _ = dict.set_item("x", sticker.x.to_object(py));
                    let _ = dict.set_item("y", sticker.y.to_object(py));
                    dict
                })
                .collect()
        })
        .collect_vec();
    DataFrameColumn::PyAny(v.to_object(py))
}

fn to_null_series(pairs: &[EventField], name: &str) -> DataFrameColumn {
    // All series are null can pick any type
    let v: Vec<Option<i32>> = vec![None; pairs.len()];
    DataFrameColumn::Series(Series::new(name, v))
}

fn column_from_pairs(
    pairs: Vec<EventField>,
    name: &str,
    py: Python,
) -> Result<DataFrameColumn, DemoParserError> {
    let field_type = find_type_of_vals(&pairs)?;

    let s = match field_type {
        None => to_null_series(&pairs, name),
        Some(Variant::Bool(_)) => to_bool_series(pairs, name),
        Some(Variant::F32(_)) => to_f32_series(pairs, name),
        Some(Variant::U32(_)) => to_u32_series(pairs, name),
        Some(Variant::I32(_)) => to_i32_series(pairs, name),
        Some(Variant::U64(_)) => to_u64_series(pairs, name),
        Some(Variant::String(_)) => to_string_series(pairs, name),
        Some(Variant::StringVec(_)) => to_py_string_col(pairs, name, py),
        Some(Variant::U64Vec(_)) => to_py_u64_col(pairs, name, py),
        Some(Variant::U32Vec(_)) => to_py_u32_col(pairs, name, py),
        Some(Variant::Stickers(_)) => to_py_sticker_col(pairs, name, py),
        _ => panic!("unknown ge key: {field_type:?}"),
    };
    Ok(s)
}

fn find_type_of_vals(pairs: &Vec<EventField>) -> Result<Option<Variant>, DemoParserError> {
    // Need to find the correct type for outgoing series
    for pair in pairs {
        if pair.data.is_none() {
            continue;
        }
        return match &pair.data {
            Some(Variant::Bool(v)) => Ok(Some(Variant::Bool(*v))),
            Some(Variant::I32(v)) => Ok(Some(Variant::I32(*v))),
            Some(Variant::F32(v)) => Ok(Some(Variant::F32(*v))),
            Some(Variant::String(s)) => Ok(Some(Variant::String(s.clone()))),
            Some(Variant::U64(u)) => Ok(Some(Variant::U64(*u))),
            Some(Variant::U32(u)) => Ok(Some(Variant::U32(*u))),
            Some(Variant::StringVec(_u)) => Ok(Some(Variant::StringVec(vec![]))),
            Some(Variant::U64Vec(_u)) => Ok(Some(Variant::U64Vec(vec![]))),
            Some(Variant::U32Vec(_u)) => Ok(Some(Variant::U64Vec(vec![]))),
            Some(Variant::Stickers(_u)) => Ok(Some(Variant::Stickers(vec![]))),
            _ => Err(DemoParserError::UnknownGameEventVariant(pair.name.to_owned()))
        };
    }
    Ok(None)
}

#[pymodule]
fn demoparser2(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DemoParser>()?;
    Ok(())
}
