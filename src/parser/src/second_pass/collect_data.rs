use crate::definitions::DemoParserError;
use super::entities::PlayerMetaData;
use super::variants::Sticker;
use super::variants::Variant;
use crate::first_pass::prop_controller::*;
use crate::maps::AGENTSMAP;
use crate::maps::BUTTONMAP;
use crate::maps::GRENADE_FRIENDLY_NAMES;
use crate::maps::PAINTKITS;
use crate::maps::PLAYER_COLOR;
use crate::maps::STICKER_ID_TO_NAME;
use crate::maps::WEAPINDICIES;
use crate::second_pass::entities::EntityType;
use crate::second_pass::parser_settings::SecondPassParser;
use crate::second_pass::variants::PropColumn;
use crate::second_pass::variants::VarVec;
use crate::serde_helper::as_string;
use itertools::Itertools;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PropType {
    Team,
    Rules,
    Custom,
    Controller,
    Player,
    Weapon,
    Button,
    Name,
    Steamid,
    Tick,
    GameTime,
}

// DONT KNOW IF THESE ARE CORRECT. SEEMS TO GIVE CORRECT VALUES
const CELL_BITS: i32 = 9;
const MAX_COORD: f32 = (1 << 14) as f32;
// https://github.com/markus-wa/demoinfocs-golang/blob/master/pkg/demoinfocs/constants/constants.go#L11
const IS_AIRBORNE_CONST: u32 = 0xFFFFFF;

#[derive(Debug, Serialize, Clone)]
pub struct ProjectileRecord {
    #[serde(serialize_with = "as_string")]
    pub steamid: Option<u64>,
    pub name: Option<String>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub tick: Option<i32>,
    pub grenade_type: Option<String>,
    pub entity_id: Option<i32>,
}

pub enum CoordinateAxis {
    X,
    Y,
    Z,
}

// This file collects the data that is converted into a dataframe in the end in parser.parse_ticks()
impl<'a> SecondPassParser<'a> {
    pub fn collect_entities(&mut self) {
        if !self.should_collect() {
            return;
        }
        if self.settings.parse_projectiles {
            self.collect_projectiles();
        }

        // iterate every player and every wanted prop name
        // if either one is missing then push None to output
        for (entity_id, player) in &self.players {
            let Some(player_steamid) = player.steamid else { continue };
            if !self.wanted_players.is_empty() && !self.wanted_players.contains(&player_steamid) {
                continue;
            }

            // iterate every wanted prop state
            // if any prop's state for this tick is not the wanted state, dont extract info from tick
            for info in &self.prop_controller.wanted_prop_state_infos {
                if !self.find_prop(&info.base, entity_id, player).is_ok_and(|state| state == info.wanted_prop_state) {
                    return;
                }
            }

            for prop_info in &self.prop_controller.prop_infos {
                let item = self.find_prop(prop_info, entity_id, player).ok();
                if self.settings.order_by_steamid {
                    self.prop_data_per_player.entry(player_steamid).or_default().entry(prop_info.id).or_default().push(item)
                } else {
                    self.prop_data.entry(prop_info.id).or_default().push(item)
                }
            }
        }
    }

    fn should_collect(&self) -> bool {
        self.prop_controller.needs_velocity || !self.has_wanted_events && (self.wanted_ticks.contains(&self.tick) || self.wanted_ticks.is_empty())
    }

    pub fn find_prop(&self, prop_info: &PropInfo, entity_id: &i32, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        match prop_info.prop_type {
            PropType::Tick => self.create_tick(),
            PropType::Name => self.create_name(player),
            PropType::Steamid => self.create_steamid(player),
            PropType::Player => self.get_prop_from_ent(&prop_info.id, entity_id),
            PropType::Team => self.find_team_prop(&prop_info.id, entity_id),
            PropType::Custom => self.create_custom_prop(prop_info.prop_name.as_str(), entity_id, prop_info, player),
            PropType::Weapon => self.find_weapon_prop(&prop_info.id, entity_id),
            PropType::Button => self.get_button_prop(prop_info, entity_id),
            PropType::Controller => self.get_controller_prop(&prop_info.id, player),
            PropType::Rules => self.get_rules_prop(prop_info),
            PropType::GameTime => Ok(Variant::F32(self.net_tick / 64.0)),
        }
    }

    pub fn get_prop_from_ent(&self, prop_id: &u32, entity_id: &i32) -> Result<Variant, PropCollectionError> {
        let Some(Some(e)) = self.entities.get(*entity_id as usize) else {
            return Err(PropCollectionError::GetPropFromEntEntityNotFound)
        };
        e.props
            .get(prop_id)
            .ok_or_else(|| PropCollectionError::GetPropFromEntPropNotFound)
            .cloned()
    }

    fn get_prop_variant(&self, prop_id: Option<u32>, entity_id: &i32) -> Option<Variant> {
        prop_id.and_then(|id| self.get_prop_from_ent(&id, entity_id).ok())
    }

    fn create_tick(&self) -> Result<Variant, PropCollectionError> {
        // This can't actually fail
        Ok(Variant::I32(self.tick))
    }

    fn create_steamid(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        match player.steamid {
            Some(steamid) => Ok(Variant::U64(steamid)),
            // Revisit this as it was related to pandas null support with u64's
            _ => Ok(Variant::U64(0)),
        }
    }

    pub fn create_name(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        let name = player.name.as_ref().ok_or_else(|| PropCollectionError::PlayerMetaDataNameNone)?;
        Ok(Variant::String(name.to_owned()))
    }

