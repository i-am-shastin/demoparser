use crate::definitions::DemoParserError;
use crate::first_pass::prop_controller::PropInfo;
use crate::first_pass::prop_controller::ITEM_PURCHASE_COST;
use crate::first_pass::prop_controller::ITEM_PURCHASE_COUNT;
use crate::first_pass::prop_controller::ITEM_PURCHASE_DEF_IDX;
use crate::first_pass::prop_controller::ITEM_PURCHASE_NEW_DEF_IDX;
use crate::first_pass::prop_controller::ENTITY_HANDLE_MISSING;
use crate::first_pass::prop_controller::WEAPON_FLOAT;
use crate::first_pass::prop_controller::WEAPON_PAINT_SEED;
use crate::first_pass::stringtables::UserInfo;
use crate::maps::HIT_GROUP;
use crate::maps::ROUND_WIN_REASON;
use crate::maps::ROUND_WIN_REASON_TO_WINNER;
use crate::maps::WEAPINDICIES;
use crate::second_pass::collect_data::PropType;
use crate::second_pass::entities::PlayerMetaData;
use crate::second_pass::parser_settings::SecondPassParser;
use crate::second_pass::variants::*;
use csgoproto::CcsUsrMsgServerRankUpdate;
use csgoproto::csvc_msg_game_event::KeyT;
use csgoproto::CnetMsgSetConVar;
use csgoproto::CsvcMsgGameEvent;
use csgoproto::CUserMessageSayText;
use csgoproto::CUserMessageSayText2;
use itertools::Itertools;
use prost::Message;
use serde::Serialize;

static INTERNALEVENTFIELDS: &[&str] = &[
    "userid",
    "attacker",
    "assister",
    "userid_pawn",
    "attacker_pawn",
    "assister_pawn",
    "victim",
    "victim_pawn",
];

#[derive(Debug, Clone)]
pub struct RoundEnd {
    pub old_value: Option<Variant>,
    pub new_value: Option<Variant>,
}

#[derive(Debug, Clone)]
pub struct RoundWinReason {
    pub reason: i32,
}

#[derive(Debug, Clone)]
pub enum GameEventInfo {
    RoundEnd(RoundEnd),
    RoundWinReason(RoundWinReason),
    FreezePeriodStart(bool),
    MatchEnd(),
    WeaponCreateHitem((Variant, i32)),
    WeaponCreateNCost((Variant, i32)),
    WeaponCreateDefIdx((Variant, i32, u32)),
    WeaponPurchaseCount((Variant, i32, u32)),
}

static ENTITIES_FIRST_EVENTS: &[&str] = &["inferno_startburn", "decoy_started", "inferno_expire"];
static REMOVEDEVENTS: &[&str] = &["server_cvar"];
static DEFAULT_EVENT_FIELD_NAMES: &[&str] = &["tick", "name", "steamid"];

// https://developer.valvesoftware.com/wiki/SteamID
const STEAMID64INDIVIDUALIDENTIFIER: u64 = 0x0110000100000000;

impl<'a> SecondPassParser<'a> {
    pub fn parse_event(&mut self, bytes: &[u8]) -> Result<Option<GameEvent>, DemoParserError> {
        let event = CsvcMsgGameEvent::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;

        // Check if this events id is found in our game event list
        let Some(event_desc) = self.ge_list.get(&event.eventid()) else { return Ok(None) };
        if let Some(event_name) = &event_desc.name {
            self.game_events_counter.insert(event_name.to_owned());
        }

        // Return early if this is not a wanted event.
        if !self.wanted_events.contains(&event_desc.name().to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(None);
        }
        if REMOVEDEVENTS.contains(&event_desc.name()) {
            return Ok(None);
        }

        // Parsing game events is this easy, the complexity comes from adding "extra" fields into events.
        let mut event_fields = event.keys
            .iter()
            .zip(&event_desc.keys)
            .map(|(ge, desc)| {
                EventField {
                    name: desc.name().to_owned(),
                    data: parse_key(ge),
                }
            })
            .collect_vec();

        if ENTITIES_FIRST_EVENTS.contains(&event_desc.name()) {
            let event = GameEvent {
                fields: event_fields,
                name: event_desc.name().to_string(),
                tick: self.tick,
            };
            return Ok(Some(event));
        }

        // Add extra fields
        event_fields.extend(self.find_extra(&event_fields)?);
        // Remove fields that user does nothing with like userid and user_pawn
        event_fields.retain(|x| !INTERNALEVENTFIELDS.contains(&x.name.as_str()));
        event_fields.iter_mut().for_each(|field| self.cleanups(field));

        let event = GameEvent {
            fields: event_fields,
            name: event_desc.name().to_string(),
            tick: self.tick,
        };
        self.game_events.push(event);
        Ok(None)
    }

