use crate::first_pass::sendtables::Field;
use crate::first_pass::sendtables::Serializer;
use crate::first_pass::sendtables::ValueField;
use crate::maps::BUTTONMAP;
use crate::maps::CUSTOM_PLAYER_PROP_IDS;
use crate::maps::TYPEHM;
use crate::second_pass::collect_data::PropType;
use crate::second_pass::parser_settings::SpecialIDs;
use crate::second_pass::variants::Variant;
use ahash::AHashMap;

pub const ENTITY_HANDLE_MISSING: i32 = 2047;
pub const SPECTATOR_TEAM_NUM: u32 = 1;
pub const BUTTONS_BASEID: u32 = 100000;
pub const NORMAL_PROP_BASEID: u32 = 1000;
pub const WEAPON_SKIN_NAME: u32 = 420420420;
pub const WEAPON_ORIGINGAL_OWNER_ID: u32 = 6942000;
pub const MY_WEAPONS_OFFSET: u32 = 500000;
pub const GRENADE_AMMO_ID: u32 = 1111111;
pub const INVENTORY_ID: u32 = 100000000;
pub const IS_ALIVE_ID: u32 = 100000001;
pub const GAME_TIME_ID: u32 = 100000002;
pub const ENTITY_ID_ID: u32 = 100000003;
pub const VELOCITY_X_ID: u32 = 100000004;
pub const VELOCITY_Y_ID: u32 = 100000005;
pub const VELOCITY_Z_ID: u32 = 100000006;
pub const VELOCITY_ID: u32 = 100000007;
pub const USERID_ID: u32 = 100000008;
pub const AGENT_SKIN_ID: u32 = 100000009;
pub const WEAPON_NAME_ID: u32 = 100000010;
pub const YAW_ID: u32 = 100000111;
pub const PITCH_ID: u32 = 100000012;
pub const TICK_ID: u32 = 100000013;
pub const STEAMID_ID: u32 = 100000014;
pub const NAME_ID: u32 = 100000015;
pub const PLAYER_X_ID: u32 = 100000016;
pub const PLAYER_Y_ID: u32 = 100000017;
pub const PLAYER_Z_ID: u32 = 100000018;
pub const WEAPON_STICKERS_ID: u32 = 100000019;
pub const INVENTORY_AS_IDS_ID: u32 = 100000020;
pub const IS_AIRBORNE_ID: u32 = 100000021;

pub const WEAPON_SKIN_ID: u32 = 10000000;
pub const WEAPON_PAINT_SEED: u32 = 10000001;
pub const WEAPON_FLOAT: u32 = 10000002;
pub const ITEM_PURCHASE_COUNT: u32 = 200000000;
pub const ITEM_PURCHASE_DEF_IDX: u32 = 300000000;
pub const ITEM_PURCHASE_COST: u32 = 400000000;
pub const ITEM_PURCHASE_HANDLE: u32 = 500000000;
pub const ITEM_PURCHASE_NEW_DEF_IDX: u32 = 600000000;
pub const FLATTENED_VEC_MAX_LEN: u32 = 100000;

pub const USERCMD_VIEWANGLE_X: u32 = 100000022;
pub const USERCMD_VIEWANGLE_Y: u32 = 100000023;
pub const USERCMD_VIEWANGLE_Z: u32 = 100000024;
pub const USERCMD_FORWARDMOVE: u32 = 100000025;
pub const USERCMD_IMPULSE: u32 = 100000026;
pub const USERCMD_MOUSE_DX: u32 = 100000027;
pub const USERCMD_MOUSE_DY: u32 = 100000028;
pub const USERCMD_BUTTONSTATE_1: u32 = 100000029;
pub const USERCMD_BUTTONSTATE_2: u32 = 100000030;
pub const USERCMD_BUTTONSTATE_3: u32 = 100000031;
pub const USERCMD_CONSUMED_SERVER_ANGLE_CHANGES: u32 = 100000032;
pub const USERCMD_LEFTMOVE: u32 = 100000033;
pub const USERCMD_WEAPON_SELECT: u32 = 100000034;
pub const USERCMD_SUBTICK_MOVE_ANALOG_FORWARD_DELTA: u32 = 100000035;
pub const USERCMD_SUBTICK_MOVE_ANALOG_LEFT_DELTA: u32 = 100000036;
pub const USERCMD_SUBTICK_MOVE_BUTTON: u32 = 100000037;
pub const USERCMD_SUBTICK_MOVE_WHEN: u32 = 100000038;
pub const USERCMD_SUBTICK_LEFT_HAND_DESIRED: u32 = 100000039;