    fn get_button_prop(&self, prop_info: &PropInfo, entity_id: &i32) -> Result<Variant, PropCollectionError> {
        let button_id = self.prop_controller.special_ids.buttons.ok_or_else(|| PropCollectionError::ButtonsSpecialIDNone)?;
        self.get_prop_from_ent(&button_id, entity_id)
            .and_then(|v| {
                if let Variant::U64(button_mask) = v {
                    let button_flag = BUTTONMAP.get(&prop_info.prop_name).ok_or_else(|| PropCollectionError::ButtonsMapNoEntryFound)?;
                    Ok(Variant::Bool(button_mask & button_flag != 0))
                } else {
                    Err(PropCollectionError::ButtonMaskNotU64Variant)
                }
            })
    }

    fn get_rules_prop(&self, prop_info: &PropInfo) -> Result<Variant, PropCollectionError> {
        self.rules_entity_id
            .ok_or_else(|| PropCollectionError::RulesEntityIdNotSet)
            .and_then(|entid| self.get_prop_from_ent(&prop_info.id, &entid))
    }

    fn get_controller_prop(&self, prop_id: &u32, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        player.controller_entid
            .ok_or_else(|| PropCollectionError::ControllerEntityIdNotSet)
            .and_then(|entid| self.get_prop_from_ent(prop_id, &entid))
    }

    fn find_owner_entid(&self, entity_id: &i32) -> Result<u32, PropCollectionError> {
        let owner_id = self.prop_controller.special_ids.grenade_owner_id.ok_or_else(|| PropCollectionError::GrenadeOwnerIdNotSet)?;
        self.get_prop_from_ent(&owner_id, entity_id)
            .and_then(|v| {
                if let Variant::U32(prop) = v {
                    Ok(prop & 0x7FF)
                } else {
                    Err(PropCollectionError::GrenadeOwnerIdPropIncorrectVariant)
                }
            })
    }

    pub fn find_player_metadata(&self, entity_id: i32) -> Result<&PlayerMetaData, PropCollectionError> {
        self.players.get(&entity_id).ok_or_else(|| PropCollectionError::PlayerNotFound)
    }

    fn find_thrower_steamid(&self, entity_id: &i32) -> Result<u64, PropCollectionError> {
        let owner_entid = self.find_owner_entid(entity_id)?;
        let metadata = self.find_player_metadata(owner_entid as i32)?;
        match metadata.steamid {
            Some(s) => Ok(s),
            // Watch out
            None => Ok(0),
        }
    }

    fn find_thrower_name(&self, entity_id: &i32) -> Result<String, PropCollectionError> {
        let owner_entid = self.find_owner_entid(entity_id)?;
        let metadata = self.find_player_metadata(owner_entid as i32)?;
        metadata.name.as_ref()
            .map(|s| s.to_owned())
            .ok_or_else(|| PropCollectionError::PlayerMetaDataNameNone)
    }

    fn find_grenade_type(&self, entity_id: &i32) -> Option<String> {
        if let Some(Some(ent)) = self.entities.get(*entity_id as usize) {
            let serializer = self.serializer_by_cls_id.get(ent.cls_id as usize)?;
            // Seperate between ct and t molotovs
            if serializer.name == "CMolotovProjectile" {
                if let Some(Variant::Bool(true)) = self.get_prop_variant(self.prop_controller.special_ids.is_incendiary_grenade, entity_id) {
                    return Some("incendiary_grenade".to_string());
                }
            }
            let name = GRENADE_FRIENDLY_NAMES.get(&serializer.name)?;
            Some(name.to_string())
        } else {
            None
        }
    }

    fn collect_projectiles(&mut self) {
        for projectile_entid in &self.projectiles {
            let Some(grenade_type) = self.find_grenade_type(projectile_entid) else { continue };
            let steamid = self.find_thrower_steamid(projectile_entid).ok();
            let name = self.find_thrower_name(projectile_entid).ok();

            // Watch out with these
            let x = self.collect_cell_coordinate_grenade(CoordinateAxis::X, projectile_entid).ok();
            let y = self.collect_cell_coordinate_grenade(CoordinateAxis::Y, projectile_entid).ok();
            let z = self.collect_cell_coordinate_grenade(CoordinateAxis::Z, projectile_entid).ok();

            self.projectile_records.push(ProjectileRecord {
                steamid,
                name,
                x,
                y,
                z,
                tick: Some(self.tick),
                grenade_type: Some(grenade_type),
                entity_id: Some(*projectile_entid),
            });
        }
    }

    fn find_weapon_name(&self, entity_id: &i32) -> Result<Variant, PropCollectionError> {
        let item_def_id = self.prop_controller.special_ids.item_def.ok_or_else(|| PropCollectionError::SpecialidsItemDefNotSet)?;
        self.find_weapon_prop(&item_def_id, entity_id)
            .and_then(|v| match v {
                Variant::U32(def_idx) => {
                    WEAPINDICIES.get(&def_idx)
                        .ok_or_else(|| PropCollectionError::WeaponIdxMappingNotFound)
                        .map(|v| Variant::String(v.to_string()))
                }
                _ => Err(PropCollectionError::WeaponDefVariantWrongType),
            })
    }

