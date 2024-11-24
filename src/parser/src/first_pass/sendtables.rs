use super::read_bits::Bitreader;
use crate::definitions::DemoParserError;
use crate::first_pass::parser_settings::FirstPassParser;
use crate::first_pass::prop_controller::FLATTENED_VEC_MAX_LEN;
use crate::first_pass::prop_controller::ITEM_PURCHASE_COST;
use crate::first_pass::prop_controller::ITEM_PURCHASE_COUNT;
use crate::first_pass::prop_controller::ITEM_PURCHASE_DEF_IDX;
use crate::first_pass::prop_controller::ITEM_PURCHASE_HANDLE;
use crate::first_pass::prop_controller::ITEM_PURCHASE_NEW_DEF_IDX;
use crate::first_pass::prop_controller::MY_WEAPONS_OFFSET;
use crate::first_pass::prop_controller::WEAPON_SKIN_ID;
use crate::maps::BASETYPE_DECODERS;
use crate::second_pass::decoder::Decoder;
use crate::second_pass::decoder::QfMapper;
use crate::second_pass::decoder::QuantalizedFloat;
use crate::second_pass::path_ops::FieldPath;
use ahash::AHashMap;
use csgoproto::ProtoFlattenedSerializerT;
use csgoproto::ProtoFlattenedSerializerFieldT;
use csgoproto::CsvcMsgFlattenedSerializer;
use lazy_static::lazy_static;
use prost::Message;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"([^<\[\*]+)(<\s(.*)\s>)?(\*)?(\[(.*)\])?").unwrap();
}

// Majority of this file is implemented based on how clarity does it: https://github.com/skadistats/clarity

#[derive(Debug, Default, Clone)]
pub struct Serializer {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Serializer {
    #[inline(always)]
    pub fn get_field<'b>(&'b self, path: &FieldPath) -> Result<&'b Field, DemoParserError> {
        if path.last > 5 {
            return Err(DemoParserError::IllegalPathOp);
        }