    #[inline(always)]
    fn cleanups(&self, field: &mut EventField) {
        // Contains some fixed like renaming weapons to be consitent.
        if field.name == "hitgroup" {
            if let Some(Variant::I32(i)) = field.data {
                let data = HIT_GROUP.get(&i).map_or_else(|| i.to_string(), |str| str.to_string());
                field.data = Some(Variant::String(data))
            }
        }
    }

    pub fn resolve_wrong_order_event(&mut self, events: &mut Vec<GameEvent>) -> Result<(), DemoParserError> {
        for event in events {
            event.fields.extend(self.find_extra(&event.fields)?);
            // Remove fields that user does nothing with like userid and user_pawn
            event.fields.retain(|x| !INTERNALEVENTFIELDS.contains(&x.name.as_str()));
            let event = GameEvent {
                fields: event.fields.clone(),
                name: event.name.to_string(),
                tick: self.tick,
            };
            self.game_events.push(event);
        }
        Ok(())
    }

    fn find_user_by_userid(&self, userid: i32) -> Option<&UserInfo> {
        self.stringtable_players.values()
            .find(|player| player.userid & 0xFF == userid)
            .or_else(|| {
                // Fallback for old demos?
                self.stringtable_players.values().find(|player| player.userid == userid)
            })
    }

    pub fn find_user_by_controller_id(&self, controller_id: i32) -> Option<&PlayerMetaData> {
        self.players.values().find(|player| player.controller_entid.is_some_and(|id| id == controller_id))
    }

    fn entity_id_from_userid(&self, userid: i32) -> Option<i32> {
        let user_steamid = self.find_user_by_userid(userid)?.steamid;
        for player in self.players.values() {
            if player.steamid.is_some_and(|id| id == user_steamid) && player.player_entity_id.is_some() {
                return player.player_entity_id;
            }
        }
        None
    }

    fn find_extra(&self, fields: &Vec<EventField>) -> Result<Vec<EventField>, DemoParserError> {
        // Always add tick to event
        let mut extra_fields = vec![
            EventField {
                name: "tick".to_owned(),
                data: Some(Variant::I32(self.tick)),
            }
        ];

        for field in fields {
            let Some(Variant::I32(id)) = field.data else { continue };

            // Fields that refer to players
            let prefix = match field.name.as_str() {
                "attacker" => "attacker",
                "userid" => "user",
                "assister" => "assister",
                "victim" => "victim",
                // edge case in some events
                "entityid" => {
                    if fields.iter().any(|x| x.name == "userid") { continue };
                    "user"
                }
                // Another edge case
                // Only add iff "userid" is missing in the event...
                "userid_pawn" => {
                    if fields.iter().any(|x| x.name == "userid" || x.name == "entityid") { continue };
                    "user"
                }
                _ => continue,
            };

            let entity_id = match field.name.as_str() {
                "entityid" => self.grenade_owner_entid_from_grenade(&field.data),
                "userid_pawn" => self.entity_id_from_user_pawn(id),
                _ => self.entity_id_from_userid(id),
            };

            if let Some(entity_id) = entity_id {
                extra_fields.extend(self.create_player_fields(entity_id, prefix));
                extra_fields.extend(self.find_extra_props_events(entity_id, prefix));
            } else {
                extra_fields.extend(self.generate_empty_fields(prefix));
            }
        }
        // Values from Teams and Rules entity. Not bound to any player so can be added to any event.
        extra_fields.extend(self.find_non_player_props());
        Ok(extra_fields)
    }