    fn collect_cell_coordinate_player(&self, axis: CoordinateAxis, entity_id: &i32) -> Result<Variant, PropCollectionError> {
        let (prop_id, offset_id) = match axis {
            CoordinateAxis::X => (
                self.prop_controller.special_ids.cell_x_player.ok_or_else(|| PropCollectionError::PlayerSpecialIDCellXMissing)?,
                self.prop_controller.special_ids.cell_x_offset_player.ok_or_else(|| PropCollectionError::PlayerSpecialIDOffsetXMissing)?
            ),
            CoordinateAxis::Y => (
                self.prop_controller.special_ids.cell_y_player.ok_or_else(|| PropCollectionError::PlayerSpecialIDCellYMissing)?,
                self.prop_controller.special_ids.cell_y_offset_player.ok_or_else(|| PropCollectionError::PlayerSpecialIDOffsetYMissing)?
            ),
            CoordinateAxis::Z => (
                self.prop_controller.special_ids.cell_z_player.ok_or_else(|| PropCollectionError::PlayerSpecialIDCellZMissing)?,
                self.prop_controller.special_ids.cell_z_offset_player.ok_or_else(|| PropCollectionError::PlayerSpecialIDOffsetZMissing)?
            ),
        };
        let offset = self.get_prop_from_ent(&offset_id, entity_id);
        let cell = self.get_prop_from_ent(&prop_id, entity_id);
        Ok(Variant::F32(coord_from_cell(cell, offset)?))
    }

    fn collect_cell_coordinate_grenade(&self, axis: CoordinateAxis, entity_id: &i32) -> Result<f32, PropCollectionError> {
        let (prop_id, offset_id) = match axis {
            CoordinateAxis::X => (
                self.prop_controller.special_ids.cell_x_grenade.ok_or_else(|| PropCollectionError::GrenadeSpecialIDCellXMissing)?,
                self.prop_controller.special_ids.cell_x_offset_grenade.ok_or_else(|| PropCollectionError::GrenadeSpecialIDOffsetXMissing)?
            ),
            CoordinateAxis::Y => (
                self.prop_controller.special_ids.cell_y_grenade.ok_or_else(|| PropCollectionError::GrenadeSpecialIDCellYMissing)?,
                self.prop_controller.special_ids.cell_y_offset_grenade.ok_or_else(|| PropCollectionError::GrenadeSpecialIDOffsetYMissing)?
            ),
            CoordinateAxis::Z => (
                self.prop_controller.special_ids.cell_z_grenade.ok_or_else(|| PropCollectionError::GrenadeSpecialIDCellZMissing)?,
                self.prop_controller.special_ids.cell_z_offset_grenade.ok_or_else(|| PropCollectionError::GrenadeSpecialIDOffsetZMissing)?
            ),
        };
        let offset = self.get_prop_from_ent(&offset_id, entity_id);
        let cell = self.get_prop_from_ent(&prop_id, entity_id);
        coord_from_cell(cell, offset)
    }

    fn find_pitch_or_yaw(&self, entity_id: &i32, idx: usize) -> Result<Variant, PropCollectionError> {
        self.prop_controller.special_ids.eye_angles
            .ok_or_else(|| PropCollectionError::SpecialidsEyeAnglesNotSet)
            .and_then(|prop_id| {
                self.get_prop_from_ent(&prop_id, entity_id).and_then(|v| match v {
                    Variant::XYZVec(v) => Ok(Variant::F32(v[idx])),
                    _ => Err(PropCollectionError::EyeAnglesWrongVariant),
                })
            })
    }

    fn create_custom_prop(&self, prop_name: &str, entity_id: &i32, prop_info: &PropInfo, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        match prop_name {
            "X" => self.collect_cell_coordinate_player(CoordinateAxis::X, entity_id),
            "Y" => self.collect_cell_coordinate_player(CoordinateAxis::Y, entity_id),
            "Z" => self.collect_cell_coordinate_player(CoordinateAxis::Z, entity_id),
            "velocity" => self.collect_velocity(player),
            "velocity_X" => self.collect_velocity_axis(CoordinateAxis::X, player),
            "velocity_Y" => self.collect_velocity_axis(CoordinateAxis::Y, player),
            "velocity_Z" => self.collect_velocity_axis(CoordinateAxis::Z, player),
            "pitch" => self.find_pitch_or_yaw(entity_id, 0),
            "yaw" => self.find_pitch_or_yaw(entity_id, 1),
            "weapon_name" => self.find_weapon_name(entity_id),
            "weapon_skin" => self.find_weapon_skin_from_player(entity_id),
            "weapon_skin_id" => self.find_weapon_skin_id_from_player(entity_id),
            "weapon_paint_seed" => self.find_skin_paint_seed(player),
            "weapon_float" => self.find_skin_float(player),
            "weapon_stickers" => self.find_stickers_from_active_weapon(player),
            "active_weapon_original_owner" => self.find_weapon_original_owner(entity_id),
            "inventory" => self.find_my_inventory(entity_id),
            "inventory_as_ids" => self.find_my_inventory_as_ids(entity_id),
            "CCSPlayerPawn.m_bSpottedByMask" => self.find_spotted(entity_id, prop_info),
            "entity_id" => Ok(Variant::I32(*entity_id)),
            "is_alive" => Ok(Variant::Bool(self.find_is_alive(entity_id))),
            "user_id" => self.get_userid(player),
            "is_airborne" => self.find_is_airborne(player),
            "agent_skin" => self.find_agent_skin(player),
            "CCSPlayerController.m_iCompTeammateColor" => self.find_player_color(player, prop_info),
            "usercmd_input_history" => self.get_prop_from_ent(&USERCMD_INPUT_HISTORY_BASEID, entity_id),
            _ => Err(PropCollectionError::UnknownCustomPropName),
        }
    }