        let mut field = self.fields.get(path.path[0] as usize).ok_or_else(|| DemoParserError::IllegalPathOp)?;
        for idx in 1..=path.last {
            field = field.get_inner(path.path[idx] as usize)?
        }
        Ok(field)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FieldInfo {
    pub decoder: Decoder,
    pub should_parse: bool,
    pub prop_id: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldCategory {
    Pointer,
    Vector,
    Array,
    Value,
}

#[derive(Debug, Clone)]
pub struct ConstructorField {
    pub var_name: String,
    pub var_type: String,
    pub send_node: String,
    pub serializer_name: Option<String>,
    pub encoder: String,
    pub encode_flags: i32,
    pub bitcount: i32,
    pub low_value: f32,
    pub high_value: f32,
    pub field_type: FieldType,

    pub decoder: Decoder,
    pub category: FieldCategory,
    pub field_enum_type: Option<Field>,
    pub serializer: Option<Serializer>,
    pub base_decoder: Option<Decoder>,
    pub child_decoder: Option<Decoder>,
}

impl<'a> FirstPassParser<'a> {
    pub fn parse_sendtable(&mut self) -> Result<AHashMap<String, Serializer>, DemoParserError> {
        let tables = self.sendtable_message.as_ref().ok_or_else(|| DemoParserError::NoSendTableMessage)?;
        let mut bitreader = Bitreader::new(tables.data());
        let n_bytes = bitreader.read_varint()?;
        let bytes = bitreader.read_n_bytes(n_bytes as usize)?;

        let serializer_msg = CsvcMsgFlattenedSerializer::decode(bytes.as_slice()).map_err(|_| DemoParserError::MalformedMessage)?;
        self.create_fields(serializer_msg)
    }

    fn create_fields(&mut self, serializer_msg: CsvcMsgFlattenedSerializer) -> Result<AHashMap<String, Serializer>, DemoParserError> {
        let mut serializers: AHashMap<String, Serializer> = AHashMap::with_capacity(serializer_msg.serializers.len());

        // Creates fields
        let mut fields = serializer_msg.fields
            .iter()
            .map(|f| {
                self.generate_field_data(f, &serializer_msg).map(Some)
            })
            .collect::<Result<Vec<_>, DemoParserError>>()?;

        // Creates serializers
        for serializer in &serializer_msg.serializers {
            let mut ser = self.generate_serializer(serializer, &mut fields, &serializer_msg, &serializers)?;
            if ser.name.contains("Player")
                || ser.name.contains("Controller")
                || ser.name.contains("Team")
                || ser.name.contains("Weapon")
                || ser.name.contains("AK")
                || ser.name.contains("cell")
                || ser.name.contains("vec")
                || ser.name.contains("Projectile")
                || ser.name.contains("Knife")
                || ser.name.contains("CDEagle")
                || ser.name.contains("Rules")
                || ser.name.contains("C4")
                || ser.name.contains("Grenade")
                || ser.name.contains("Flash")
                || ser.name.contains("Molo")
                || ser.name.contains("Inc")
                || ser.name.contains("Infer")
            {
                // Assign id to each prop and other metadata things.
                // When collecting values we use the id as key.
                self.prop_controller.find_prop_name_paths(&mut ser);
            }
            serializers.insert(ser.name.clone(), ser);
        }
        // Related to prop collection
        self.prop_controller.set_custom_propinfos();
        Ok(serializers)
    }

    fn generate_serializer(
        &mut self,
        serializer_msg: &ProtoFlattenedSerializerT,
        field_data: &mut [Option<ConstructorField>],
        big: &CsvcMsgFlattenedSerializer,
        serializers: &AHashMap<String, Serializer>,
    ) -> Result<Serializer, DemoParserError> {
        let sid = big.symbols.get(serializer_msg.serializer_name_sym() as usize).ok_or_else(|| DemoParserError::MalformedMessage)?;
        let fields = serializer_msg.fields_index
            .iter()
            .map(|i| {
                if let Some(Some(f)) = field_data.get_mut(*i as usize) {
                    if f.field_enum_type.is_none() {
                        f.field_enum_type = Some(create_field(f, serializers)?)
                    }
                    if let Some(field) = &f.field_enum_type {
                        return Ok(field.clone());
                    }
                }
                Ok(Field::None)
            })
            .collect::<Result<Vec<_>, DemoParserError>>()?;

        Ok(Serializer {
            name: sid.to_owned(),
            fields,
        })
    }

    fn generate_field_data(
        &mut self,
        msg: &ProtoFlattenedSerializerFieldT,
        big: &CsvcMsgFlattenedSerializer,
    ) -> Result<ConstructorField, DemoParserError> {
        let name = big.symbols.get(msg.var_type_sym() as usize).ok_or_else(|| DemoParserError::MalformedMessage)?;
        let field_type = find_field_type(name)?;
        let mut field = field_from_msg(msg, big, field_type)?;

        field.category = find_category(&field);
        field.decoder = match field.var_name.as_str() {
            "m_PredFloatVariables" | "m_OwnerOnlyPredNetFloatVariables" => Decoder::NoscaleDecoder,
            "m_OwnerOnlyPredNetVectorVariables" | "m_PredVectorVariables" => Decoder::VectorNoscaleDecoder,
            "m_pGameModeRules" => Decoder::GameModeRulesDecoder,
            _ => {
                if field.encoder == "qangle_precise" {
                    Decoder::QanglePresDecoder
                } else {
                    field.find_decoder(&mut self.qf_mapper)
                }
            }
        };
        
        Ok(field)
    }
}

// Design from https://github.com/skadistats/clarity
#[derive(Debug, Clone)]
pub enum Field {
    Array(ArrayField),
    Vector(VectorField),
    Serializer(SerializerField),
    Pointer(PointerField),
    Value(ValueField),
    None,
}

impl Field {
    #[inline(always)]
    pub fn get_inner(&self, idx: usize) -> Result<&Field, DemoParserError> {
        match self {
            Field::Array(inner) => Ok(&inner.field_enum),
            Field::Vector(inner) => Ok(&inner.field_enum),
            Field::Serializer(inner) => inner.serializer.fields.get(idx).ok_or_else(|| DemoParserError::IllegalPathOp),
            Field::Pointer(inner) => inner.serializer.fields.get(idx).ok_or_else(|| DemoParserError::IllegalPathOp),
            _ => Err(DemoParserError::IllegalPathOp),
        }
    }

    #[inline(always)]
    pub fn get_inner_mut(&mut self, idx: usize) -> Result<&mut Field, DemoParserError> {
        match self {
            Field::Array(inner) => Ok(&mut inner.field_enum),
            Field::Vector(inner) => Ok(&mut inner.field_enum),
            Field::Serializer(inner) => inner.serializer.fields.get_mut(idx).ok_or_else(|| DemoParserError::IllegalPathOp),
            Field::Pointer(inner) => inner.serializer.fields.get_mut(idx).ok_or_else(|| DemoParserError::IllegalPathOp),
            _ => Err(DemoParserError::IllegalPathOp),
        }
    }

    #[inline(always)]
    pub fn get_decoder(&self) -> Result<Decoder, DemoParserError> {
        match self {
            Field::Value(inner) => Ok(inner.decoder),
            Field::Vector(_) => Ok(Decoder::UnsignedDecoder),
            Field::Pointer(inner) => Ok(inner.decoder),
            // Illegal
            _ => Err(DemoParserError::FieldNoDecoder),
        }
    }

    #[inline(always)]
    pub fn get_field_info(&self, path: &FieldPath) -> Option<FieldInfo> {
        let mut fi = match self {
            Field::Value(v) => FieldInfo {
                decoder: v.decoder,
                should_parse: v.should_parse,
                prop_id: v.prop_id,
            },
            Field::Vector(v) => match self.get_inner(0) {
                Ok(Field::Value(inner)) => FieldInfo {
                    decoder: v.decoder,
                    should_parse: inner.should_parse,
                    prop_id: inner.prop_id,
                },
                _ => return None,
            },
            _ => return None,
        };
    
        // Flatten vector props
        if fi.prop_id == MY_WEAPONS_OFFSET {
            if path.last == 1 {
                // TODO
                // Why is this part here?
            } else {
                fi.prop_id = MY_WEAPONS_OFFSET + path.path[2] as u32 + 1;
            }
        }
        if fi.prop_id == WEAPON_SKIN_ID {
            fi.prop_id = WEAPON_SKIN_ID + path.path[1] as u32;
        }
        if path.path[1] != 1 {
            if fi.prop_id >= ITEM_PURCHASE_COUNT && fi.prop_id < ITEM_PURCHASE_COUNT + FLATTENED_VEC_MAX_LEN {
                fi.prop_id = ITEM_PURCHASE_COUNT + path.path[2] as u32;
            }
            if fi.prop_id >= ITEM_PURCHASE_DEF_IDX && fi.prop_id < ITEM_PURCHASE_DEF_IDX + FLATTENED_VEC_MAX_LEN {
                fi.prop_id = ITEM_PURCHASE_DEF_IDX + path.path[2] as u32;
            }
            if fi.prop_id >= ITEM_PURCHASE_COST && fi.prop_id < ITEM_PURCHASE_COST + FLATTENED_VEC_MAX_LEN {
                fi.prop_id = ITEM_PURCHASE_COST + path.path[2] as u32;
            }
            if fi.prop_id >= ITEM_PURCHASE_HANDLE && fi.prop_id < ITEM_PURCHASE_HANDLE + FLATTENED_VEC_MAX_LEN {
                fi.prop_id = ITEM_PURCHASE_HANDLE + path.path[2] as u32;
            }
            if fi.prop_id >= ITEM_PURCHASE_NEW_DEF_IDX && fi.prop_id < ITEM_PURCHASE_NEW_DEF_IDX + FLATTENED_VEC_MAX_LEN {
                fi.prop_id = ITEM_PURCHASE_NEW_DEF_IDX + path.path[2] as u32;
            }
        }
        Some(fi)
    }
}

#[derive(Debug, Clone)]
pub struct ArrayField {
    pub field_enum: Box<Field>,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct VectorField {
    pub field_enum: Box<Field>,
    pub decoder: Decoder,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueField {
    pub decoder: Decoder,
    pub name: String,
    pub should_parse: bool,
    pub prop_id: u32,
    pub full_name: String,
}

#[derive(Debug, Clone)]
pub struct SerializerField {
    pub serializer: Serializer,
}

#[derive(Debug, Clone)]
pub struct PointerField {
    pub decoder: Decoder,
    pub serializer: Serializer,
}

impl ArrayField {
    pub fn new(field: Field, length: usize) -> ArrayField {
        ArrayField {
            field_enum: Box::new(field),
            length,
        }
    }
}

impl PointerField {
    pub fn new(serializer: &Serializer) -> PointerField {
        let decoder = if serializer.name == "CCSGameModeRules" {
            Decoder::GameModeRulesDecoder
        } else {
            Decoder::BooleanDecoder
        };
        PointerField {
            serializer: serializer.clone(),
            decoder,
        }
    }
}

impl SerializerField {
    pub fn new(serializer: &Serializer) -> SerializerField {
        SerializerField {
            serializer: serializer.clone(),
        }
    }
}

impl ValueField {
    pub fn new(decoder: Decoder, name: &str) -> ValueField {
        ValueField {
            decoder,
            name: name.to_string(),
            prop_id: 0,
            should_parse: false,
            full_name: "None ".to_string() + name,
        }
    }
}

impl VectorField {
    pub fn new(field: Field) -> VectorField {
        VectorField {
            field_enum: Box::new(field),
            decoder: Decoder::UnsignedDecoder,
        }
    }
}

fn field_from_msg(
    field: &ProtoFlattenedSerializerFieldT,
    serializer_msg: &CsvcMsgFlattenedSerializer,
    field_type: FieldType,
) -> Result<ConstructorField, DemoParserError> {
    let ser_name = if field.field_serializer_name_sym.is_some() {
        Some(serializer_msg.symbols.get(field.field_serializer_name_sym() as usize).ok_or_else(|| DemoParserError::MalformedMessage)?.clone())
    } else {
        None
    };
    let enc_name = if field.var_encoder_sym.is_some() {
        serializer_msg.symbols.get(field.var_encoder_sym() as usize).ok_or_else(|| DemoParserError::MalformedMessage)?.to_owned()
    } else {
        String::new()
    };

    let var_name = serializer_msg.symbols.get(field.var_name_sym() as usize).ok_or_else(|| DemoParserError::MalformedMessage)?.clone();
    let var_type = serializer_msg.symbols.get(field.var_type_sym() as usize).ok_or_else(|| DemoParserError::MalformedMessage)?.clone();
    let send_node = serializer_msg.symbols.get(field.send_node_sym() as usize).ok_or_else(|| DemoParserError::MalformedMessage)?.clone();

    Ok(ConstructorField {
        field_enum_type: None,
        bitcount: field.bit_count(),
        var_name,
        var_type,
        send_node,
        serializer_name: ser_name,
        encoder: enc_name,
        encode_flags: field.encode_flags(),
        low_value: field.low_value(),
        high_value: field.high_value(),

        field_type,
        serializer: None,
        decoder: Decoder::BaseDecoder,
        base_decoder: None,
        child_decoder: None,

        category: FieldCategory::Value,
    })
}

fn create_field(
    fd: &ConstructorField,
    serializers: &AHashMap<String, Serializer>,
) -> Result<Field, DemoParserError> {
    /*
    TODO
    let element_type = match fd.category {
        FieldCategory::Array => fd.field_type.element_type.as_ref(),
        FieldCategory::Vector => fd.field_type.generic_type.as_ref(),
        _ => Box::new(fd.field_type.clone()),
    };
    */
    let field = match fd.serializer_name.as_ref() {
        Some(name) => {
            let serializer = serializers.get(name.as_str()).ok_or_else(|| DemoParserError::MalformedMessage)?;
            if fd.category == FieldCategory::Pointer {
                Field::Pointer(PointerField::new(serializer))
            } else {
                Field::Serializer(SerializerField::new(serializer))
            }
        }
        None => Field::Value(ValueField::new(fd.decoder, &fd.var_name)),
    };
    match fd.category {
        FieldCategory::Array => Ok(Field::Array(ArrayField::new(field, fd.field_type.count.unwrap_or(0) as usize))),
        FieldCategory::Vector => Ok(Field::Vector(VectorField::new(field))),
        _ => Ok(field),
    }
}

fn find_field_type(name: &str) -> Result<FieldType, DemoParserError> {
    let captures = RE.captures(name).ok_or_else(|| DemoParserError::MalformedMessage)?;
    let base_type = captures.get(1).map_or_else(String::new, |s| s.as_str().to_owned());
    let pointer = POINTER_TYPES.contains(&name) || captures.get(4).is_some_and(|s| s.as_str() == "*");
    // let generic_type = match captures.get(3) {
    //     Some(generic) => Some(Box::new(find_field_type(generic.as_str())?)),
    //     None => None,
    // };
    let count = captures.get(6).map(|n| n.as_str().parse::<i32>().unwrap_or(0));

    Ok(FieldType {
        base_type,
        pointer,
        // generic_type,
        count,
    })
    // if count.is_some() {
    //     ft.element_type = Some(Box::new(for_string(field_type_map, to_string(&ft, true))?));
    // }
}

impl ConstructorField {
    fn find_decoder(&self, qf_map: &mut QfMapper) -> Decoder {
        if self.var_name == "m_iClip1" {
            return Decoder::AmmoDecoder;
        }
        match BASETYPE_DECODERS.get(&self.field_type.base_type) {
            Some(decoder) => *decoder,
            None => match self.field_type.base_type.as_str() {
                "float32" => self.find_float_decoder(qf_map),
                "Vector" => self.find_vector_type(3, qf_map),
                "Vector2D" => self.find_vector_type(2, qf_map),
                "Vector4D" => self.find_vector_type(4, qf_map),
                "uint64" => self.find_uint_decoder(),
                "QAngle" => self.find_qangle_decoder(),
                "CHandle" => Decoder::UnsignedDecoder,
                "CNetworkedQuantizedFloat" => self.find_float_decoder(qf_map),
                "CStrongHandle" => self.find_uint_decoder(),
                "CEntityHandle" => self.find_uint_decoder(),
                _ => Decoder::UnsignedDecoder,
            },
        }
    }

    fn find_qangle_decoder(&self) -> Decoder {
        if self.var_name == "m_angEyeAngles" {
            Decoder::QanglePitchYawDecoder
        } else if self.bitcount != 0 {
            Decoder::Qangle3Decoder
        } else {
            Decoder::QangleVarDecoder
        }
    }

    fn find_float_decoder(&self, qf_map: &mut QfMapper) -> Decoder {
        if self.encoder == "coord" {
            return Decoder::FloatCoordDecoder;
        }
        if self.var_name == "m_flSimulationTime" || self.var_name == "m_flAnimTime" || self.encoder == "m_flSimulationTime" {
            return Decoder::FloatSimulationTimeDecoder;
        }
        if self.bitcount <= 0 || self.bitcount >= 32 {
            return Decoder::NoscaleDecoder;
        }

        let qf = QuantalizedFloat::new(
            self.bitcount as u32,
            Some(self.encode_flags),
            Some(self.low_value),
            Some(self.high_value),
        );
        let idx = qf_map.idx;
        qf_map.map.insert(idx, qf);
        qf_map.idx += 1;
        Decoder::QuantalizedFloatDecoder(idx as u8)
    }

    fn find_uint_decoder(&self) -> Decoder {
        if self.encoder == "fixed64" {
            Decoder::Fixed64Decoder
        } else {
            Decoder::Unsigned64Decoder
        }
    }

    fn find_vector_type(&self, n: u32, qf_map: &mut QfMapper) -> Decoder {
        if n == 3 && self.encoder == "normal" {
            return Decoder::VectorNormalDecoder;
        }
        let float_type = self.find_float_decoder(qf_map);
        match float_type {
            Decoder::NoscaleDecoder => Decoder::VectorNoscaleDecoder,
            Decoder::FloatCoordDecoder => Decoder::VectorFloatCoordDecoder,
            // This one should not happen
            _ => Decoder::VectorNormalDecoder,
        }
    }
}

fn find_category(field: &ConstructorField) -> FieldCategory {
    if is_pointer(field) {
        FieldCategory::Pointer
    } else if is_vector(field) {
        FieldCategory::Vector
    } else if is_array(field) {
        FieldCategory::Array
    } else {
        FieldCategory::Value
    }
}

fn is_pointer(field: &ConstructorField) -> bool {
    field.field_type.pointer || POINTER_TYPES.contains(&field.field_type.base_type.as_str())
}

fn is_vector(field: &ConstructorField) -> bool {
    field.serializer_name.is_some() || matches!(field.field_type.base_type.as_str(), "CUtlVector" | "CNetworkUtlVectorBase")
}

fn is_array(field: &ConstructorField) -> bool {
    field.field_type.count.is_some() && field.field_type.base_type != "char"
}

// fn for_string(field_type_map: &mut AHashMap<String, FieldType>, field_type_string: String) -> Result<FieldType, DemoParserError> {
//     match field_type_map.get(&field_type_string) {
//         Some(s) => Ok(s.clone()),
//         None => {
//             let field_type = find_field_type(&field_type_string, field_type_map)?;
//             field_type_map.insert(field_type_string, field_type.clone());
//             Ok(field_type.clone())
//         }
//     }
// }

// fn to_string(ft: &FieldType, omit_count: bool) -> String {
//     // Function is rarely called
//     let mut s = ft.base_type.to_owned();

//     if let Some(gt) = &ft.generic_type {
//         s += "< ";
//         s += &to_string(gt, true);
//         s += " >";
//     }
//     if ft.pointer {
//         s += "*";
//     }
//     if !omit_count && ft.count.is_some() {
//         if let Some(c) = ft.count {
//             s += "[";
//             s += &c.to_string();
//             s += "]";
//         }
//     }
//     s
// }

const POINTER_TYPES: &[&str] = &[
    "CBodyComponent",
    "CLightComponent",
    "CPhysicsComponent",
    "CRenderComponent",
    "CPlayerLocalData",
];

#[derive(Debug, Clone)]
pub struct FieldType {
    pub base_type: String,
    // pub generic_type: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: Option<i32>,
    // pub element_type: Option<Box<FieldType>>,
}