    fn entity_id_from_user_pawn(&self, pawn_handle: i32) -> Option<i32> {
        Some(pawn_handle & 0x7FF)
    }

    fn grenade_owner_entid_from_grenade(&self, id_field: &Option<Variant>) -> Option<i32> {
        let prop_id = self.prop_controller.special_ids.grenade_owner_id?;
        if let Some(Variant::I32(id)) = id_field {
            if let Ok(Variant::U32(entity_id)) = self.get_prop_from_ent(&prop_id, id) {
                return Some((entity_id & 0x7FF) as i32);
            }
        }
        None
    }

    fn generate_empty_fields(&self, prefix: &str) -> Vec<EventField> {
        // when pointer fails for some reason we need to add None to output
        let mut extra_fields = vec![
            EventField {
                name: prefix.to_owned() + "_steamid",
                data: None,
            },
            EventField {
                name: prefix.to_owned() + "_name",
                data: None,
            }
        ];
        for prop_info in &self.prop_controller.prop_infos {
            if !prop_info.is_player_prop {
                continue;
            }
            // These are meant for entities and should not be collected here
            if DEFAULT_EVENT_FIELD_NAMES.contains(&prop_info.prop_name.as_str()) {
                continue;
            }
            extra_fields.push(EventField {
                name: prefix.to_owned() + "_" + &prop_info.prop_friendly_name,
                data: None,
            });
        }
        extra_fields
    }

    fn find_non_player_props(&self) -> Vec<EventField> {
        self.prop_controller.prop_infos
            .iter()
            .flat_map(|prop_info| {
                match prop_info.prop_type {
                    PropType::Team => self.find_other_team_props(prop_info),
                    PropType::Rules => self.find_other_rules_props(prop_info),
                    PropType::GameTime => vec![EventField {
                        data: Some(Variant::F32(self.net_tick / 64.0)),
                        name: "game_time".to_string(),
                    }],
                    _ => vec![],
                }
            })
            .collect_vec()
    }

    fn find_other_rules_props(&self, prop_info: &PropInfo) -> Vec<EventField> {
        let data = self.rules_entity_id.and_then(|entid| self.get_prop_from_ent(&prop_info.id, &entid).ok());
        vec![EventField {
            name: prop_info.prop_friendly_name.to_owned(),
            data,
        }]
    }

    fn find_other_team_props(&self, prop_info: &PropInfo) -> Vec<EventField> {
        let t_prop = self.teams.team2_entid.and_then(|entid| self.get_prop_from_ent(&prop_info.id, &entid).ok());
        let ct_prop = self.teams.team3_entid.and_then(|entid| self.get_prop_from_ent(&prop_info.id, &entid).ok());
        vec![
            EventField {
                name: "t_".to_owned() + &prop_info.prop_friendly_name,
                data: t_prop,
            },
            EventField {
                name: "ct_".to_owned() + &prop_info.prop_friendly_name,
                data: ct_prop,
            }
        ]
    }

    fn find_extra_props_events(&self, entity_id: i32, prefix: &str) -> Vec<EventField> {
        self.prop_controller.prop_infos
            .iter()
            .filter_map(|prop_info| {
                // These props are collected in find_non_player_props()
                if !prop_info.is_player_prop {
                    return None;
                }
                // These are meant for entities and should not be collected here
                if DEFAULT_EVENT_FIELD_NAMES.contains(&prop_info.prop_name.as_str()) {
                    return None;
                }

                let data = self.players.get(&entity_id).and_then(|player_md| self.find_prop(prop_info, &entity_id, player_md).ok());
                Some(EventField {
                    name: prefix.to_owned() + "_" + &prop_info.prop_friendly_name,
                    data,
                })
            })
            .collect_vec()
    }