    fn get_userid(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        self.stringtable_players
            .values()
            .find(|user| player.steamid.is_some_and(|id| id == user.steamid))
            .ok_or_else(|| PropCollectionError::UseridNotFound)
            .map(|user| Variant::I32(user.userid))
    }

    fn find_player_color(&self, player: &PlayerMetaData, prop_info: &PropInfo) -> Result<Variant, PropCollectionError> {
        if let Ok(Variant::I32(v)) = self.get_controller_prop(&prop_info.id, player) {
            let color = if let Some(col) = PLAYER_COLOR.get(&v) {
                col.to_string()
            } else {
                v.to_string()
            };
            return Ok(Variant::String(color));
        }
        Err(PropCollectionError::UseridNotFound)
    }

    fn find_is_airborne(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        if let Some(entity_id) = &player.player_entity_id {
            if let Some(Variant::U32(value)) = self.get_prop_variant(self.prop_controller.special_ids.is_airborne, entity_id) {
                return Ok(Variant::Bool(value == IS_AIRBORNE_CONST));
            }
        }
        Ok(Variant::Bool(false))
    }

    fn find_skin_float(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        let player_entity_id = &player.player_entity_id.ok_or_else(|| PropCollectionError::PlayerNotFound)?;
        self.find_weapon_prop(&WEAPON_FLOAT, player_entity_id)
    }

    fn find_stickers_from_active_weapon(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        let prop_id = self.prop_controller.special_ids.active_weapon.ok_or_else(|| PropCollectionError::SpecialidsActiveWeaponNotSet)?;
        let entity_id = player.player_entity_id.ok_or_else(|| PropCollectionError::PlayerNotFound)?;
        self.get_prop_from_ent(&prop_id, &entity_id)
            .and_then(|v| match v {
                Variant::U32(weap_handle) => {
                    // Could be more specific
                    let weapon_entity_id = (weap_handle & 0x7FF) as i32;
                    self.find_stickers(&weapon_entity_id)
                },
                _ => Err(PropCollectionError::WeaponHandleIncorrectVariant)
            })
    }

    pub fn find_stickers(&self, weapon_entity_id: &i32) -> Result<Variant, PropCollectionError> {
        // indicies 0..4 info about skin. 4..24 info about stickers. 5 MAX STICKERS (4 idx per sticker),
        let stickers = (4..=24).step_by(4)
            .filter_map(|idx| {
                let sticker_id_id = WEAPON_SKIN_ID + idx;
                let sticker_wear_id = WEAPON_SKIN_ID + idx + 1;
                let sticker_x = WEAPON_SKIN_ID + idx + 2;
                let sticker_y = WEAPON_SKIN_ID + idx + 3;
                self.find_sticker(weapon_entity_id, sticker_id_id, sticker_wear_id, sticker_x, sticker_y)
            })
            .collect_vec();
        Ok(Variant::Stickers(stickers))
    }

    fn find_sticker(&self, entity_id: &i32, sticker_id_id: u32, sticker_wear_id: u32, sticker_x: u32, sticker_y: u32) -> Option<Sticker> {
        let id = self.get_prop_from_ent(&sticker_id_id, entity_id).ok()?;
        let wear = self.get_prop_from_ent(&sticker_wear_id, entity_id).ok()?;
        let sticker_x = self.get_prop_from_ent(&sticker_x, entity_id).ok()?;
        let sticker_y = self.get_prop_from_ent(&sticker_y, entity_id).ok()?;

        if let (Variant::F32(id), Variant::F32(wear), Variant::F32(x), Variant::F32(y)) = (id, wear, sticker_x, sticker_y) {
            Some(Sticker {
                id: id.to_bits(),
                name: STICKER_ID_TO_NAME.get(&id.to_bits()).unwrap_or(&"unknown").to_string(),
                wear: if wear < 0.0000000 { 0.0 } else { wear },
                x,
                y,
            })
        } else {
            None
        }
    }

    fn find_skin_paint_seed(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        if let Some(player_entity_id) = &player.player_entity_id {
            if let Variant::F32(f) = self.find_weapon_prop(&WEAPON_PAINT_SEED, player_entity_id)? {
                return Ok(Variant::U32(f as u32));
            }
        }
        Ok(Variant::U32(0))
    }

    fn find_agent_skin(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        let id = self.prop_controller.special_ids.agent_skin_idx.ok_or_else(|| PropCollectionError::AgentSpecialIdNotSet)?;
        match self.get_controller_prop(&id, player) {
            Ok(Variant::U32(agent_id)) => {
                AGENTSMAP.get(&agent_id)
                    .ok_or_else(|| PropCollectionError::AgentIdNotFound)
                    .map(|agent| Variant::String(agent.to_string()))
            },
            Ok(_) => Err(PropCollectionError::AgentIncorrectVariant),
            Err(_) => Err(PropCollectionError::AgentPropNotFound),
        }
    }

    fn collect_velocity(&self, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        if let Some(s) = player.steamid {
            let steamids = self.prop_data.get(&STEAMID_ID);
            let indicies = self.find_wanted_indicies(steamids, s);

            let x = self.velocity_from_indicies(&indicies, CoordinateAxis::X)?;
            let y = self.velocity_from_indicies(&indicies, CoordinateAxis::Y)?;

            if let (Variant::F32(x), Variant::F32(y)) = (x, y) {
                return Ok(Variant::F32((f32::powi(x, 2) + f32::powi(y, 2)).sqrt()));
            }
        }
        Err(PropCollectionError::PlayerNotFound)
    }

