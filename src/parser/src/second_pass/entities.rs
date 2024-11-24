use crate::definitions::DemoParserError;
use crate::first_pass::prop_controller::PropController;
use crate::first_pass::prop_controller::FLATTENED_VEC_MAX_LEN;
use crate::first_pass::prop_controller::ITEM_PURCHASE_COST;
use crate::first_pass::prop_controller::ITEM_PURCHASE_COUNT;
use crate::first_pass::prop_controller::ITEM_PURCHASE_DEF_IDX;
use crate::first_pass::prop_controller::ITEM_PURCHASE_HANDLE;
use crate::first_pass::read_bits::Bitreader;
use crate::first_pass::sendtables::Field;
use crate::first_pass::sendtables::FieldInfo;
use crate::second_pass::game_events::GameEventInfo;
use crate::second_pass::game_events::RoundEnd;
use crate::second_pass::game_events::RoundWinReason;
use crate::second_pass::huffman_table::HUFFMAN_TREE;
use crate::second_pass::parser_settings::SecondPassParser;
use crate::second_pass::path_ops::FieldPath;
use crate::second_pass::variants::Variant;
use nohash::IntMap;
use csgoproto::CsvcMsgPacketEntities;
use prost::Message;

const NSERIALBITS: u32 = 17;
const STOP_READING_SYMBOL: u8 = 39;
const HUFFMAN_CODE_MAXLEN: u32 = 17;

#[derive(Debug, Clone)]
pub struct Entity {
    pub cls_id: u32,
    pub entity_id: i32,
    pub props: IntMap<u32, Variant>,
    pub entity_type: EntityType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerMetaData {
    pub player_entity_id: Option<i32>,
    pub steamid: Option<u64>,
    pub controller_entid: Option<i32>,
    pub name: Option<String>,
    pub team_num: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    PlayerController,
    Rules,
    Projectile,
    Team,
    Normal,
    C4,
}

impl<'a> SecondPassParser<'a> {
    pub fn parse_packet_ents(&mut self, bytes: &[u8], is_fullpacket: bool) -> Result<(), DemoParserError> {
        let msg = CsvcMsgPacketEntities::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;

        let mut bitreader = Bitreader::new(msg.entity_data());
        let mut entity_id: i32 = -1;
        let mut events_to_emit = vec![];
        for _ in 0..msg.updated_entries() {
            entity_id += 1 + (bitreader.read_u_bit_var()? as i32);

            // Read 2 bits to know which operation should be done to the entity.
            match bitreader.read_nbits(2)? {
                // Delete
                0b01 | 0b11 => {
                    self.projectiles.remove(&entity_id);
                    if let Some(entry) = self.entities.get_mut(entity_id as usize) {
                        *entry = None;
                    }
                },
                // CreateAndUpdate
                0b10 => {
                    self.create_new_entity(&mut bitreader, entity_id)?;
                    self.update_entity(&mut bitreader, entity_id, false, &mut events_to_emit, is_fullpacket)?;
                },
                // Update
                0b00 => {
                    // Most entities pass through here. Seems like entities that are not updated.
                    if msg.has_pvs_vis_bits() > 0 && bitreader.read_nbits(2)? & 0x01 == 1 {
                        continue;
                    }
                    self.update_entity(&mut bitreader, entity_id, false, &mut events_to_emit, is_fullpacket)?;
                },
                _ => return Err(DemoParserError::ImpossibleCmd),
            };
        }
        if !events_to_emit.is_empty() {
            self.emit_events(&events_to_emit)?;
        }
        Ok(())
    }

    fn update_entity(
        &mut self,
        bitreader: &mut Bitreader,
        entity_id: i32,
        is_baseline: bool,
        events_to_emit: &mut Vec<GameEventInfo>,
        is_fullpacket: bool,
    ) -> Result<(), DemoParserError> {
        let n_updates = self.parse_paths(bitreader)?;
        if n_updates == 0 {
            return Ok(())
        }

        let extract_event = !is_fullpacket && !is_baseline;
        self.decode_entity_update(bitreader, entity_id, n_updates, extract_event, events_to_emit)?;
        self.gather_extra_info(&entity_id, is_baseline)
    }