    fn create_player_fields(&self, entity_id: i32, prefix: &str) -> Vec<EventField> {
        let (name, steamid) = self.players.get(&entity_id).map_or((None, None), |player_md| (
            player_md.name.as_ref().map(|name| Variant::String(name.clone())),
            player_md.steamid.map(|steamid| Variant::String(steamid.to_string()))
        ));
        vec![
            EventField {
                name: prefix.to_owned() + "_name",
                data: name,
            },
            EventField {
                name: prefix.to_owned() + "_steamid",
                data: steamid,
            }
        ]
    }

    fn player_from_steamid32(&self, steamid32: i32) -> Option<i32> {
        for player in self.players.values() {
            if let Some(steamid) = player.steamid {
                if steamid - STEAMID64INDIVIDUALIDENTIFIER == steamid32 as u64 && player.player_entity_id.is_some() {
                    return player.player_entity_id;
                }
            }
        }
        None
    }

    pub fn create_custom_event_parse_convars(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        self.game_events_counter.insert("server_cvar".to_string());
        if !self.wanted_events.contains(&"server_cvar".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(());
        }

        let convar = CnetMsgSetConVar::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        if let Some(cv) = &convar.convars {
            let mut fields = cv.cvars
                .iter()
                .flat_map(|var| {
                    [
                        EventField {
                            data: Some(Variant::String(var.value().to_string())),
                            name: "value".to_string(),
                        },
                        EventField {
                            data: Some(Variant::String(var.name().to_string())),
                            name: "name".to_string(),
                        },
                    ]
                })
                .collect_vec();
            fields.extend(self.find_non_player_props());
            fields.push(EventField {
                data: Some(Variant::I32(self.tick)),
                name: "tick".to_string(),
            });
            let ge = GameEvent {
                name: "server_cvar".to_string(),
                fields,
                tick: self.tick,
            };
            self.game_events.push(ge);
        }
        Ok(())
    }

    fn contains_round_end_event(events: &[GameEventInfo]) -> bool {
        events.iter().any(|e| matches!(e, &GameEventInfo::RoundEnd(_)))
    }

    fn contains_freeze_period_start(events: &[GameEventInfo]) -> bool {
        events.iter().any(|e| matches!(e, &GameEventInfo::FreezePeriodStart(_)))
    }

    fn contains_match_end(events: &[GameEventInfo]) -> bool {
        events.iter().any(|e| matches!(e, &GameEventInfo::MatchEnd()))
    }

    fn contains_weapon_create(events: &[GameEventInfo]) -> bool {
        events.iter().any(|e| matches!(e, &GameEventInfo::WeaponCreateDefIdx(_)))
    }

    pub fn emit_events(&mut self, events: &[GameEventInfo]) -> Result<(), DemoParserError> {
        if SecondPassParser::contains_round_end_event(events) {
            self.create_custom_event_round_end(events)?;
        }
        if SecondPassParser::contains_freeze_period_start(events) {
            self.create_custom_event_round_officially_ended(events)?;
            self.create_custom_event_round_start(events)?;
        }
        if SecondPassParser::contains_match_end(events) {
            self.create_custom_event_match_end(events)?;
        }
        if SecondPassParser::contains_weapon_create(events) {
            self.create_custom_event_weapon_purchase(events);
        }
        self.create_custom_event_weapon_sold(events);
        Ok(())
    }