    fn collect_velocity_axis(&self, axis: CoordinateAxis, player: &PlayerMetaData) -> Result<Variant, PropCollectionError> {
        let player_steamid = player.steamid.ok_or_else(|| PropCollectionError::PlayerNotFound)?;
        let steamids = self.prop_data.get(&STEAMID_ID);
        let indicies = self.find_wanted_indicies(steamids, player_steamid);
        self.velocity_from_indicies(&indicies, axis)
    }

    fn find_most_recent_coordinate_idx(&self, optv: Option<&PropColumn>, wanted_steamid: u64) -> Option<usize> {
        if let Some(VarVec::U64(steamid_vec)) = &optv?.data {
            for idx in (0..steamid_vec.len()).rev() {
                if steamid_vec[idx].is_some_and(|id| id == wanted_steamid) {
                    return Some(idx);
                }
            }
        }
        None
    }

    fn find_last_coordinate_idx(&self, optv: Option<&PropColumn>, wanted_steamid: u64, cur_idx: Option<usize>) -> Option<usize> {
        let cur_idx = cur_idx?;
        if let VarVec::U64(steamid_vec) = optv?.data.as_ref()? {
            // iterate backwards until steamid is our wanted player and > 1sec ago
            for idx in (0..steamid_vec.len()).rev() {
                if steamid_vec[idx].is_some_and(|id| id == wanted_steamid) && idx != cur_idx {
                    return Some(idx);
                }
            }
        }
        None
    }

    fn find_wanted_indicies(&self, optv: Option<&PropColumn>, wanted_steamid: u64) -> Vec<usize> {
        let idx1 = self.find_most_recent_coordinate_idx(optv, wanted_steamid);
        let idx2 = self.find_last_coordinate_idx(optv, wanted_steamid, idx1);
        if let (Some(idx1), Some(idx2)) = (idx1, idx2) {
            return vec![idx1, idx2];
        }
        vec![]
    }

    fn velocity_from_indicies(&self, indicies: &[usize], axis: CoordinateAxis) -> Result<Variant, PropCollectionError> {
        let column = match axis {
            CoordinateAxis::X => self.prop_data.get(&PLAYER_X_ID),
            CoordinateAxis::Y => self.prop_data.get(&PLAYER_Y_ID),
            CoordinateAxis::Z => self.prop_data.get(&PLAYER_Z_ID),
        };
        if let Some(propcol) = column {
            if let Some((Some(v1), Some(v2))) = self.index_coordinates_from_propcol(propcol, indicies) {
                return Ok(Variant::F32((v1 * 64.0) - (v2 * 64.0)));
            }
        }
        Err(PropCollectionError::VelocityNotFound)
    }

    fn index_coordinates_from_propcol(&self, propcol: &PropColumn, indicies: &[usize]) -> Option<(Option<f32>, Option<f32>)> {
        if indicies.len() != 2 {
            return None;
        }
        if let Some(VarVec::F32(steamid_vec)) = &propcol.data {
            let first = steamid_vec[indicies[0]];
            let second = steamid_vec[indicies[1]];
            return Some((first, second));
        }
        None
    }

    fn find_is_alive(&self, entity_id: &i32) -> bool {
        self.prop_controller.special_ids.life_state.is_some_and(|id| {
            self.get_prop_from_ent(&id, entity_id).is_ok_and(|v| v == Variant::U32(0))
        })
    }

    fn find_spotted(&self, entity_id: &i32, prop_info: &PropInfo) -> Result<Variant, PropCollectionError> {
        self.get_prop_from_ent(&prop_info.id, entity_id)
            .and_then(|v| match v {
                Variant::U32(mask) => Ok(Variant::U64Vec(self.steamids_from_mask(mask))),
                _ => Err(PropCollectionError::SpottedIncorrectVariant),
            })
    }

    fn steamids_from_mask(&self, uid: u32) -> Vec<u64> {
        (0..16)
            .filter_map(|i| {
                if (uid & (1 << i)) == 0 {
                    return None
                }
                self.find_user_by_controller_id(i + 1).map(|user| user.steamid.unwrap_or(0))
            })
            .collect_vec()
    }

    fn find_my_inventory(&self, entity_id: &i32) -> Result<Variant, PropCollectionError> {
        let mut names = vec![];
        let mut unique_eids = vec![];

        if !self.find_is_alive(entity_id) {
            return Ok(Variant::StringVec(names));
        }
        let inventory_max_len = match self.get_prop_from_ent(&MY_WEAPONS_OFFSET, entity_id) {
            Ok(Variant::U32(p)) => p,
            _ => return Err(PropCollectionError::InventoryMaxNotFound),
        };
        for i in 1..=inventory_max_len {
            let prop_id = MY_WEAPONS_OFFSET + i;
            let Ok(Variant::U32(x)) = self.get_prop_from_ent(&prop_id, entity_id) else { continue };
            let eid = (x & ((1 << 14) - 1)) as i32;

            // Sometimes multiple references to same eid?
            if unique_eids.contains(&eid) {
                continue;
            }
            unique_eids.push(eid);

            let Some(item_def_id) = &self.prop_controller.special_ids.item_def else { continue };
            let Ok(prop) = self.get_prop_from_ent(item_def_id, &eid) else { continue };

            self.insert_equipment_name(&mut names, prop, entity_id);
        }
        Ok(Variant::StringVec(names))
    }