    #[inline(never)]
    fn parse_paths(&mut self, bitreader: &mut Bitreader) -> Result<usize, DemoParserError> {
        /*
        Create a field path by decoding using a Huffman tree.
        The huffman tree can be found at the bottom of entities_utils.rs

        A field path is a "path trough a struct" where
        the struct can have normal fields but also pointers
        to other (nested) structs.

        Example:

        The array will be filled with these:

        Struct Field{
            wanted_information: Option<T>,
            Pointer: bool,
            fields: Option<Vec<Field>>
        },

        (struct is simplified for this example. In reality it also includes field name etc.)


        Path to each of the fields in the below fields list: [
            [0], [1, 0], [1, 1], [2]
        ]
        and they would map to:
        [0] => FloatDecoder,
        [1, 0] => IntegerDecoder,
        [1, 1] => StringDecoder,
        [2] => VectorDecoder,

        fields = [
            Field{
                wanted_information: FloatDecoder,
                pointer: false,
                fields: None,
            },
            Field{
                wanted_information: None,
                pointer: true,
                fields: Some(
                    [
                        Field{
                            wanted_information: IntegerDecoder,
                            pointer: false,
                            fields: Some(
                        },
                        Field{
                            wanted_information: StringDecoder,
                            pointer: flase,
                            fields: Some(
                        }
                    ]
                ),
            },
            Field{
                wanted_information: VectorDecoder,
                pointer: false,
                fields: None,
            },
        ]
        Not sure what the maximum depth of these structs are, but others seem to use
        7 as the max length of field path so maybe that?

        Personally I find this path idea horribly complicated. Why is this chosen over
        the way it was done in source 1 demos?
        */

        // Create an "empty" path ([-1, 0, 0, 0, 0, 0, 0])
        // For perfomance reasons have them always the same len
        let mut field_path = FieldPath::default();
        let mut idx = 0;

        // Do huffman decoding with a lookup table instead of reading one bit at a time
        // and traversing a tree.
        // Here we peek ("HUFFMAN_CODE_MAXLEN" == 17) amount of bits and see from a table which
        // symbol it maps to and how many bits should be consumed from the stream.
        // The symbol is then mapped into an op for filling the field path.
        loop {
            if bitreader.bits_left < HUFFMAN_CODE_MAXLEN {
                bitreader.refill();
            }

            let peeked_bits = bitreader.peek(HUFFMAN_CODE_MAXLEN) as usize;
            let (symbol, code_len) = HUFFMAN_TREE[peeked_bits];
            bitreader.consume(code_len as u32);
            if symbol == STOP_READING_SYMBOL {
                break;
            }
            field_path.do_op(symbol, bitreader)?;
            self.write_fp(&mut field_path, idx)?;
            idx += 1;
        }
        Ok(idx)
    }

    #[inline(never)]
    fn decode_entity_update(
        &mut self,
        bitreader: &mut Bitreader,
        entity_id: i32,
        n_updates: usize,
        extract_event: bool,
        events_to_emit: &mut Vec<GameEventInfo>,
    ) -> Result<(), DemoParserError> {
        let Some(Some(entity)) = self.entities.get_mut(entity_id as usize) else {
            return Err(DemoParserError::EntityNotFound)
        };
        let serializer = self.serializer_by_cls_id.get(entity.cls_id as usize).ok_or_else(|| DemoParserError::ClassNotFound)?;

        for path in self.paths.iter().take(n_updates) {
            let field = serializer.get_field(path)?;
            let decoder = field.get_decoder()?;
            let result = bitreader.decode(&decoder, self.qf_mapper)?;
            let Some(field_info) = field.get_field_info(path) else { continue };

            if extract_event {
                if let Some(event) = SecondPassParser::extract_event(entity, &result, field_info.prop_id, self.prop_controller) {
                    events_to_emit.push(event);
                }
            }
            if self.is_debug_mode {
                SecondPassParser::debug_inspect(
                    &result,
                    field,
                    self.tick,
                    field_info,
                    path,
                    extract_event,
                    // class,
                    &entity.cls_id,
                    &entity_id,
                );
            }

            SecondPassParser::insert_field(entity, result, field_info);
        }
        Ok(())
    }