    fn create_custom_event_weapon_sold(&mut self, events: &[GameEventInfo]) {
        // This event is always emitted and is always removed in the end.
        for e in events  {
            let GameEventInfo::WeaponPurchaseCount((Variant::U32(0), entid, prop_id)) = e else { continue };
            let Ok(player) = self.find_player_metadata(*entid) else { continue };

            let inventory_slot = prop_id - ITEM_PURCHASE_COUNT;
            let weapon_name = self
                .get_prop_from_ent(&(ITEM_PURCHASE_NEW_DEF_IDX + inventory_slot), entid)
                .ok()
                .and_then(|v| {
                    if let Variant::U32(id) = v {
                        WEAPINDICIES.get(&id).map(|name| Variant::String(name.to_string()))
                    } else {
                        None
                    }
                });

            let mut fields = vec![
                EventField {
                    data: self.create_name(player).ok(),
                    name: "name".to_string(),
                },
                EventField {
                    data: Some(Variant::U64(player.steamid.unwrap_or(0))),
                    name: "steamid".to_string(),
                },
                EventField {
                    data: Some(Variant::I32(self.tick)),
                    name: "tick".to_string(),
                },
                EventField {
                    data: weapon_name,
                    name: "weapon_name".to_string(),
                },
                EventField {
                    data: self.get_prop_from_ent(&(ITEM_PURCHASE_COST + inventory_slot), entid).ok(),
                    name: "cost".to_string(),
                },
                EventField {
                    data: Some(Variant::U32(inventory_slot)),
                    name: "inventory_slot".to_string(),
                }
            ];
            fields.extend(self.find_extra_props_events(*entid, "user"));
            fields.extend(self.find_non_player_props());
            let ge = GameEvent {
                name: "item_sold".to_string(),
                fields,
                tick: self.tick,
            };
            self.game_events.push(ge);
            self.game_events_counter.insert("item_sold".to_string());
        }
    }

    fn combine_purchase_events(events: &[GameEventInfo]) -> Vec<PurchaseEvent> {
        // Vec<Gameventinfo> --> Vec<(def_idx, weapon_cost)>
        // Filter purchase events
        let filtered_events = events
            .iter()
            .filter(|e| {
                matches!(e, GameEventInfo::WeaponCreateDefIdx(_) | GameEventInfo::WeaponCreateNCost(_) | GameEventInfo::WeaponCreateHitem(_))
            })
            .collect_vec();
        let mut purchases = vec![];
        let mut ptr = 0;
        while ptr < filtered_events.len() {
            let entry_1 = filtered_events.get(ptr);
            let entry_2 = filtered_events.get(ptr + 1);
            let entry_3 = filtered_events.get(ptr + 2);

            match (entry_1, entry_2, entry_3) {
                (
                    Some(GameEventInfo::WeaponCreateDefIdx((Variant::U32(def), entid, prop_id))),
                    Some(GameEventInfo::WeaponCreateNCost((Variant::I32(cost), _))),
                    Some(GameEventInfo::WeaponCreateHitem((Variant::U64(handle), _))),
                ) => {
                    purchases.push(PurchaseEvent {
                        cost: *cost,
                        name: WEAPINDICIES.get(def).map(|n| n.to_string()),
                        entid: *entid,
                        weapon_entid: (handle & 0x7FF) as i32,
                        inventory_slot: (prop_id - ITEM_PURCHASE_DEF_IDX),
                    });
                    ptr += 3;
                }
                (
                    Some(GameEventInfo::WeaponCreateDefIdx((Variant::U32(def), entid, prop_id))),
                    Some(GameEventInfo::WeaponCreateNCost((Variant::I32(cost), _))),
                    _,
                ) => {
                    purchases.push(PurchaseEvent {
                        cost: *cost,
                        name: WEAPINDICIES.get(def).map(|n| n.to_string()),
                        entid: *entid,
                        weapon_entid: ENTITY_HANDLE_MISSING,
                        inventory_slot: (prop_id - ITEM_PURCHASE_DEF_IDX),
                    });
                    ptr += 2;
                }
                _ => ptr += 1,
            }
        }
        purchases
    }