    fn find_my_inventory_as_ids(&self, entity_id: &i32) -> Result<Variant, PropCollectionError> {
        let mut ids = vec![];
        let mut unique_eids = vec![];

        if !self.find_is_alive(entity_id) {
            return Ok(Variant::U32Vec(ids));
        }
        let inventory_max_len = match self.get_prop_from_ent(&MY_WEAPONS_OFFSET, entity_id) {
            Ok(Variant::U32(p)) => p,
            _ => return Err(PropCollectionError::InventoryMaxNotFound),
        };
        for i in 1..=inventory_max_len {
            let prop_id = MY_WEAPONS_OFFSET + i;
            let Ok(Variant::U32(eid)) = self.get_prop_from_ent(&prop_id, entity_id) else { continue };
            let eid = (eid & ((1 << 14) - 1)) as i32;

            // Sometimes multiple references to same eid?
            if unique_eids.contains(&eid) {
                continue;
            }
            unique_eids.push(eid);

            let Some(item_def_id) = &self.prop_controller.special_ids.item_def else { continue };
            let Ok(prop) = self.get_prop_from_ent(item_def_id, &eid) else { continue };

            self.insert_equipment_id(&mut ids, prop, entity_id);
        }
        Ok(Variant::U32Vec(ids))
    }

    fn insert_equipment_id(&self, ids: &mut Vec<u32>, prop: Variant, player_entid: &i32) {
        let Variant::U32(def_idx) = prop else { return };
        let Some(weap_name) = WEAPINDICIES.get(&def_idx) else { return };

        match *weap_name {
            // Check how many flashbangs player has (only prop that works like this)
            "flashbang" => {
                if self.get_prop_from_ent(&GRENADE_AMMO_ID, player_entid).is_ok_and(|v| v == Variant::U32(2)) {
                    ids.push(def_idx);
                }
                ids.push(def_idx);
            }
            // c4 seems bugged. Find c4 entity and check owner from it.
            "c4" => {
                if self.find_c4_owner().is_some_and(|c4_owner_id| c4_owner_id == *player_entid) {
                    ids.push(def_idx);
                }
            }
            _ => ids.push(def_idx)
        }
    }

    fn insert_equipment_name(&self, names: &mut Vec<String>, prop: Variant, player_entid: &i32) {
        let Variant::U32(def_idx) = prop else { return };
        let Some(weap_name) = WEAPINDICIES.get(&def_idx) else { return };

        match *weap_name {
            // Check how many flashbangs player has (only prop that works like this)
            "flashbang" => {
                if self.get_prop_from_ent(&GRENADE_AMMO_ID, player_entid).is_ok_and(|v| v == Variant::U32(2)) {
                    names.push(weap_name.to_string());
                }
                names.push(weap_name.to_string());
            }
            // c4 seems bugged. Find c4 entity and check owner from it.
            "c4" => {
                if self.find_c4_owner().is_some_and(|c4_owner_id| c4_owner_id == *player_entid) {
                    names.push(weap_name.to_string());
                }
            }
            _ => names.push(weap_name.to_string())
        }
    }

    fn find_c4_owner(&self) -> Option<i32> {
        let c4ent = self.c4_entity_id?;
        let id = self.prop_controller.special_ids.h_owner_entity?;
        self.get_prop_from_ent(&id, &c4ent)
            .ok()
            .and_then(|var| {
                if let Variant::U32(u) = var {
                    Some((u & 0x7FF) as i32)
                } else {
                    None
                }
            })
    }

    fn find_weapon_original_owner(&self, entity_id: &i32) -> Result<Variant, PropCollectionError> {
        let low_id = self.prop_controller.special_ids.orig_own_low.ok_or_else(|| PropCollectionError::OriginalOwnerXuidIdLowNotSet)?;
        let high_id = self.prop_controller.special_ids.orig_own_high.ok_or_else(|| PropCollectionError::OriginalOwnerXuidIdHighNotSet)?;
        let low_bits = match self.find_weapon_prop(&low_id, entity_id) {
            Ok(Variant::U32(value)) => value,
            Ok(_) => return Err(PropCollectionError::OriginalOwnerXuidlowIncorrectVariant),
            Err(_e) => return Err(PropCollectionError::OriginalOwnerXuidLowNotFound),
        };
        let high_bits = match self.find_weapon_prop(&high_id, entity_id) {
            Ok(Variant::U32(value)) => value,
            Ok(_) => return Err(PropCollectionError::OriginalOwnerXuidHighIncorrectVariant),
            Err(_e) => return Err(PropCollectionError::OriginalOwnerXuidHighNotFound),
        };
        let combined = (high_bits as u64) << 32 | (low_bits as u64);
        Ok(Variant::String(combined.to_string()))
    }

    pub fn find_weapon_skin(&self, weapon_entity_id: &i32) -> Result<Variant, PropCollectionError> {
        self.get_prop_from_ent(&WEAPON_SKIN_ID, weapon_entity_id)
            .and_then(|var| {
                if let Variant::F32(f) = var {
                    // The value is stored as a float for some reason
                    if f.fract() == 0.0 && f >= 0.0 {
                        let idx = f as u32;
                        let kit = PAINTKITS.get(&idx).ok_or_else(|| PropCollectionError::WeaponSkinNoSkinMapping)?;
                        Ok(Variant::String(kit.to_string()))
                    } else {
                        Err(PropCollectionError::WeaponSkinFloatConvertionError)
                    }
                } else {
                    Err(PropCollectionError::WeaponSkinIdxIncorrectVariant)
                }
            })
    }

