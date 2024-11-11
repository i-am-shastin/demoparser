use itertools::Itertools;
use memmap2::Mmap;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Bool(bool),
    U32(u32),
    I32(i32),
    I16(i16),
    F32(f32),
    U64(u64),
    U8(u8),
    String(String),
    XYVec([f32; 2]),
    XYZVec([f32; 3]),
    // Todo change to Vec<T>
    StringVec(Vec<String>),
    U32Vec(Vec<u32>),
    U64Vec(Vec<u64>),
    Stickers(Vec<Sticker>),
    InputHistory(Vec<InputHistory>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Sticker {
    pub name: String,
    pub wear: f32,
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InputHistory {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub render_tick_count: i32,
    pub render_tick_fraction: f32,
    pub player_tick_count: i32,
    pub player_tick_fraction: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarVec {
    U32(Vec<Option<u32>>),
    Bool(Vec<Option<bool>>),
    U64(Vec<Option<u64>>),
    F32(Vec<Option<f32>>),
    I32(Vec<Option<i32>>),
    String(Vec<Option<String>>),
    StringVec(Vec<Vec<String>>),
    U64Vec(Vec<Vec<u64>>),
    U32Vec(Vec<Vec<u32>>),
    XYVec(Vec<Option<[f32; 2]>>),
    XYZVec(Vec<Option<[f32; 3]>>),
    Stickers(Vec<Vec<Sticker>>),
    InputHistory(Vec<Vec<InputHistory>>),
}

impl VarVec {
    pub fn new(item: &Variant) -> Self {
        match item {
            Variant::Bool(_) => VarVec::Bool(vec![]),
            Variant::I32(_) => VarVec::I32(vec![]),
            Variant::F32(_) => VarVec::F32(vec![]),
            Variant::String(_) => VarVec::String(vec![]),
            Variant::U64(_) => VarVec::U64(vec![]),
            Variant::U32(_) => VarVec::U32(vec![]),
            Variant::StringVec(_) => VarVec::StringVec(vec![]),
            Variant::U64Vec(_) => VarVec::U64Vec(vec![]),
            Variant::U32Vec(_) => VarVec::U32Vec(vec![]),
            Variant::XYVec(_) => VarVec::XYVec(vec![]),
            Variant::XYZVec(_) => VarVec::XYZVec(vec![]),
            Variant::Stickers(_) => VarVec::Stickers(vec![]),
            Variant::I16(_) => VarVec::I32(vec![]),
            Variant::U8(_) => VarVec::I32(vec![]),
            Variant::InputHistory(_) => VarVec::InputHistory(vec![]),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PropColumn {
    pub data: Option<VarVec>,
    pub num_nones: usize,
}

impl PropColumn {
    pub fn slice_to_new(&self, indicies: &[usize]) -> Option<PropColumn> {
        let data = match &self.data {
            Some(VarVec::Bool(b)) => VarVec::Bool(indicies.iter().map(|x| b[*x]).collect_vec()),
            Some(VarVec::I32(b)) => VarVec::I32(indicies.iter().map(|x| b[*x]).collect_vec()),
            Some(VarVec::F32(b)) => VarVec::F32(indicies.iter().map(|x| b[*x]).collect_vec()),
            Some(VarVec::String(b)) => VarVec::String(indicies.iter().map(|x| b[*x].to_owned()).collect_vec()),
            Some(VarVec::U32(b)) => VarVec::U32(indicies.iter().map(|x| b[*x]).collect_vec()),
            Some(VarVec::U64(b)) => VarVec::U64(indicies.iter().map(|x| b[*x]).collect_vec()),
            Some(VarVec::StringVec(b)) => VarVec::StringVec(indicies.iter().map(|x| b[*x].to_owned()).collect_vec()),
            Some(VarVec::U64Vec(b)) => VarVec::U64Vec(indicies.iter().map(|x| b[*x].to_owned()).collect_vec()),
            Some(VarVec::U32Vec(b)) => VarVec::U32Vec(indicies.iter().map(|x| b[*x].to_owned()).collect_vec()),
            Some(VarVec::XYVec(b)) => VarVec::XYVec(indicies.iter().map(|x| b[*x]).collect_vec()),
            Some(VarVec::XYZVec(b)) => VarVec::XYZVec(indicies.iter().map(|x| b[*x]).collect_vec()),
            Some(VarVec::Stickers(b)) => VarVec::Stickers(indicies.iter().map(|x| b[*x].to_owned()).collect_vec()),
            Some(VarVec::InputHistory(b)) => VarVec::InputHistory(indicies.iter().map(|x| b[*x].to_owned()).collect_vec()),
            None => {
                return Some(PropColumn {
                    data: None,
                    num_nones: indicies.len(),
                })
            }
        };
        Some(PropColumn {
            data: Some(data),
            num_nones: 0,
        })
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match &self.data {
            Some(VarVec::Bool(b)) => b.len(),
            Some(VarVec::I32(b)) => b.len(),
            Some(VarVec::F32(b)) => b.len(),
            Some(VarVec::String(b)) => b.len(),
            Some(VarVec::U32(b)) => b.len(),
            Some(VarVec::U64(b)) => b.len(),
            Some(VarVec::StringVec(b)) => b.len(),
            Some(VarVec::U64Vec(b)) => b.len(),
            Some(VarVec::U32Vec(b)) => b.len(),
            Some(VarVec::XYVec(b)) => b.len(),
            Some(VarVec::XYZVec(b)) => b.len(),
            Some(VarVec::Stickers(b)) => b.len(),
            Some(VarVec::InputHistory(b)) => b.len(),
            None => self.num_nones,
        }
    }

    pub fn extend_from(&mut self, other: &mut PropColumn) {
        if self.data.is_none() {
            if let Some(other_data) = &other.data {
                self.resolve_vec_type(other_data);
            } else {
                self.num_nones += other.num_nones;
                return;
            }
        }
        let Some(data) = &mut self.data else { return };

        if let Some(other_data) = &other.data {
            match (data, other_data) {
                (VarVec::Bool(v), VarVec::Bool(v_other)) => v.extend_from_slice(v_other),
                (VarVec::I32(v), VarVec::I32(v_other)) => v.extend_from_slice(v_other),
                (VarVec::F32(v), VarVec::F32(v_other)) => v.extend_from_slice(v_other),
                (VarVec::String(v), VarVec::String(v_other)) => v.extend_from_slice(v_other),
                (VarVec::U32(v), VarVec::U32(v_other)) => v.extend_from_slice(v_other),
                (VarVec::U64(v), VarVec::U64(v_other)) => v.extend_from_slice(v_other),
                (VarVec::StringVec(v), VarVec::StringVec(v_other)) => v.extend_from_slice(v_other),
                (VarVec::U64Vec(v), VarVec::U64Vec(v_other)) => v.extend_from_slice(v_other),
                (VarVec::XYVec(v), VarVec::XYVec(v_other)) => v.extend_from_slice(v_other),
                (VarVec::XYZVec(v), VarVec::XYZVec(v_other)) => v.extend_from_slice(v_other),
                (VarVec::Stickers(v), VarVec::Stickers(v_other)) => v.extend_from_slice(v_other),
                (VarVec::InputHistory(v), VarVec::InputHistory(v_other)) => v.extend_from_slice(v_other),
                (VarVec::U32Vec(v), VarVec::U32Vec(v_other)) => v.extend_from_slice(v_other),
                (_, _) => {}
            }
        } else {
            data.push_n_nones(other.num_nones);
        }
    }

    fn resolve_vec_type(&mut self, v: &VarVec) {
        let mut data = match v {
            VarVec::Bool(_) => VarVec::Bool(vec![]),
            VarVec::F32(_) => VarVec::F32(vec![]),
            VarVec::I32(_) => VarVec::I32(vec![]),
            VarVec::String(_) => VarVec::String(vec![]),
            VarVec::U32(_) => VarVec::U32(vec![]),
            VarVec::U64(_) => VarVec::U64(vec![]),
            VarVec::StringVec(_) => VarVec::StringVec(vec![]),
            VarVec::U64Vec(_) => VarVec::U64Vec(vec![]),
            VarVec::XYVec(_) => VarVec::XYVec(vec![]),
            VarVec::XYZVec(_) => VarVec::XYZVec(vec![]),
            VarVec::Stickers(_) => VarVec::Stickers(vec![]),
            VarVec::U32Vec(_) => VarVec::U32Vec(vec![]),
            VarVec::InputHistory(_) => VarVec::InputHistory(vec![]),
        };
        data.push_n_nones(self.num_nones);
        self.data = Some(data);
    }

    #[inline(always)]
    pub fn push(&mut self, item: Option<Variant>) {
        if self.data.is_none() {
            match &item {
                None => self.num_nones += 1,
                Some(p) => {
                    let mut var_vec = VarVec::new(p);
                    var_vec.push_n_nones(self.num_nones);
                    self.num_nones = 0;
                    self.data = Some(var_vec);
                }
            }
        }
        if let Some(var_vec) = &mut self.data {
            var_vec.push_variant(item);
        }
    }
}

impl VarVec {
    #[inline(always)]
    pub fn push_variant(&mut self, item: Option<Variant>) {
        if item.is_none() {
            return self.push_n_nones(1);
        }
        match (item, self) {
            (Some(Variant::F32(p)), VarVec::F32(f)) => f.push(Some(p)),
            (Some(Variant::I32(p)), VarVec::I32(f)) => f.push(Some(p)),
            (Some(Variant::String(p)), VarVec::String(f)) => f.push(Some(p)),
            (Some(Variant::U32(p)), VarVec::U32(f)) => f.push(Some(p)),
            (Some(Variant::U64(p)), VarVec::U64(f)) => f.push(Some(p)),
            (Some(Variant::Bool(p)), VarVec::Bool(f)) => f.push(Some(p)),
            (Some(Variant::StringVec(p)), VarVec::StringVec(f)) => f.push(p),
            (Some(Variant::U64Vec(p)), VarVec::U64Vec(f)) => f.push(p),
            (Some(Variant::U32Vec(p)), VarVec::U32Vec(f)) => f.push(p),
            (Some(Variant::XYVec(p)), VarVec::XYVec(f)) => f.push(Some(p)),
            (Some(Variant::XYZVec(p)), VarVec::XYZVec(f)) => f.push(Some(p)),
            (Some(Variant::Stickers(p)), VarVec::Stickers(f)) => f.push(p),
            (Some(Variant::InputHistory(p)), VarVec::InputHistory(f)) => f.push(p),
            _ => {}
        }
    }

    fn push_n_nones(&mut self, count: usize) {
        match self {
            VarVec::I32(f) => f.resize(f.len() + count, None),
            VarVec::F32(f) => f.resize(f.len() + count, None),
            VarVec::String(f) => f.resize(f.len() + count, None),
            VarVec::U32(f) => f.resize(f.len() + count, None),
            VarVec::U64(f) => f.resize(f.len() + count, None),
            VarVec::Bool(f) => f.resize(f.len() + count, None),
            VarVec::StringVec(f) => f.resize(f.len() + count, vec![]),
            VarVec::U64Vec(f) => f.resize(f.len() + count, vec![]),
            VarVec::XYVec(f) => f.resize(f.len() + count, None),
            VarVec::XYZVec(f) => f.resize(f.len() + count, None),
            VarVec::U32Vec(f) => f.resize(f.len() + count, vec![]),
            VarVec::Stickers(f) => f.resize(f.len() + count, vec![]),
            VarVec::InputHistory(f) => f.resize(f.len() + count, vec![]),
        }
    }
}

#[allow(dead_code)]
pub fn filter_to_vec<Wanted>(v: impl IntoIterator<Item = impl TryInto<Wanted>>) -> Vec<Wanted> {
    v.into_iter().filter_map(|x| x.try_into().ok()).collect()
}

impl Serialize for Variant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Variant::Bool(b) => serializer.serialize_bool(*b),
            Variant::F32(f) => serializer.serialize_f32(*f),
            Variant::I16(i) => serializer.serialize_i16(*i),
            Variant::I32(i) => serializer.serialize_i32(*i),
            Variant::String(s) => serializer.serialize_str(s),
            Variant::U32(u) => serializer.serialize_u32(*u),
            Variant::U64(u) => serializer.serialize_str(&u.to_string()),
            Variant::U8(u) => serializer.serialize_u8(*u),
            Variant::StringVec(v) => serializer.collect_seq(v),
            Variant::XYVec(v) => serializer.collect_seq(v),
            Variant::XYZVec(v) => serializer.collect_seq(v),
            Variant::U32Vec(v) => serializer.collect_seq(v),
            Variant::U64Vec(v) => serializer.collect_seq(v),
            Variant::Stickers(v) => serializer.collect_seq(v),
            Variant::InputHistory(v) => serializer.collect_seq(v)
        }
    }
}

#[derive(Debug)]
pub enum BytesVariant {
    Mmap(Mmap),
    Vec(Vec<u8>),
}

impl<Idx> std::ops::Index<Idx> for BytesVariant
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    #[inline(always)]
    fn index(&self, i: Idx) -> &Self::Output {
        match self {
            Self::Mmap(m) => &m[i],
            Self::Vec(v) => &v[i],
        }
    }
}

impl From<Vec<u8>> for BytesVariant {
    fn from(val: Vec<u8>) -> Self {
        BytesVariant::Vec(val)
    }
}

impl BytesVariant {
    pub fn get_len(&self) -> usize {
        match self {
            Self::Mmap(m) => m.len(),
            Self::Vec(v) => v.len(),
        }
    }
}