    fn create_custom_event_weapon_purchase(&mut self, events: &[GameEventInfo]) {
        self.game_events_counter.insert("item_purchase".to_string());
        if !self.wanted_events.contains(&"item_purchase".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return;
        }

        let purchases = SecondPassParser::combine_purchase_events(events);
        for purchase in purchases {
            let Some(buy_zone_id) = self.prop_controller.special_ids.in_buy_zone else { continue };
            let Ok(Variant::Bool(true)) = self.get_prop_from_ent(&buy_zone_id, &purchase.entid) else { continue };
            let Ok(player) = self.find_player_metadata(purchase.entid) else { continue };

            let mut fields = vec![
                EventField {
                    data: purchase.name.map(Variant::String),
                    name: "item_name".to_string(),
                },
                EventField {
                    data: self.create_name(player).ok(),
                    name: "name".to_string(),
                },
                EventField {
                    data: Some(Variant::U64(player.steamid.unwrap_or(0))),
                    name: "steamid".to_string(),
                },
                EventField {
                    data: Some(Variant::U32(purchase.inventory_slot)),
                    name: "inventory_slot".to_string(),
                },
                EventField {
                    data: Some(Variant::I32(purchase.cost)),
                    name: "cost".to_string(),
                },
                EventField {
                    data: Some(Variant::I32(self.tick)),
                    name: "tick".to_string(),
                },
                EventField {
                    data: self.get_prop_from_ent(&WEAPON_FLOAT, &purchase.weapon_entid).ok(),
                    name: "float".to_string(),
                },
                EventField {
                    data: self.find_weapon_skin(&purchase.weapon_entid).ok(),
                    name: "skin".to_string(),
                },
                EventField {
                    data: self.find_weapon_skin_id(&purchase.weapon_entid).ok(),
                    name: "skin_id".to_string(),
                },
                EventField {
                    data: self.get_prop_from_ent(&WEAPON_PAINT_SEED, &purchase.weapon_entid).ok(),
                    name: "paint_seed".to_string(),
                },
                EventField {
                    data: self.find_stickers(&purchase.weapon_entid).ok(),
                    name: "stickers".to_string(),
                }
            ];
            let custom_name = self.prop_controller.special_ids.custom_name.and_then(|custom_name_id| {
                self.get_prop_from_ent(&custom_name_id, &purchase.weapon_entid).ok()}
            );
            fields.push(EventField {
                data: custom_name,
                name: "custom_name".to_string(),
            });
            fields.extend(self.find_extra_props_events(purchase.entid, "user"));
            fields.extend(self.find_non_player_props());
            let ge = GameEvent {
                name: "item_purchase".to_string(),
                fields,
                tick: self.tick,
            };
            self.game_events.push(ge);
        }
    }

    fn extract_win_reason(&self, events: &[GameEventInfo]) -> Option<Variant> {
        for event in events {
            let GameEventInfo::RoundWinReason(reason) = event else { continue };
            let reason = ROUND_WIN_REASON.get(&reason.reason).map_or_else(|| reason.reason.to_string(), |name| name.to_string());
            return Some(Variant::String(reason));
        }
        None
    }

    fn extract_winner(&self, events: &[GameEventInfo]) -> Option<Variant> {
        for event in events {
            let GameEventInfo::RoundWinReason(reason) = event else { continue };
            let winner = ROUND_WIN_REASON_TO_WINNER.get(&reason.reason).map_or_else(|| reason.reason.to_string(), |name| name.to_string());
            return Some(Variant::String(winner));
        }
        None
    }