    fn find_weapon_skin_id_from_player(&self, player_entid: &i32) -> Result<Variant, PropCollectionError> {
        let prop_id = self.prop_controller.special_ids.active_weapon.ok_or_else(|| PropCollectionError::SpecialidsActiveWeaponNotSet)?;
        self.get_prop_from_ent(&prop_id, player_entid)
            .and_then(|var| {
                if let Variant::U32(weap_handle) = var {
                    let weapon_entity_id = (weap_handle & 0x7FF) as i32;
                    self.find_weapon_skin_id(&weapon_entity_id)
                } else {
                    Err(PropCollectionError::WeaponHandleIncorrectVariant)
                }
            })
    }

    pub fn find_weapon_skin_id(&self, weapon_entity_id: &i32) -> Result<Variant, PropCollectionError> {
        self.get_prop_from_ent(&WEAPON_SKIN_ID, weapon_entity_id)
            .and_then(|var| {
                if let Variant::F32(f) = var {
                    // The value is stored as a float for some reason
                    if f.fract() == 0.0 && f >= 0.0 {
                        Ok(Variant::U32(f as u32))
                    } else {
                        Err(PropCollectionError::WeaponSkinFloatConvertionError)
                    }
                } else {
                    Err(PropCollectionError::WeaponSkinIdxIncorrectVariant)
                }
            })
    }

    fn find_weapon_skin_from_player(&self, player_entid: &i32) -> Result<Variant, PropCollectionError> {
        let prop_id = self.prop_controller.special_ids.active_weapon.ok_or_else(|| PropCollectionError::SpecialidsActiveWeaponNotSet)?;
        self.get_prop_from_ent(&prop_id, player_entid)
            .and_then(|var| {
                if let Variant::U32(weap_handle) = var {
                    let weapon_entity_id = (weap_handle & 0x7FF) as i32;
                    self.find_weapon_skin(&weapon_entity_id)
                } else {
                    Err(PropCollectionError::WeaponHandleIncorrectVariant)
                }
            })
    }

    fn find_weapon_prop(&self, prop: &u32, player_entid: &i32) -> Result<Variant, PropCollectionError> {
        let prop_id = self.prop_controller.special_ids.active_weapon.ok_or_else(|| PropCollectionError::SpecialidsActiveWeaponNotSet)?;
        self.get_prop_from_ent(&prop_id, player_entid)
            .and_then(|var| {
                if let Variant::U32(weap_handle) = var {
                    // Could be more specific
                    let weapon_entity_id = (weap_handle & 0x7FF) as i32;
                    self.get_prop_from_ent(prop, &weapon_entity_id)
                        .map_err(|e| {
                            match e {
                                PropCollectionError::GetPropFromEntEntityNotFound => PropCollectionError::WeaponEntityNotFound,
                                PropCollectionError::GetPropFromEntPropNotFound => PropCollectionError::WeaponEntityWantedPropNotFound,
                                _ => e,
                            }
                        })
                } else {
                    Err(PropCollectionError::WeaponHandleIncorrectVariant)
                }
            })
    }

    fn find_team_prop(&self, prop: &u32, player_entid: &i32) -> Result<Variant, PropCollectionError> {
        let prop_id = self.prop_controller.special_ids.player_team_pointer.ok_or_else(|| PropCollectionError::SpecialidsPlayerTeamPointerNotSet)?;
        self.get_prop_from_ent(&prop_id, player_entid)
            .and_then(|var| {
                if let Variant::U32(team_num) = var {
                    let team_entid = match team_num {
                        // 1 should be spectator
                        1 => self.teams.team1_entid,
                        2 => self.teams.team2_entid,
                        3 => self.teams.team3_entid,
                        _ => return Err(PropCollectionError::IllegalTeamValue),
                    };
                    // Get prop from team entity
                    team_entid
                        .ok_or_else(|| PropCollectionError::TeamEntityIdNotSet)
                        .and_then(|eid| self.get_prop_from_ent(prop, &eid))
                } else {
                    Err(PropCollectionError::TeamNumIncorrectVariant)
                }
            })
    }