    fn extract_event(
        entity: &Entity,
        result: &Variant,
        prop_id: u32,
        prop_controller: &PropController,
    ) -> Option<GameEventInfo> {
        // Might want to start splitting this function
        if prop_controller.special_ids.round_end_count.is_some_and(|id| prop_id == id) {
            return Some(GameEventInfo::RoundEnd(RoundEnd {
                old_value: entity.props.get(&prop_id).cloned(),
                new_value: Some(result.clone()),
            }))
        }
        if prop_controller.special_ids.round_win_reason.is_some_and(|id| prop_id == id) {
            return if let Variant::I32(reason) = result {
                Some(GameEventInfo::RoundWinReason(RoundWinReason { reason: *reason }))
            } else {
                None
            }
        }
        if prop_controller.special_ids.round_start_count.is_some_and(|id| prop_id == id) {
            return Some(GameEventInfo::FreezePeriodStart(true))
        }
        if prop_controller.special_ids.match_end_count.is_some_and(|id| prop_id == id) {
            return Some(GameEventInfo::MatchEnd())
        }

        if (ITEM_PURCHASE_COST..ITEM_PURCHASE_COST + FLATTENED_VEC_MAX_LEN).contains(&prop_id) {
            return Some(GameEventInfo::WeaponCreateNCost((result.clone(), entity.entity_id)))
        }
        if (ITEM_PURCHASE_HANDLE..ITEM_PURCHASE_HANDLE + FLATTENED_VEC_MAX_LEN).contains(&prop_id) {
            return Some(GameEventInfo::WeaponCreateHitem((result.clone(), entity.entity_id)))
        }
        if (ITEM_PURCHASE_COUNT..ITEM_PURCHASE_COUNT + FLATTENED_VEC_MAX_LEN).contains(&prop_id) {
            return Some(GameEventInfo::WeaponPurchaseCount((
                result.clone(),
                entity.entity_id,
                prop_id,
            )))
        }
        if (ITEM_PURCHASE_DEF_IDX..ITEM_PURCHASE_DEF_IDX + FLATTENED_VEC_MAX_LEN).contains(&prop_id) {
            return Some(GameEventInfo::WeaponCreateDefIdx((
                result.clone(),
                entity.entity_id,
                prop_id,
            )))
        }

        None
    }

    #[allow(clippy::too_many_arguments)]
    fn debug_inspect(
        _result: &Variant,
        field: &Field,
        _tick: i32,
        _field_info: FieldInfo,
        _path: &FieldPath,
        _extract_event: bool,
        // _cls: &Class,
        _cls_id: &u32,
        _entity_id: &i32,
    ) {
        if let Field::Value(_v) = field {
            // if _v.full_name.contains("Services") {
            //     println!("{:?} {:?} {:?} {:?}", _path, field_info, _v.full_name, _result);
            // }
        }
    }

    fn insert_field(entity: &mut Entity, result: Variant, field_info: FieldInfo) {
        if field_info.should_parse {
            entity.props.insert(field_info.prop_id, result);
        }
    }

    #[inline]
    fn write_fp(&mut self, field_path: &mut FieldPath, idx: usize) -> Result<(), DemoParserError> {
        match self.paths.get_mut(idx) {
            Some(entry) => *entry = *field_path,
            // need to extend vec (rare)
            None => {
                self.paths.resize_with(idx + 1, FieldPath::default);
                let entry = self.paths.get_mut(idx).ok_or_else(|| DemoParserError::VectorResizeFailure)?;
                *entry = *field_path;
            }
        }
        Ok(())
    }

    fn create_new_entity(&mut self, bitreader: &mut Bitreader, entity_id: i32) -> Result<(), DemoParserError> {
        let cls_id: u32 = bitreader.read_nbits(8)?;
        // Both of these are not used. Don't think they are interesting for the parser
        let _serial = bitreader.read_nbits(NSERIALBITS)?;
        let _unknown = bitreader.read_varint();

        let entity_type = self.check_entity_type(cls_id)?;
        match entity_type {
            EntityType::Projectile => {
                self.projectiles.insert(entity_id);
            }
            EntityType::Rules => self.rules_entity_id = Some(entity_id),
            EntityType::C4 => self.c4_entity_id = Some(entity_id),
            _ => {}
        };

        if self.entities.len() as i32 <= entity_id {
            // if corrupt, this can cause oom allocations
            if entity_id > 100_000 {
                return Err(DemoParserError::VectorResizeFailure);
            }
            self.entities.resize(entity_id as usize + 1, None);
        }

        let entry = self.entities.get_mut(entity_id as usize).ok_or_else(|| DemoParserError::EntityNotFound)?;
        let entity = Entity {
            entity_id,
            cls_id,
            props: IntMap::default(),
            entity_type,
        };
        *entry = Some(entity);

        // Insert baselines
        if let Some(baseline_bytes) = self.baselines.get(&cls_id) {
            let b = &baseline_bytes.clone();
            let mut br = Bitreader::new(b);
            self.update_entity(&mut br, entity_id, true, &mut vec![], false)?;
        }
        Ok(())
    }

    fn check_entity_type(&self, cls_id: u32) -> Result<EntityType, DemoParserError> {
        let serializer = self.serializer_by_cls_id.get(cls_id as usize).ok_or_else(|| DemoParserError::ClassNotFound)?;
        match serializer.name.as_str() {
            "CCSPlayerController" => Ok(EntityType::PlayerController),
            "CCSGameRulesProxy" => Ok(EntityType::Rules),
            "CCSTeam" => Ok(EntityType::Team),
            "CC4" => Ok(EntityType::C4),
            _ => {
                if serializer.name.contains("Projectile") || serializer.name == "CIncendiaryGrenade" {
                    return Ok(EntityType::Projectile);
                }
                Ok(EntityType::Normal)
            }
        }
    }
}