    fn extract_round_end<'b>(&self, events: &'b [GameEventInfo]) -> Option<&'b RoundEnd> {
        for event in events {
            if let GameEventInfo::RoundEnd(round_end) = event {
                return Some(round_end);
            }
        }
        None
    }

    fn create_custom_event_round_end(&mut self, events: &[GameEventInfo]) -> Result<(), DemoParserError> {
        self.game_events_counter.insert("round_end".to_string());
        if !self.wanted_events.contains(&"round_end".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(());
        }

        let Some(event) = self.extract_round_end(events) else { return Ok(()) };

        if let (Some(Variant::U32(old)), Some(Variant::U32(new))) = (&event.old_value, &event.new_value) {
            if new - old != 1 {
                return Ok(());
            }
            let mut fields = vec![
                EventField {
                    data: Some(Variant::U32(old + 1)),
                    name: "round".to_string(),
                },
                EventField {
                    data: SecondPassParser::extract_win_reason(self, events),
                    name: "reason".to_string(),
                },
                EventField {
                    data: SecondPassParser::extract_winner(self, events),
                    name: "winner".to_string(),
                },
                EventField {
                    data: Some(Variant::I32(self.tick)),
                    name: "tick".to_string(),
                }
            ];
            fields.extend(self.find_non_player_props());
            let ge = GameEvent {
                name: "round_end".to_string(),
                fields,
                tick: self.tick,
            };
            self.game_events.push(ge);
        }
        Ok(())
    }

    fn create_custom_event_round_officially_ended(&mut self, _events: &[GameEventInfo]) -> Result<(), DemoParserError> {
        self.game_events_counter.insert("round_officially_ended".to_string());
        if !self.wanted_events.contains(&"round_officially_ended".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(());
        }

        // if round is 1 then we shouldn't publish `round_officially_ended`
        // as there is no prior round
        // keep an eye on this for potential bugs, possibly during match medic
        if let Some(Variant::I32(x)) = self.find_current_round() {
            if x <= 1 {
                return Ok(());
            }
        }

        let mut fields = self.find_non_player_props();
        fields.push(EventField {
            data: Some(Variant::I32(self.tick)),
            name: "tick".to_string(),
        });
        let ge = GameEvent {
            name: "round_officially_ended".to_string(),
            fields,
            tick: self.tick,
        };
        self.game_events.push(ge);

        Ok(())
    }

    fn create_custom_event_match_end(&mut self, _events: &[GameEventInfo]) -> Result<(), DemoParserError> {
        self.game_events_counter.insert("cs_win_panel_match".to_string());
        if !self.wanted_events.contains(&"cs_win_panel_match".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(());
        }

        let mut fields = self.find_non_player_props();
        fields.push(EventField {
            data: Some(Variant::I32(self.tick)),
            name: "tick".to_string(),
        });
        let ge = GameEvent {
            name: "cs_win_panel_match".to_string(),
            fields,
            tick: self.tick,
        };
        self.game_events.push(ge);

        Ok(())
    }

    fn find_current_round(&self) -> Option<Variant> {
        let prop_id = self.prop_controller.special_ids.total_rounds_played?;
        let entid = self.rules_entity_id?;

        match self.get_prop_from_ent(&prop_id, &entid) {
            Ok(Variant::I32(val)) => Some(Variant::I32(val + 1)),
            _ => None
        }
    }

    pub fn create_custom_event_chat_message(&mut self, msg_bytes: &[u8]) -> Result<(), DemoParserError> {
        self.game_events_counter.insert("chat_message".to_string());
        if !self.wanted_events.contains(&"chat_message".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(());
        }

        let chat_msg = CUserMessageSayText2::decode(msg_bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        let mut fields = vec![
            EventField {
                data: Some(Variant::String(chat_msg.param2().to_owned())),
                name: "chat_message".to_string(),
            },
            EventField {
                data: Some(Variant::I32(self.tick)),
                name: "tick".to_string(),
            }
        ];

        let controller_id = chat_msg.entityindex();
        let entity_id = self.find_user_by_controller_id(controller_id).map_or(i32::MAX, |md| md.player_entity_id.unwrap_or(i32::MAX));

        fields.extend(self.create_player_fields(entity_id, "user"));
        fields.extend(self.find_extra_props_events(entity_id, "user"));
        fields.extend(self.find_non_player_props());
        let ge = GameEvent {
            name: "chat_message".to_string(),
            fields,
            tick: self.tick,
        };
        self.game_events.push(ge);
        Ok(())
    }

    pub fn create_custom_event_server_message(&mut self, msg_bytes: &[u8]) -> Result<(), DemoParserError> {
        self.game_events_counter.insert("server_message".to_string());
        if !self.wanted_events.contains(&"server_message".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(());
        }

        let chat_msg = CUserMessageSayText::decode(msg_bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        let mut fields = vec![
            EventField {
                data: Some(Variant::String(chat_msg.text().to_owned())),
                name: "server_message".to_string(),
            },
            EventField {
                data: Some(Variant::I32(self.tick)),
                name: "tick".to_string(),
            }
        ];
        fields.extend(self.find_non_player_props());
        let ge = GameEvent {
            name: "server_message".to_string(),
            fields,
            tick: self.tick,
        };
        self.game_events.push(ge);
        Ok(())
    }

    fn create_custom_event_round_start(&mut self, _events: &[GameEventInfo]) -> Result<(), DemoParserError> {
        self.game_events_counter.insert("round_start".to_string());
        if !self.wanted_events.contains(&"round_start".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(());
        }

        let mut fields = vec![
            EventField {
                data: self.find_current_round(),
                name: "round".to_string(),
            },
            EventField {
                data: Some(Variant::I32(self.tick)),
                name: "tick".to_string(),
            }
        ];
        fields.extend(self.find_non_player_props());
        let ge = GameEvent {
            name: "round_start".to_string(),
            fields,
            tick: self.tick,
        };
        self.game_events.push(ge);
        Ok(())
    }

    pub fn create_custom_event_rank_update(&mut self, msg_bytes: &[u8]) -> Result<(), DemoParserError> {
        self.game_events_counter.insert("rank_update".to_string());
        if !self.wanted_events.contains(&"rank_update".to_string()) && self.wanted_events.first() != Some(&"all".to_string()) {
            return Ok(());
        }

        let update_msg = CcsUsrMsgServerRankUpdate::decode(msg_bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        for update in update_msg.rank_update {
            let Some(entity_id) = update.account_id.and_then(|id| self.player_from_steamid32(id)) else { continue };

            let mut fields = vec![
                EventField {
                    data: Some(Variant::I32(update.num_wins())),
                    name: "num_wins".to_string(),
                },
                EventField {
                    data: Some(Variant::I32(update.rank_old())),
                    name: "rank_old".to_string(),
                },
                EventField {
                    data: Some(Variant::I32(update.rank_new())),
                    name: "rank_new".to_string(),
                },
                EventField {
                    data: Some(Variant::F32(update.rank_change())),
                    name: "rank_change".to_string(),
                },
                EventField {
                    data: Some(Variant::I32(update.rank_type_id())),
                    name: "rank_type_id".to_string(),
                },
                EventField {
                    data: Some(Variant::I32(self.tick)),
                    name: "tick".to_string(),
                }
            ];

            fields.extend(self.create_player_fields(entity_id, "user"));
            fields.extend(self.find_extra_props_events(entity_id, "user"));
            fields.extend(self.find_non_player_props());
            let ge = GameEvent {
                name: "rank_update".to_string(),
                fields,
                tick: self.tick,
            };
            self.game_events.push(ge);
        }
        Ok(())
    }
}

// what is this shit
fn parse_key(key: &KeyT) -> Option<Variant> {
    match key.r#type() {
        1 => Some(Variant::String(key.val_string().to_owned())),
        2 => Some(Variant::F32(key.val_float())),
        // These seem to return an i32
        3 => Some(Variant::I32(key.val_long())),
        4 => Some(Variant::I32(key.val_short())),
        5 => Some(Variant::I32(key.val_byte())),
        6 => Some(Variant::Bool(key.val_bool())),
        7 => Some(Variant::U64(key.val_uint64())),
        8 => Some(Variant::I32(key.val_long())),
        9 => Some(Variant::I32(key.val_short())),
        _ => None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PurchaseEvent {
    pub entid: i32,
    pub cost: i32,
    pub name: Option<String>,
    pub weapon_entid: i32,
    pub inventory_slot: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EventField {
    pub name: String,
    pub data: Option<Variant>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GameEvent {
    #[serde(rename = "event_name")]
    pub name: String,
    #[serde(flatten)]
    pub fields: Vec<EventField>,
    pub tick: i32,
}