pub const USERCMD_ATTACK_START_HISTORY_INDEX_1: u32 = 100000040;
pub const USERCMD_ATTACK_START_HISTORY_INDEX_2: u32 = 100000041;
pub const USERCMD_ATTACK_START_HISTORY_INDEX_3: u32 = 100000042;

pub const USERCMD_INPUT_HISTORY_BASEID: u32 = 100001000;
pub const INPUT_HISTORY_X_OFFSET: u32 = 0;
pub const INPUT_HISTORY_Y_OFFSET: u32 = 1;
pub const INPUT_HISTORY_Z_OFFSET: u32 = 2;
pub const INPUT_HISTORY_RENDER_TICK_COUNT_OFFSET: u32 = 3;
pub const INPUT_HISTORY_RENDER_TICK_FRACTION_OFFSET: u32 = 4;
pub const INPUT_HISTORY_PLAYER_TICK_COUNT_OFFSET: u32 = 5;
pub const INPUT_HISTORY_PLAYER_TICK_FRACTION_OFFSET: u32 = 6;

#[derive(Clone, Debug)]
pub struct PropController {
    pub id: u32,
    pub wanted_player_props: Vec<String>,
    pub prop_infos: Vec<PropInfo>,
    pub name_to_id: AHashMap<String, u32>,
    pub id_to_name: AHashMap<u32, String>,
    pub special_ids: SpecialIDs,
    pub real_name_to_og_name: AHashMap<String, String>,
    pub name_to_special_id: AHashMap<String, u32>,
    pub wanted_other_props: Vec<String>,
    pub needs_velocity: bool,
    pub path_to_name: AHashMap<[i32; 7], String>,
    pub wanted_prop_states: AHashMap<String, Variant>,
    pub wanted_prop_state_infos: Vec<WantedPropStateInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropInfo {
    pub id: u32,
    pub prop_type: PropType,
    pub prop_name: String,
    pub prop_friendly_name: String,
    pub is_player_prop: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WantedPropStateInfo {
    pub base: PropInfo,
    pub wanted_prop_state: Variant,
}

pub enum PropCollectionType {
    Player,
    Rules,
    Team,
}

impl PropController {
    pub fn new(
        wanted_player_props: Vec<String>,
        wanted_other_props: Vec<String>,
        wanted_prop_states: AHashMap<String, Variant>,
        real_name_to_og_name: AHashMap<String, String>,
        needs_velocity: bool,
        wanted_events: &[String],
    ) -> Self {
        PropController {
            id: NORMAL_PROP_BASEID,
            wanted_player_props,
            prop_infos: vec![],
            name_to_id: AHashMap::default(),
            special_ids: SpecialIDs::default(),
            id_to_name: AHashMap::default(),
            name_to_special_id: AHashMap::default(),
            wanted_other_props,
            real_name_to_og_name,
            needs_velocity: !wanted_events.is_empty() && needs_velocity,
            path_to_name: AHashMap::default(),
            wanted_prop_states,
            wanted_prop_state_infos: vec![],
        }
    }

    pub fn set_custom_propinfos(&mut self) {
        let mut someid = BUTTONS_BASEID;
        let mut someid2 = BUTTONS_BASEID;
        for button_name in BUTTONMAP.keys() {
            if self.wanted_player_props.contains(&button_name.to_string()) {
                self.prop_infos.push(PropInfo {
                    id: someid,
                    prop_type: PropType::Button,
                    prop_name: button_name.to_string(),
                    prop_friendly_name: button_name.to_string(),
                    is_player_prop: true,
                });
                someid += 1;
            }
            if let Some(wanted_state) = self.wanted_prop_states.get(&button_name.to_string()) {
                self.wanted_prop_state_infos.push(WantedPropStateInfo {
                    base: PropInfo {
                        id: someid2,
                        prop_type: PropType::Button,
                        prop_name: button_name.to_string(),
                        prop_friendly_name: button_name.to_string(),
                        is_player_prop: true,
                    },
                    wanted_prop_state: wanted_state.clone(),
                });
                someid2 += 1;
            }
        }

        for (custom_prop_name, custom_prop_id) in CUSTOM_PLAYER_PROP_IDS.entries() {
            if self.wanted_player_props.contains(&custom_prop_name.to_string()) {
                self.prop_infos.push(PropInfo {
                    id: *custom_prop_id,
                    prop_type: *TYPEHM.get(custom_prop_name).unwrap_or(&PropType::Custom),
                    prop_name: custom_prop_name.to_string(),
                    prop_friendly_name: self
                        .real_name_to_og_name
                        .get(&custom_prop_name.to_string())
                        .unwrap_or(&custom_prop_name.to_string())
                        .to_string(),
                    is_player_prop: true,
                })
            }
            if let Some(wanted_state) = self.wanted_prop_states.get(&custom_prop_name.to_string()) {
                self.wanted_prop_state_infos.push(WantedPropStateInfo {
                    base: PropInfo {
                        id: *custom_prop_id,
                        prop_type: *TYPEHM.get(custom_prop_name).unwrap_or(&PropType::Custom),
                        prop_name: custom_prop_name.to_string(),
                        prop_friendly_name: self
                            .real_name_to_og_name
                            .get(&custom_prop_name.to_string())
                            .unwrap_or(&custom_prop_name.to_string())
                            .to_string(),
                        is_player_prop: true,
                    },
                    wanted_prop_state: wanted_state.clone(),
                })
            }
        }

        if self.wanted_player_props.contains(&"game_time".to_string()) {
            self.prop_infos.push(PropInfo {
                id: GAME_TIME_ID,
                prop_type: PropType::GameTime,
                prop_name: "game_time".to_string(),
                prop_friendly_name: "game_time".to_string(),
                is_player_prop: true,
            });
        }
        if let Some(wanted_state) = self.wanted_prop_states.get(&"game_time".to_string()) {
            self.wanted_prop_state_infos.push(WantedPropStateInfo {
                base: PropInfo {
                    id: GAME_TIME_ID,
                    prop_type: PropType::GameTime,
                    prop_name: "game_time".to_string(),
                    prop_friendly_name: "game_time".to_string(),
                    is_player_prop: true,
                },
                wanted_prop_state: wanted_state.clone(),
            });
        }
        // Can also be non-player prop
        if self.wanted_other_props.contains(&"game_time".to_string()) {
            self.prop_infos.push(PropInfo {
                id: GAME_TIME_ID,
                prop_type: PropType::GameTime,
                prop_name: "game_time".to_string(),
                prop_friendly_name: "game_time".to_string(),
                is_player_prop: false,
            });
        }
        self.prop_infos.push(PropInfo {
            id: TICK_ID,
            prop_type: PropType::Tick,
            prop_name: "tick".to_string(),
            prop_friendly_name: "tick".to_string(),
            is_player_prop: true,
        });
        self.prop_infos.push(PropInfo {
            id: STEAMID_ID,
            prop_type: PropType::Steamid,
            prop_name: "steamid".to_string(),
            prop_friendly_name: "steamid".to_string(),
            is_player_prop: true,
        });
        self.prop_infos.push(PropInfo {
            id: NAME_ID,
            prop_type: PropType::Name,
            prop_name: "name".to_string(),
            prop_friendly_name: "name".to_string(),
            is_player_prop: true,
        });
    }

    pub fn find_prop_name_paths(&mut self, ser: &mut Serializer) {
        self.traverse_fields(&mut ser.fields, &ser.name, vec![])
    }

    fn set_id(&mut self, prop_name: &str, f: &mut ValueField, is_grenade_or_weapon: bool) {
        match self.name_to_id.get(prop_name) {
            // If we already have an id for prop of same name then use that id.
            // Mainly for weapon props. For example CAK47.m_iClip1 and CWeaponSCAR20.m_iClip1
            // are the "same" prop. (they have same path and we want to refer to it with one id not ~20)
            Some(id) => {
                f.prop_id = *id;
            }
            None => {
                self.name_to_id.insert(prop_name.to_string(), self.id);
                f.prop_id = self.id;
                self.insert_propinfo(prop_name, f);
                self.set_special_ids(prop_name, is_grenade_or_weapon, Some(f.prop_id));
            }
        }
        self.id_to_name.insert(f.prop_id, prop_name.to_string());
    }

    fn insert_propinfo(&mut self, prop_name: &str, f: &mut ValueField) {
        if let Some(prop_type) = TYPEHM.get(prop_name) {
            if self.wanted_player_props.contains(&prop_name.to_string()) {
                self.prop_infos.push(PropInfo {
                    id: f.prop_id,
                    prop_type: *prop_type,
                    prop_name: prop_name.to_string(),
                    prop_friendly_name: self
                        .real_name_to_og_name
                        .get(&prop_name.to_string())
                        .unwrap_or(&prop_name.to_string())
                        .to_string(),
                    is_player_prop: true,
                })
            }
            if self.wanted_other_props.contains(&prop_name.to_string()) {
                self.prop_infos.push(PropInfo {
                    id: f.prop_id,
                    prop_type: *prop_type,
                    prop_name: prop_name.to_string(),
                    prop_friendly_name: self
                        .real_name_to_og_name
                        .get(&prop_name.to_string())
                        .unwrap_or(&prop_name.to_string())
                        .to_string(),
                    is_player_prop: false,
                })
            }
            if let Some(wanted_state) = self.wanted_prop_states.get(&prop_name.to_string()) {
                self.wanted_prop_state_infos.push(WantedPropStateInfo {
                    base: PropInfo {
                        id: f.prop_id,
                        prop_type: *prop_type,
                        prop_name: prop_name.to_string(),
                        prop_friendly_name: self
                            .real_name_to_og_name
                            .get(&prop_name.to_string())
                            .unwrap_or(&prop_name.to_string())
                            .to_string(),
                        is_player_prop: true,
                    },
                    wanted_prop_state: wanted_state.clone(),
                });
            }
        }
    }

    fn handle_prop(&mut self, ser_name: &str, f: &mut ValueField, path: Vec<i32>) {
        f.full_name = ser_name.to_owned() + "." + &f.name;
        f.should_parse = true;

        // CAK47.m_iClip1 => ["CAK47", "m_iClip1"]
        let name_parts: Vec<&str> = f.full_name.split(".").collect();
        let is_player = name_parts[0].contains("Player");
        let is_weapon_prop = !is_player && (name_parts[0].contains("Weapon") || name_parts[0].contains("AK"))
            || name_parts[0].contains("Knife")
            || name_parts[0].contains("CDEagle")
            || name_parts[0].contains("C4")
            || name_parts[0].contains("Molo")
            || name_parts[0].contains("Inc")
            || name_parts[0].contains("Infer");

        let is_projectile_prop = !is_player && (name_parts[0].contains("Projectile") || name_parts[0].contains("Grenade") || name_parts[0].contains("Flash"));
        let is_grenade_or_weapon = is_weapon_prop || is_projectile_prop;

        // Strip first part of name from grenades and weapons.
        // if weapon prop: CAK47.m_iClip1 => m_iClip1
        // if grenade: CSmokeGrenadeProjectile.CBodyComponentBaseAnimGraph.m_cellX => CBodyComponentBaseAnimGraph.m_cellX
        let prop_name = if is_grenade_or_weapon {
            name_parts[1..].join(".")
        } else {
            f.full_name.to_owned()
        };
        self.set_id(&prop_name, f, is_grenade_or_weapon);

        match f.full_name.as_str() {
            "CCSPlayerPawn.CCSPlayer_WeaponServices.m_hMyWeapons" => f.prop_id = MY_WEAPONS_OFFSET,
            "CCSPlayerPawn.CCSPlayer_ActionTrackingServices.WeaponPurchaseCount_t.m_nCount" => f.prop_id = ITEM_PURCHASE_COUNT,
            "CCSPlayerPawn.CCSPlayer_ActionTrackingServices.WeaponPurchaseCount_t.m_nItemDefIndex" => f.prop_id = ITEM_PURCHASE_NEW_DEF_IDX,
            "CCSPlayerPawn.CCSPlayer_BuyServices.SellbackPurchaseEntry_t.m_unDefIdx" => f.prop_id = ITEM_PURCHASE_DEF_IDX,
            "CCSPlayerPawn.CCSPlayer_BuyServices.SellbackPurchaseEntry_t.m_nCost" => f.prop_id = ITEM_PURCHASE_COST,
            "CCSPlayerPawn.CCSPlayer_BuyServices.SellbackPurchaseEntry_t.m_hItem" => f.prop_id = ITEM_PURCHASE_HANDLE,
            _ => if prop_name.contains("CEconItemAttribute.m_iRawValue32") {
                f.prop_id = WEAPON_SKIN_ID
            },
        };

        let mut a = [0; 7];
        for (idx, v) in path.iter().enumerate() {
            a[idx] = *v;
        }
        self.path_to_name.insert(a, prop_name);
        self.id += 1;
    }

    #[inline(always)]
    fn set_special_ids(&mut self, name: &str, is_grenade_or_weapon: bool, id: Option<u32>) {
        if is_grenade_or_weapon {
            match name {
                "m_bIsIncGrenade" => self.special_ids.is_incendiary_grenade = id,
                "m_hOwnerEntity" => self.special_ids.h_owner_entity = id,
                "m_nOwnerId" => self.special_ids.grenade_owner_id = id,
                "CBodyComponentBaseAnimGraph.m_vecX" => self.special_ids.cell_x_offset_grenade = id,
                "CBodyComponentBaseAnimGraph.m_vecY" => self.special_ids.cell_y_offset_grenade = id,
                "CBodyComponentBaseAnimGraph.m_vecZ" => self.special_ids.cell_z_offset_grenade = id,
                "CBodyComponentBaseAnimGraph.m_cellX" => self.special_ids.cell_x_grenade = id,
                "CBodyComponentBaseAnimGraph.m_cellY" => self.special_ids.cell_y_grenade = id,
                "CBodyComponentBaseAnimGraph.m_cellZ" => self.special_ids.cell_z_grenade = id,
                "m_iItemDefinitionIndex" => self.special_ids.item_def = id,
                "m_OriginalOwnerXuidLow" => self.special_ids.orig_own_low = id,
                "m_OriginalOwnerXuidHigh" => self.special_ids.orig_own_high = id,
                "m_szCustomName" => self.special_ids.custom_name = id,
                _ => {}
            };
        } else {
            match name {
                "CCSTeam.m_iTeamNum" => self.special_ids.team_team_num = id,
                "CCSGameRulesProxy.CCSGameRules.m_nRoundStartCount" => self.special_ids.round_start_count = id,
                "CCSGameRulesProxy.CCSGameRules.m_nRoundEndCount" => self.special_ids.round_end_count = id,
                "CCSGameRulesProxy.CCSGameRules.m_nMatchEndCount" => self.special_ids.match_end_count = id,
                "CCSGameRulesProxy.CCSGameRules.m_eRoundWinReason" => self.special_ids.round_win_reason = id,
                "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed" => self.special_ids.total_rounds_played = id,
                "CBasePlayerWeapon.m_nOwnerId" => self.special_ids.weapon_owner_pointer = id,
                "CCSPlayerController.m_iTeamNum" => self.special_ids.teamnum = id,
                "CCSPlayerController.m_iszPlayerName" => self.special_ids.player_name = id,
                "CCSPlayerController.m_steamID" => self.special_ids.steamid = id,
                "CCSPlayerController.m_hPlayerPawn" => self.special_ids.player_pawn = id,
                "CCSPlayerController.m_nPawnCharacterDefIndex" => self.special_ids.agent_skin_idx = id,
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecX" => self.special_ids.cell_x_offset_player = id,
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecY" => self.special_ids.cell_y_offset_player = id,
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_vecZ" => self.special_ids.cell_z_offset_player = id,
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellX" => self.special_ids.cell_x_player = id,
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellY" => self.special_ids.cell_y_player = id,
                "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_cellZ" => self.special_ids.cell_z_player = id,
                "CCSPlayerPawn.CCSPlayer_MovementServices.m_nButtonDownMaskPrev" => self.special_ids.buttons = id,
                "CCSPlayerPawn.CCSPlayer_WeaponServices.m_hActiveWeapon" => self.special_ids.active_weapon = id,
                "CCSPlayerPawn.m_angEyeAngles" => self.special_ids.eye_angles = id,
                "CCSPlayerPawn.m_iTeamNum" => self.special_ids.player_team_pointer = id,
                "CCSPlayerPawn.m_lifeState" => self.special_ids.life_state = id,
                "CCSPlayerPawn.m_bInBuyZone" => self.special_ids.in_buy_zone = id,
                "CCSPlayerPawn.m_hGroundEntity" => self.special_ids.is_airborn = id,
                _ => {}
            };
        }
    }

    fn traverse_fields(&mut self, fields: &mut [Field], ser_name: &str, path_og: Vec<i32>) {
        for (idx, f) in fields.iter_mut().enumerate() {
            let mut path = path_og.clone();
            path.push(idx as i32);
            match f {
                Field::Value(f) => self.handle_prop(ser_name, f, path),
                Field::Serializer(ser) => self.traverse_fields(&mut ser.serializer.fields, &(ser_name.to_owned() + "." + &ser.serializer.name), path),
                Field::Pointer(ser) => self.traverse_fields(&mut ser.serializer.fields, &(ser_name.to_owned() + "." + &ser.serializer.name), path),
                Field::Array(ser) => {
                    if let Field::Value(f) = &mut ser.field_enum.as_mut() {
                        self.handle_prop(ser_name, f, path);
                    }
                },
                Field::Vector(_) => {
                    match f.get_inner_mut(0) {
                        Ok(Field::Serializer(s)) => {
                            for (inner_idx, f) in &mut s.serializer.fields.iter_mut().enumerate() {
                                if let Field::Value(f) = f {
                                    let mut inner_path = path.clone();
                                    inner_path.push(inner_idx as i32);
                                    self.handle_prop(ser_name, f, inner_path);
                                }
                            }
                            self.traverse_fields(&mut s.serializer.fields, &(ser_name.to_owned() + "." + &s.serializer.name), path_og.clone())
                        },
                        Ok(Field::Value(f)) => {
                            self.handle_prop(ser_name, f, path);
                        },
                        _ => {},
                    }
                }
                _ => {}
            }
        }
    }
}