    pub fn gather_extra_info(&mut self, entity_id: &i32, is_baseline: bool) -> Result<(), DemoParserError> {
        // Boring stuff.. function does some bookkeeping
        let entity = match self.entities.get(*entity_id as usize) {
            Some(Some(entity)) => entity,
            _ => return Err(DemoParserError::EntityNotFound),
        };
        if !(entity.entity_type == EntityType::PlayerController || entity.entity_type == EntityType::Team) {
            return Ok(());
        }

        if entity.entity_type == EntityType::Team && !is_baseline {
            if let Some(Variant::U32(t)) = self.get_prop_variant(self.prop_controller.special_ids.team_team_num, entity_id) {
                match t {
                    1 => self.teams.team1_entid = Some(*entity_id),
                    2 => self.teams.team2_entid = Some(*entity_id),
                    3 => self.teams.team3_entid = Some(*entity_id),
                    _ => {}
                }
            }
        }

        let team_num = match self.get_prop_variant(self.prop_controller.special_ids.teamnum, entity_id) {
            Some(Variant::U32(team_num)) => Some(team_num),
            Some(_) => return Err(DemoParserError::IncorrectMetaDataProp),
            None => None,
        };
        let name = match self.get_prop_variant(self.prop_controller.special_ids.player_name, entity_id) {
            Some(Variant::String(name)) => Some(name),
            Some(_) => return Err(DemoParserError::IncorrectMetaDataProp),
            None => None,
        };
        let steamid = match self.get_prop_variant(self.prop_controller.special_ids.steamid, entity_id) {
            Some(Variant::U64(steamid)) => Some(steamid),
            Some(_) => return Err(DemoParserError::IncorrectMetaDataProp),
            None => None,
        };
        let player_entity_id = match self.get_prop_variant(self.prop_controller.special_ids.player_pawn, entity_id) {
            Some(Variant::U32(handle)) => Some((handle & 0x7FF) as i32),
            Some(_) => return Err(DemoParserError::IncorrectMetaDataProp),
            None => None,
        };

        if let Some(e) = player_entity_id {
            if !(e == ENTITY_HANDLE_MISSING || steamid.is_some_and(|id| id == 0) || team_num.is_some_and(|num| num == SPECTATOR_TEAM_NUM)) {
                if let Some(eid) = self.should_remove(steamid) {
                    self.players.remove(&eid);
                }
                self.players.insert(
                    e,
                    PlayerMetaData {
                        name,
                        team_num,
                        player_entity_id,
                        steamid,
                        controller_entid: Some(*entity_id),
                    },
                );
            }
        }
        Ok(())
    }

    fn should_remove(&self, steamid: Option<u64>) -> Option<i32> {
        for (entid, player) in &self.players {
            if player.steamid == steamid {
                return Some(*entid);
            }
        }
        None
    }
}

fn coord_from_cell(cell: Result<Variant, PropCollectionError>, offset: Result<Variant, PropCollectionError>) -> Result<f32, PropCollectionError> {
    // Both cell and offset are needed for calculation
    match (offset, cell) {
        (Ok(Variant::F32(offset)), Ok(Variant::U32(cell))) => {
            let cell_coord = (cell as f32 * (1 << CELL_BITS) as f32) - MAX_COORD;
            Ok(cell_coord + offset)
        }
        (Err(_), Err(_)) => Err(PropCollectionError::CoordinateBothNone),
        (Ok(Variant::F32(_offset)), Err(_)) => Err(PropCollectionError::CoordinateCellNone),
        (Err(_), Ok(Variant::U32(_cell))) => Err(PropCollectionError::CoordinateOffsetNone),
        (_, _) => Err(PropCollectionError::CoordinateIncorrectTypes),
    }
}

#[derive(Debug, PartialEq)]
pub enum PropCollectionError {
    PlayerSpecialIDCellXMissing,
    PlayerSpecialIDCellYMissing,
    PlayerSpecialIDCellZMissing,
    PlayerSpecialIDOffsetXMissing,
    PlayerSpecialIDOffsetYMissing,
    PlayerSpecialIDOffsetZMissing,
    GrenadeSpecialIDCellXMissing,
    GrenadeSpecialIDCellYMissing,
    GrenadeSpecialIDCellZMissing,
    GrenadeSpecialIDOffsetXMissing,
    GrenadeSpecialIDOffsetYMissing,
    GrenadeSpecialIDOffsetZMissing,
    CoordinateOffsetNone,
    CoordinateCellNone,
    CoordinateIncorrectTypes,
    CoordinateBothNone,
    GrenadeOffsetVariantNone,
    PlayerMetaDataNameNone,
    ButtonsSpecialIDNone,
    ButtonsMapNoEntryFound,
    GetPropFromEntEntityNotFound,
    GetPropFromEntPropNotFound,
    ButtonMaskNotU64Variant,
    RulesEntityIdNotSet,
    ControllerEntityIdNotSet,
    SpecialidsEyeAnglesNotSet,
    SpecialidsItemDefNotSet,
    EyeAnglesWrongVariant,
    WeaponIdxMappingNotFound,
    WeaponDefVariantWrongType,
    SpecialidsPlayerTeamPointerNotSet,
    TeamNumIncorrectVariant,
    IllegalTeamValue,
    TeamEntityIdNotSet,
    GrenadeOwnerIdNotSet,
    GrenadeOwnerIdPropIncorrectVariant,
    PlayerNotFound,
    SpecialidsActiveWeaponNotSet,
    WeaponHandleIncorrectVariant,
    UnknownCustomPropName,
    UnknownCoordinateAxis,
    WeaponEntityNotFound,
    WeaponEntityWantedPropNotFound,
    WeaponSkinFloatConvertionError,
    WeaponSkinNoSkinMapping,
    WeaponSkinIdxIncorrectVariant,
    OriginalOwnerXuidIdLowNotSet,
    OriginalOwnerXuidIdHighNotSet,
    OriginalOwnerXuidLowNotFound,
    OriginalOwnerXuidHighNotFound,
    OriginalOwnerXuidlowIncorrectVariant,
    OriginalOwnerXuidHighIncorrectVariant,
    SpottedIncorrectVariant,
    VelocityNotFound,
    AgentIdNotFound,
    AgentIncorrectVariant,
    AgentPropNotFound,
    AgentSpecialIdNotSet,
    UseridNotFound,
    InventoryMaxNotFound,
}

impl std::error::Error for PropCollectionError {}

impl fmt::Display for PropCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
