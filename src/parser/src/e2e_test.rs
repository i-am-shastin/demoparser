#![allow(non_snake_case)]

use crate::first_pass::parser_settings::ParserInputs;
use crate::first_pass::prop_controller::PropController;
use crate::first_pass::prop_controller::*;
use crate::parse_demo::DemoOutput;
use crate::parse_demo::Parser;
use crate::second_pass::game_events::GameEvent;
use crate::second_pass::parser_settings::create_huffman_lookup_table;
use ahash::AHashMap;
use itertools::Itertools;
use memmap2::MmapOptions;
use std::collections::BTreeMap;
use std::fs::File;

pub fn _create_ge_tests() {
    let wanted_props = vec![
        "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_flLastTeleportTime".to_string(),
        "CCSPlayerPawn.CCSPlayer_BulletServices.m_totalHitsOnServer".to_string(),
        "CCSPlayerPawn.CCSPlayer_ItemServices.m_bHasDefuser".to_string(),
        "CCSPlayerPawn.CCSPlayer_ItemServices.m_bHasHelmet".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_arrForceSubtickMoveWhen".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDesiresDuck".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDuckOverride".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bOldJumpPressed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_fStashGrenadeParameterWhen".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckAmount".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckSpeed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpUntil".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpVel".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flOffsetTickCompleteTime".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flOffsetTickStashedSpeed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flStamina".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_nButtonDownMaskPrev".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_nLadderSurfacePropIndex".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_vecLadderNormal".to_string(),
        "CCSPlayerPawn.CCSPlayer_WeaponServices.m_iAmmo".to_string(),
        "CCSPlayerPawn.m_ArmorValue".to_string(),
        "CCSPlayerPawn.m_MoveType".to_string(),
        "CCSPlayerPawn.m_aimPunchAngle".to_string(),
        "CCSPlayerPawn.m_aimPunchAngleVel".to_string(),
        "CCSPlayerPawn.m_aimPunchTickBase".to_string(),
        "CCSPlayerPawn.m_aimPunchTickFraction".to_string(),
        "CCSPlayerPawn.m_angEyeAngles".to_string(),
        "CCSPlayerPawn.m_bClientRagdoll".to_string(),
        "CCSPlayerPawn.m_bClientSideRagdoll".to_string(),
        "CCSPlayerPawn.m_bHasMovedSinceSpawn".to_string(),
        "CCSPlayerPawn.m_bInBombZone".to_string(),
        "CCSPlayerPawn.m_bInBuyZone".to_string(),
        "CCSPlayerPawn.m_bIsBuyMenuOpen".to_string(),
        "CCSPlayerPawn.m_bIsDefusing".to_string(),
        "CCSPlayerPawn.m_bIsScoped".to_string(),
        "CCSPlayerPawn.m_bIsWalking".to_string(),
        "CCSPlayerPawn.m_bKilledByHeadshot".to_string(),
        "CCSPlayerPawn.m_bRagdollDamageHeadshot".to_string(),
        "CCSPlayerPawn.m_bResumeZoom".to_string(),
        "CCSPlayerPawn.m_bSpotted".to_string(),
        "CCSPlayerPawn.m_bSpottedByMask".to_string(),
        "CCSPlayerPawn.m_bWaitForNoAttack".to_string(),
        "CCSPlayerPawn.m_fFlags".to_string(),
        "CCSPlayerPawn.m_fMolotovDamageTime".to_string(),
        "CCSPlayerPawn.m_flCreateTime".to_string(),
        "CCSPlayerPawn.m_flDeathTime".to_string(),
        "CCSPlayerPawn.m_flEmitSoundTime".to_string(),
        "CCSPlayerPawn.m_flFlashDuration".to_string(),
        "CCSPlayerPawn.m_flFlashMaxAlpha".to_string(),
        "CCSPlayerPawn.m_flHitHeading".to_string(),
        "CCSPlayerPawn.m_flProgressBarStartTime".to_string(),
        "CCSPlayerPawn.m_flSlopeDropHeight".to_string(),
        "CCSPlayerPawn.m_flSlopeDropOffset".to_string(),
        "CCSPlayerPawn.m_flTimeOfLastInjury".to_string(),
        "CCSPlayerPawn.m_flVelocityModifier".to_string(),
        "CCSPlayerPawn.m_iHealth".to_string(),
        "CCSPlayerPawn.m_iMoveState".to_string(),
        "CCSPlayerPawn.m_iPlayerState".to_string(),
        "CCSPlayerPawn.m_iProgressBarDuration".to_string(),
        "CCSPlayerPawn.m_iShotsFired".to_string(),
        "CCSPlayerPawn.m_iTeamNum".to_string(),
        "CCSPlayerPawn.m_lifeState".to_string(),
        "CCSPlayerPawn.m_nCollisionFunctionMask".to_string(),
        "CCSPlayerPawn.m_nEnablePhysics".to_string(),
        "CCSPlayerPawn.m_nEntityId".to_string(),
        "CCSPlayerPawn.m_nForceBone".to_string(),
        "CCSPlayerPawn.m_nHierarchyId".to_string(),
        "CCSPlayerPawn.m_nHitBodyPart".to_string(),
        "CCSPlayerPawn.m_nInteractsAs".to_string(),
        "CCSPlayerPawn.m_nInteractsExclude".to_string(),
        "CCSPlayerPawn.m_nInteractsWith".to_string(),
        "CCSPlayerPawn.m_nLastConcurrentKilled".to_string(),
        "CCSPlayerPawn.m_nLastKillerIndex".to_string(),
        "CCSPlayerPawn.m_nRagdollDamageBone".to_string(),
        "CCSPlayerPawn.m_nWhichBombZone".to_string(),
        "CCSPlayerPawn.m_qDeathEyeAngles".to_string(),
        "CCSPlayerPawn.m_szLastPlaceName".to_string(),
        "CCSPlayerPawn.m_szRagdollDamageWeaponName".to_string(),
        "CCSPlayerPawn.m_thirdPersonHeading".to_string(),
        "CCSPlayerPawn.m_ubInterpolationFrame".to_string(),
        "CCSPlayerPawn.m_unCurrentEquipmentValue".to_string(),
        "CCSPlayerPawn.m_unFreezetimeEndEquipmentValue".to_string(),
        "CCSPlayerPawn.m_unRoundStartEquipmentValue".to_string(),
        "CCSPlayerPawn.m_vDecalForwardAxis".to_string(),
        "CCSPlayerPawn.m_vDecalPosition".to_string(),
        "CCSPlayerPawn.m_vHeadConstraintOffset".to_string(),
        "CCSPlayerPawn.m_vRagdollDamageForce".to_string(),
        "CCSPlayerPawn.m_vRagdollDamagePosition".to_string(),
        "CCSPlayerPawn.m_vRagdollServerOrigin".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bBombDropped".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bBombPlanted".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bFreezePeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bGameRestart".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bHasMatchStarted".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bTeamIntroPeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bWarmupPeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_eRoundWinReason".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fMatchStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fRoundStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodEnd".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodStart".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_flGameStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_flRestartRoundTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_gamePhase".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_CT".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_T".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_RoundResults".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveCTLoses".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveTerroristLoses".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iRoundTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iRoundWinStatus".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_nRoundsPlayedThisPhase".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_timeUntilNextPhaseStarts".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iAssists".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iCashEarned".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDeaths".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEnemiesFlashed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEquipmentValue".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iHeadShotKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKillReward".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iLiveTime".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iMoneySaved".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iObjective".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iUtilityDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iAssists".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iCashEarned".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDeaths".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemiesFlashed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemy3Ks".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEquipmentValue".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iHeadShotKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKillReward".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iLiveTime".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iMoneySaved".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iNumRoundKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iNumRoundKillsHeadshots".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iObjective".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iUtilityDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_unTotalRoundDamageDealt".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iAccount".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iCashSpentThisRound".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iStartAccount".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iTotalCashSpent".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsFriendly".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsLeader".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsTeacher".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicLevel".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_rank".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_unMusicID".to_string(),
        "CCSPlayerController.m_bCanControlObservedBot".to_string(),
        "CCSPlayerController.m_bEverPlayedOnTeam".to_string(),
        "CCSPlayerController.m_bPawnHasDefuser".to_string(),
        "CCSPlayerController.m_bPawnHasHelmet".to_string(),
        "CCSPlayerController.m_bPawnIsAlive".to_string(),
        "CCSPlayerController.m_fFlags".to_string(),
        "CCSPlayerController.m_flCreateTime".to_string(),
        "CCSPlayerController.m_hOriginalControllerOfCurrentPawn".to_string(),
        "CCSPlayerController.m_hPawn".to_string(),
        "CCSPlayerController.m_hPlayerPawn".to_string(),
        "CCSPlayerController.m_iCompTeammateColor".to_string(),
        "CCSPlayerController.m_iCompetitiveRankType".to_string(),
        "CCSPlayerController.m_iCompetitiveRanking".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Loss".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Tie".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Win".to_string(),
        "CCSPlayerController.m_iCompetitiveWins".to_string(),
        "CCSPlayerController.m_iConnected".to_string(),
        "CCSPlayerController.m_iMVPs".to_string(),
        "CCSPlayerController.m_iPawnArmor".to_string(),
        "CCSPlayerController.m_iPawnHealth".to_string(),
        "CCSPlayerController.m_iPawnLifetimeEnd".to_string(),
        "CCSPlayerController.m_iPawnLifetimeStart".to_string(),
        "CCSPlayerController.m_iPendingTeamNum".to_string(),
        "CCSPlayerController.m_iPing".to_string(),
        "CCSPlayerController.m_iScore".to_string(),
        "CCSPlayerController.m_iTeamNum".to_string(),
        "CCSPlayerController.m_iszPlayerName".to_string(),
        "CCSPlayerController.m_nDisconnectionTick".to_string(),
        "CCSPlayerController.m_nPawnCharacterDefIndex".to_string(),
        "CCSPlayerController.m_nQuestProgressReason".to_string(),
        "CCSPlayerController.m_nTickBase".to_string(),
        "CCSPlayerController.m_steamID".to_string(),
        "CCSPlayerController.m_szCrosshairCodes".to_string(),
        "CCSPlayerController.m_unActiveQuestId".to_string(),
        "CCSPlayerController.m_unPlayerTvControlFlags".to_string(),
        "CBodyComponentBaseAnimGraph.m_MeshGroupMask".to_string(),
        "CBodyComponentBaseAnimGraph.m_angRotation".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellX".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellY".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellZ".to_string(),
        "CBodyComponentBaseAnimGraph.m_hParent".to_string(),
        "CBodyComponentBaseAnimGraph.m_hSequence".to_string(),
        "CBodyComponentBaseAnimGraph.m_nAnimLoopMode".to_string(),
        "CBodyComponentBaseAnimGraph.m_nIdealMotionType".to_string(),
        "CBodyComponentBaseAnimGraph.m_nNewSequenceParity".to_string(),
        "CBodyComponentBaseAnimGraph.m_nRandomSeedOffset".to_string(),
        "CBodyComponentBaseAnimGraph.m_nResetEventsParity".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecX".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecY".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecZ".to_string(),
        "CEconItemAttribute.m_bSetBonus".to_string(),
        "CEconItemAttribute.m_flInitialValue".to_string(),
        "CEconItemAttribute.m_iAttributeDefinitionIndex".to_string(),
        "CEconItemAttribute.m_iRawValue32".to_string(),
        "CEconItemAttribute.m_nRefundableCurrency".to_string(),
        "m_MoveType".to_string(),
        "m_OriginalOwnerXuidHigh".to_string(),
        "m_OriginalOwnerXuidLow".to_string(),
        "m_bBurstMode".to_string(),
        "m_bInReload".to_string(),
        "m_bReloadVisuallyComplete".to_string(),
        "m_fAccuracyPenalty".to_string(),
        "m_fEffects".to_string(),
        "m_fLastShotTime".to_string(),
        "m_flCreateTime".to_string(),
        "m_flDroppedAtTime".to_string(),
        "m_flFireSequenceStartTime".to_string(),
        "m_flNextPrimaryAttackTickRatio".to_string(),
        "m_flNextSecondaryAttackTickRatio".to_string(),
        "m_flRecoilIndex".to_string(),
        "m_flSimulationTime".to_string(),
        "m_flTimeSilencerSwitchComplete".to_string(),
        "m_hOuter".to_string(),
        "m_hOwnerEntity".to_string(),
        "m_hPrevOwner".to_string(),
        "m_iAccountID".to_string(),
        "m_iClip1".to_string(),
        "m_iClip2".to_string(),
        "m_iEntityQuality".to_string(),
        "m_iInventoryPosition".to_string(),
        "m_iIronSightMode".to_string(),
        "m_iItemIDHigh".to_string(),
        "m_iItemIDLow".to_string(),
        "m_iState".to_string(),
        "m_nAddDecal".to_string(),
        "m_nCollisionFunctionMask".to_string(),
        "m_nDropTick".to_string(),
        "m_nEnablePhysics".to_string(),
        "m_nEntityId".to_string(),
        "m_nFireSequenceStartTimeChange".to_string(),
        "m_nHierarchyId".to_string(),
        "m_nInteractsAs".to_string(),
        "m_nNextPrimaryAttackTick".to_string(),
        "m_nNextSecondaryAttackTick".to_string(),
        "m_nNextThinkTick".to_string(),
        "m_nOwnerId".to_string(),
        "m_nSubclassID".to_string(),
        "m_nViewModelIndex".to_string(),
        "m_pReserveAmmo".to_string(),
        "m_ubInterpolationFrame".to_string(),
        "m_usSolidFlags".to_string(),
        "m_vDecalForwardAxis".to_string(),
        "m_vDecalPosition".to_string(),
        "m_weaponMode".to_string(),
        "X".to_string(),
        "Y".to_string(),
        "Z".to_string(),
        "velocity".to_string(),
        "velocity_X".to_string(),
        "velocity_Y".to_string(),
        "velocity_Z".to_string(),
        "pitch".to_string(),
        "yaw".to_string(),
        "weapon_name".to_string(),
        "weapon_skin".to_string(),
        "weapon_skin_id".to_string(),
        "active_weapon_original_owner".to_string(),
        "inventory".to_string(),
        "inventory_as_ids".to_string(),
        "entity_id".to_string(),
        "is_alive".to_string(),
        "user_id".to_string(),
        "agent_skin".to_string(),
        "is_airborne".to_string(),
        "usercmd_viewangle_x".to_string(),
        "usercmd_viewangle_y".to_string(),
        "usercmd_viewangle_z".to_string(),
        "usercmd_forward_move".to_string(),
        "usercmd_left_move".to_string(),
        "usercmd_impulse".to_string(),
        "usercmd_mouse_dx".to_string(),
        "usercmd_mouse_dy".to_string(),
        "usercmd_buttonstate_1".to_string(),
        "usercmd_buttonstate_2".to_string(),
        "usercmd_buttonstate_3".to_string(),
        "usercmd_weapon_select".to_string(),
        "usercmd_left_hand_desired".to_string(),
        "usercmd_input_history".to_string(),
    ];

    let wanted_events = vec!["all".to_string()];
    let huf = create_huffman_lookup_table();

    let settings = ParserInputs {
        wanted_player_props: wanted_props.clone(),
        wanted_events: wanted_events,
        real_name_to_og_name: AHashMap::default(),
        wanted_other_props: vec![],
        parse_ents: true,
        wanted_players: vec![],
        wanted_ticks: (0..5).into_iter().map(|x| x * 10000).collect_vec(),
        wanted_prop_states: AHashMap::default(),
        parse_projectiles: true,
        only_header: false,
        count_props: false,
        only_convars: false,
        huffman_lookup_table: &huf,
        order_by_steamid: false,
    };

    let mut ds = Parser::new(settings, crate::parse_demo::ParsingMode::ForceMultiThreaded);
    // ds.is_multithreadable = false;
    let file = File::open("test_demo.dem".to_string()).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let d = ds.parse_demo(&mmap).unwrap();

    let _v = d.game_events.iter().filter(|x| x.name != "wplayer_hurt").collect_vec();
    let events = vec![
        "hltv_versioninfo",
        "round_freeze_end",
        "weapon_reload",
        "cs_pre_restart",
        "weapon_fire",
        "player_death",
        "smokegrenade_expired",
        "item_equip",
        "bomb_planted",
        "bomb_exploded",
        "round_prestart",
        "cs_round_final_beep",
        "smokegrenade_detonate",
        "player_footstep",
        "buytime_ended",
        "player_jump",
        "weapon_zoom",
        "round_poststart",
        "bomb_pickup",
        "player_blind",
        "bomb_begindefuse",
        "inferno_startburn",
        "player_disconnect",
        "player_hurt",
        "bomb_beginplant",
        "round_officially_ended",
        "item_pickup",
        "player_spawn",
        "other_death",
        "bomb_defused",
        "begin_new_match",
        "cs_win_panel_round",
        "cs_win_panel_match",
        "cs_round_start_beep",
        "bomb_dropped",
        "inferno_expire",
        "round_end",
        "round_start",
        "round_time_warning",
        "flashbang_detonate",
        "round_mvp",
        "round_announce_match_start",
        "hegrenade_detonate",
        "usercmd_viewangle_x",
        "usercmd_viewangle_y",
        "usercmd_viewangle_z",
        "usercmd_forward_move",
        "usercmd_left_move",
        "usercmd_impulse",
        "usercmd_mouse_dx",
        "usercmd_mouse_dy",
        "usercmd_buttonstate_1",
        "usercmd_buttonstate_2",
        "usercmd_buttonstate_3",
        "usercmd_weapon_select",
        "usercmd_left_hand_desired",
        "usercmd_input_history",
    ];

    for name in events {
        let mut v = d.game_events.iter().filter(|x| x.name == name).collect_vec();
        v.truncate(2);
        let test_name = "game_event_".to_string() + &name.replace(".", "_");
        let s = "".to_string();
        let s = s + &format!("fn {}() {{", test_name);
        let s = s + "use crate::second_pass::variants::Variant::*;";
        let s = s + &format!("let prop = ({:?}, {:#?});", name, v);
        let s = s.replace("[", "vec![");
        let s = s.replace("\")", "\".to_string())");
        let s = s.replace("\",", "\".to_string(),");

        println!("#[test]");
        println!("{}", s);
        println!("assert_eq!(out.2[{:?}], prop.1);", name);
        println!("}}");
    }
}

pub fn _create_tests() {
    let wanted_events = vec![];
    let wanted_props = vec![
        "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_flLastTeleportTime".to_string(),
        "CCSPlayerPawn.CCSPlayer_BulletServices.m_totalHitsOnServer".to_string(),
        "CCSPlayerPawn.CCSPlayer_ItemServices.m_bHasDefuser".to_string(),
        "CCSPlayerPawn.CCSPlayer_ItemServices.m_bHasHelmet".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_arrForceSubtickMoveWhen".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDesiresDuck".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDuckOverride".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bOldJumpPressed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_fStashGrenadeParameterWhen".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckAmount".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckSpeed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpUntil".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpVel".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flOffsetTickCompleteTime".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flOffsetTickStashedSpeed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flStamina".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_nButtonDownMaskPrev".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_nLadderSurfacePropIndex".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_vecLadderNormal".to_string(),
        "CCSPlayerPawn.CCSPlayer_WeaponServices.m_iAmmo".to_string(),
        "CCSPlayerPawn.m_ArmorValue".to_string(),
        "CCSPlayerPawn.m_MoveType".to_string(),
        "CCSPlayerPawn.m_aimPunchAngle".to_string(),
        "CCSPlayerPawn.m_aimPunchAngleVel".to_string(),
        "CCSPlayerPawn.m_aimPunchTickBase".to_string(),
        "CCSPlayerPawn.m_aimPunchTickFraction".to_string(),
        "CCSPlayerPawn.m_angEyeAngles".to_string(),
        "CCSPlayerPawn.m_bClientRagdoll".to_string(),
        "CCSPlayerPawn.m_bClientSideRagdoll".to_string(),
        "CCSPlayerPawn.m_bHasMovedSinceSpawn".to_string(),
        "CCSPlayerPawn.m_bInBombZone".to_string(),
        "CCSPlayerPawn.m_bInBuyZone".to_string(),
        "CCSPlayerPawn.m_bIsBuyMenuOpen".to_string(),
        "CCSPlayerPawn.m_bIsDefusing".to_string(),
        "CCSPlayerPawn.m_bIsScoped".to_string(),
        "CCSPlayerPawn.m_bIsWalking".to_string(),
        "CCSPlayerPawn.m_bKilledByHeadshot".to_string(),
        "CCSPlayerPawn.m_bRagdollDamageHeadshot".to_string(),
        "CCSPlayerPawn.m_bResumeZoom".to_string(),
        "CCSPlayerPawn.m_bSpotted".to_string(),
        "CCSPlayerPawn.m_bSpottedByMask".to_string(),
        "CCSPlayerPawn.m_bWaitForNoAttack".to_string(),
        "CCSPlayerPawn.m_fFlags".to_string(),
        "CCSPlayerPawn.m_fMolotovDamageTime".to_string(),
        "CCSPlayerPawn.m_flCreateTime".to_string(),
        "CCSPlayerPawn.m_flDeathTime".to_string(),
        "CCSPlayerPawn.m_flEmitSoundTime".to_string(),
        "CCSPlayerPawn.m_flFlashDuration".to_string(),
        "CCSPlayerPawn.m_flFlashMaxAlpha".to_string(),
        "CCSPlayerPawn.m_flHitHeading".to_string(),
        "CCSPlayerPawn.m_flProgressBarStartTime".to_string(),
        "CCSPlayerPawn.m_flSlopeDropHeight".to_string(),
        "CCSPlayerPawn.m_flSlopeDropOffset".to_string(),
        "CCSPlayerPawn.m_flTimeOfLastInjury".to_string(),
        "CCSPlayerPawn.m_flVelocityModifier".to_string(),
        "CCSPlayerPawn.m_iHealth".to_string(),
        "CCSPlayerPawn.m_iMoveState".to_string(),
        "CCSPlayerPawn.m_iPlayerState".to_string(),
        "CCSPlayerPawn.m_iProgressBarDuration".to_string(),
        "CCSPlayerPawn.m_iShotsFired".to_string(),
        "CCSPlayerPawn.m_iTeamNum".to_string(),
        "CCSPlayerPawn.m_lifeState".to_string(),
        "CCSPlayerPawn.m_nCollisionFunctionMask".to_string(),
        "CCSPlayerPawn.m_nEnablePhysics".to_string(),
        "CCSPlayerPawn.m_nEntityId".to_string(),
        "CCSPlayerPawn.m_nForceBone".to_string(),
        "CCSPlayerPawn.m_nHierarchyId".to_string(),
        "CCSPlayerPawn.m_nHitBodyPart".to_string(),
        "CCSPlayerPawn.m_nInteractsAs".to_string(),
        "CCSPlayerPawn.m_nInteractsExclude".to_string(),
        "CCSPlayerPawn.m_nInteractsWith".to_string(),
        "CCSPlayerPawn.m_nLastConcurrentKilled".to_string(),
        "CCSPlayerPawn.m_nLastKillerIndex".to_string(),
        "CCSPlayerPawn.m_nRagdollDamageBone".to_string(),
        "CCSPlayerPawn.m_nWhichBombZone".to_string(),
        "CCSPlayerPawn.m_qDeathEyeAngles".to_string(),
        "CCSPlayerPawn.m_szLastPlaceName".to_string(),
        "CCSPlayerPawn.m_szRagdollDamageWeaponName".to_string(),
        "CCSPlayerPawn.m_thirdPersonHeading".to_string(),
        "CCSPlayerPawn.m_ubInterpolationFrame".to_string(),
        "CCSPlayerPawn.m_unCurrentEquipmentValue".to_string(),
        "CCSPlayerPawn.m_unFreezetimeEndEquipmentValue".to_string(),
        "CCSPlayerPawn.m_unRoundStartEquipmentValue".to_string(),
        "CCSPlayerPawn.m_vDecalForwardAxis".to_string(),
        "CCSPlayerPawn.m_vDecalPosition".to_string(),
        "CCSPlayerPawn.m_vHeadConstraintOffset".to_string(),
        "CCSPlayerPawn.m_vRagdollDamageForce".to_string(),
        "CCSPlayerPawn.m_vRagdollDamagePosition".to_string(),
        "CCSPlayerPawn.m_vRagdollServerOrigin".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bBombDropped".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bBombPlanted".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bFreezePeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bGameRestart".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bHasMatchStarted".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bTeamIntroPeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bWarmupPeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_eRoundWinReason".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fMatchStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fRoundStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodEnd".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodStart".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_flGameStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_flRestartRoundTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_gamePhase".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_CT".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_T".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_RoundResults".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveCTLoses".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveTerroristLoses".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iRoundTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iRoundWinStatus".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_nRoundsPlayedThisPhase".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_timeUntilNextPhaseStarts".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iAssists".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iCashEarned".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDeaths".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEnemiesFlashed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEquipmentValue".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iHeadShotKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKillReward".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iLiveTime".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iMoneySaved".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iObjective".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iUtilityDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iAssists".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iCashEarned".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDeaths".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemiesFlashed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemy3Ks".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEquipmentValue".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iHeadShotKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKillReward".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iLiveTime".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iMoneySaved".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iNumRoundKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iNumRoundKillsHeadshots".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iObjective".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iUtilityDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_unTotalRoundDamageDealt".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iAccount".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iCashSpentThisRound".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iStartAccount".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iTotalCashSpent".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsFriendly".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsLeader".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsTeacher".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicLevel".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_rank".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_unMusicID".to_string(),
        "CCSPlayerController.m_bCanControlObservedBot".to_string(),
        "CCSPlayerController.m_bEverPlayedOnTeam".to_string(),
        "CCSPlayerController.m_bPawnHasDefuser".to_string(),
        "CCSPlayerController.m_bPawnHasHelmet".to_string(),
        "CCSPlayerController.m_bPawnIsAlive".to_string(),
        "CCSPlayerController.m_fFlags".to_string(),
        "CCSPlayerController.m_flCreateTime".to_string(),
        "CCSPlayerController.m_hOriginalControllerOfCurrentPawn".to_string(),
        "CCSPlayerController.m_hPawn".to_string(),
        "CCSPlayerController.m_hPlayerPawn".to_string(),
        "CCSPlayerController.m_iCompTeammateColor".to_string(),
        "CCSPlayerController.m_iCompetitiveRankType".to_string(),
        "CCSPlayerController.m_iCompetitiveRanking".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Loss".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Tie".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Win".to_string(),
        "CCSPlayerController.m_iCompetitiveWins".to_string(),
        "CCSPlayerController.m_iConnected".to_string(),
        "CCSPlayerController.m_iMVPs".to_string(),
        "CCSPlayerController.m_iPawnArmor".to_string(),
        "CCSPlayerController.m_iPawnHealth".to_string(),
        "CCSPlayerController.m_iPawnLifetimeEnd".to_string(),
        "CCSPlayerController.m_iPawnLifetimeStart".to_string(),
        "CCSPlayerController.m_iPendingTeamNum".to_string(),
        "CCSPlayerController.m_iPing".to_string(),
        "CCSPlayerController.m_iScore".to_string(),
        "CCSPlayerController.m_iTeamNum".to_string(),
        "CCSPlayerController.m_iszPlayerName".to_string(),
        "CCSPlayerController.m_nDisconnectionTick".to_string(),
        "CCSPlayerController.m_nPawnCharacterDefIndex".to_string(),
        "CCSPlayerController.m_nQuestProgressReason".to_string(),
        "CCSPlayerController.m_nTickBase".to_string(),
        "CCSPlayerController.m_steamID".to_string(),
        "CCSPlayerController.m_szCrosshairCodes".to_string(),
        "CCSPlayerController.m_unActiveQuestId".to_string(),
        "CCSPlayerController.m_unPlayerTvControlFlags".to_string(),
        "CBodyComponentBaseAnimGraph.m_MeshGroupMask".to_string(),
        "CBodyComponentBaseAnimGraph.m_angRotation".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellX".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellY".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellZ".to_string(),
        "CBodyComponentBaseAnimGraph.m_hParent".to_string(),
        "CBodyComponentBaseAnimGraph.m_hSequence".to_string(),
        "CBodyComponentBaseAnimGraph.m_nAnimLoopMode".to_string(),
        "CBodyComponentBaseAnimGraph.m_nIdealMotionType".to_string(),
        "CBodyComponentBaseAnimGraph.m_nNewSequenceParity".to_string(),
        "CBodyComponentBaseAnimGraph.m_nRandomSeedOffset".to_string(),
        "CBodyComponentBaseAnimGraph.m_nResetEventsParity".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecX".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecY".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecZ".to_string(),
        "CEconItemAttribute.m_bSetBonus".to_string(),
        "CEconItemAttribute.m_flInitialValue".to_string(),
        "CEconItemAttribute.m_iAttributeDefinitionIndex".to_string(),
        "CEconItemAttribute.m_iRawValue32".to_string(),
        "CEconItemAttribute.m_nRefundableCurrency".to_string(),
        "m_MoveType".to_string(),
        "m_OriginalOwnerXuidHigh".to_string(),
        "m_OriginalOwnerXuidLow".to_string(),
        "m_bBurstMode".to_string(),
        "m_bInReload".to_string(),
        "m_bReloadVisuallyComplete".to_string(),
        "m_fAccuracyPenalty".to_string(),
        "m_fEffects".to_string(),
        "m_fLastShotTime".to_string(),
        "m_flCreateTime".to_string(),
        "m_flDroppedAtTime".to_string(),
        "m_flFireSequenceStartTime".to_string(),
        "m_flNextPrimaryAttackTickRatio".to_string(),
        "m_flNextSecondaryAttackTickRatio".to_string(),
        "m_flRecoilIndex".to_string(),
        "m_flSimulationTime".to_string(),
        "m_flTimeSilencerSwitchComplete".to_string(),
        "m_hOuter".to_string(),
        "m_hOwnerEntity".to_string(),
        "m_hPrevOwner".to_string(),
        "m_iAccountID".to_string(),
        "m_iClip1".to_string(),
        "m_iClip2".to_string(),
        "m_iEntityQuality".to_string(),
        "m_iInventoryPosition".to_string(),
        "m_iIronSightMode".to_string(),
        "m_iItemIDHigh".to_string(),
        "m_iItemIDLow".to_string(),
        "m_iState".to_string(),
        "m_nAddDecal".to_string(),
        "m_nCollisionFunctionMask".to_string(),
        "m_nDropTick".to_string(),
        "m_nEnablePhysics".to_string(),
        "m_nEntityId".to_string(),
        "m_nFireSequenceStartTimeChange".to_string(),
        "m_nHierarchyId".to_string(),
        "m_nInteractsAs".to_string(),
        "m_nNextPrimaryAttackTick".to_string(),
        "m_nNextSecondaryAttackTick".to_string(),
        "m_nNextThinkTick".to_string(),
        "m_nOwnerId".to_string(),
        "m_nSubclassID".to_string(),
        "m_nViewModelIndex".to_string(),
        "m_pReserveAmmo".to_string(),
        "m_ubInterpolationFrame".to_string(),
        "m_usSolidFlags".to_string(),
        "m_vDecalForwardAxis".to_string(),
        "m_vDecalPosition".to_string(),
        "m_weaponMode".to_string(),
        "X".to_string(),
        "Y".to_string(),
        "Z".to_string(),
        "velocity".to_string(),
        "velocity_X".to_string(),
        "velocity_Y".to_string(),
        "velocity_Z".to_string(),
        "pitch".to_string(),
        "yaw".to_string(),
        "weapon_name".to_string(),
        "weapon_skin".to_string(),
        "weapon_skin_id".to_string(),
        "active_weapon_original_owner".to_string(),
        "inventory".to_string(),
        "inventory_as_ids".to_string(),
        "entity_id".to_string(),
        "is_alive".to_string(),
        "user_id".to_string(),
        "agent_skin".to_string(),
        "is_airborne".to_string(),
        "usercmd_viewangle_x".to_string(),
        "usercmd_viewangle_y".to_string(),
        "usercmd_viewangle_z".to_string(),
        "usercmd_forward_move".to_string(),
        "usercmd_left_move".to_string(),
        "usercmd_impulse".to_string(),
        "usercmd_mouse_dx".to_string(),
        "usercmd_mouse_dy".to_string(),
        "usercmd_buttonstate_1".to_string(),
        "usercmd_buttonstate_2".to_string(),
        "usercmd_buttonstate_3".to_string(),
        "usercmd_weapon_select".to_string(),
        "usercmd_left_hand_desired".to_string(),
        "usercmd_input_history".to_string(),
    ];
    let huf = create_huffman_lookup_table();

    let settings = ParserInputs {
        wanted_player_props: wanted_props.clone(),
        wanted_events: wanted_events,
        real_name_to_og_name: AHashMap::default(),
        wanted_other_props: vec![],
        parse_ents: true,
        wanted_players: vec![],
        wanted_ticks: (0..5).into_iter().map(|x| x * 10000).collect_vec(),
        wanted_prop_states: AHashMap::default(),
        parse_projectiles: true,
        only_header: false,
        count_props: false,
        only_convars: false,
        huffman_lookup_table: &huf,
        order_by_steamid: false,
    };

    let mut ds = Parser::new(settings, crate::parse_demo::ParsingMode::ForceMultiThreaded);
    let file = File::open("test_demo.dem".to_string()).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let d = ds.parse_demo(&mmap).unwrap();
    let mut custom = AHashMap::default();

    custom.insert(WEAPON_ORIGINGAL_OWNER_ID, "active_weapon_original_owner");
    custom.insert(INVENTORY_ID, "inventory");
    custom.insert(INVENTORY_AS_IDS_ID, "inventory_as_ids");

    custom.insert(USERID_ID, "user_id");
    custom.insert(VELOCITY_X_ID, "velocity_X");
    custom.insert(VELOCITY_Y_ID, "velocity_Y");
    custom.insert(VELOCITY_Z_ID, "velocity_Z");
    custom.insert(VELOCITY_ID, "velocity");
    custom.insert(IS_ALIVE_ID, "is_alive");
    custom.insert(ENTITY_ID_ID, "entity_id");
    custom.insert(GAME_TIME_ID, "game_time");
    custom.insert(WEAPON_SKIN_NAME, "weapon_skin");
    custom.insert(WEAPON_SKIN_ID, "weapon_skin_id");

    custom.insert(WEAPON_NAME_ID, "weapon_name");
    custom.insert(PITCH_ID, "pitch");
    custom.insert(YAW_ID, "yaw");
    custom.insert(PLAYER_X_ID, "X");
    custom.insert(PLAYER_Y_ID, "Y");
    custom.insert(PLAYER_Z_ID, "Z");
    custom.insert(TICK_ID, "tick");
    custom.insert(STEAMID_ID, "steamid");
    custom.insert(NAME_ID, "name");
    custom.insert(WEAPON_STICKERS_ID, "weapon_stickers");
    custom.insert(IS_AIRBORNE_ID, "is_airborne");
    custom.insert(USERCMD_INPUT_HISTORY_BASEID, "usercmd_input_history");
    // Currently test demo is too old to contain these :/
    custom.insert(USERCMD_VIEWANGLE_X, "usercmd_viewangle_x");
    custom.insert(USERCMD_VIEWANGLE_Y, "usercmd_viewangle_y");
    custom.insert(USERCMD_VIEWANGLE_Z, "usercmd_viewangle_z");
    custom.insert(USERCMD_FORWARDMOVE, "usercmd_forward_move");
    custom.insert(USERCMD_LEFTMOVE, "usercmd_left_move");
    custom.insert(USERCMD_IMPULSE, "usercmd_impulsee");
    custom.insert(USERCMD_MOUSE_DX, "usercmd_mouse_dx");
    custom.insert(USERCMD_MOUSE_DY, "usercmd_mouse_dy");
    custom.insert(USERCMD_BUTTONSTATE_1, "usercmd_buttonstate_1");
    custom.insert(USERCMD_BUTTONSTATE_2, "usercmd_buttonstate_2");
    custom.insert(USERCMD_BUTTONSTATE_3, "usercmd_buttonstate_3");
    custom.insert(USERCMD_WEAPON_SELECT, "usercmd_weapon_select");
    custom.insert(USERCMD_SUBTICK_LEFT_HAND_DESIRED, "usercmd_left_hand_desired");

    for (k, v) in d.df {
        if let Some(real_name) = d.prop_controller.id_to_name.get(&k) {
            let test_name = real_name.replace(".", "_");
            let s = "".to_string();
            let s = s + &format!("fn {}() {{", test_name);
            let s = s + &format!("let prop = ({:?}, {:?});", real_name, v);
            let mut s = s.replace("\"))", "\".to_string()))");
            if !s.contains("XY") {
                s = s.replace("[", "vec![");
            }
            println!("#[test]");
            println!("{}", s);
            println!("let prop_id = out.1.name_to_id[prop.0];");
            println!("assert_eq!(out.0.df[&prop_id], prop.1);");
            println!("}}");
        } else {
            if let Some(name) = custom.get(&k) {
                let test_name = name.replace(".", "_");
                let s = "".to_string();
                let s = s + &format!("fn {}() {{", test_name);
                let mut s = s + &format!("let prop = ({:?}, {:?});", name, v);
                if !s.contains("XY") {
                    s = s.replace("[", "vec![");
                }
                let s = s.replace("\"))", "\".to_string())))");
                println!("#[test]");
                println!("{:#?}", s);
                println!("assert_eq!(out.0[prop.0], prop.1);");
                println!("}}");
            }
        }
    }
}

fn create_data() -> (DemoOutput, PropController, BTreeMap<String, Vec<GameEvent>>) {
    let wanted_props = vec![
        "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_flLastTeleportTime".to_string(),
        "CCSPlayerPawn.CCSPlayer_BulletServices.m_totalHitsOnServer".to_string(),
        "CCSPlayerPawn.CCSPlayer_ItemServices.m_bHasDefuser".to_string(),
        "CCSPlayerPawn.CCSPlayer_ItemServices.m_bHasHelmet".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_arrForceSubtickMoveWhen".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDesiresDuck".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDuckOverride".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_bOldJumpPressed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_fStashGrenadeParameterWhen".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckAmount".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckSpeed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpUntil".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpVel".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flOffsetTickCompleteTime".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flOffsetTickStashedSpeed".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_flStamina".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_nButtonDownMaskPrev".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_nLadderSurfacePropIndex".to_string(),
        "CCSPlayerPawn.CCSPlayer_MovementServices.m_vecLadderNormal".to_string(),
        "CCSPlayerPawn.CCSPlayer_WeaponServices.m_iAmmo".to_string(),
        "CCSPlayerPawn.m_ArmorValue".to_string(),
        "CCSPlayerPawn.m_MoveType".to_string(),
        "CCSPlayerPawn.m_aimPunchAngle".to_string(),
        "CCSPlayerPawn.m_aimPunchAngleVel".to_string(),
        "CCSPlayerPawn.m_aimPunchTickBase".to_string(),
        "CCSPlayerPawn.m_aimPunchTickFraction".to_string(),
        "CCSPlayerPawn.m_angEyeAngles".to_string(),
        "CCSPlayerPawn.m_bClientRagdoll".to_string(),
        "CCSPlayerPawn.m_bClientSideRagdoll".to_string(),
        "CCSPlayerPawn.m_bHasMovedSinceSpawn".to_string(),
        "CCSPlayerPawn.m_bInBombZone".to_string(),
        "CCSPlayerPawn.m_bInBuyZone".to_string(),
        "CCSPlayerPawn.m_bIsBuyMenuOpen".to_string(),
        "CCSPlayerPawn.m_bIsDefusing".to_string(),
        "CCSPlayerPawn.m_bIsScoped".to_string(),
        "CCSPlayerPawn.m_bIsWalking".to_string(),
        "CCSPlayerPawn.m_bKilledByHeadshot".to_string(),
        "CCSPlayerPawn.m_bRagdollDamageHeadshot".to_string(),
        "CCSPlayerPawn.m_bResumeZoom".to_string(),
        "CCSPlayerPawn.m_bSpotted".to_string(),
        "CCSPlayerPawn.m_bSpottedByMask".to_string(),
        "CCSPlayerPawn.m_bWaitForNoAttack".to_string(),
        "CCSPlayerPawn.m_fFlags".to_string(),
        "CCSPlayerPawn.m_fMolotovDamageTime".to_string(),
        "CCSPlayerPawn.m_flCreateTime".to_string(),
        "CCSPlayerPawn.m_flDeathTime".to_string(),
        "CCSPlayerPawn.m_flEmitSoundTime".to_string(),
        "CCSPlayerPawn.m_flFlashDuration".to_string(),
        "CCSPlayerPawn.m_flFlashMaxAlpha".to_string(),
        "CCSPlayerPawn.m_flHitHeading".to_string(),
        "CCSPlayerPawn.m_flProgressBarStartTime".to_string(),
        "CCSPlayerPawn.m_flSlopeDropHeight".to_string(),
        "CCSPlayerPawn.m_flSlopeDropOffset".to_string(),
        "CCSPlayerPawn.m_flTimeOfLastInjury".to_string(),
        "CCSPlayerPawn.m_flVelocityModifier".to_string(),
        "CCSPlayerPawn.m_iHealth".to_string(),
        "CCSPlayerPawn.m_iMoveState".to_string(),
        "CCSPlayerPawn.m_iPlayerState".to_string(),
        "CCSPlayerPawn.m_iProgressBarDuration".to_string(),
        "CCSPlayerPawn.m_iShotsFired".to_string(),
        "CCSPlayerPawn.m_iTeamNum".to_string(),
        "CCSPlayerPawn.m_lifeState".to_string(),
        "CCSPlayerPawn.m_nCollisionFunctionMask".to_string(),
        "CCSPlayerPawn.m_nEnablePhysics".to_string(),
        "CCSPlayerPawn.m_nEntityId".to_string(),
        "CCSPlayerPawn.m_nForceBone".to_string(),
        "CCSPlayerPawn.m_nHierarchyId".to_string(),
        "CCSPlayerPawn.m_nHitBodyPart".to_string(),
        "CCSPlayerPawn.m_nInteractsAs".to_string(),
        "CCSPlayerPawn.m_nInteractsExclude".to_string(),
        "CCSPlayerPawn.m_nInteractsWith".to_string(),
        "CCSPlayerPawn.m_nLastConcurrentKilled".to_string(),
        "CCSPlayerPawn.m_nLastKillerIndex".to_string(),
        "CCSPlayerPawn.m_nRagdollDamageBone".to_string(),
        "CCSPlayerPawn.m_nWhichBombZone".to_string(),
        "CCSPlayerPawn.m_qDeathEyeAngles".to_string(),
        "CCSPlayerPawn.m_szLastPlaceName".to_string(),
        "CCSPlayerPawn.m_szRagdollDamageWeaponName".to_string(),
        "CCSPlayerPawn.m_thirdPersonHeading".to_string(),
        "CCSPlayerPawn.m_ubInterpolationFrame".to_string(),
        "CCSPlayerPawn.m_unCurrentEquipmentValue".to_string(),
        "CCSPlayerPawn.m_unFreezetimeEndEquipmentValue".to_string(),
        "CCSPlayerPawn.m_unRoundStartEquipmentValue".to_string(),
        "CCSPlayerPawn.m_vDecalForwardAxis".to_string(),
        "CCSPlayerPawn.m_vDecalPosition".to_string(),
        "CCSPlayerPawn.m_vHeadConstraintOffset".to_string(),
        "CCSPlayerPawn.m_vRagdollDamageForce".to_string(),
        "CCSPlayerPawn.m_vRagdollDamagePosition".to_string(),
        "CCSPlayerPawn.m_vRagdollServerOrigin".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bBombDropped".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bBombPlanted".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bFreezePeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bGameRestart".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bHasMatchStarted".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bTeamIntroPeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_bWarmupPeriod".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_eRoundWinReason".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fMatchStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fRoundStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodEnd".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodStart".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_flGameStartTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_flRestartRoundTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_gamePhase".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_CT".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_T".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_RoundResults".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveCTLoses".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveTerroristLoses".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iRoundTime".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_iRoundWinStatus".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_nRoundsPlayedThisPhase".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_timeUntilNextPhaseStarts".to_string(),
        "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iAssists".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iCashEarned".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDeaths".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEnemiesFlashed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEquipmentValue".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iHeadShotKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKillReward".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iLiveTime".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iMoneySaved".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iObjective".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iUtilityDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iAssists".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iCashEarned".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDeaths".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemiesFlashed".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemy3Ks".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEquipmentValue".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iHeadShotKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKillReward".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iLiveTime".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iMoneySaved".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iNumRoundKills".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iNumRoundKillsHeadshots".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iObjective".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iUtilityDamage".to_string(),
        "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_unTotalRoundDamageDealt".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iAccount".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iCashSpentThisRound".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iStartAccount".to_string(),
        "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iTotalCashSpent".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsFriendly".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsLeader".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsTeacher".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicLevel".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_rank".to_string(),
        "CCSPlayerController.CCSPlayerController_InventoryServices.m_unMusicID".to_string(),
        "CCSPlayerController.m_bCanControlObservedBot".to_string(),
        "CCSPlayerController.m_bEverPlayedOnTeam".to_string(),
        "CCSPlayerController.m_bPawnHasDefuser".to_string(),
        "CCSPlayerController.m_bPawnHasHelmet".to_string(),
        "CCSPlayerController.m_bPawnIsAlive".to_string(),
        "CCSPlayerController.m_fFlags".to_string(),
        "CCSPlayerController.m_flCreateTime".to_string(),
        "CCSPlayerController.m_hOriginalControllerOfCurrentPawn".to_string(),
        "CCSPlayerController.m_hPawn".to_string(),
        "CCSPlayerController.m_hPlayerPawn".to_string(),
        "CCSPlayerController.m_iCompTeammateColor".to_string(),
        "CCSPlayerController.m_iCompetitiveRankType".to_string(),
        "CCSPlayerController.m_iCompetitiveRanking".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Loss".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Tie".to_string(),
        "CCSPlayerController.m_iCompetitiveRankingPredicted_Win".to_string(),
        "CCSPlayerController.m_iCompetitiveWins".to_string(),
        "CCSPlayerController.m_iConnected".to_string(),
        "CCSPlayerController.m_iMVPs".to_string(),
        "CCSPlayerController.m_iPawnArmor".to_string(),
        "CCSPlayerController.m_iPawnHealth".to_string(),
        "CCSPlayerController.m_iPawnLifetimeEnd".to_string(),
        "CCSPlayerController.m_iPawnLifetimeStart".to_string(),
        "CCSPlayerController.m_iPendingTeamNum".to_string(),
        "CCSPlayerController.m_iPing".to_string(),
        "CCSPlayerController.m_iScore".to_string(),
        "CCSPlayerController.m_iTeamNum".to_string(),
        "CCSPlayerController.m_iszPlayerName".to_string(),
        "CCSPlayerController.m_nDisconnectionTick".to_string(),
        "CCSPlayerController.m_nPawnCharacterDefIndex".to_string(),
        "CCSPlayerController.m_nQuestProgressReason".to_string(),
        "CCSPlayerController.m_nTickBase".to_string(),
        "CCSPlayerController.m_steamID".to_string(),
        "CCSPlayerController.m_szCrosshairCodes".to_string(),
        "CCSPlayerController.m_unActiveQuestId".to_string(),
        "CCSPlayerController.m_unPlayerTvControlFlags".to_string(),
        "CBodyComponentBaseAnimGraph.m_MeshGroupMask".to_string(),
        "CBodyComponentBaseAnimGraph.m_angRotation".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellX".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellY".to_string(),
        "CBodyComponentBaseAnimGraph.m_cellZ".to_string(),
        "CBodyComponentBaseAnimGraph.m_hParent".to_string(),
        "CBodyComponentBaseAnimGraph.m_hSequence".to_string(),
        "CBodyComponentBaseAnimGraph.m_nAnimLoopMode".to_string(),
        "CBodyComponentBaseAnimGraph.m_nIdealMotionType".to_string(),
        "CBodyComponentBaseAnimGraph.m_nNewSequenceParity".to_string(),
        "CBodyComponentBaseAnimGraph.m_nRandomSeedOffset".to_string(),
        "CBodyComponentBaseAnimGraph.m_nResetEventsParity".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecX".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecY".to_string(),
        "CBodyComponentBaseAnimGraph.m_vecZ".to_string(),
        "CEconItemAttribute.m_bSetBonus".to_string(),
        "CEconItemAttribute.m_flInitialValue".to_string(),
        "CEconItemAttribute.m_iAttributeDefinitionIndex".to_string(),
        "CEconItemAttribute.m_iRawValue32".to_string(),
        "CEconItemAttribute.m_nRefundableCurrency".to_string(),
        "m_MoveType".to_string(),
        "m_OriginalOwnerXuidHigh".to_string(),
        "m_OriginalOwnerXuidLow".to_string(),
        "m_bBurstMode".to_string(),
        "m_bInReload".to_string(),
        "m_bReloadVisuallyComplete".to_string(),
        "m_fAccuracyPenalty".to_string(),
        "m_fEffects".to_string(),
        "m_fLastShotTime".to_string(),
        "m_flCreateTime".to_string(),
        "m_flDroppedAtTime".to_string(),
        "m_flFireSequenceStartTime".to_string(),
        "m_flNextPrimaryAttackTickRatio".to_string(),
        "m_flNextSecondaryAttackTickRatio".to_string(),
        "m_flRecoilIndex".to_string(),
        "m_flSimulationTime".to_string(),
        "m_flTimeSilencerSwitchComplete".to_string(),
        "m_hOuter".to_string(),
        "m_hOwnerEntity".to_string(),
        "m_hPrevOwner".to_string(),
        "m_iAccountID".to_string(),
        "m_iClip1".to_string(),
        "m_iClip2".to_string(),
        "m_iEntityQuality".to_string(),
        "m_iInventoryPosition".to_string(),
        "m_iIronSightMode".to_string(),
        "m_iItemIDHigh".to_string(),
        "m_iItemIDLow".to_string(),
        "m_iState".to_string(),
        "m_nAddDecal".to_string(),
        "m_nCollisionFunctionMask".to_string(),
        "m_nDropTick".to_string(),
        "m_nEnablePhysics".to_string(),
        "m_nEntityId".to_string(),
        "m_nFireSequenceStartTimeChange".to_string(),
        "m_nHierarchyId".to_string(),
        "m_nInteractsAs".to_string(),
        "m_nNextPrimaryAttackTick".to_string(),
        "m_nNextSecondaryAttackTick".to_string(),
        "m_nNextThinkTick".to_string(),
        "m_nOwnerId".to_string(),
        "m_nSubclassID".to_string(),
        "m_nViewModelIndex".to_string(),
        "m_pReserveAmmo".to_string(),
        "m_ubInterpolationFrame".to_string(),
        "m_usSolidFlags".to_string(),
        "m_vDecalForwardAxis".to_string(),
        "m_vDecalPosition".to_string(),
        "m_weaponMode".to_string(),
        "X".to_string(),
        "Y".to_string(),
        "Z".to_string(),
        "velocity".to_string(),
        "velocity_X".to_string(),
        "velocity_Y".to_string(),
        "velocity_Z".to_string(),
        "pitch".to_string(),
        "yaw".to_string(),
        "weapon_name".to_string(),
        "weapon_skin".to_string(),
        "weapon_skin_id".to_string(),
        "active_weapon_original_owner".to_string(),
        "inventory".to_string(),
        "inventory_as_ids".to_string(),
        "entity_id".to_string(),
        "is_alive".to_string(),
        "user_id".to_string(),
        "agent_skin".to_string(),
        "weapon_stickers".to_string(),
        "weapon_float".to_string(),
        "weapon_paint_seed".to_string(),
        "is_airborne".to_string(),
    ];

    let wanted_events = vec![];
    let huf = create_huffman_lookup_table();

    let settings = ParserInputs {
        wanted_player_props: wanted_props.clone(),
        wanted_events: wanted_events,
        real_name_to_og_name: AHashMap::default(),
        wanted_other_props: vec![],
        parse_ents: true,
        wanted_players: vec![],
        wanted_ticks: (0..5).into_iter().map(|x| x * 10000).collect_vec(),
        wanted_prop_states: AHashMap::default(),
        parse_projectiles: true,
        only_header: false,
        count_props: false,
        only_convars: false,
        huffman_lookup_table: &huf,
        order_by_steamid: false,
    };

    let mut ds = Parser::new(settings, crate::parse_demo::ParsingMode::ForceMultiThreaded);
    let file = File::open("test_demo.dem".to_string()).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let out1 = ds.parse_demo(&mmap).unwrap();

    let wanted_events = vec!["all".to_string()];
    let huf = create_huffman_lookup_table();

    let settings = ParserInputs {
        wanted_player_props: vec![],
        wanted_events: wanted_events,
        real_name_to_og_name: AHashMap::default(),
        wanted_other_props: vec![],
        parse_ents: true,
        wanted_players: vec![],
        wanted_ticks: (0..5).into_iter().map(|x| x * 10000).collect_vec(),
        wanted_prop_states: AHashMap::default(),
        parse_projectiles: true,
        only_header: false,
        count_props: false,
        only_convars: false,
        huffman_lookup_table: &huf,
        order_by_steamid: false,
    };
    let mut ds = Parser::new(settings, crate::parse_demo::ParsingMode::ForceMultiThreaded);
    let file = File::open("test_demo.dem".to_string()).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let out2 = ds.parse_demo(&mmap).unwrap();

    let events = vec![
        "hltv_versioninfo",
        "round_freeze_end",
        "weapon_reload",
        "cs_pre_restart",
        "weapon_fire",
        "player_death",
        "smokegrenade_expired",
        "item_equip",
        "bomb_planted",
        "bomb_exploded",
        "round_prestart",
        "cs_round_final_beep",
        "smokegrenade_detonate",
        "player_footstep",
        "buytime_ended",
        "player_jump",
        "weapon_zoom",
        "round_poststart",
        "bomb_pickup",
        "player_blind",
        "bomb_begindefuse",
        "inferno_startburn",
        "player_disconnect",
        "player_hurt",
        "bomb_beginplant",
        "round_officially_ended",
        "item_pickup",
        "player_spawn",
        "other_death",
        "bomb_defused",
        "begin_new_match",
        "cs_win_panel_round",
        "cs_win_panel_match",
        "cs_round_start_beep",
        "bomb_dropped",
        "inferno_expire",
        "round_end",
        "round_start",
        "round_time_warning",
        "flashbang_detonate",
        "round_mvp",
        "round_announce_match_start",
        "hegrenade_detonate",
        "item_purchase",
    ];
    let mut hm = BTreeMap::default();

    for name in events {
        let mut v = out2.game_events.iter().map(|x| x.clone()).filter(|x| x.name == name).collect_vec();
        v.truncate(2);
        hm.insert(name.to_string(), v);
    }
    (out1, out2.prop_controller, hm)
}

#[cfg(test)]
mod tests {
    use crate::e2e_test::create_data;
    use crate::first_pass::parser_settings::ParserInputs;
    use crate::first_pass::prop_controller::PropController;
    use crate::first_pass::prop_controller::PITCH_ID;
    use crate::first_pass::prop_controller::PLAYER_Y_ID;
    use crate::first_pass::prop_controller::WEAPON_NAME_ID;
    use crate::first_pass::prop_controller::YAW_ID;
    use crate::first_pass::prop_controller::*;
    use crate::parse_demo::DemoOutput;
    use crate::parse_demo::Parser;
    use crate::second_pass::game_events::EventField;
    use crate::second_pass::game_events::GameEvent;
    use crate::second_pass::parser_settings::create_huffman_lookup_table;
    use crate::second_pass::variants::PropColumn;
    use crate::second_pass::variants::Sticker;
    use crate::second_pass::variants::VarVec;
    use crate::second_pass::variants::VarVec::String;
    use crate::second_pass::variants::VarVec::*;
    use crate::second_pass::variants::Variant;
    use ahash::AHashMap;
    use lazy_static::lazy_static;
    use memmap2::MmapOptions;
    use std::collections::BTreeMap;
    use std::fs::File;

    lazy_static! {
        static ref out: (DemoOutput, PropController, BTreeMap<std::string::String, Vec<GameEvent>>) = create_data();
    }

    #[test]
    fn test_parse_ticks_prop_state_filter() {
        let huf = create_huffman_lookup_table();
        let huf2 = create_huffman_lookup_table();

        let settings = ParserInputs {
            wanted_players: vec![76561198244754626],
            real_name_to_og_name: AHashMap::default(),
            wanted_player_props: vec!["X".to_string(), "CCSGameRulesProxy.CCSGameRules.m_bBombPlanted".to_string()],
            wanted_events: vec![],
            wanted_other_props: vec![],
            parse_ents: true,
            wanted_ticks: vec![],
            wanted_prop_states: AHashMap::default(),
            parse_projectiles: true,
            only_header: false,
            count_props: false,
            only_convars: false,
            huffman_lookup_table: &huf,
            order_by_steamid: false,
        };

        let mut wanted_prop_states: AHashMap<std::string::String, Variant> = AHashMap::default();
        wanted_prop_states.insert("CCSGameRulesProxy.CCSGameRules.m_bBombPlanted".to_string(), Variant::Bool(true));
        let settings_with_filter = ParserInputs {
            wanted_players: vec![76561198244754626],
            real_name_to_og_name: AHashMap::default(),
            wanted_player_props: vec!["X".to_string()],
            wanted_events: vec![],
            wanted_other_props: vec![],
            parse_ents: true,
            wanted_ticks: vec![],
            wanted_prop_states: wanted_prop_states,
            parse_projectiles: true,
            only_header: false,
            count_props: false,
            only_convars: false,
            huffman_lookup_table: &huf2,
            order_by_steamid: false,
        };

        let mut ds = Parser::new(settings, crate::parse_demo::ParsingMode::ForceMultiThreaded);
        let mut ds_with_filter = Parser::new(settings_with_filter, crate::parse_demo::ParsingMode::ForceMultiThreaded);
        let file = File::open("test_demo.dem").unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
        let output = ds.parse_demo(&mmap).unwrap();
        let output_with_filter = ds_with_filter.parse_demo(&mmap).unwrap();

        let positions = match output.df.get(&PLAYER_X_ID).unwrap().data.clone().unwrap_or(VarVec::F32(vec![])) {
            VarVec::F32(positions_vec) => positions_vec,
            _ => vec![],
        };
        let bomb_prop_id = output
            .prop_controller
            .prop_infos
            .iter()
            .find(|prop| prop.prop_name == "CCSGameRulesProxy.CCSGameRules.m_bBombPlanted")
            .map(|prop| prop.id)
            .unwrap();
        let bomb = match output.df.get(&bomb_prop_id).unwrap().data.clone().unwrap_or(VarVec::Bool(vec![])) {
            VarVec::Bool(bomb_vec) => bomb_vec,
            _ => vec![],
        };
        let manually_filtered_positions: Vec<Option<f32>> = positions
            .iter()
            .zip(bomb.iter())
            .filter_map(|(xPos, bombPlanted)| match bombPlanted {
                Some(true) => Some(*xPos),
                _ => None,
            })
            .collect();
        let automatically_filtered_positions = match output_with_filter.df.get(&PLAYER_X_ID).unwrap().data.clone().unwrap_or(VarVec::F32(vec![])) {
            VarVec::F32(positions_vec) => positions_vec,
            _ => vec![],
        };

        assert_eq!(manually_filtered_positions, automatically_filtered_positions);
    }

    #[test]
    fn test_player_filter() {
        let huf = create_huffman_lookup_table();

        let settings = ParserInputs {
            wanted_players: vec![76561198244754626],
            real_name_to_og_name: AHashMap::default(),
            wanted_player_props: vec!["X".to_string()],
            wanted_events: vec![],
            wanted_other_props: vec!["CCSTeam.m_iScore".to_string()],
            parse_ents: true,
            wanted_ticks: vec![10000, 10001],
            wanted_prop_states: AHashMap::default(),
            parse_projectiles: true,
            only_header: false,
            count_props: false,
            only_convars: false,
            huffman_lookup_table: &huf,
            order_by_steamid: false,
        };

        let mut ds = Parser::new(settings, crate::parse_demo::ParsingMode::ForceMultiThreaded);
        let file = File::open("test_demo.dem").unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
        let output = ds.parse_demo(&mmap).unwrap();

        let steamids = output.df.get(&STEAMID_ID).unwrap();

        assert_eq!(steamids.data, Some(VarVec::U64(vec![Some(76561198244754626), Some(76561198244754626)])));
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_nNewSequenceParity() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_nNewSequenceParity",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(4),
                    Some(3),
                    Some(0),
                    Some(3),
                    Some(0),
                    Some(3),
                    Some(6),
                    None,
                    Some(3),
                    Some(4),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(3),
                    None,
                    Some(2),
                    Some(7),
                    None,
                    Some(3),
                    None,
                    Some(7),
                    Some(3),
                    Some(7),
                    Some(5),
                    Some(0),
                    Some(3),
                    Some(3),
                    Some(4),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(5),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_nDisconnectionTick() {
        let prop = ("CCSPlayerController.m_nDisconnectionTick", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_iMatchStats_PlayersAlive_CT() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_CT",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nRagdollDamageBone() {
        let prop = (
            "CCSPlayerPawn.m_nRagdollDamageBone",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(6),
                    Some(4),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(13),
                    Some(6),
                    Some(0),
                    Some(3),
                    Some(6),
                    Some(6),
                    Some(4),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(2),
                    Some(1),
                    Some(6),
                    Some(3),
                    Some(1),
                    Some(14),
                    Some(4),
                    Some(6),
                    Some(4),
                    Some(4),
                    Some(1),
                    Some(3),
                    Some(6),
                    Some(1),
                    Some(5),
                    Some(25),
                    Some(6),
                    Some(4),
                    Some(4),
                    Some(6),
                    Some(0),
                    Some(1),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iMoneySaved() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iMoneySaved",
            PropColumn { data: None, num_nones: 40 },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nWhichBombZone() {
        let prop = (
            "CCSPlayerPawn.m_nWhichBombZone",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flTimeOfLastInjury() {
        let prop = (
            "CCSPlayerPawn.m_flTimeOfLastInjury",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(355.78125),
                    Some(0.0),
                    Some(0.0),
                    Some(373.28125),
                    Some(373.32813),
                    Some(0.0),
                    Some(0.0),
                    Some(352.03125),
                    Some(375.32813),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(535.7969),
                    Some(529.4531),
                    Some(527.15625),
                    Some(529.46875),
                    Some(0.0),
                    Some(532.7969),
                    Some(532.15625),
                    Some(538.2656),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_MoveType() {
        let prop = (
            "m_MoveType",
            PropColumn {
                data: Some(U64(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CEconItemAttribute_m_iRawValue32() {
        let prop = ("CEconItemAttribute.m_iRawValue32", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_iState() {
        let prop = (
            "m_iState",
            PropColumn {
                data: Some(U32(vec![
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    None,
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    None,
                    Some(2),
                    Some(2),
                    None,
                    Some(2),
                    None,
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_flDroppedAtTime() {
        let prop = (
            "m_flDroppedAtTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(295.59375),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(400.71875),
                    Some(0.0),
                    None,
                    Some(0.0),
                    None,
                    None,
                    None,
                    Some(645.8594),
                    Some(693.40625),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_iEntityQuality() {
        let prop = (
            "m_iEntityQuality",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(3),
                    Some(4),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(3),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(4),
                    Some(4),
                    Some(0),
                    Some(0),
                    None,
                    Some(9),
                    Some(0),
                    Some(0),
                    Some(4),
                    Some(9),
                    Some(4),
                    Some(4),
                    None,
                    Some(0),
                    Some(4),
                    None,
                    Some(0),
                    None,
                    Some(4),
                    None,
                    Some(0),
                    Some(4),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(9),
                    Some(3),
                    None,
                    Some(4),
                    Some(3),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_unFreezetimeEndEquipmentValue() {
        let prop = (
            "CCSPlayerPawn.m_unFreezetimeEndEquipmentValue",
            PropColumn {
                data: Some(U32(vec![
                    Some(700),
                    Some(800),
                    Some(200),
                    Some(200),
                    Some(500),
                    Some(850),
                    Some(850),
                    Some(300),
                    Some(650),
                    Some(1000),
                    Some(4300),
                    Some(5900),
                    Some(5100),
                    Some(2750),
                    Some(5600),
                    Some(5300),
                    Some(5700),
                    Some(1950),
                    Some(4200),
                    Some(3100),
                    Some(5100),
                    Some(700),
                    Some(5100),
                    Some(5100),
                    Some(4100),
                    Some(3100),
                    Some(4250),
                    Some(2600),
                    Some(300),
                    Some(4200),
                    Some(5000),
                    Some(5950),
                    Some(1000),
                    Some(5100),
                    Some(4900),
                    Some(5500),
                    Some(4050),
                    Some(7150),
                    Some(5450),
                    Some(5500),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_nQuestProgressReason() {
        let prop = ("CCSPlayerController.m_nQuestProgressReason", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_nLadderSurfacePropIndex() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_nLadderSurfacePropIndex",
            PropColumn {
                data: Some(I32(vec![
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }

    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iUtilityDamage() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iUtilityDamage",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(52),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(26),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(9),
                    Some(52),
                    Some(44),
                    Some(36),
                    Some(0),
                    Some(0),
                    Some(26),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(9),
                    Some(52),
                    Some(44),
                    Some(36),
                    Some(0),
                    Some(0),
                    Some(26),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_vDecalPosition() {
        let prop = (
            "m_vDecalPosition",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    None,
                    None,
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iUtilityDamage() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iUtilityDamage",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(9),
                    Some(52),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(26),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(9),
                    Some(52),
                    Some(61),
                    Some(85),
                    Some(7),
                    Some(0),
                    Some(26),
                    Some(28),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iCashEarned() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iCashEarned",
            PropColumn { data: None, num_nones: 40 },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_iIronSightMode() {
        let prop = (
            "m_iIronSightMode",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(2),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nInteractsAs() {
        let prop = (
            "CCSPlayerPawn.m_nInteractsAs",
            PropColumn {
                data: Some(U64(vec![
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(131073),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(131073),
                    Some(393216),
                    Some(393216),
                    Some(131073),
                    Some(393216),
                    Some(131073),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                    Some(393216),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_aimPunchTickBase() {
        let prop = (
            "CCSPlayerPawn.m_aimPunchTickBase",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(22042),
                    Some(24327),
                    Some(21769),
                    Some(24331),
                    Some(23889),
                    Some(23821),
                    Some(23829),
                    Some(23090),
                    Some(24140),
                    Some(24346),
                    Some(34297),
                    Some(34438),
                    Some(34290),
                    Some(34301),
                    Some(34379),
                    Some(33886),
                    Some(34303),
                    Some(34099),
                    Some(34460),
                    Some(34449),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iCompetitiveRankingPredicted_Tie() {
        let prop = (
            "CCSPlayerController.m_iCompetitiveRankingPredicted_Tie",
            PropColumn {
                data: Some(I32(vec![
                    Some(18190),
                    Some(18205),
                    Some(9999),
                    Some(14999),
                    Some(18947),
                    Some(16302),
                    Some(19140),
                    Some(13703),
                    Some(17769),
                    Some(0),
                    Some(18190),
                    Some(18205),
                    Some(9999),
                    Some(14999),
                    Some(18947),
                    Some(16302),
                    Some(19140),
                    Some(13703),
                    Some(17769),
                    Some(0),
                    Some(18190),
                    Some(18205),
                    Some(9999),
                    Some(14999),
                    Some(18947),
                    Some(16302),
                    Some(19140),
                    Some(13703),
                    Some(17769),
                    Some(0),
                    Some(18190),
                    Some(18205),
                    Some(9999),
                    Some(14999),
                    Some(18947),
                    Some(16302),
                    Some(19140),
                    Some(13703),
                    Some(17769),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iAssists() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iAssists",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nEnablePhysics() {
        let prop = (
            "CCSPlayerPawn.m_nEnablePhysics",
            PropColumn {
                data: Some(U32(vec![
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nCollisionFunctionMask() {
        let prop = (
            "CCSPlayerPawn.m_nCollisionFunctionMask",
            PropColumn {
                data: Some(U32(vec![
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(61),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(61),
                    Some(55),
                    Some(55),
                    Some(61),
                    Some(55),
                    Some(61),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                    Some(55),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nEntityId() {
        let prop = (
            "m_nEntityId",
            PropColumn {
                data: Some(U32(vec![
                    Some(2863005796),
                    Some(3036479887),
                    Some(2120941719),
                    Some(4235690213),
                    Some(1433075922),
                    Some(304808084),
                    Some(2706145503),
                    Some(1545961596),
                    Some(2937749601),
                    Some(1441267959),
                    Some(1518600463),
                    Some(263651450),
                    Some(1014530310),
                    Some(1158250718),
                    None,
                    Some(2453012885),
                    Some(3180888201),
                    Some(2863038564),
                    Some(4231168408),
                    Some(2700902803),
                    Some(1518600463),
                    Some(4231594273),
                    None,
                    Some(1014530310),
                    Some(2891416009),
                    None,
                    Some(1300365714),
                    None,
                    Some(3346038996),
                    Some(2700902803),
                    Some(406749301),
                    Some(3036545423),
                    Some(3820749197),
                    Some(290488640),
                    Some(1720516840),
                    Some(1848639916),
                    Some(205815901),
                    Some(2891481545),
                    Some(4041703655),
                    Some(2618360313),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_flCreateTime() {
        let prop = (
            "m_flCreateTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(223.32813),
                    Some(216.92188),
                    Some(218.46875),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(70.765625),
                    Some(320.0),
                    Some(321.26563),
                    Some(319.25),
                    Some(318.21875),
                    None,
                    Some(329.42188),
                    Some(220.39063),
                    Some(317.23438),
                    Some(331.28125),
                    Some(328.26563),
                    Some(320.0),
                    Some(498.6875),
                    None,
                    Some(319.25),
                    Some(503.26563),
                    None,
                    Some(502.98438),
                    None,
                    Some(499.89063),
                    Some(328.26563),
                    Some(570.0781),
                    Some(567.5781),
                    Some(686.1094),
                    Some(683.3125),
                    Some(683.3125),
                    Some(564.2031),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                    Some(564.2031),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_hOriginalControllerOfCurrentPawn() {
        let prop = (
            "CCSPlayerController.m_hOriginalControllerOfCurrentPawn",
            PropColumn {
                data: Some(U32(vec![
                    Some(13582340),
                    Some(4669446),
                    Some(6569987),
                    Some(11747330),
                    Some(8536072),
                    Some(4685833),
                    Some(11337738),
                    Some(1327105),
                    Some(14581767),
                    Some(8355845),
                    Some(13582340),
                    Some(4669446),
                    Some(6569987),
                    Some(11747330),
                    Some(8536072),
                    Some(4685833),
                    Some(11337738),
                    Some(1327105),
                    Some(14581767),
                    Some(8355845),
                    Some(13582340),
                    Some(4669446),
                    Some(6569987),
                    Some(11747330),
                    Some(8536072),
                    Some(4685833),
                    Some(11337738),
                    Some(1327105),
                    Some(14581767),
                    Some(8355845),
                    Some(13582340),
                    Some(4669446),
                    Some(6569987),
                    Some(11747330),
                    Some(8536072),
                    Some(4685833),
                    Some(11337738),
                    Some(1327105),
                    Some(14581767),
                    Some(8355845),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_vRagdollServerOrigin() {
        let prop = (
            "CCSPlayerPawn.m_vRagdollServerOrigin",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-1187.1215, -391.00195, -55.96875]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-687.58527, -1591.4546, -167.96875]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-613.96985, -1326.2135, -167.96875]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-146.60188, -1949.172, -167.96875]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_vRagdollDamagePosition() {
        let prop = (
            "CCSPlayerPawn.m_vRagdollDamagePosition",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([-1177.3091, -393.89706, -11.370972]),
                    Some([-504.80264, -1454.2224, -24.087448]),
                    Some([-912.3319, -1481.2107, -105.44267]),
                    Some([-1161.0746, -606.4787, -109.58214]),
                    Some([-828.7623, -2534.7646, 27.780762]),
                    Some([-1329.447, -975.2356, -124.65142]),
                    Some([-1105.1072, -604.2416, -92.25015]),
                    Some([-803.9081, 10.710815, -107.94034]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-1034.5951, -1490.5745, -118.766106]),
                    Some([-2302.8567, 320.36478, -112.783165]),
                    Some([-314.79352, -922.4806, -103.29794]),
                    Some([-2290.6543, 482.9251, -113.752785]),
                    Some([-1182.3367, -394.19177, 3.4436035]),
                    Some([-859.2573, -2460.5, -29.6559]),
                    Some([-1329.447, -975.2356, -124.65142]),
                    Some([-229.48274, -2385.9326, -121.67381]),
                    Some([-562.3154, -1690.9232, -136.90385]),
                    Some([-2169.6887, 690.8084, -70.36412]),
                    Some([-1034.5951, -1490.5745, -118.766106]),
                    Some([-2291.2625, 373.48062, -122.94483]),
                    Some([-693.6147, -1593.5266, -125.98947]),
                    Some([-2290.6543, 482.9251, -113.752785]),
                    Some([-746.0539, -1414.7804, -111.439384]),
                    Some([-627.2682, -1325.6891, -110.70767]),
                    Some([-2332.9026, -290.97577, -117.977135]),
                    Some([-157.4043, -1945.0631, -124.29493]),
                    Some([-573.5157, -1003.0835, -133.33249]),
                    Some([-2169.6887, 690.8084, -70.36412]),
                    Some([-118.10736, -1666.3192, -124.282166]),
                    Some([-1308.5262, -1171.368, -50.149086]),
                    Some([-630.76263, -1662.4001, -144.75473]),
                    Some([-999.77985, 391.99142, -305.51318]),
                    Some([-764.8695, -110.90314, -113.249084]),
                    Some([-627.2682, -1325.6891, -110.70767]),
                    Some([-582.8451, -781.82654, -199.15674]),
                    Some([-614.638, -1577.8723, -81.12184]),
                    Some([-778.0574, -1577.9769, -124.16199]),
                    Some([63.989655, -2358.9756, 4.193103]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nNextSecondaryAttackTick() {
        let prop = (
            "m_nNextSecondaryAttackTick",
            PropColumn {
                data: Some(U32(vec![
                    Some(28714),
                    Some(28006),
                    Some(28260),
                    Some(28444),
                    Some(29020),
                    Some(28768),
                    Some(28968),
                    Some(28788),
                    Some(29050),
                    Some(28726),
                    Some(44648),
                    Some(49060),
                    Some(46326),
                    Some(48414),
                    None,
                    Some(46920),
                    Some(47992),
                    Some(43736),
                    Some(48966),
                    Some(48852),
                    Some(68608),
                    Some(68916),
                    None,
                    Some(68616),
                    Some(69172),
                    None,
                    Some(68956),
                    None,
                    Some(68940),
                    Some(68702),
                    Some(87976),
                    Some(89042),
                    Some(88524),
                    Some(88024),
                    Some(89002),
                    Some(88172),
                    Some(87766),
                    Some(88972),
                    Some(88838),
                    Some(88626),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_cellY() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_cellY",
            PropColumn {
                data: Some(U32(vec![
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    None,
                    None,
                    None,
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_hSequence() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_hSequence",
            PropColumn {
                data: Some(U64(vec![
                    Some(1),
                    Some(17),
                    Some(4),
                    Some(18),
                    Some(18),
                    Some(18),
                    Some(18),
                    Some(18),
                    Some(18),
                    Some(18),
                    Some(3),
                    Some(4),
                    Some(3),
                    Some(16),
                    None,
                    Some(4),
                    Some(19),
                    Some(4),
                    Some(4),
                    Some(18),
                    Some(16),
                    Some(4),
                    None,
                    Some(16),
                    Some(25),
                    None,
                    Some(11),
                    None,
                    Some(18),
                    Some(4),
                    Some(11),
                    Some(3),
                    Some(3),
                    Some(18),
                    Some(18),
                    Some(4),
                    Some(18),
                    Some(4),
                    Some(4),
                    Some(18),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn velocity_Y() {
        let prop = ("velocity_Y", PropColumn { data: None, num_nones: 40 });
        assert_eq!(out.0.df[&VELOCITY_Y_ID], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_timeUntilNextPhaseStarts() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_timeUntilNextPhaseStarts",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_iNumConsecutiveCTLoses() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveCTLoses",
            PropColumn {
                data: Some(I32(vec![
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_flStamina() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_flStamina",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(15.22197),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iTeamNum() {
        let prop = (
            "CCSPlayerController.m_iTeamNum",
            PropColumn {
                data: Some(U32(vec![
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }

    #[test]
    fn CCSPlayerController_m_iszPlayerName() {
        let prop = (
            "CCSPlayerController.m_iszPlayerName",
            PropColumn {
                data: Some(String(vec![
                    Some("123".to_string()),
                    Some("Trahun <3 V".to_string()),
                    Some("Голова, глаза".to_string()),
                    Some("NIGHTSOUL".to_string()),
                    Some("Dog".to_string()),
                    Some("miu miu".to_string()),
                    Some("-ExΩtiC-".to_string()),
                    Some("Подсосник blick'a".to_string()),
                    Some("povergo".to_string()),
                    Some("IMI Negev".to_string()),
                    Some("123".to_string()),
                    Some("Trahun <3 V".to_string()),
                    Some("Голова, глаза".to_string()),
                    Some("NIGHTSOUL".to_string()),
                    Some("Dog".to_string()),
                    Some("miu miu".to_string()),
                    Some("-ExΩtiC-".to_string()),
                    Some("Подсосник blick'a".to_string()),
                    Some("povergo".to_string()),
                    Some("IMI Negev".to_string()),
                    Some("123".to_string()),
                    Some("Trahun <3 V".to_string()),
                    Some("Голова, глаза".to_string()),
                    Some("NIGHTSOUL".to_string()),
                    Some("Dog".to_string()),
                    Some("miu miu".to_string()),
                    Some("-ExΩtiC-".to_string()),
                    Some("Подсосник blick'a".to_string()),
                    Some("povergo".to_string()),
                    Some("IMI Negev".to_string()),
                    Some("123".to_string()),
                    Some("Trahun <3 V".to_string()),
                    Some("Голова, глаза".to_string()),
                    Some("NIGHTSOUL".to_string()),
                    Some("Dog".to_string()),
                    Some("miu miu".to_string()),
                    Some("-ExΩtiC-".to_string()),
                    Some("Подсосник blick'a".to_string()),
                    Some("povergo".to_string()),
                    Some("IMI Negev".to_string()),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_vHeadConstraintOffset() {
        let prop = (
            "CCSPlayerPawn.m_vHeadConstraintOffset",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn velocity() {
        let prop = ("velocity", PropColumn { data: None, num_nones: 40 });
        assert_eq!(out.0.df[&VELOCITY_ID], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iDamage() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDamage",
            PropColumn {
                data: Some(I32(vec![
                    Some(100),
                    Some(28),
                    Some(185),
                    Some(0),
                    Some(194),
                    Some(0),
                    Some(33),
                    Some(100),
                    Some(107),
                    Some(115),
                    Some(61),
                    Some(52),
                    Some(185),
                    Some(100),
                    Some(99),
                    Some(0),
                    Some(26),
                    Some(139),
                    Some(100),
                    Some(71),
                    Some(28),
                    Some(60),
                    Some(44),
                    Some(50),
                    Some(46),
                    Some(76),
                    Some(130),
                    Some(108),
                    Some(23),
                    Some(100),
                    Some(188),
                    Some(41),
                    Some(44),
                    Some(150),
                    Some(153),
                    Some(76),
                    Some(130),
                    Some(108),
                    Some(100),
                    Some(100),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_nPawnCharacterDefIndex() {
        let prop = (
            "CCSPlayerController.m_nPawnCharacterDefIndex",
            PropColumn {
                data: Some(U32(vec![
                    Some(5036),
                    Some(5037),
                    Some(5036),
                    Some(5036),
                    Some(5037),
                    Some(5037),
                    Some(5037),
                    Some(5036),
                    Some(5037),
                    Some(5036),
                    Some(5036),
                    Some(5037),
                    Some(5036),
                    Some(5036),
                    Some(5037),
                    Some(5037),
                    Some(5037),
                    Some(5036),
                    Some(5037),
                    Some(5036),
                    Some(5036),
                    Some(5037),
                    Some(5036),
                    Some(5036),
                    Some(5037),
                    Some(5037),
                    Some(5037),
                    Some(5036),
                    Some(5037),
                    Some(5036),
                    Some(5036),
                    Some(5037),
                    Some(5036),
                    Some(5036),
                    Some(5037),
                    Some(5037),
                    Some(5037),
                    Some(5036),
                    Some(5037),
                    Some(5036),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iPendingTeamNum() {
        let prop = (
            "CCSPlayerController.m_iPendingTeamNum",
            PropColumn {
                data: Some(U32(vec![
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_iClip1() {
        let prop = (
            "m_iClip1",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(7),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(30),
                    Some(30),
                    Some(30),
                    Some(30),
                    None,
                    Some(30),
                    Some(30),
                    Some(30),
                    Some(25),
                    Some(8),
                    Some(16),
                    Some(7),
                    None,
                    Some(19),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    Some(9),
                    Some(7),
                    Some(0),
                    Some(20),
                    Some(30),
                    Some(0),
                    Some(0),
                    Some(13),
                    Some(0),
                    None,
                    Some(12),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_iInventoryPosition() {
        let prop = (
            "m_iInventoryPosition",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(13),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(67),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(68),
                    Some(42),
                    Some(0),
                    Some(0),
                    None,
                    Some(59),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(14),
                    Some(68),
                    Some(5),
                    None,
                    Some(0),
                    Some(165),
                    None,
                    Some(0),
                    None,
                    Some(17),
                    Some(14),
                    Some(0),
                    Some(165),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(259),
                    Some(67),
                    None,
                    Some(0),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn user_id() {
        let prop = (
            "user_id",
            PropColumn {
                data: Some(I32(vec![
                    Some(3),
                    Some(5),
                    Some(2),
                    Some(1),
                    Some(7),
                    Some(8),
                    Some(9),
                    Some(0),
                    Some(6),
                    Some(4),
                    Some(3),
                    Some(5),
                    Some(2),
                    Some(1),
                    Some(7),
                    Some(8),
                    Some(9),
                    Some(0),
                    Some(6),
                    Some(4),
                    Some(3),
                    Some(5),
                    Some(2),
                    Some(1),
                    Some(7),
                    Some(8),
                    Some(9),
                    Some(0),
                    Some(6),
                    Some(4),
                    Some(3),
                    Some(5),
                    Some(2),
                    Some(1),
                    Some(7),
                    Some(8),
                    Some(9),
                    Some(0),
                    Some(6),
                    Some(4),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&USERID_ID], prop.1);
    }
    #[test]
    fn m_hOwnerEntity() {
        let prop = (
            "m_hOwnerEntity",
            PropColumn {
                data: Some(U32(vec![
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    None,
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    None,
                    Some(1146999),
                    Some(15466622),
                    None,
                    Some(9863309),
                    None,
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    None,
                    Some(6389962),
                    Some(2998498),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CBodyComponentBaseAnimGraph_m_flLastTeleportTime() {
        let prop = (
            "CCSPlayerPawn.CBodyComponentBaseAnimGraph.m_flLastTeleportTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(314.75),
                    Some(314.75),
                    Some(314.75),
                    Some(314.75),
                    Some(314.75),
                    Some(314.75),
                    Some(314.75),
                    Some(314.75),
                    Some(314.75),
                    Some(314.75),
                    Some(484.0),
                    Some(484.0),
                    Some(484.0),
                    Some(484.0),
                    Some(484.0),
                    Some(484.0),
                    Some(484.0),
                    Some(484.0),
                    Some(484.0),
                    Some(484.0),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                    Some(683.3125),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_aimPunchAngle() {
        let prop = (
            "CCSPlayerPawn.m_aimPunchAngle",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.6257424, 0.0391259, 0.0]),
                    Some([0.7700784, 0.02841704, 0.0]),
                    Some([0.0752761, -0.003006973, 0.0]),
                    Some([-5.096302, 0.34865546, 1.8917837e-6]),
                    Some([0.6225749, 0.013033606, 0.0]),
                    Some([-5.0283604, -1.4114164, -2.156072e-6]),
                    Some([0.19490086, 0.015310853, 0.0]),
                    Some([0.3671517, 0.028899848, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-2.304628, -0.09649117, -3.8175546e-8]),
                    Some([0.13787195, -0.0026435894, 0.0]),
                    Some([-0.24363059, -0.11250786, -8.143596e-7]),
                    Some([-0.6233528, 0.00069459085, 4.5761978e-10]),
                    Some([-1.3367857, -0.12772557, -3.002365e-8]),
                    Some([-0.9370359, -0.047985114, -6.6237595e-7]),
                    Some([-2.941906, 0.2179837, 1.5002497e-6]),
                    Some([-4.5515127, 1.190071, 1.1232146e-6]),
                    Some([-0.087496266, 0.008882257, 1.797804e-9]),
                    Some([-0.5217251, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nOwnerId() {
        let prop = (
            "m_nOwnerId",
            PropColumn {
                data: Some(U32(vec![
                    Some(174817384),
                    Some(3920199789),
                    Some(4014669938),
                    Some(237174903),
                    Some(702021758),
                    Some(4025155718),
                    Some(3006070925),
                    Some(3048603807),
                    Some(3535995082),
                    Some(576422114),
                    Some(174817384),
                    Some(3920199789),
                    Some(4014669938),
                    Some(237174903),
                    None,
                    Some(4025155718),
                    Some(3006070925),
                    Some(3048603807),
                    Some(3535995082),
                    Some(576422114),
                    Some(174817384),
                    Some(3920199789),
                    None,
                    Some(237174903),
                    Some(702021758),
                    None,
                    Some(3006070925),
                    None,
                    Some(3535995082),
                    Some(576422114),
                    Some(174817384),
                    Some(3920199789),
                    Some(4014669938),
                    Some(237174903),
                    Some(702021758),
                    Some(4025155718),
                    Some(3006070925),
                    None,
                    Some(3535995082),
                    Some(576422114),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iPawnLifetimeEnd() {
        let prop = (
            "CCSPlayerController.m_iPawnLifetimeEnd",
            PropColumn {
                data: Some(I32(vec![
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_arrForceSubtickMoveWhen() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_arrForceSubtickMoveWhen",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(382.15256),
                    Some(382.0967),
                    Some(381.8938),
                    Some(382.24454),
                    Some(372.66974),
                    Some(382.1154),
                    Some(381.3079),
                    Some(361.11783),
                    Some(382.22818),
                    Some(382.2228),
                    Some(538.49634),
                    Some(538.511),
                    Some(535.7737),
                    Some(538.4727),
                    Some(538.371),
                    Some(529.3963),
                    Some(538.42053),
                    Some(532.6795),
                    Some(538.38007),
                    Some(537.43854),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_fMolotovDamageTime() {
        let prop = (
            "CCSPlayerPawn.m_fMolotovDamageTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bRagdollDamageHeadshot() {
        let prop = (
            "CCSPlayerPawn.m_bRagdollDamageHeadshot",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_iMoveState() {
        let prop = (
            "CCSPlayerPawn.m_iMoveState",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_hPrevOwner() {
        let prop = (
            "m_hPrevOwner",
            PropColumn {
                data: Some(U32(vec![
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    None,
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    None,
                    Some(16777215),
                    Some(16777215),
                    None,
                    Some(16777215),
                    None,
                    None,
                    None,
                    Some(9863309),
                    Some(15466622),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    Some(16777215),
                    None,
                    Some(16777215),
                    Some(16777215),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_OriginalOwnerXuidHigh() {
        let prop = (
            "m_OriginalOwnerXuidHigh",
            PropColumn {
                data: Some(U32(vec![
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    None,
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    None,
                    Some(17825793),
                    Some(17825793),
                    None,
                    Some(17825793),
                    None,
                    None,
                    None,
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    Some(17825793),
                    None,
                    Some(17825793),
                    Some(17825793),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_bDuckOverride() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDuckOverride",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InventoryServices_m_nPersonaDataPublicCommendsLeader() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsLeader",
            PropColumn {
                data: Some(I32(vec![
                    Some(42),
                    Some(13),
                    Some(38),
                    Some(158),
                    Some(30),
                    Some(20),
                    Some(24),
                    Some(13),
                    Some(42),
                    Some(43),
                    Some(42),
                    Some(13),
                    Some(38),
                    Some(158),
                    Some(30),
                    Some(20),
                    Some(24),
                    Some(13),
                    Some(42),
                    Some(43),
                    Some(42),
                    Some(13),
                    Some(38),
                    Some(158),
                    Some(30),
                    Some(20),
                    Some(24),
                    Some(13),
                    Some(42),
                    Some(43),
                    Some(42),
                    Some(13),
                    Some(38),
                    Some(158),
                    Some(30),
                    Some(20),
                    Some(24),
                    Some(13),
                    Some(42),
                    Some(43),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_OriginalOwnerXuidLow() {
        let prop = (
            "m_OriginalOwnerXuidLow",
            PropColumn {
                data: Some(U32(vec![
                    Some(305101042),
                    Some(364577347),
                    Some(158538184),
                    Some(284488898),
                    Some(234429022),
                    Some(112783799),
                    Some(297778383),
                    Some(3754702),
                    Some(320710059),
                    Some(242088265),
                    Some(305101042),
                    Some(364577347),
                    Some(158538184),
                    Some(284488898),
                    None,
                    Some(112783799),
                    Some(3754702),
                    Some(3754702),
                    Some(320710059),
                    Some(242088265),
                    Some(305101042),
                    Some(364577347),
                    None,
                    Some(158538184),
                    Some(234429022),
                    None,
                    Some(297778383),
                    None,
                    Some(320710059),
                    Some(242088265),
                    Some(297778383),
                    Some(234429022),
                    Some(158538184),
                    Some(284488898),
                    Some(234429022),
                    Some(112783799),
                    Some(297778383),
                    None,
                    Some(320710059),
                    Some(242088265),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InventoryServices_m_unMusicID() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InventoryServices.m_unMusicID",
            PropColumn {
                data: Some(U32(vec![
                    Some(70),
                    Some(70),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(70),
                    Some(1),
                    Some(70),
                    Some(70),
                    Some(68),
                    Some(70),
                    Some(70),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(70),
                    Some(1),
                    Some(70),
                    Some(70),
                    Some(68),
                    Some(70),
                    Some(70),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(70),
                    Some(1),
                    Some(70),
                    Some(70),
                    Some(68),
                    Some(70),
                    Some(70),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(70),
                    Some(1),
                    Some(70),
                    Some(70),
                    Some(68),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_weaponMode() {
        let prop = (
            "m_weaponMode",
            PropColumn {
                data: Some(U64(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(1),
                    None,
                    Some(0),
                    None,
                    None,
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(1),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nForceBone() {
        let prop = (
            "CCSPlayerPawn.m_nForceBone",
            PropColumn {
                data: Some(I32(vec![
                    Some(14),
                    Some(6),
                    Some(4),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(13),
                    Some(6),
                    Some(0),
                    Some(4),
                    Some(6),
                    Some(6),
                    Some(4),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(4),
                    Some(6),
                    Some(6),
                    Some(1),
                    Some(14),
                    Some(4),
                    Some(6),
                    Some(4),
                    Some(4),
                    Some(1),
                    Some(1),
                    Some(4),
                    Some(-1),
                    Some(5),
                    Some(25),
                    Some(6),
                    Some(4),
                    Some(4),
                    Some(6),
                    Some(0),
                    Some(1),
                    Some(-1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nViewModelIndex() {
        let prop = (
            "m_nViewModelIndex",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iEnemiesFlashed() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEnemiesFlashed",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(3),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_bBurstMode() {
        let prop = (
            "m_bBurstMode",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    None,
                    None,
                    None,
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_iNumConsecutiveTerroristLoses() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveTerroristLoses",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_fMatchStartTime() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_fMatchStartTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                    Some(70.78125),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iCompetitiveRankType() {
        let prop = (
            "CCSPlayerController.m_iCompetitiveRankType",
            PropColumn {
                data: Some(I32(vec![
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                    Some(11),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_nRoundsPlayedThisPhase() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_nRoundsPlayedThisPhase",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_vecLadderNormal() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_vecLadderNormal",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([2043.002, 0.0, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([2043.002, 0.0, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([2043.002, 0.0, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([0.0, -2046.0005, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([2043.002, 0.0, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                    Some([2043.002, 0.0, 0.0]),
                    Some([0.0, 0.0, 1.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_angRotation() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_angRotation",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    None,
                    None,
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nNextPrimaryAttackTick() {
        let prop = (
            "m_nNextPrimaryAttackTick",
            PropColumn {
                data: Some(U32(vec![
                    Some(28714),
                    Some(28006),
                    Some(28260),
                    Some(28444),
                    Some(29020),
                    Some(28768),
                    Some(28968),
                    Some(28788),
                    Some(29050),
                    Some(28726),
                    Some(44648),
                    Some(49060),
                    Some(46326),
                    Some(48414),
                    None,
                    Some(46920),
                    Some(47992),
                    Some(43736),
                    Some(48966),
                    Some(48852),
                    Some(68608),
                    Some(68916),
                    None,
                    Some(68616),
                    Some(69172),
                    None,
                    Some(68956),
                    None,
                    Some(68940),
                    Some(68702),
                    Some(87976),
                    Some(89042),
                    Some(88524),
                    Some(88024),
                    Some(89002),
                    Some(88172),
                    Some(87766),
                    Some(88972),
                    Some(88838),
                    Some(88626),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn velocity_X() {
        let prop = ("velocity_X", PropColumn { data: None, num_nones: 40 });
        assert_eq!(out.0.df[&VELOCITY_X_ID], prop.1);
    }
    #[test]
    fn m_fAccuracyPenalty() {
        let prop = (
            "m_fAccuracyPenalty",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0042),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.00641),
                    Some(0.10217),
                    Some(0.00641),
                    Some(0.0133),
                    None,
                    Some(0.004900002),
                    Some(0.0048100017),
                    Some(0.0133),
                    Some(0.00985),
                    Some(0.0030019027),
                    Some(0.0064100022),
                    Some(0.020077005),
                    None,
                    Some(0.0064100022),
                    Some(0.1046),
                    None,
                    Some(0.0),
                    None,
                    Some(0.054671634),
                    Some(0.0317),
                    Some(0.0),
                    Some(0.0049),
                    Some(0.00641),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0049),
                    Some(0.0),
                    None,
                    Some(0.0049),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_iHealth() {
        let prop = (
            "CCSPlayerPawn.m_iHealth",
            PropColumn {
                data: Some(I32(vec![
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(48),
                    Some(100),
                    Some(100),
                    Some(1),
                    Some(0),
                    Some(100),
                    Some(100),
                    Some(74),
                    Some(29),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(0),
                    Some(48),
                    Some(20),
                    Some(0),
                    Some(100),
                    Some(0),
                    Some(50),
                    Some(77),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                    Some(100),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_ArmorValue() {
        let prop = (
            "CCSPlayerPawn.m_ArmorValue",
            PropColumn {
                data: Some(I32(vec![
                    Some(100),
                    Some(0),
                    Some(0),
                    Some(100),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(100),
                    Some(0),
                    Some(100),
                    Some(66),
                    Some(100),
                    Some(100),
                    Some(89),
                    Some(0),
                    Some(100),
                    Some(100),
                    Some(0),
                    Some(94),
                    Some(100),
                    Some(46),
                    Some(0),
                    Some(0),
                    Some(78),
                    Some(48),
                    Some(0),
                    Some(100),
                    Some(0),
                    Some(0),
                    Some(54),
                    Some(56),
                    Some(0),
                    Some(100),
                    Some(100),
                    Some(0),
                    Some(86),
                    Some(0),
                    Some(100),
                    Some(0),
                    Some(46),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iCompetitiveWins() {
        let prop = (
            "CCSPlayerController.m_iCompetitiveWins",
            PropColumn {
                data: Some(I32(vec![
                    Some(125),
                    Some(56),
                    Some(37),
                    Some(63),
                    Some(60),
                    Some(24),
                    Some(59),
                    Some(41),
                    Some(38),
                    Some(19),
                    Some(125),
                    Some(56),
                    Some(37),
                    Some(63),
                    Some(60),
                    Some(24),
                    Some(59),
                    Some(41),
                    Some(38),
                    Some(19),
                    Some(125),
                    Some(56),
                    Some(37),
                    Some(63),
                    Some(60),
                    Some(24),
                    Some(59),
                    Some(41),
                    Some(38),
                    Some(19),
                    Some(125),
                    Some(56),
                    Some(37),
                    Some(63),
                    Some(60),
                    Some(24),
                    Some(59),
                    Some(41),
                    Some(38),
                    Some(19),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iCompetitiveRankingPredicted_Win() {
        let prop = (
            "CCSPlayerController.m_iCompetitiveRankingPredicted_Win",
            PropColumn {
                data: Some(I32(vec![
                    Some(18345),
                    Some(18265),
                    Some(10120),
                    Some(15115),
                    Some(19012),
                    Some(16590),
                    Some(19201),
                    Some(14167),
                    Some(18082),
                    Some(0),
                    Some(18345),
                    Some(18265),
                    Some(10120),
                    Some(15115),
                    Some(19012),
                    Some(16590),
                    Some(19201),
                    Some(14167),
                    Some(18082),
                    Some(0),
                    Some(18345),
                    Some(18265),
                    Some(10120),
                    Some(15115),
                    Some(19012),
                    Some(16590),
                    Some(19201),
                    Some(14167),
                    Some(18082),
                    Some(0),
                    Some(18345),
                    Some(18265),
                    Some(10120),
                    Some(15115),
                    Some(19012),
                    Some(16590),
                    Some(19201),
                    Some(14167),
                    Some(18082),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iAssists() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iAssists",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_bEverPlayedOnTeam() {
        let prop = (
            "CCSPlayerController.m_bEverPlayedOnTeam",
            PropColumn {
                data: Some(Bool(vec![
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_pReserveAmmo() {
        let prop = (
            "m_pReserveAmmo",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(81),
                    None,
                    Some(0),
                    Some(63),
                    Some(0),
                    Some(81),
                    Some(81),
                    Some(60),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(1),
                    None,
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(80),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_fFlags() {
        let prop = (
            "CCSPlayerController.m_fFlags",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_iTeamNum() {
        let prop = (
            "CCSPlayerPawn.m_iTeamNum",
            PropColumn {
                data: Some(U32(vec![
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CEconItemAttribute_m_iAttributeDefinitionIndex() {
        let prop = (
            "CEconItemAttribute.m_iAttributeDefinitionIndex",
            PropColumn {
                data: Some(U32(vec![
                    None,
                    Some(75),
                    None,
                    None,
                    None,
                    None,
                    Some(75),
                    None,
                    None,
                    None,
                    Some(8),
                    Some(75),
                    None,
                    Some(81),
                    None,
                    Some(121),
                    None,
                    Some(81),
                    None,
                    Some(125),
                    Some(8),
                    Some(121),
                    None,
                    None,
                    Some(75),
                    None,
                    None,
                    None,
                    Some(8),
                    None,
                    None,
                    Some(75),
                    None,
                    None,
                    None,
                    Some(81),
                    Some(75),
                    None,
                    None,
                    Some(75),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iCompTeammateColor() {
        let prop = (
            "CCSPlayerController.m_iCompTeammateColor",
            PropColumn {
                data: Some(String(vec![
                    Some("orange".to_string()),
                    Some("green".to_string()),
                    Some("green".to_string()),
                    Some("yellow".to_string()),
                    Some("yellow".to_string()),
                    Some("purple".to_string()),
                    Some("blue".to_string()),
                    Some("purple".to_string()),
                    Some("orange".to_string()),
                    Some("blue".to_string()),
                    Some("orange".to_string()),
                    Some("green".to_string()),
                    Some("green".to_string()),
                    Some("yellow".to_string()),
                    Some("yellow".to_string()),
                    Some("purple".to_string()),
                    Some("blue".to_string()),
                    Some("purple".to_string()),
                    Some("orange".to_string()),
                    Some("blue".to_string()),
                    Some("orange".to_string()),
                    Some("green".to_string()),
                    Some("green".to_string()),
                    Some("yellow".to_string()),
                    Some("yellow".to_string()),
                    Some("purple".to_string()),
                    Some("blue".to_string()),
                    Some("purple".to_string()),
                    Some("orange".to_string()),
                    Some("blue".to_string()),
                    Some("orange".to_string()),
                    Some("green".to_string()),
                    Some("green".to_string()),
                    Some("yellow".to_string()),
                    Some("yellow".to_string()),
                    Some("purple".to_string()),
                    Some("blue".to_string()),
                    Some("purple".to_string()),
                    Some("orange".to_string()),
                    Some("blue".to_string()),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bIsBuyMenuOpen() {
        let prop = (
            "CCSPlayerPawn.m_bIsBuyMenuOpen",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iPawnArmor() {
        let prop = ("CCSPlayerController.m_iPawnArmor", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_unCurrentEquipmentValue() {
        let prop = (
            "CCSPlayerPawn.m_unCurrentEquipmentValue",
            PropColumn {
                data: Some(U32(vec![
                    Some(2400),
                    Some(200),
                    Some(700),
                    Some(4200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(4200),
                    Some(200),
                    Some(1000),
                    Some(4300),
                    Some(5900),
                    Some(5100),
                    Some(2750),
                    Some(5600),
                    Some(5300),
                    Some(5700),
                    Some(1950),
                    Some(4200),
                    Some(3100),
                    Some(5100),
                    Some(700),
                    Some(5100),
                    Some(5100),
                    Some(4100),
                    Some(4100),
                    Some(4450),
                    Some(2600),
                    Some(300),
                    Some(4200),
                    Some(5000),
                    Some(3100),
                    Some(3900),
                    Some(5100),
                    Some(3100),
                    Some(5300),
                    Some(200),
                    Some(6950),
                    Some(200),
                    Some(5500),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn weapon_skin() {
        let prop = (
            "weapon_skin",
            PropColumn {
                data: Some(String(vec![
                    None,
                    Some("Lore".to_string()),
                    None,
                    None,
                    None,
                    None,
                    Some("Scorched".to_string()),
                    None,
                    None,
                    None,
                    None,
                    Some("TheEmperor".to_string()),
                    None,
                    Some("Monkeyflage".to_string()),
                    None,
                    Some("Magnesium".to_string()),
                    None,
                    Some("Monkeyflage".to_string()),
                    None,
                    Some("Necropos".to_string()),
                    None,
                    Some("Printstream".to_string()),
                    None,
                    None,
                    Some("BorealForest".to_string()),
                    None,
                    None,
                    None,
                    Some("FacilityDraft".to_string()),
                    Some("Necropos".to_string()),
                    None,
                    Some("BorealForest".to_string()),
                    None,
                    None,
                    None,
                    Some("Ivory".to_string()),
                    Some("Scorched".to_string()),
                    None,
                    None,
                    Some("CaseHardened".to_string()),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&WEAPON_SKIN_NAME], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_thirdPersonHeading() {
        let prop = (
            "CCSPlayerPawn.m_thirdPersonHeading",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, -130.40326, 0.0]),
                    Some([0.0, -28.618698, 0.0]),
                    Some([0.0, 117.88812, 0.0]),
                    Some([0.0, -85.43003, 0.0]),
                    Some([0.0, 105.99988, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 94.00006, 0.0]),
                    Some([0.0, -116.81728, 0.0]),
                    Some([0.0, -13.908691, 0.0]),
                    Some([0.0, -27.106018, 0.0]),
                    Some([0.0, -35.857315, 0.0]),
                    Some([0.0, -66.82949, 0.0]),
                    Some([0.0, 121.426056, 0.0]),
                    Some([0.0, -121.45386, 0.0]),
                    Some([0.0, -64.759254, 0.0]),
                    Some([0.0, 35.886505, 0.0]),
                    Some([0.0, 82.49875, 0.0]),
                    Some([0.0, 94.52533, 0.0]),
                    Some([0.0, -45.80612, 0.0]),
                    Some([0.0, -145.5589, 0.0]),
                    Some([0.0, -60.42137, 0.0]),
                    Some([0.0, -22.725555, 0.0]),
                    Some([0.0, -86.54137, 0.0]),
                    Some([0.0, -33.746216, 0.0]),
                    Some([0.0, -37.22786, 0.0]),
                    Some([0.0, 114.522156, 0.0]),
                    Some([0.0, 68.98898, 0.0]),
                    Some([0.0, 7.3034973, 0.0]),
                    Some([0.0, -170.91225, 0.0]),
                    Some([0.0, 7.812317, 0.0]),
                    Some([0.0, -171.97037, 0.0]),
                    Some([0.0, 90.21939, 0.0]),
                    Some([0.0, -107.425, 0.0]),
                    Some([0.0, -55.725403, 0.0]),
                    Some([0.0, 108.01416, 0.0]),
                    Some([0.0, -112.60025, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 162.4205, 0.0]),
                    Some([0.0, -38.886444, 0.0]),
                    Some([0.0, 61.380615, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_hOuter() {
        let prop = (
            "m_hOuter",
            PropColumn {
                data: Some(U32(vec![
                    Some(5439588),
                    Some(8290703),
                    Some(3506327),
                    Some(3916005),
                    Some(11894994),
                    Some(1409172),
                    Some(10895583),
                    Some(1228924),
                    Some(9257057),
                    Some(15991031),
                    Some(4325647),
                    Some(14385274),
                    Some(3948806),
                    Some(8700126),
                    None,
                    Some(1769877),
                    Some(13385865),
                    Some(5455972),
                    Some(1655192),
                    Some(8274323),
                    Some(4325647),
                    Some(1868065),
                    None,
                    Some(3948806),
                    Some(2867657),
                    None,
                    Some(12648850),
                    None,
                    Some(12075220),
                    Some(8274323),
                    Some(2048117),
                    Some(8323471),
                    Some(14549389),
                    Some(11026752),
                    Some(4620520),
                    Some(1573292),
                    Some(2244701),
                    Some(2900425),
                    Some(7586023),
                    Some(557561),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iNumRoundKillsHeadshots() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iNumRoundKillsHeadshots",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_vRagdollDamageForce() {
        let prop = (
            "CCSPlayerPawn.m_vRagdollDamageForce",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([-9610.976, 21332.68, 11337.813]),
                    Some([7039.566, 18674.207, 1310.9368]),
                    Some([-20683.145, 15521.969, -2697.4182]),
                    Some([-25677.807, 1769.0138, 3677.0781]),
                    Some([-2866.7861, -25648.094, 3155.458]),
                    Some([-5430.6943, 30720.87, -419.36847]),
                    Some([4164.3174, 19542.664, 861.86615]),
                    Some([18510.584, -18258.072, -33.348263]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-21124.611, -11389.467, 175.63977]),
                    Some([-1726.7971, -31108.934, -1640.9121]),
                    Some([7496.364, -23256.37, 8885.152]),
                    Some([-11601.938, -16035.477, -2873.767]),
                    Some([-11208.161, 21473.85, 12591.308]),
                    Some([-20624.291, -23337.209, 1858.3392]),
                    Some([-5430.6943, 30720.87, -419.36847]),
                    Some([19226.12, 1142.8672, -5389.8203]),
                    Some([-23706.945, -3664.9272, -741.007]),
                    Some([-10969.59, 23518.8, 1591.8785]),
                    Some([-21124.611, -11389.467, 175.63977]),
                    Some([-16354.684, 12656.852, -3650.8167]),
                    Some([4053.6904, 23649.143, -534.3961]),
                    Some([-11601.938, -16035.477, -2873.767]),
                    Some([-5234.1226, -30757.06, -217.84229]),
                    Some([15397.523, -18387.809, 897.1388]),
                    Some([-7462.935, -22706.879, 2168.4565]),
                    Some([21494.084, 10667.385, -459.76852]),
                    Some([8644.244, -22377.998, -708.7194]),
                    Some([-10969.59, 23518.8, 1591.8785]),
                    Some([23958.57, -1248.9443, -653.4906]),
                    Some([385.308, 3664.9614, -6844.78]),
                    Some([1639.9626, -23939.91, -437.4075]),
                    Some([-29294.576, -1303.0881, -10656.917]),
                    Some([-13464.551, 24548.348, -290.74048]),
                    Some([15397.523, -18387.809, 897.1388]),
                    Some([30393.809, -5101.4565, -4861.2236]),
                    Some([2015.5789, -23902.805, 770.2814]),
                    Some([7895.492, -26860.441, -421.86017]),
                    Some([-19675.791, -3585.4912, -86.97464]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InventoryServices_m_nPersonaDataPublicLevel() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicLevel",
            PropColumn {
                data: Some(I32(vec![
                    Some(29),
                    Some(31),
                    Some(9),
                    Some(38),
                    Some(40),
                    Some(35),
                    Some(2),
                    Some(20),
                    Some(28),
                    Some(27),
                    Some(29),
                    Some(31),
                    Some(9),
                    Some(38),
                    Some(40),
                    Some(35),
                    Some(2),
                    Some(20),
                    Some(28),
                    Some(27),
                    Some(29),
                    Some(31),
                    Some(9),
                    Some(38),
                    Some(40),
                    Some(35),
                    Some(2),
                    Some(20),
                    Some(28),
                    Some(27),
                    Some(29),
                    Some(31),
                    Some(9),
                    Some(38),
                    Some(40),
                    Some(35),
                    Some(2),
                    Some(20),
                    Some(28),
                    Some(27),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_totalRoundsPlayed() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(4),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_fWarmupPeriodStart() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodStart",
            PropColumn {
                data: Some(F32(vec![
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                    Some(54.765625),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_bPawnHasHelmet() {
        let prop = ("CCSPlayerController.m_bPawnHasHelmet", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bInBuyZone() {
        let prop = (
            "CCSPlayerPawn.m_bInBuyZone",
            PropColumn {
                data: Some(Bool(vec![
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CEconItemAttribute_m_flInitialValue() {
        let prop = (
            "CEconItemAttribute.m_flInitialValue",
            PropColumn {
                data: Some(F32(vec![
                    None,
                    Some(9.733886e22),
                    None,
                    None,
                    None,
                    None,
                    Some(9.422597e22),
                    None,
                    None,
                    None,
                    Some(0.43085524),
                    Some(9.344775e22),
                    None,
                    Some(0.0),
                    None,
                    Some(6.363e-42),
                    None,
                    Some(0.0),
                    None,
                    Some(6.711e-42),
                    Some(0.43085524),
                    Some(7.636e-42),
                    None,
                    None,
                    Some(3.1171214e22),
                    None,
                    None,
                    None,
                    Some(0.07466238),
                    Some(6.711e-42),
                    None,
                    Some(3.1171214e22),
                    None,
                    None,
                    None,
                    Some(0.0),
                    Some(9.422597e22),
                    None,
                    None,
                    Some(3.525688e22),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iKills() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKills",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(0),
                    Some(1),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(5),
                    Some(1),
                    Some(1),
                    Some(5),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(5),
                    Some(3),
                    Some(3),
                    Some(8),
                    Some(1),
                    Some(1),
                    Some(7),
                    Some(5),
                    Some(2),
                    Some(5),
                    Some(7),
                    Some(4),
                    Some(3),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bHasMovedSinceSpawn() {
        let prop = (
            "CCSPlayerPawn.m_bHasMovedSinceSpawn",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flFlashMaxAlpha() {
        let prop = (
            "CCSPlayerPawn.m_flFlashMaxAlpha",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(255.0),
                    Some(255.0),
                    Some(0.0),
                    Some(255.0),
                    Some(0.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(0.0),
                    Some(0.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                    Some(255.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iHeadShotKills() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iHeadShotKills",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(4),
                    Some(1),
                    Some(1),
                    Some(3),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(3),
                    Some(2),
                    Some(3),
                    Some(6),
                    Some(1),
                    Some(1),
                    Some(4),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(3),
                    Some(2),
                    Some(3),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_nAnimLoopMode() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_nAnimLoopMode",
            PropColumn {
                data: Some(U32(vec![
                    Some(4),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_cellZ() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_cellZ",
            PropColumn {
                data: Some(U32(vec![
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    None,
                    None,
                    None,
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }

    #[test]
    fn m_iAccountID() {
        let prop = (
            "m_iAccountID",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(364577347),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(297778383),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(305101042),
                    Some(364577347),
                    Some(0),
                    Some(0),
                    None,
                    Some(112783799),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(242088265),
                    Some(305101042),
                    Some(364577347),
                    None,
                    Some(0),
                    Some(234429022),
                    None,
                    Some(0),
                    None,
                    Some(320710059),
                    Some(242088265),
                    Some(0),
                    Some(234429022),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(112783799),
                    Some(297778383),
                    None,
                    Some(1),
                    Some(242088265),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iCompetitiveRankingPredicted_Loss() {
        let prop = (
            "CCSPlayerController.m_iCompetitiveRankingPredicted_Loss",
            PropColumn {
                data: Some(I32(vec![
                    Some(17823),
                    Some(18015),
                    Some(9879),
                    Some(14895),
                    Some(18469),
                    Some(16148),
                    Some(18631),
                    Some(13632),
                    Some(17588),
                    Some(0),
                    Some(17823),
                    Some(18015),
                    Some(9879),
                    Some(14895),
                    Some(18469),
                    Some(16148),
                    Some(18631),
                    Some(13632),
                    Some(17588),
                    Some(0),
                    Some(17823),
                    Some(18015),
                    Some(9879),
                    Some(14895),
                    Some(18469),
                    Some(16148),
                    Some(18631),
                    Some(13632),
                    Some(17588),
                    Some(0),
                    Some(17823),
                    Some(18015),
                    Some(9879),
                    Some(14895),
                    Some(18469),
                    Some(16148),
                    Some(18631),
                    Some(13632),
                    Some(17588),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn entity_id() {
        let prop = (
            "entity_id",
            PropColumn {
                data: Some(I32(vec![
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&ENTITY_ID_ID], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_fWarmupPeriodEnd() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodEnd",
            PropColumn {
                data: Some(F32(vec![
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                    Some(69.765625),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_bDesiresDuck() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDesiresDuck",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_nRandomSeedOffset() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_nRandomSeedOffset",
            PropColumn {
                data: Some(I32(vec![
                    Some(23958827),
                    Some(94238205),
                    Some(821912157),
                    Some(-974379763),
                    Some(-529426837),
                    Some(-265493256),
                    Some(-706971072),
                    Some(19183598),
                    Some(-826435715),
                    Some(739735424),
                    Some(-318294761),
                    Some(259269252),
                    Some(368845761),
                    Some(-518486264),
                    None,
                    Some(550904609),
                    Some(68704265),
                    Some(636426175),
                    Some(330607055),
                    Some(-966642112),
                    Some(-318294761),
                    Some(-881717410),
                    None,
                    Some(368845761),
                    Some(-521444696),
                    None,
                    Some(-158322847),
                    None,
                    Some(-183492278),
                    Some(-966642112),
                    Some(149183010),
                    Some(-98125674),
                    Some(-818501016),
                    Some(-32576437),
                    Some(-3691338),
                    Some(898728176),
                    Some(-655850456),
                    Some(1013750409),
                    Some(-537760741),
                    Some(817940211),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_angEyeAngles() {
        let prop = (
            "CCSPlayerPawn.m_angEyeAngles",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([-2.2631836, -120.846176, 0.0]),
                    Some([-3.1108398, -4.5582886, 0.0]),
                    Some([3.06073, 118.377686, 0.0]),
                    Some([2.2748566, -88.57178, 0.0]),
                    Some([0.7041626, 107.24957, 0.0]),
                    Some([-8.254517, 17.670471, 0.0]),
                    Some([-2.8248596, 92.258026, 0.0]),
                    Some([0.12634277, -116.438255, 0.0]),
                    Some([0.035369873, -38.2623, 0.0]),
                    Some([14.900543, -26.279297, 0.0]),
                    Some([7.879257, -34.673187, 0.0]),
                    Some([2.2020721, -67.40662, 0.0]),
                    Some([0.66708374, 123.82553, 0.0]),
                    Some([-12.976913, -121.45386, 0.0]),
                    Some([27.015717, -64.44443, 0.0]),
                    Some([0.4576416, 35.886505, 0.0]),
                    Some([-3.1307678, 82.49875, 0.0]),
                    Some([15.228088, 95.833405, 0.0]),
                    Some([-5.262451, -45.82364, 0.0]),
                    Some([1.1556244, -145.5589, 0.0]),
                    Some([-11.231812, -62.706528, 0.0]),
                    Some([0.9702301, -42.504395, 0.0]),
                    Some([8.397675, -106.25805, 0.0]),
                    Some([0.72372437, -33.746216, 0.0]),
                    Some([4.839813, -44.32846, 0.0]),
                    Some([5.5968475, 143.18549, 0.0]),
                    Some([-14.218018, 80.36258, 0.0]),
                    Some([-5.181427, -83.19191, 0.0]),
                    Some([0.9328003, -170.05429, 0.0]),
                    Some([4.334442, 7.7237244, 0.0]),
                    Some([-58.338776, -170.79552, 0.0]),
                    Some([-1.9713593, 88.6806, 0.0]),
                    Some([3.4926147, -105.864944, 0.0]),
                    Some([-5.066757, -56.005898, 0.0]),
                    Some([7.515335, 136.82407, 0.0]),
                    Some([-9.539215, -88.32802, 0.0]),
                    Some([-0.89434814, 5.202362, 0.0]),
                    Some([6.603119, 165.99347, 0.0]),
                    Some([4.4350433, -38.887817, 0.0]),
                    Some([10.610733, 81.79184, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_nIdealMotionType() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_nIdealMotionType",
            PropColumn {
                data: Some(I32(vec![
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    None,
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    None,
                    Some(3),
                    Some(3),
                    None,
                    Some(3),
                    None,
                    None,
                    None,
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    None,
                    Some(3),
                    Some(3),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_vecZ() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_vecZ",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    None,
                    None,
                    None,
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iLiveTime() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iLiveTime",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(42),
                    Some(82),
                    Some(48),
                    Some(43),
                    Some(20),
                    Some(83),
                    Some(39),
                    Some(112),
                    Some(0),
                    Some(59),
                    Some(67),
                    Some(103),
                    Some(75),
                    Some(81),
                    Some(48),
                    Some(83),
                    Some(97),
                    Some(145),
                    Some(23),
                    Some(59),
                    Some(138),
                    Some(199),
                    Some(75),
                    Some(105),
                    Some(187),
                    Some(173),
                    Some(125),
                    Some(246),
                    Some(23),
                    Some(112),
                    Some(197),
                    Some(255),
                    Some(210),
                    Some(130),
                    Some(187),
                    Some(274),
                    Some(182),
                    Some(328),
                    Some(58),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nFireSequenceStartTimeChange() {
        let prop = (
            "m_nFireSequenceStartTimeChange",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(3),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(19),
                    None,
                    Some(0),
                    Some(41),
                    Some(0),
                    Some(9),
                    Some(11),
                    Some(93),
                    Some(0),
                    None,
                    Some(35),
                    Some(20),
                    None,
                    Some(0),
                    None,
                    Some(4),
                    Some(18),
                    Some(0),
                    Some(11),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(5),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_bOldJumpPressed() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_bOldJumpPressed",
            PropColumn {
                data: Some(Bool(vec![
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_MeshGroupMask() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_MeshGroupMask",
            PropColumn {
                data: Some(U64(vec![
                    Some(18446744073709551615),
                    Some(18446744073709551615),
                    Some(1),
                    Some(18446744073709551615),
                    Some(18446744073709551615),
                    Some(18446744073709551615),
                    Some(18446744073709551615),
                    Some(18446744073709551615),
                    Some(18446744073709551615),
                    Some(18446744073709551615),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(1),
                    None,
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(2),
                    None,
                    Some(1),
                    Some(2),
                    None,
                    Some(18446744073709551615),
                    None,
                    Some(2),
                    None,
                    Some(18446744073709551615),
                    Some(2),
                    Some(1),
                    Some(18446744073709551615),
                    Some(18446744073709551615),
                    Some(2),
                    Some(18446744073709551615),
                    None,
                    Some(1),
                    Some(18446744073709551615),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_bBombPlanted() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_bBombPlanted",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nHierarchyId() {
        let prop = (
            "CCSPlayerPawn.m_nHierarchyId",
            PropColumn {
                data: Some(U32(vec![
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iHeadShotKills() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iHeadShotKills",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nInteractsWith() {
        let prop = (
            "CCSPlayerPawn.m_nInteractsWith",
            PropColumn {
                data: Some(U64(vec![
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(34361847825),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(34361847825),
                    Some(2895889),
                    Some(2895889),
                    Some(34361847825),
                    Some(2895889),
                    Some(34361847825),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                    Some(2895889),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn X() {
        let prop = (
            "X",
            PropColumn {
                data: Some(F32(vec![
                    Some(1296.0),
                    Some(-1976.0),
                    Some(1376.0),
                    Some(1216.0),
                    Some(-1598.0),
                    Some(-1972.0),
                    Some(-1656.0),
                    Some(1296.0),
                    Some(-1830.0),
                    Some(1216.0),
                    Some(-1805.442),
                    Some(-897.31177),
                    Some(-310.4124),
                    Some(-944.03143),
                    Some(-1187.1215),
                    Some(-880.5703),
                    Some(-1162.164),
                    Some(280.03235),
                    Some(-821.3002),
                    Some(342.03232),
                    Some(-328.90698),
                    Some(-1374.0292),
                    Some(-688.80054),
                    Some(-926.01624),
                    Some(-482.53815),
                    Some(-611.6175),
                    Some(-1056.753),
                    Some(-140.86218),
                    Some(481.38428),
                    Some(60.47976),
                    Some(1216.0),
                    Some(-1632.0),
                    Some(1296.0),
                    Some(1376.0),
                    Some(-1656.0),
                    Some(-1656.0),
                    Some(-1902.0),
                    Some(1376.0),
                    Some(-1776.0),
                    Some(1216.0),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&PLAYER_X_ID], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_bGameRestart() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_bGameRestart",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bIsDefusing() {
        let prop = (
            "CCSPlayerPawn.m_bIsDefusing",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_iRoundWinStatus() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_iRoundWinStatus",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_flDuckSpeed() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckSpeed",
            PropColumn {
                data: Some(F32(vec![
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(5.236247),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(6.28749),
                    Some(4.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                    Some(8.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_flTimeSilencerSwitchComplete() {
        let prop = (
            "m_flTimeSilencerSwitchComplete",
            PropColumn {
                data: Some(F32(vec![
                    Some(224.32813),
                    Some(218.79688),
                    Some(220.78125),
                    Some(222.21875),
                    Some(226.71875),
                    Some(224.75),
                    Some(226.3125),
                    Some(224.90625),
                    Some(226.95313),
                    Some(224.42188),
                    Some(348.8125),
                    Some(383.28958),
                    Some(361.92188),
                    Some(357.14063),
                    None,
                    Some(366.57083),
                    Some(359.14063),
                    Some(341.6875),
                    Some(382.54688),
                    Some(346.28125),
                    Some(529.2969),
                    Some(538.40625),
                    None,
                    Some(526.71875),
                    Some(519.2115),
                    None,
                    Some(538.71875),
                    None,
                    Some(520.3281),
                    Some(536.7344),
                    Some(687.3125),
                    Some(695.649),
                    Some(691.59375),
                    Some(687.6875),
                    Some(695.3281),
                    Some(688.84375),
                    Some(685.6719),
                    Some(695.10834),
                    Some(694.0469),
                    Some(692.3906),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn weapon_skin_id() {
        let prop = (
            "weapon_skin_id",
            PropColumn {
                data: Some(U32(vec![
                    None,
                    Some(1104),
                    None,
                    None,
                    None,
                    None,
                    Some(175),
                    None,
                    None,
                    None,
                    Some(1221),
                    Some(844),
                    None,
                    Some(1150),
                    None,
                    Some(811),
                    None,
                    Some(1150),
                    None,
                    Some(538),
                    Some(1221),
                    Some(962),
                    None,
                    None,
                    Some(77),
                    None,
                    None,
                    None,
                    Some(777),
                    Some(538),
                    None,
                    Some(77),
                    None,
                    None,
                    None,
                    Some(357),
                    Some(175),
                    None,
                    None,
                    Some(44),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&WEAPON_SKIN_ID], prop.1);
    }

    #[test]
    fn CCSPlayerPawn_m_unRoundStartEquipmentValue() {
        let prop = (
            "CCSPlayerPawn.m_unRoundStartEquipmentValue",
            PropColumn {
                data: Some(U32(vec![
                    Some(700),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(3700),
                    Some(200),
                    Some(2900),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(4400),
                    Some(200),
                    Some(3900),
                    Some(4200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(2300),
                    Some(200),
                    Some(3300),
                    Some(4200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(5300),
                    Some(200),
                    Some(200),
                    Some(200),
                    Some(4300),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flSlopeDropHeight() {
        let prop = (
            "CCSPlayerPawn.m_flSlopeDropHeight",
            PropColumn {
                data: Some(F32(vec![
                    Some(-167.96875),
                    Some(-291.16428),
                    Some(-167.96875),
                    Some(-166.95316),
                    Some(-265.4476),
                    Some(-290.30576),
                    Some(-267.80722),
                    Some(-167.96875),
                    Some(-267.46368),
                    Some(-164.61621),
                    Some(-47.96875),
                    Some(-125.77912),
                    Some(-167.96875),
                    Some(-263.96875),
                    Some(-55.96875),
                    Some(-169.43985),
                    Some(-167.96875),
                    Some(-128.24706),
                    Some(-167.96875),
                    Some(-166.80313),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-133.09007),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-39.96875),
                    Some(-39.96875),
                    Some(-164.61621),
                    Some(-262.63724),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-267.90018),
                    Some(-267.80722),
                    Some(-276.14798),
                    Some(-167.96875),
                    Some(-266.18854),
                    Some(-166.95316),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_iMatchStats_PlayersAlive_T() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_T",
            PropColumn {
                data: Some(I32(vec![
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_flGameStartTime() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_flGameStartTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                    Some(70.765625),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_bPawnIsAlive() {
        let prop = (
            "CCSPlayerController.m_bPawnIsAlive",
            PropColumn {
                data: Some(Bool(vec![
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_nTickBase() {
        let prop = ("CCSPlayerController.m_nTickBase", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_MoveType() {
        let prop = (
            "CCSPlayerPawn.m_MoveType",
            PropColumn {
                data: Some(U64(vec![
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_flSimulationTime() {
        let prop = (
            "m_flSimulationTime",
            PropColumn {
                data: Some(F32(vec![
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(1479.4668),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_ubInterpolationFrame() {
        let prop = (
            "CCSPlayerPawn.m_ubInterpolationFrame",
            PropColumn {
                data: Some(U32(vec![
                    Some(3),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(1),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(1),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_iProgressBarDuration() {
        let prop = (
            "CCSPlayerPawn.m_iProgressBarDuration",
            PropColumn {
                data: Some(I32(vec![
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                    Some(-1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iKillReward() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKillReward",
            PropColumn { data: None, num_nones: 40 },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iObjective() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iObjective",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_flCreateTime() {
        let prop = (
            "CCSPlayerController.m_flCreateTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InGameMoneyServices_m_iAccount() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iAccount",
            PropColumn {
                data: Some(I32(vec![
                    Some(1950),
                    Some(1900),
                    Some(3650),
                    Some(50),
                    Some(2200),
                    Some(2050),
                    Some(2050),
                    Some(50),
                    Some(2650),
                    Some(3150),
                    Some(50),
                    Some(0),
                    Some(950),
                    Some(600),
                    Some(4400),
                    Some(450),
                    Some(1650),
                    Some(1400),
                    Some(200),
                    Some(100),
                    Some(5950),
                    Some(2600),
                    Some(2550),
                    Some(5900),
                    Some(1900),
                    Some(150),
                    Some(600),
                    Some(8750),
                    Some(1850),
                    Some(4500),
                    Some(6150),
                    Some(1500),
                    Some(2750),
                    Some(1600),
                    Some(3250),
                    Some(2700),
                    Some(2000),
                    Some(150),
                    Some(1550),
                    Some(3000),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InventoryServices_m_rank() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InventoryServices.m_rank",
            PropColumn {
                data: Some(U64(vec![
                    Some(0),
                    Some(996),
                    Some(1339),
                    Some(0),
                    Some(996),
                    Some(1030),
                    Some(923),
                    Some(969),
                    Some(0),
                    Some(1340),
                    Some(0),
                    Some(996),
                    Some(1339),
                    Some(0),
                    Some(996),
                    Some(1030),
                    Some(923),
                    Some(969),
                    Some(0),
                    Some(1340),
                    Some(0),
                    Some(996),
                    Some(1339),
                    Some(0),
                    Some(996),
                    Some(1030),
                    Some(923),
                    Some(969),
                    Some(0),
                    Some(1340),
                    Some(0),
                    Some(996),
                    Some(1339),
                    Some(0),
                    Some(996),
                    Some(1030),
                    Some(923),
                    Some(969),
                    Some(0),
                    Some(1340),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn pitch() {
        let prop = (
            "pitch",
            PropColumn {
                data: Some(F32(vec![
                    Some(-2.2631836),
                    Some(-3.1108398),
                    Some(3.06073),
                    Some(2.2748566),
                    Some(0.7041626),
                    Some(-8.254517),
                    Some(-2.8248596),
                    Some(0.12634277),
                    Some(0.035369873),
                    Some(14.900543),
                    Some(7.879257),
                    Some(2.2020721),
                    Some(0.66708374),
                    Some(-12.976913),
                    Some(27.015717),
                    Some(0.4576416),
                    Some(-3.1307678),
                    Some(15.228088),
                    Some(-5.262451),
                    Some(1.1556244),
                    Some(-11.231812),
                    Some(0.9702301),
                    Some(8.397675),
                    Some(0.72372437),
                    Some(4.839813),
                    Some(5.5968475),
                    Some(-14.218018),
                    Some(-5.181427),
                    Some(0.9328003),
                    Some(4.334442),
                    Some(-58.338776),
                    Some(-1.9713593),
                    Some(3.4926147),
                    Some(-5.066757),
                    Some(7.515335),
                    Some(-9.539215),
                    Some(-0.89434814),
                    Some(6.603119),
                    Some(4.4350433),
                    Some(10.610733),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&PITCH_ID], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_vDecalForwardAxis() {
        let prop = (
            "CCSPlayerPawn.m_vDecalForwardAxis",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.9962316, 0.04332799, 0.07513555]),
                    Some([-0.36965293, 0.8204877, 0.43606976]),
                    Some([0.3519783, 0.9337104, 0.06554684]),
                    Some([-0.7955056, 0.59699875, -0.103746854]),
                    Some([-0.98760796, 0.06803899, 0.14142609]),
                    Some([-0.110260986, -0.98646504, 0.12136375]),
                    Some([-0.17406072, 0.9846432, -0.013441297]),
                    Some([0.20821588, 0.97713315, 0.04309331]),
                    Some([0.71194553, -0.70223355, -0.0012826255]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.47314593, 0.46949002, 0.745461]),
                    Some([-0.055346057, -0.99708104, -0.052593328]),
                    Some([0.28832167, -0.8944757, 0.34173664]),
                    Some([0.403741, -0.78814685, -0.46456194]),
                    Some([-0.4105553, 0.7865878, 0.46122]),
                    Some([-0.66103494, -0.74798745, 0.059562158]),
                    Some([-0.17406072, 0.9846432, -0.013441297]),
                    Some([-0.25435287, -0.96658885, 0.031789355]),
                    Some([-0.8547955, -0.5187766, -0.013978702]),
                    Some([-0.4219073, 0.90456927, 0.061226096]),
                    Some([0.1608407, 0.986972, -0.0040686773]),
                    Some([-0.77879447, 0.6027072, -0.1738484]),
                    Some([0.16890377, 0.985381, -0.022266503]),
                    Some([-0.62725496, 0.7759586, -0.06662918]),
                    Some([-0.25199124, 0.9140058, -0.3179526]),
                    Some([0.6415635, -0.76615864, 0.037380785]),
                    Some([-0.31095564, -0.94612, 0.09035235]),
                    Some([0.8955867, 0.4444743, -0.01915702]),
                    Some([0.92318046, 0.08607937, 0.37460384]),
                    Some([-0.9850848, -0.17148674, -0.014149931]),
                    Some([0.64448905, 0.7635272, 0.040744133]),
                    Some([0.04956518, 0.47145265, -0.8804975]),
                    Some([0.06833177, -0.9974961, -0.01822531]),
                    Some([-0.9389287, -0.041765645, -0.34156787]),
                    Some([-0.48087683, 0.8767267, -0.010383588]),
                    Some([0.98124886, 0.0036932125, -0.19271004]),
                    Some([0.97416055, -0.16350822, -0.15580845]),
                    Some([0.08398245, -0.9959502, 0.032095056]),
                    Some([0.28198183, -0.9593014, -0.015066432]),
                    Some([-0.98698556, 0.15377362, -0.047044586]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_flOffsetTickStashedSpeed() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_flOffsetTickStashedSpeed",
            PropColumn {
                data: Some(F32(vec![
                    Some(248.19319),
                    Some(123.80049),
                    Some(0.0),
                    Some(236.50314),
                    Some(75.04947),
                    Some(118.488304),
                    Some(69.17062),
                    Some(33.747005),
                    Some(43.19094),
                    Some(240.35031),
                    Some(111.799995),
                    Some(223.33702),
                    Some(0.0),
                    Some(0.0),
                    Some(100.603096),
                    Some(71.43422),
                    Some(0.0),
                    Some(0.0),
                    Some(148.41351),
                    Some(101.91207),
                    Some(214.99998),
                    Some(161.77322),
                    Some(24.236797),
                    Some(0.0),
                    Some(225.0),
                    Some(0.0),
                    Some(119.39063),
                    Some(69.76295),
                    Some(45.415276),
                    Some(25.288815),
                    Some(245.0),
                    Some(104.00001),
                    Some(12.693243),
                    Some(188.73602),
                    Some(186.24919),
                    Some(111.92654),
                    Some(216.28789),
                    Some(42.646595),
                    Some(29.218338),
                    Some(118.57139),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iKills() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKills",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_fStashGrenadeParameterWhen() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_fStashGrenadeParameterWhen",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn is_alive() {
        let prop = (
            "is_alive",
            PropColumn {
                data: Some(Bool(vec![
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&IS_ALIVE_ID], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_bTeamIntroPeriod() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_bTeamIntroPeriod",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flEmitSoundTime() {
        let prop = (
            "CCSPlayerPawn.m_flEmitSoundTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(223.6875),
                    Some(216.92188),
                    Some(218.46875),
                    Some(220.53125),
                    Some(216.92188),
                    Some(216.92188),
                    Some(216.92188),
                    Some(221.5625),
                    Some(216.92188),
                    Some(225.84375),
                    Some(344.42188),
                    Some(381.73438),
                    Some(361.5),
                    Some(379.67188),
                    Some(373.32813),
                    Some(372.20313),
                    Some(370.98438),
                    Some(360.25),
                    Some(379.04688),
                    Some(381.375),
                    Some(538.40625),
                    Some(538.3906),
                    Some(535.7969),
                    Some(535.3281),
                    Some(538.3594),
                    Some(529.46875),
                    Some(536.5),
                    Some(532.7969),
                    Some(538.2656),
                    Some(538.2656),
                    Some(685.375),
                    Some(693.5156),
                    Some(686.1094),
                    Some(685.7969),
                    Some(688.3281),
                    Some(683.3125),
                    Some(683.3125),
                    Some(687.28125),
                    Some(683.3125),
                    Some(688.6094),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_flJumpUntil() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpUntil",
            PropColumn {
                data: Some(F32(vec![
                    Some(216.04475),
                    Some(135.80637),
                    Some(129.18237),
                    Some(131.097),
                    Some(133.1981),
                    Some(115.438866),
                    Some(144.381),
                    Some(135.77995),
                    Some(202.29239),
                    Some(216.84464),
                    Some(343.86166),
                    Some(381.74268),
                    Some(339.4241),
                    Some(379.6832),
                    Some(365.14783),
                    Some(371.47992),
                    Some(340.85938),
                    Some(360.2568),
                    Some(347.99414),
                    Some(337.08),
                    Some(482.85815),
                    Some(537.3743),
                    Some(511.2774),
                    Some(522.0915),
                    Some(538.371),
                    Some(511.39636),
                    Some(530.2532),
                    Some(531.3966),
                    Some(535.84375),
                    Some(511.25818),
                    Some(683.0324),
                    Some(606.3953),
                    Some(628.438),
                    Some(673.52167),
                    Some(591.38965),
                    Some(652.89545),
                    Some(643.85236),
                    Some(641.73694),
                    Some(597.8334),
                    Some(672.13086),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_iShotsFired() {
        let prop = (
            "CCSPlayerPawn.m_iShotsFired",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(9),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(3),
                    Some(0),
                    Some(11),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_fFlags() {
        let prop = (
            "CCSPlayerPawn.m_fFlags",
            PropColumn {
                data: Some(U32(vec![
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65664),
                    Some(65665),
                    Some(65665),
                    Some(536936577),
                    Some(65665),
                    Some(65667),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(536936577),
                    Some(65665),
                    Some(65664),
                    Some(536936577),
                    Some(65665),
                    Some(536936577),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                    Some(65665),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iDamage() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDamage",
            PropColumn {
                data: Some(I32(vec![
                    Some(100),
                    Some(28),
                    Some(185),
                    Some(0),
                    Some(194),
                    Some(0),
                    Some(33),
                    Some(100),
                    Some(107),
                    Some(115),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(561),
                    Some(171),
                    Some(185),
                    Some(390),
                    Some(513),
                    Some(183),
                    Some(280),
                    Some(439),
                    Some(207),
                    Some(215),
                    Some(850),
                    Some(212),
                    Some(277),
                    Some(615),
                    Some(673),
                    Some(333),
                    Some(578),
                    Some(766),
                    Some(307),
                    Some(215),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }

    #[test]
    fn CCSPlayerPawn_CCSPlayer_ItemServices_m_bHasHelmet() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_ItemServices.m_bHasHelmet",
            PropColumn {
                data: Some(Bool(vec![
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_vDecalPosition() {
        let prop = (
            "CCSPlayerPawn.m_vDecalPosition",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([-1271.9409, -996.25165, -121.87482]),
                    Some([-1177.3091, -393.89706, -11.370972]),
                    Some([-504.80264, -1454.2224, -24.087448]),
                    Some([-912.3319, -1481.2107, -105.44267]),
                    Some([-1161.0746, -606.4787, -109.58214]),
                    Some([-828.7623, -2534.7646, 27.780762]),
                    Some([-1329.447, -975.2356, -124.65142]),
                    Some([-1105.1072, -604.2416, -92.25015]),
                    Some([-803.9081, 10.710815, -107.94034]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-1281.7133, 771.99084, -78.4]),
                    Some([-2302.8567, 320.36478, -112.783165]),
                    Some([-314.79352, -922.4806, -103.29794]),
                    Some([-992.5512, -770.82446, -215.84972]),
                    Some([-1182.3367, -394.19177, 3.4436035]),
                    Some([-859.2573, -2460.5, -29.6559]),
                    Some([-1329.447, -975.2356, -124.65142]),
                    Some([369.29617, -1527.9995, -136.44029]),
                    Some([-797.92896, -895.142, -123.14453]),
                    Some([-2169.6887, 690.8084, -70.36412]),
                    Some([-576.4873, -368.66467, -108.3066]),
                    Some([-2291.2625, 373.48062, -122.94483]),
                    Some([-693.6147, -1593.5266, -125.98947]),
                    Some([-730.95447, -1178.6669, -116.54382]),
                    Some([-723.25714, -22.978605, -74.05778]),
                    Some([-627.2682, -1325.6891, -110.70767]),
                    Some([-2332.9026, -290.97577, -117.977135]),
                    Some([-157.4043, -1945.0631, -124.29493]),
                    Some([147.05904, -1916.9562, 2.3034515]),
                    Some([63.547302, -2375.304, 18.088778]),
                    Some([-616.40906, -1986.2017, -178.4]),
                    Some([-1308.5262, -1171.368, -50.149086]),
                    Some([-630.76263, -1662.4001, -144.75473]),
                    Some([-999.77985, 391.99142, -305.51318]),
                    Some([-764.8695, -110.90314, -113.249084]),
                    Some([694.5594, -1563.0651, -208.42299]),
                    Some([-582.8451, -781.82654, -199.15674]),
                    Some([-614.638, -1577.8723, -81.12184]),
                    Some([-778.0574, -1577.9769, -124.16199]),
                    Some([-616.40906, -1986.2017, -178.4]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn Z() {
        let prop = (
            "Z",
            PropColumn {
                data: Some(F32(vec![
                    Some(-167.96875),
                    Some(-287.81226),
                    Some(-167.96875),
                    Some(-163.96875),
                    Some(-264.90796),
                    Some(-286.98108),
                    Some(-267.20642),
                    Some(-167.96875),
                    Some(-265.5996),
                    Some(-163.96875),
                    Some(-47.96875),
                    Some(-125.77911),
                    Some(-167.96875),
                    Some(-263.96875),
                    Some(-55.96875),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-122.07074),
                    Some(-167.96875),
                    Some(-165.46875),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-133.09009),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-39.96875),
                    Some(-39.96875),
                    Some(-163.96875),
                    Some(-261.0),
                    Some(-167.96875),
                    Some(-167.96875),
                    Some(-267.4583),
                    Some(-267.20642),
                    Some(-273.9879),
                    Some(-167.96875),
                    Some(-263.96875),
                    Some(-163.96875),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&PLAYER_Z_ID], prop.1);
    }
    #[test]
    fn m_fLastShotTime() {
        let prop = (
            "m_fLastShotTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(194.07813),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(373.84375),
                    None,
                    Some(0.0),
                    Some(372.34375),
                    Some(0.0),
                    Some(375.79688),
                    Some(380.42188),
                    Some(535.90625),
                    Some(0.0),
                    None,
                    Some(535.96875),
                    Some(537.1875),
                    None,
                    Some(0.0),
                    None,
                    Some(538.4375),
                    Some(530.0),
                    Some(0.0),
                    Some(679.9219),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(623.8281),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iCompetitiveRanking() {
        let prop = (
            "CCSPlayerController.m_iCompetitiveRanking",
            PropColumn {
                data: Some(I32(vec![
                    Some(18244),
                    Some(18153),
                    Some(9999),
                    Some(14999),
                    Some(18891),
                    Some(16250),
                    Some(19084),
                    Some(13756),
                    Some(17709),
                    Some(0),
                    Some(18244),
                    Some(18153),
                    Some(9999),
                    Some(14999),
                    Some(18891),
                    Some(16250),
                    Some(19084),
                    Some(13756),
                    Some(17709),
                    Some(0),
                    Some(18244),
                    Some(18153),
                    Some(9999),
                    Some(14999),
                    Some(18891),
                    Some(16250),
                    Some(19084),
                    Some(13756),
                    Some(17709),
                    Some(0),
                    Some(18244),
                    Some(18153),
                    Some(9999),
                    Some(14999),
                    Some(18891),
                    Some(16250),
                    Some(19084),
                    Some(13756),
                    Some(17709),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iPawnLifetimeStart() {
        let prop = (
            "CCSPlayerController.m_iPawnLifetimeStart",
            PropColumn {
                data: Some(I32(vec![
                    Some(216),
                    Some(216),
                    Some(216),
                    Some(216),
                    Some(216),
                    Some(216),
                    Some(216),
                    Some(216),
                    Some(216),
                    Some(216),
                    Some(314),
                    Some(314),
                    Some(314),
                    Some(314),
                    Some(0),
                    Some(314),
                    Some(314),
                    Some(314),
                    Some(314),
                    Some(314),
                    Some(484),
                    Some(484),
                    Some(0),
                    Some(484),
                    Some(484),
                    Some(0),
                    Some(484),
                    Some(0),
                    Some(484),
                    Some(484),
                    Some(683),
                    Some(683),
                    Some(683),
                    Some(683),
                    Some(683),
                    Some(683),
                    Some(683),
                    Some(683),
                    Some(683),
                    Some(683),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flDeathTime() {
        let prop = (
            "CCSPlayerPawn.m_flDeathTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(139.95313),
                    Some(179.32813),
                    Some(145.9375),
                    Some(140.34375),
                    Some(117.96875),
                    Some(181.21875),
                    Some(136.70313),
                    Some(209.92188),
                    Some(0.0),
                    Some(296.04688),
                    Some(262.0625),
                    Some(258.57813),
                    Some(264.76563),
                    Some(373.32813),
                    Some(265.20313),
                    Some(181.21875),
                    Some(295.59375),
                    Some(270.32813),
                    Some(259.9375),
                    Some(296.04688),
                    Some(450.6875),
                    Some(535.7969),
                    Some(264.76563),
                    Some(456.15625),
                    Some(529.46875),
                    Some(457.79688),
                    Some(532.7969),
                    Some(470.1875),
                    Some(259.9375),
                    Some(557.2031),
                    Some(607.71875),
                    Some(640.40625),
                    Some(679.4219),
                    Some(609.2031),
                    Some(529.46875),
                    Some(645.8594),
                    Some(642.1406),
                    Some(619.6875),
                    Some(539.15625),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nNextThinkTick() {
        let prop = (
            "m_nNextThinkTick",
            PropColumn {
                data: Some(U32(vec![
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    None,
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    None,
                    Some(1),
                    Some(1),
                    None,
                    Some(1),
                    None,
                    None,
                    None,
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(88948),
                    Some(1),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_flRestartRoundTime() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_flRestartRoundTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bClientSideRagdoll() {
        let prop = (
            "CCSPlayerPawn.m_bClientSideRagdoll",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flCreateTime() {
        let prop = (
            "CCSPlayerPawn.m_flCreateTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(4.796875),
                    Some(5.109375),
                    Some(5.578125),
                    Some(6.40625),
                    Some(3.515625),
                    Some(11.640625),
                    Some(13.890625),
                    Some(10.984375),
                    Some(46.359375),
                    Some(54.765625),
                    Some(4.796875),
                    Some(5.109375),
                    Some(5.578125),
                    Some(6.40625),
                    Some(3.515625),
                    Some(11.640625),
                    Some(13.890625),
                    Some(10.984375),
                    Some(46.359375),
                    Some(54.765625),
                    Some(4.796875),
                    Some(5.109375),
                    Some(5.578125),
                    Some(6.40625),
                    Some(3.515625),
                    Some(11.640625),
                    Some(13.890625),
                    Some(10.984375),
                    Some(46.359375),
                    Some(54.765625),
                    Some(4.796875),
                    Some(5.109375),
                    Some(5.578125),
                    Some(6.40625),
                    Some(3.515625),
                    Some(11.640625),
                    Some(13.890625),
                    Some(10.984375),
                    Some(46.359375),
                    Some(54.765625),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iEquipmentValue() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEquipmentValue",
            PropColumn { data: None, num_nones: 40 },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_bPawnHasDefuser() {
        let prop = ("CCSPlayerController.m_bPawnHasDefuser", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_szCrosshairCodes() {
        let prop = (
            "CCSPlayerController.m_szCrosshairCodes",
            PropColumn {
                data: Some(String(vec![
                    Some("CSGO-aAhcL-YbVFH-SpAvG-dM3PV-9mUCP".to_string()),
                    Some("CSGO-oVQVm-VnzOz-RNsAf-FaZTC-z5VeL".to_string()),
                    Some("CSGO-VisQq-mTARE-sN3Ei-mo3TC-VfdBN".to_string()),
                    Some("CSGO-VisQq-mTARE-sN3Ei-mo3TC-VfdBN".to_string()),
                    Some("CSGO-GjYG4-fYXAt-8U47O-Q4UUK-ATnMC".to_string()),
                    Some("CSGO-poWfA-tp4rq-Vi7M5-6B8BY-RMQNN".to_string()),
                    Some("CSGO-YSQdL-YGttX-OzwmR-huLBx-VY2UO".to_string()),
                    Some("CSGO-MhQ4q-zxGhe-axFhr-xf8PV-ssmHB".to_string()),
                    Some("CSGO-CVyXT-B96An-hy9CE-ubFy4-5a4NF".to_string()),
                    Some("CSGO-EOoDJ-6Ouo9-JqKiO-kfcGW-FnHrL".to_string()),
                    Some("CSGO-aAhcL-YbVFH-SpAvG-dM3PV-9mUCP".to_string()),
                    Some("CSGO-oVQVm-VnzOz-RNsAf-FaZTC-z5VeL".to_string()),
                    Some("CSGO-VisQq-mTARE-sN3Ei-mo3TC-VfdBN".to_string()),
                    Some("CSGO-VisQq-mTARE-sN3Ei-mo3TC-VfdBN".to_string()),
                    Some("CSGO-GjYG4-fYXAt-8U47O-Q4UUK-ATnMC".to_string()),
                    Some("CSGO-poWfA-tp4rq-Vi7M5-6B8BY-RMQNN".to_string()),
                    Some("CSGO-YSQdL-YGttX-OzwmR-huLBx-VY2UO".to_string()),
                    Some("CSGO-MhQ4q-zxGhe-axFhr-xf8PV-ssmHB".to_string()),
                    Some("CSGO-CVyXT-B96An-hy9CE-ubFy4-5a4NF".to_string()),
                    Some("CSGO-EOoDJ-6Ouo9-JqKiO-kfcGW-FnHrL".to_string()),
                    Some("CSGO-aAhcL-YbVFH-SpAvG-dM3PV-9mUCP".to_string()),
                    Some("CSGO-oVQVm-VnzOz-RNsAf-FaZTC-z5VeL".to_string()),
                    Some("CSGO-VisQq-mTARE-sN3Ei-mo3TC-VfdBN".to_string()),
                    Some("CSGO-VisQq-mTARE-sN3Ei-mo3TC-VfdBN".to_string()),
                    Some("CSGO-GjYG4-fYXAt-8U47O-Q4UUK-ATnMC".to_string()),
                    Some("CSGO-poWfA-tp4rq-Vi7M5-6B8BY-RMQNN".to_string()),
                    Some("CSGO-YSQdL-YGttX-OzwmR-huLBx-VY2UO".to_string()),
                    Some("CSGO-MhQ4q-zxGhe-axFhr-xf8PV-ssmHB".to_string()),
                    Some("CSGO-CVyXT-B96An-hy9CE-ubFy4-5a4NF".to_string()),
                    Some("CSGO-EOoDJ-6Ouo9-JqKiO-kfcGW-FnHrL".to_string()),
                    Some("CSGO-aAhcL-YbVFH-SpAvG-dM3PV-9mUCP".to_string()),
                    Some("CSGO-oVQVm-VnzOz-RNsAf-FaZTC-z5VeL".to_string()),
                    Some("CSGO-VisQq-mTARE-sN3Ei-mo3TC-VfdBN".to_string()),
                    Some("CSGO-VisQq-mTARE-sN3Ei-mo3TC-VfdBN".to_string()),
                    Some("CSGO-GjYG4-fYXAt-8U47O-Q4UUK-ATnMC".to_string()),
                    Some("CSGO-poWfA-tp4rq-Vi7M5-6B8BY-RMQNN".to_string()),
                    Some("CSGO-YSQdL-YGttX-OzwmR-huLBx-VY2UO".to_string()),
                    Some("CSGO-MhQ4q-zxGhe-axFhr-xf8PV-ssmHB".to_string()),
                    Some("CSGO-CVyXT-B96An-hy9CE-ubFy4-5a4NF".to_string()),
                    Some("CSGO-EOoDJ-6Ouo9-JqKiO-kfcGW-FnHrL".to_string()),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn yaw() {
        let prop = (
            "yaw",
            PropColumn {
                data: Some(F32(vec![
                    Some(-120.846176),
                    Some(-4.5582886),
                    Some(118.377686),
                    Some(-88.57178),
                    Some(107.24957),
                    Some(17.670471),
                    Some(92.258026),
                    Some(-116.438255),
                    Some(-38.2623),
                    Some(-26.279297),
                    Some(-34.673187),
                    Some(-67.40662),
                    Some(123.82553),
                    Some(-121.45386),
                    Some(-64.44443),
                    Some(35.886505),
                    Some(82.49875),
                    Some(95.833405),
                    Some(-45.82364),
                    Some(-145.5589),
                    Some(-62.706528),
                    Some(-42.504395),
                    Some(-106.25805),
                    Some(-33.746216),
                    Some(-44.32846),
                    Some(143.18549),
                    Some(80.36258),
                    Some(-83.19191),
                    Some(-170.05429),
                    Some(7.7237244),
                    Some(-170.79552),
                    Some(88.6806),
                    Some(-105.864944),
                    Some(-56.005898),
                    Some(136.82407),
                    Some(-88.32802),
                    Some(5.202362),
                    Some(165.99347),
                    Some(-38.887817),
                    Some(81.79184),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&YAW_ID], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bIsScoped() {
        let prop = (
            "CCSPlayerPawn.m_bIsScoped",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_hPawn() {
        let prop = (
            "CCSPlayerController.m_hPawn",
            PropColumn {
                data: Some(U32(vec![
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15548543),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(13779057),
                    Some(1146999),
                    Some(15466622),
                    Some(15614085),
                    Some(9863309),
                    Some(7274587),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nEntityId() {
        let prop = (
            "CCSPlayerPawn.m_nEntityId",
            PropColumn {
                data: Some(U32(vec![
                    Some(174817384),
                    Some(3920199789),
                    Some(4014669938),
                    Some(237174903),
                    Some(702021758),
                    Some(4025155718),
                    Some(3006070925),
                    Some(3048603807),
                    Some(3535995082),
                    Some(576422114),
                    Some(174817384),
                    Some(3920199789),
                    Some(4014669938),
                    Some(237174903),
                    Some(702021758),
                    Some(4025155718),
                    Some(3006070925),
                    Some(3048603807),
                    Some(3535995082),
                    Some(576422114),
                    Some(174817384),
                    Some(3920199789),
                    Some(4014669938),
                    Some(237174903),
                    Some(702021758),
                    Some(4025155718),
                    Some(3006070925),
                    Some(3048603807),
                    Some(3535995082),
                    Some(576422114),
                    Some(174817384),
                    Some(3920199789),
                    Some(4014669938),
                    Some(237174903),
                    Some(702021758),
                    Some(4025155718),
                    Some(3006070925),
                    Some(3048603807),
                    Some(3535995082),
                    Some(576422114),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_lifeState() {
        let prop = (
            "CCSPlayerPawn.m_lifeState",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flSlopeDropOffset() {
        let prop = (
            "CCSPlayerPawn.m_flSlopeDropOffset",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(3.3520203),
                    Some(0.0),
                    Some(2.9844055),
                    Some(0.53964233),
                    Some(3.3246765),
                    Some(0.60079956),
                    Some(0.0),
                    Some(1.8640747),
                    Some(0.64746094),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(1.4710999),
                    Some(0.0),
                    Some(6.1763077),
                    Some(0.0),
                    Some(1.3343811),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.64746094),
                    Some(1.6372375),
                    Some(0.0),
                    Some(0.0),
                    Some(0.441864),
                    Some(0.60079956),
                    Some(2.1600342),
                    Some(0.0),
                    Some(2.2197876),
                    Some(2.9844055),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_iItemIDLow() {
        let prop = (
            "m_iItemIDLow",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(914452028),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(854287206),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(3479784940),
                    Some(845495747),
                    Some(0),
                    Some(0),
                    None,
                    Some(929347954),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1683145287),
                    Some(3479784940),
                    Some(923585316),
                    None,
                    Some(0),
                    Some(1944159844),
                    None,
                    Some(0),
                    None,
                    Some(199113976),
                    Some(1683145287),
                    Some(0),
                    Some(1944159844),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(903512841),
                    Some(854287206),
                    None,
                    Some(0),
                    Some(2526560054),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn inventory() {
        let prop = (
            "inventory",
            PropColumn {
                data: Some(StringVec(vec![
                    vec![
                        "knife_t".to_string(),
                        "Desert Eagle".to_string(),
                        "Smoke Grenade".to_string(),
                        "Flashbang".to_string(),
                    ],
                    vec!["knife_survival_bowie".to_string(), "USP-S".to_string()],
                    vec!["knife_t".to_string(), "Desert Eagle".to_string()],
                    vec![
                        "knife_t".to_string(),
                        "Glock-18".to_string(),
                        "AK-47".to_string(),
                        "High Explosive Grenade".to_string(),
                    ],
                    vec!["knife".to_string(), "USP-S".to_string()],
                    vec!["knife".to_string(), "P2000".to_string()],
                    vec!["knife_m9_bayonet".to_string(), "USP-S".to_string()],
                    vec![
                        "knife_t".to_string(),
                        "Glock-18".to_string(),
                        "C4".to_string(),
                        "AK-47".to_string(),
                        "Smoke Grenade".to_string(),
                    ],
                    vec!["knife".to_string(), "USP-S".to_string()],
                    vec!["knife_t".to_string()],
                    vec!["knife_t".to_string(), "Glock-18".to_string(), "AK-47".to_string(), "Flashbang".to_string()],
                    vec!["knife_survival_bowie".to_string(), "USP-S".to_string(), "M4A4".to_string()],
                    vec!["knife_t".to_string(), "Glock-18".to_string(), "AK-47".to_string()],
                    vec!["knife_t".to_string(), "Glock-18".to_string(), "MAC-10".to_string()],
                    vec![],
                    vec!["knife".to_string(), "P2000".to_string(), "M4A4".to_string()],
                    vec![
                        "knife_m9_bayonet".to_string(),
                        "USP-S".to_string(),
                        "AK-47".to_string(),
                        "Smoke Grenade".to_string(),
                    ],
                    vec![
                        "knife_t".to_string(),
                        "Glock-18".to_string(),
                        "MAC-10".to_string(),
                        "Smoke Grenade".to_string(),
                        "Flashbang".to_string(),
                        "C4".to_string(),
                    ],
                    vec!["knife".to_string(), "USP-S".to_string(), "FAMAS".to_string()],
                    vec![
                        "knife_tactical".to_string(),
                        "Glock-18".to_string(),
                        "SSG 08".to_string(),
                        "Flashbang".to_string(),
                    ],
                    vec!["knife_t".to_string(), "AK-47".to_string(), "Desert Eagle".to_string()],
                    vec!["knife_survival_bowie".to_string(), "Desert Eagle".to_string()],
                    vec![],
                    vec!["knife_t".to_string(), "Glock-18".to_string(), "AK-47".to_string()],
                    vec!["knife".to_string(), "USP-S".to_string(), "M4A1-S".to_string()],
                    vec![],
                    vec![
                        "knife_m9_bayonet".to_string(),
                        "USP-S".to_string(),
                        "M4A1-S".to_string(),
                        "Flashbang".to_string(),
                    ],
                    vec![],
                    vec!["knife".to_string()],
                    vec!["knife_tactical".to_string(), "Smoke Grenade".to_string(), "Flashbang".to_string()],
                    vec![
                        "knife_t".to_string(),
                        "Glock-18".to_string(),
                        "AK-47".to_string(),
                        "Smoke Grenade".to_string(),
                        "Flashbang".to_string(),
                    ],
                    vec!["knife_survival_bowie".to_string(), "USP-S".to_string(), "M4A1-S".to_string()],
                    vec!["knife_t".to_string(), "Glock-18".to_string(), "AK-47".to_string()],
                    vec![
                        "knife_t".to_string(),
                        "Glock-18".to_string(),
                        "AK-47".to_string(),
                        "High Explosive Grenade".to_string(),
                        "Smoke Grenade".to_string(),
                        "Flashbang".to_string(),
                    ],
                    vec!["knife".to_string(), "USP-S".to_string()],
                    vec![
                        "knife".to_string(),
                        "P2000".to_string(),
                        "Smoke Grenade".to_string(),
                        "High Explosive Grenade".to_string(),
                        "Incendiary Grenade".to_string(),
                    ],
                    vec!["knife_m9_bayonet".to_string(), "USP-S".to_string()],
                    vec![
                        "knife_t".to_string(),
                        "Glock-18".to_string(),
                        "AWP".to_string(),
                        "Smoke Grenade".to_string(),
                        "High Explosive Grenade".to_string(),
                    ],
                    vec!["knife".to_string(), "USP-S".to_string()],
                    vec![
                        "knife_tactical".to_string(),
                        "AK-47".to_string(),
                        "R8 Revolver".to_string(),
                        "High Explosive Grenade".to_string(),
                        "Smoke Grenade".to_string(),
                        "Flashbang".to_string(),
                    ],
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&INVENTORY_ID], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iDeaths() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDeaths",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CEconItemAttribute_m_bSetBonus() {
        let prop = (
            "CEconItemAttribute.m_bSetBonus",
            PropColumn {
                data: Some(Bool(vec![
                    None,
                    Some(false),
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    None,
                    None,
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    None,
                    Some(false),
                    None,
                    Some(false),
                    None,
                    Some(false),
                    Some(false),
                    Some(false),
                    None,
                    None,
                    Some(false),
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    None,
                    Some(false),
                    None,
                    None,
                    None,
                    Some(false),
                    Some(false),
                    None,
                    None,
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_iRoundTime() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_iRoundTime",
            PropColumn {
                data: Some(I32(vec![
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                    Some(115),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_fEffects() {
        let prop = (
            "m_fEffects",
            PropColumn {
                data: Some(U32(vec![
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    None,
                    None,
                    None,
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_bBombDropped() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_bBombDropped",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_fRoundStartTime() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_fRoundStartTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(236.92188),
                    Some(236.92188),
                    Some(236.92188),
                    Some(236.92188),
                    Some(236.92188),
                    Some(236.92188),
                    Some(236.92188),
                    Some(236.92188),
                    Some(236.92188),
                    Some(236.92188),
                    Some(334.75),
                    Some(334.75),
                    Some(334.75),
                    Some(334.75),
                    Some(334.75),
                    Some(334.75),
                    Some(334.75),
                    Some(334.75),
                    Some(334.75),
                    Some(334.75),
                    Some(504.0),
                    Some(504.0),
                    Some(504.0),
                    Some(504.0),
                    Some(504.0),
                    Some(504.0),
                    Some(504.0),
                    Some(504.0),
                    Some(504.0),
                    Some(504.0),
                    Some(703.3125),
                    Some(703.3125),
                    Some(703.3125),
                    Some(703.3125),
                    Some(703.3125),
                    Some(703.3125),
                    Some(703.3125),
                    Some(703.3125),
                    Some(703.3125),
                    Some(703.3125),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_nButtonDownMaskPrev() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_nButtonDownMaskPrev",
            PropColumn {
                data: Some(U64(vec![
                    Some(0),
                    Some(0),
                    Some(8589934592),
                    Some(34359738384),
                    Some(8589934600),
                    Some(8589934600),
                    Some(8589934592),
                    Some(8),
                    Some(8589934592),
                    Some(0),
                    Some(66560),
                    Some(512),
                    Some(66560),
                    Some(66048),
                    Some(0),
                    Some(66560),
                    Some(12),
                    Some(8589934592),
                    Some(512),
                    Some(1024),
                    Some(512),
                    Some(524),
                    Some(2),
                    Some(1024),
                    Some(1026),
                    Some(0),
                    Some(65536),
                    Some(2),
                    Some(1025),
                    Some(65536),
                    Some(0),
                    Some(0),
                    Some(8589934592),
                    Some(0),
                    Some(8589934592),
                    Some(8),
                    Some(0),
                    Some(8),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InventoryServices_m_nPersonaDataPublicCommendsFriendly() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsFriendly",
            PropColumn {
                data: Some(I32(vec![
                    Some(46),
                    Some(13),
                    Some(44),
                    Some(169),
                    Some(27),
                    Some(19),
                    Some(23),
                    Some(13),
                    Some(48),
                    Some(43),
                    Some(46),
                    Some(13),
                    Some(44),
                    Some(169),
                    Some(27),
                    Some(19),
                    Some(23),
                    Some(13),
                    Some(48),
                    Some(43),
                    Some(46),
                    Some(13),
                    Some(44),
                    Some(169),
                    Some(27),
                    Some(19),
                    Some(23),
                    Some(13),
                    Some(48),
                    Some(43),
                    Some(46),
                    Some(13),
                    Some(44),
                    Some(169),
                    Some(27),
                    Some(19),
                    Some(23),
                    Some(13),
                    Some(48),
                    Some(43),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_flNextSecondaryAttackTickRatio() {
        let prop = (
            "m_flNextSecondaryAttackTickRatio",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.5333328),
                    Some(0.0),
                    Some(0.26667786),
                    None,
                    Some(0.5333328),
                    Some(0.8666687),
                    Some(0.0),
                    Some(0.0),
                    Some(0.56849414),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.3865242),
                    Some(0.2666626),
                    None,
                    Some(0.0),
                    None,
                    Some(0.08510375),
                    Some(0.0),
                    Some(0.0),
                    Some(0.5333328),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.93333435),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InGameMoneyServices_m_iCashSpentThisRound() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iCashSpentThisRound",
            PropColumn {
                data: Some(I32(vec![
                    Some(1700),
                    Some(0),
                    Some(700),
                    Some(4000),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(4000),
                    Some(0),
                    Some(1000),
                    Some(4100),
                    Some(5700),
                    Some(4900),
                    Some(2550),
                    Some(1900),
                    Some(5100),
                    Some(2800),
                    Some(1750),
                    Some(4000),
                    Some(2900),
                    Some(700),
                    Some(700),
                    Some(1200),
                    Some(900),
                    Some(3900),
                    Some(3900),
                    Some(4250),
                    Some(300),
                    Some(300),
                    Some(900),
                    Some(800),
                    Some(0),
                    Some(3700),
                    Some(4900),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(6750),
                    Some(0),
                    Some(1200),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InGameMoneyServices_m_iTotalCashSpent() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iTotalCashSpent",
            PropColumn {
                data: Some(I32(vec![
                    Some(2400),
                    Some(800),
                    Some(700),
                    Some(4000),
                    Some(800),
                    Some(650),
                    Some(650),
                    Some(4300),
                    Some(650),
                    Some(1800),
                    Some(6500),
                    Some(6500),
                    Some(5600),
                    Some(6550),
                    Some(2700),
                    Some(5750),
                    Some(5150),
                    Some(6050),
                    Some(7200),
                    Some(7050),
                    Some(8300),
                    Some(7200),
                    Some(10500),
                    Some(8650),
                    Some(8500),
                    Some(9650),
                    Some(10100),
                    Some(7000),
                    Some(8850),
                    Some(9450),
                    Some(13900),
                    Some(12950),
                    Some(15200),
                    Some(18450),
                    Some(12400),
                    Some(12050),
                    Some(13950),
                    Some(20700),
                    Some(14100),
                    Some(16150),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nEnablePhysics() {
        let prop = (
            "m_nEnablePhysics",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_bWarmupPeriod() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_bWarmupPeriod",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_flJumpVel() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpVel",
            PropColumn {
                data: Some(F32(vec![
                    Some(256.23972),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(295.74338),
                    Some(289.49338),
                    Some(227.28023),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(275.47427),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(252.05167),
                    Some(289.49338),
                    Some(274.70984),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                    Some(289.49338),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }

    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_bHasMatchStarted() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_bHasMatchStarted",
            PropColumn {
                data: Some(Bool(vec![
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_iMatchStats_RoundResults() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_iMatchStats_RoundResults",
            PropColumn {
                data: Some(I32(vec![
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CEconItemAttribute_m_nRefundableCurrency() {
        let prop = (
            "CEconItemAttribute.m_nRefundableCurrency",
            PropColumn {
                data: Some(I32(vec![
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    Some(0),
                    None,
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(0),
                    None,
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(0),
                    Some(0),
                    None,
                    None,
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_vecY() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_vecY",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    None,
                    None,
                    None,
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bSpotted() {
        let prop = (
            "CCSPlayerPawn.m_bSpotted",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iObjective() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iObjective",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iDeaths() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDeaths",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(4),
                    Some(4),
                    Some(2),
                    Some(3),
                    Some(5),
                    Some(3),
                    Some(3),
                    Some(4),
                    Some(1),
                    Some(2),
                    Some(6),
                    Some(5),
                    Some(4),
                    Some(4),
                    Some(5),
                    Some(5),
                    Some(4),
                    Some(6),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_usSolidFlags() {
        let prop = (
            "m_usSolidFlags",
            PropColumn {
                data: Some(U32(vec![
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    None,
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    None,
                    Some(3),
                    Some(2),
                    None,
                    Some(2),
                    None,
                    None,
                    None,
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    None,
                    Some(2),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_flNextPrimaryAttackTickRatio() {
        let prop = (
            "m_flNextPrimaryAttackTickRatio",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.5333328),
                    Some(0.0),
                    Some(0.26667786),
                    None,
                    Some(0.5333328),
                    Some(0.8666687),
                    Some(0.0),
                    Some(0.0),
                    Some(0.56849414),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.3865242),
                    Some(0.2666626),
                    None,
                    Some(0.0),
                    None,
                    Some(0.08510375),
                    Some(0.0),
                    Some(0.0),
                    Some(0.5333328),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.93333435),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InventoryServices_m_nPersonaDataPublicCommendsTeacher() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsTeacher",
            PropColumn {
                data: Some(I32(vec![
                    Some(44),
                    Some(12),
                    Some(43),
                    Some(164),
                    Some(28),
                    Some(16),
                    Some(22),
                    Some(13),
                    Some(39),
                    Some(41),
                    Some(44),
                    Some(12),
                    Some(43),
                    Some(164),
                    Some(28),
                    Some(16),
                    Some(22),
                    Some(13),
                    Some(39),
                    Some(41),
                    Some(44),
                    Some(12),
                    Some(43),
                    Some(164),
                    Some(28),
                    Some(16),
                    Some(22),
                    Some(13),
                    Some(39),
                    Some(41),
                    Some(44),
                    Some(12),
                    Some(43),
                    Some(164),
                    Some(28),
                    Some(16),
                    Some(22),
                    Some(13),
                    Some(39),
                    Some(41),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_unTotalRoundDamageDealt() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_unTotalRoundDamageDealt",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(52),
                    Some(0),
                    Some(100),
                    Some(99),
                    Some(0),
                    Some(26),
                    Some(0),
                    Some(0),
                    Some(71),
                    Some(28),
                    Some(0),
                    Some(44),
                    Some(50),
                    Some(46),
                    Some(76),
                    Some(130),
                    Some(108),
                    Some(23),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_unPlayerTvControlFlags() {
        let prop = ("CCSPlayerController.m_unPlayerTvControlFlags", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_vDecalForwardAxis() {
        let prop = (
            "m_vDecalForwardAxis",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    None,
                    None,
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    None,
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_BulletServices_m_totalHitsOnServer() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_BulletServices.m_totalHitsOnServer",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(5),
                    Some(4),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(4),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(6),
                    Some(5),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_szLastPlaceName() {
        let prop = (
            "CCSPlayerPawn.m_szLastPlaceName",
            PropColumn {
                data: Some(String(vec![
                    Some("TSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("TSpawn".to_string()),
                    Some("TSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("TSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("TSpawn".to_string()),
                    Some("Apartments".to_string()),
                    Some("Catwalk".to_string()),
                    Some("Middle".to_string()),
                    Some("Underpass".to_string()),
                    Some("Ladder".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("PalaceInterior".to_string()),
                    Some("Jungle".to_string()),
                    Some("TopofMid".to_string()),
                    Some("BombsiteA".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("BombsiteA".to_string()),
                    Some("Jungle".to_string()),
                    Some("Catwalk".to_string()),
                    Some("BombsiteA".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("BombsiteA".to_string()),
                    Some("PalaceInterior".to_string()),
                    Some("PalaceInterior".to_string()),
                    Some("TSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("TSpawn".to_string()),
                    Some("TSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("TSpawn".to_string()),
                    Some("CTSpawn".to_string()),
                    Some("TSpawn".to_string()),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nInteractsAs() {
        let prop = (
            "m_nInteractsAs",
            PropColumn {
                data: Some(U64(vec![
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    None,
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    None,
                    Some(131265),
                    Some(131265),
                    None,
                    Some(131265),
                    None,
                    None,
                    None,
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    Some(131265),
                    None,
                    Some(131265),
                    Some(131265),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_bReloadVisuallyComplete() {
        let prop = (
            "m_bReloadVisuallyComplete",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    None,
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    None,
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    None,
                    None,
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_flFireSequenceStartTime() {
        let prop = (
            "m_flFireSequenceStartTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.040438853),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.040459678),
                    None,
                    Some(0.0),
                    Some(0.047859564),
                    Some(0.0),
                    Some(0.041319698),
                    Some(0.023653211),
                    Some(0.049818836),
                    Some(0.0),
                    None,
                    Some(0.041941315),
                    Some(0.08304096),
                    None,
                    Some(0.0),
                    None,
                    Some(0.048115853),
                    Some(0.02575165),
                    Some(0.0),
                    Some(0.09012867),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.033517454),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nHierarchyId() {
        let prop = (
            "m_nHierarchyId",
            PropColumn {
                data: Some(U32(vec![
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    None,
                    Some(134),
                    Some(141),
                    Some(159),
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    None,
                    Some(119),
                    Some(126),
                    None,
                    Some(141),
                    None,
                    Some(202),
                    Some(226),
                    Some(104),
                    Some(109),
                    Some(114),
                    Some(119),
                    Some(126),
                    Some(134),
                    Some(141),
                    None,
                    Some(202),
                    Some(226),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_steamID() {
        let prop = (
            "CCSPlayerController.m_steamID",
            PropColumn {
                data: Some(U64(vec![
                    Some(76561198265366770),
                    Some(76561198324843075),
                    Some(76561198118803912),
                    Some(76561198244754626),
                    Some(76561198194694750),
                    Some(76561198073049527),
                    Some(76561198258044111),
                    Some(76561197964020430),
                    Some(76561198280975787),
                    Some(76561198202353993),
                    Some(76561198265366770),
                    Some(76561198324843075),
                    Some(76561198118803912),
                    Some(76561198244754626),
                    Some(76561198194694750),
                    Some(76561198073049527),
                    Some(76561198258044111),
                    Some(76561197964020430),
                    Some(76561198280975787),
                    Some(76561198202353993),
                    Some(76561198265366770),
                    Some(76561198324843075),
                    Some(76561198118803912),
                    Some(76561198244754626),
                    Some(76561198194694750),
                    Some(76561198073049527),
                    Some(76561198258044111),
                    Some(76561197964020430),
                    Some(76561198280975787),
                    Some(76561198202353993),
                    Some(76561198265366770),
                    Some(76561198324843075),
                    Some(76561198118803912),
                    Some(76561198244754626),
                    Some(76561198194694750),
                    Some(76561198073049527),
                    Some(76561198258044111),
                    Some(76561197964020430),
                    Some(76561198280975787),
                    Some(76561198202353993),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nAddDecal() {
        let prop = (
            "m_nAddDecal",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_ItemServices_m_bHasDefuser() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_ItemServices.m_bHasDefuser",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_bFreezePeriod() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_bFreezePeriod",
            PropColumn {
                data: Some(Bool(vec![
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flHitHeading() {
        let prop = (
            "CCSPlayerPawn.m_flHitHeading",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(42.779312),
                    Some(0.0),
                    Some(0.0),
                    Some(0.10296631),
                    Some(2.3213272),
                    Some(0.0),
                    Some(0.0),
                    Some(-43.442574),
                    Some(1.8940296),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(-13.185135),
                    Some(2.3559723),
                    Some(14.304581),
                    Some(15.419861),
                    Some(0.0),
                    Some(-160.90855),
                    Some(2.4393005),
                    Some(2.0423546),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flVelocityModifier() {
        let prop = (
            "CCSPlayerPawn.m_flVelocityModifier",
            PropColumn {
                data: Some(F32(vec![
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(0.97448665),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(0.47221145),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                    Some(1.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_WeaponServices_m_iAmmo() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_WeaponServices.m_iAmmo",
            PropColumn {
                data: Some(U32(vec![
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iPawnHealth() {
        let prop = ("CCSPlayerController.m_iPawnHealth", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flProgressBarStartTime() {
        let prop = (
            "CCSPlayerPawn.m_flProgressBarStartTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(297.75),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(297.75),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(297.75),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nLastConcurrentKilled() {
        let prop = (
            "CCSPlayerPawn.m_nLastConcurrentKilled",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(3),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(4),
                    Some(3),
                    Some(2),
                    Some(1),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_flRecoilIndex() {
        let prop = (
            "m_flRecoilIndex",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    None,
                    Some(1.930572),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bIsWalking() {
        let prop = (
            "CCSPlayerPawn.m_bIsWalking",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }

    #[test]
    fn CCSPlayerPawn_m_bResumeZoom() {
        let prop = (
            "CCSPlayerPawn.m_bResumeZoom",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iLiveTime() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iLiveTime",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(42),
                    Some(82),
                    Some(48),
                    Some(43),
                    Some(20),
                    Some(83),
                    Some(39),
                    Some(112),
                    Some(0),
                    Some(59),
                    Some(25),
                    Some(21),
                    Some(27),
                    Some(38),
                    Some(28),
                    Some(160),
                    Some(58),
                    Some(33),
                    Some(23),
                    Some(51),
                    Some(18),
                    Some(31),
                    Some(51),
                    Some(24),
                    Some(25),
                    Some(25),
                    Some(28),
                    Some(38),
                    Some(51),
                    Some(152),
                    Some(36),
                    Some(31),
                    Some(40),
                    Some(84),
                    Some(124),
                    Some(40),
                    Some(28),
                    Some(47),
                    Some(134),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_iItemIDHigh() {
        let prop = (
            "m_iItemIDHigh",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(8),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(8),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(7),
                    Some(8),
                    Some(0),
                    Some(0),
                    None,
                    Some(8),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(5),
                    Some(7),
                    Some(8),
                    None,
                    Some(0),
                    Some(7),
                    None,
                    Some(0),
                    None,
                    Some(8),
                    None,
                    Some(0),
                    Some(7),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(8),
                    Some(8),
                    None,
                    Some(0),
                    Some(7),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bWaitForNoAttack() {
        let prop = (
            "CCSPlayerPawn.m_bWaitForNoAttack",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bClientRagdoll() {
        let prop = (
            "CCSPlayerPawn.m_bClientRagdoll",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nCollisionFunctionMask() {
        let prop = (
            "m_nCollisionFunctionMask",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn is_airborne() {
        let prop = (
            "is_airborne",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&IS_AIRBORNE_ID], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iEnemy3Ks() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemy3Ks",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iPing() {
        let prop = (
            "CCSPlayerController.m_iPing",
            PropColumn {
                data: Some(U32(vec![
                    Some(43),
                    Some(18),
                    Some(56),
                    Some(67),
                    Some(1),
                    Some(19),
                    Some(3),
                    Some(52),
                    Some(9),
                    Some(33),
                    Some(43),
                    Some(18),
                    Some(56),
                    Some(67),
                    Some(1),
                    Some(19),
                    Some(3),
                    Some(53),
                    Some(9),
                    Some(31),
                    Some(43),
                    Some(18),
                    Some(51),
                    Some(67),
                    Some(1),
                    Some(20),
                    Some(3),
                    Some(53),
                    Some(9),
                    Some(32),
                    Some(43),
                    Some(12),
                    Some(52),
                    Some(67),
                    Some(1),
                    Some(19),
                    Some(3),
                    Some(52),
                    Some(9),
                    Some(32),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_bInReload() {
        let prop = (
            "m_bInReload",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    Some(true),
                    None,
                    Some(false),
                    None,
                    None,
                    None,
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    None,
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bKilledByHeadshot() {
        let prop = (
            "CCSPlayerPawn.m_bKilledByHeadshot",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_iPlayerState() {
        let prop = (
            "CCSPlayerPawn.m_iPlayerState",
            PropColumn {
                data: Some(U64(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(8),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(8),
                    Some(0),
                    Some(0),
                    Some(8),
                    Some(0),
                    Some(8),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_bCanControlObservedBot() {
        let prop = ("CCSPlayerController.m_bCanControlObservedBot", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn inventory_as_ids() {
        let prop = (
            "inventory_as_ids",
            PropColumn {
                data: Some(U32Vec(vec![
                    vec![59, 1, 45, 43],
                    vec![514, 61],
                    vec![59, 1],
                    vec![59, 4, 7, 44],
                    vec![42, 61],
                    vec![42, 32],
                    vec![508, 61],
                    vec![59, 4, 49, 7, 45],
                    vec![42, 61],
                    vec![59],
                    vec![59, 4, 7, 43],
                    vec![514, 61, 16],
                    vec![59, 4, 7],
                    vec![59, 4, 17],
                    vec![],
                    vec![42, 32, 16],
                    vec![508, 61, 7, 45],
                    vec![59, 4, 17, 45, 43, 49],
                    vec![42, 61, 10],
                    vec![509, 4, 40, 43],
                    vec![59, 7, 1],
                    vec![514, 1],
                    vec![],
                    vec![59, 4, 7],
                    vec![42, 61, 60],
                    vec![],
                    vec![508, 61, 60, 43],
                    vec![],
                    vec![42],
                    vec![509, 45, 43],
                    vec![59, 4, 7, 45, 43],
                    vec![514, 61, 60],
                    vec![59, 4, 7],
                    vec![59, 4, 7, 44, 45, 43],
                    vec![42, 61],
                    vec![42, 32, 45, 44, 48],
                    vec![508, 61],
                    vec![59, 4, 9, 45, 44],
                    vec![42, 61],
                    vec![509, 7, 64, 44, 45, 43],
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&INVENTORY_AS_IDS_ID], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_eRoundWinReason() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_eRoundWinReason",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_iClip2() {
        let prop = (
            "m_iClip2",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nLastKillerIndex() {
        let prop = (
            "CCSPlayerPawn.m_nLastKillerIndex",
            PropColumn {
                data: Some(U64(vec![
                    Some(1),
                    Some(452),
                    Some(404),
                    Some(404),
                    Some(228),
                    Some(318),
                    Some(208),
                    Some(252),
                    Some(452),
                    Some(1),
                    Some(252),
                    Some(238),
                    Some(404),
                    Some(252),
                    Some(238),
                    Some(318),
                    Some(208),
                    Some(282),
                    Some(318),
                    Some(218),
                    Some(252),
                    Some(318),
                    Some(282),
                    Some(252),
                    Some(208),
                    Some(238),
                    Some(238),
                    Some(282),
                    Some(208),
                    Some(218),
                    Some(252),
                    Some(238),
                    Some(282),
                    Some(268),
                    Some(318),
                    Some(238),
                    Some(208),
                    Some(282),
                    Some(318),
                    Some(404),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_aimPunchAngleVel() {
        let prop = (
            "CCSPlayerPawn.m_aimPunchAngleVel",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-68.96222, -18.955631, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-69.75149, -34.54583, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-0.07983507, -0.0102492375, 0.0]),
                    Some([-24.388832, 8.735709, 0.0]),
                    Some([-60.76184, 6.060968, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([-34.808304, 0.03878641, 0.0]),
                    Some([-44.095695, -2.2964132, 0.0]),
                    Some([-46.364265, -5.10213, 0.0]),
                    Some([-28.697758, -3.439148, 0.0]),
                    Some([-52.065315, -10.179849, 0.0]),
                    Some([-47.481705, 15.148917, 0.0]),
                    Some([-38.53029, 3.1640582, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_flOffsetTickCompleteTime() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_flOffsetTickCompleteTime",
            PropColumn {
                data: Some(F32(vec![
                    Some(214.70894),
                    Some(139.11497),
                    Some(179.13269),
                    Some(145.6925),
                    Some(139.75066),
                    Some(117.80774),
                    Some(181.02057),
                    Some(136.5264),
                    Some(209.8517),
                    Some(213.52574),
                    Some(382.15256),
                    Some(381.5084),
                    Some(381.8938),
                    Some(382.24454),
                    Some(372.66974),
                    Some(382.1154),
                    Some(381.3079),
                    Some(361.11783),
                    Some(382.22818),
                    Some(382.2228),
                    Some(538.49634),
                    Some(538.511),
                    Some(535.7737),
                    Some(538.4727),
                    Some(538.34064),
                    Some(529.3963),
                    Some(538.42053),
                    Some(532.6795),
                    Some(538.38007),
                    Some(537.43854),
                    Some(682.8653),
                    Some(607.44965),
                    Some(639.7773),
                    Some(679.336),
                    Some(609.0068),
                    Some(682.9732),
                    Some(645.4027),
                    Some(641.43146),
                    Some(619.48346),
                    Some(681.91705),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_aimPunchTickFraction() {
        let prop = (
            "CCSPlayerPawn.m_aimPunchTickFraction",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.9446808),
                    Some(0.0),
                    Some(0.6961353),
                    Some(0.0),
                    Some(0.0),
                    Some(0.56849414),
                    Some(0.6000004),
                    Some(0.0),
                    Some(0.64061844),
                    Some(0.98652405),
                    Some(0.6779499),
                    Some(0.0),
                    Some(0.47893262),
                    Some(0.0),
                    Some(0.48510334),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iConnected() {
        let prop = (
            "CCSPlayerController.m_iConnected",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_ubInterpolationFrame() {
        let prop = (
            "m_ubInterpolationFrame",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(1),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iMoneySaved() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iMoneySaved",
            PropColumn {
                data: Some(I32(vec![
                    Some(100),
                    Some(0),
                    Some(800),
                    Some(800),
                    Some(300),
                    Some(150),
                    Some(150),
                    Some(500),
                    Some(150),
                    Some(0),
                    Some(50),
                    Some(1900),
                    Some(950),
                    Some(50),
                    Some(4400),
                    Some(450),
                    Some(1650),
                    Some(1400),
                    Some(200),
                    Some(100),
                    Some(5950),
                    Some(2600),
                    Some(2550),
                    Some(5600),
                    Some(1900),
                    Some(1150),
                    Some(200),
                    Some(8750),
                    Some(1850),
                    Some(4500),
                    Some(5950),
                    Some(2600),
                    Some(2550),
                    Some(5600),
                    Some(1900),
                    Some(1150),
                    Some(200),
                    Some(8750),
                    Some(1850),
                    Some(4500),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nInteractsExclude() {
        let prop = (
            "CCSPlayerPawn.m_nInteractsExclude",
            PropColumn {
                data: Some(U64(vec![
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(262672),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(262672),
                    Some(512),
                    Some(512),
                    Some(262672),
                    Some(512),
                    Some(262672),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                    Some(512),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_nHitBodyPart() {
        let prop = (
            "CCSPlayerPawn.m_nHitBodyPart",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(5),
                    Some(2),
                    Some(1),
                    Some(2),
                    Some(0),
                    Some(3),
                    Some(3),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bInBombZone() {
        let prop = (
            "CCSPlayerPawn.m_bInBombZone",
            PropColumn {
                data: Some(Bool(vec![
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_szRagdollDamageWeaponName() {
        let prop = (
            "CCSPlayerPawn.m_szRagdollDamageWeaponName",
            PropColumn {
                data: Some(String(vec![
                    Some("".to_string()),
                    Some("glock".to_string()),
                    Some("hkp2000".to_string()),
                    Some("hkp2000".to_string()),
                    Some("glock".to_string()),
                    Some("p250".to_string()),
                    Some("deagle".to_string()),
                    Some("fiveseven".to_string()),
                    Some("glock".to_string()),
                    Some("".to_string()),
                    Some("ak47".to_string()),
                    Some("ak47".to_string()),
                    Some("mp9".to_string()),
                    Some("usp_silencer".to_string()),
                    Some("mac10".to_string()),
                    Some("ak47".to_string()),
                    Some("deagle".to_string()),
                    Some("usp_silencer".to_string()),
                    Some("ak47".to_string()),
                    Some("usp_silencer".to_string()),
                    Some("ak47".to_string()),
                    Some("mac10".to_string()),
                    Some("m4a1_silencer".to_string()),
                    Some("usp_silencer".to_string()),
                    Some("ak47".to_string()),
                    Some("ak47".to_string()),
                    Some("ak47".to_string()),
                    Some("m4a1_silencer".to_string()),
                    Some("ak47".to_string()),
                    Some("usp_silencer".to_string()),
                    Some("m4a1_silencer".to_string()),
                    Some("hegrenade".to_string()),
                    Some("m4a1_silencer".to_string()),
                    Some("m4a1_silencer".to_string()),
                    Some("awp".to_string()),
                    Some("ak47".to_string()),
                    Some("ak47".to_string()),
                    Some("m4a1_silencer".to_string()),
                    Some("awp".to_string()),
                    Some("p250".to_string()),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_InGameMoneyServices_m_iStartAccount() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iStartAccount",
            PropColumn {
                data: Some(I32(vec![
                    Some(3650),
                    Some(1900),
                    Some(4350),
                    Some(4050),
                    Some(2200),
                    Some(2050),
                    Some(2050),
                    Some(4050),
                    Some(2650),
                    Some(4150),
                    Some(4150),
                    Some(5700),
                    Some(5850),
                    Some(2550),
                    Some(6300),
                    Some(5550),
                    Some(4450),
                    Some(3150),
                    Some(4200),
                    Some(3000),
                    Some(6650),
                    Some(3300),
                    Some(3750),
                    Some(6500),
                    Some(5800),
                    Some(4050),
                    Some(4250),
                    Some(9050),
                    Some(2150),
                    Some(5400),
                    Some(6950),
                    Some(1500),
                    Some(6450),
                    Some(6500),
                    Some(3250),
                    Some(2700),
                    Some(2000),
                    Some(6900),
                    Some(1550),
                    Some(4200),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nSubclassID() {
        let prop = (
            "m_nSubclassID",
            PropColumn {
                data: Some(U32(vec![
                    Some(3901722307),
                    Some(2511498851),
                    Some(628863847),
                    Some(1058568214),
                    Some(294910436),
                    Some(294910436),
                    Some(1638561588),
                    Some(1058568214),
                    Some(294910436),
                    Some(1058568214),
                    Some(2242710103),
                    Some(2746029779),
                    Some(2242710103),
                    Some(3427122024),
                    None,
                    Some(2746029779),
                    Some(2242710103),
                    Some(3427122024),
                    Some(2973572455),
                    Some(3902792221),
                    Some(2242710103),
                    Some(628863847),
                    None,
                    Some(2242710103),
                    Some(4152478990),
                    None,
                    Some(1977124450),
                    None,
                    Some(1573262270),
                    Some(3902792221),
                    Some(3901722307),
                    Some(4152478990),
                    Some(2242710103),
                    Some(1058568214),
                    Some(294910436),
                    Some(1721431921),
                    Some(1638561588),
                    Some(3748345010),
                    Some(2343690088),
                    Some(2282479884),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_hParent() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_hParent",
            PropColumn {
                data: Some(U32(vec![
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    None,
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    None,
                    Some(1146999),
                    Some(15466622),
                    None,
                    Some(9863309),
                    None,
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    None,
                    Some(6389962),
                    Some(2998498),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSGameRulesProxy_CCSGameRules_m_gamePhase() {
        let prop = (
            "CCSGameRulesProxy.CCSGameRules.m_gamePhase",
            PropColumn {
                data: Some(I32(vec![
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn weapon_name() {
        let prop = (
            "weapon_name",
            PropColumn {
                data: Some(String(vec![
                    Some("Smoke Grenade".to_string()),
                    Some("knife_survival_bowie".to_string()),
                    Some("Desert Eagle".to_string()),
                    Some("knife_t".to_string()),
                    Some("knife".to_string()),
                    Some("knife".to_string()),
                    Some("knife_m9_bayonet".to_string()),
                    Some("knife_t".to_string()),
                    Some("knife".to_string()),
                    Some("knife_t".to_string()),
                    Some("AK-47".to_string()),
                    Some("M4A4".to_string()),
                    Some("AK-47".to_string()),
                    Some("MAC-10".to_string()),
                    None,
                    Some("M4A4".to_string()),
                    Some("AK-47".to_string()),
                    Some("MAC-10".to_string()),
                    Some("FAMAS".to_string()),
                    Some("SSG 08".to_string()),
                    Some("AK-47".to_string()),
                    Some("Desert Eagle".to_string()),
                    None,
                    Some("AK-47".to_string()),
                    Some("M4A1-S".to_string()),
                    None,
                    Some("Flashbang".to_string()),
                    None,
                    None,
                    None,
                    Some("Smoke Grenade".to_string()),
                    Some("M4A1-S".to_string()),
                    Some("AK-47".to_string()),
                    Some("knife_t".to_string()),
                    Some("knife".to_string()),
                    Some("P2000".to_string()),
                    Some("knife_m9_bayonet".to_string()),
                    None,
                    Some("USP-S".to_string()),
                    Some("knife_tactical".to_string()),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&WEAPON_NAME_ID], prop.1);
    }
    #[test]
    fn velocity_Z() {
        let prop = ("velocity_Z", PropColumn { data: None, num_nones: 40 });
        assert_eq!(out.0.df[&VELOCITY_Z_ID], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iCashEarned() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iCashEarned",
            PropColumn {
                data: Some(I32(vec![
                    Some(3650),
                    Some(1900),
                    Some(4350),
                    Some(4050),
                    Some(2200),
                    Some(2050),
                    Some(2050),
                    Some(4050),
                    Some(2650),
                    Some(4150),
                    Some(4150),
                    Some(300),
                    Some(5850),
                    Some(600),
                    Some(6300),
                    Some(5550),
                    Some(4450),
                    Some(3150),
                    Some(3050),
                    Some(3000),
                    Some(6650),
                    Some(3300),
                    Some(3750),
                    Some(5900),
                    Some(5800),
                    Some(4050),
                    Some(600),
                    Some(9050),
                    Some(2150),
                    Some(5400),
                    Some(7950),
                    Some(5850),
                    Some(3950),
                    Some(7600),
                    Some(5750),
                    Some(3400),
                    Some(3850),
                    Some(10150),
                    Some(5400),
                    Some(5900),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_CCSPlayer_MovementServices_m_flDuckAmount() {
        let prop = (
            "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckAmount",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(1.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.47264498),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn tick() {
        let prop = (
            "tick",
            PropColumn {
                data: Some(I32(vec![
                    Some(10000),
                    Some(10000),
                    Some(10000),
                    Some(10000),
                    Some(10000),
                    Some(10000),
                    Some(10000),
                    Some(10000),
                    Some(10000),
                    Some(10000),
                    Some(20000),
                    Some(20000),
                    Some(20000),
                    Some(20000),
                    Some(20000),
                    Some(20000),
                    Some(20000),
                    Some(20000),
                    Some(20000),
                    Some(20000),
                    Some(30000),
                    Some(30000),
                    Some(30000),
                    Some(30000),
                    Some(30000),
                    Some(30000),
                    Some(30000),
                    Some(30000),
                    Some(30000),
                    Some(30000),
                    Some(40000),
                    Some(40000),
                    Some(40000),
                    Some(40000),
                    Some(40000),
                    Some(40000),
                    Some(40000),
                    Some(40000),
                    Some(40000),
                    Some(40000),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&TICK_ID], prop.1);
    }
    #[test]
    fn name() {
        let prop = (
            "name",
            PropColumn {
                data: Some(String(vec![
                    Some("123".to_string()),
                    Some("Trahun <3 V".to_string()),
                    Some("Голова, глаза".to_string()),
                    Some("NIGHTSOUL".to_string()),
                    Some("Dog".to_string()),
                    Some("miu miu".to_string()),
                    Some("-ExΩtiC-".to_string()),
                    Some("Подсосник blick'a".to_string()),
                    Some("povergo".to_string()),
                    Some("IMI Negev".to_string()),
                    Some("123".to_string()),
                    Some("Trahun <3 V".to_string()),
                    Some("Голова, глаза".to_string()),
                    Some("NIGHTSOUL".to_string()),
                    Some("Dog".to_string()),
                    Some("miu miu".to_string()),
                    Some("-ExΩtiC-".to_string()),
                    Some("Подсосник blick'a".to_string()),
                    Some("povergo".to_string()),
                    Some("IMI Negev".to_string()),
                    Some("123".to_string()),
                    Some("Trahun <3 V".to_string()),
                    Some("Голова, глаза".to_string()),
                    Some("NIGHTSOUL".to_string()),
                    Some("Dog".to_string()),
                    Some("miu miu".to_string()),
                    Some("-ExΩtiC-".to_string()),
                    Some("Подсосник blick'a".to_string()),
                    Some("povergo".to_string()),
                    Some("IMI Negev".to_string()),
                    Some("123".to_string()),
                    Some("Trahun <3 V".to_string()),
                    Some("Голова, глаза".to_string()),
                    Some("NIGHTSOUL".to_string()),
                    Some("Dog".to_string()),
                    Some("miu miu".to_string()),
                    Some("-ExΩtiC-".to_string()),
                    Some("Подсосник blick'a".to_string()),
                    Some("povergo".to_string()),
                    Some("IMI Negev".to_string()),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&NAME_ID], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iNumRoundKills() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iNumRoundKills",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_nResetEventsParity() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_nResetEventsParity",
            PropColumn {
                data: Some(I32(vec![
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(2),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(4),
                    Some(3),
                    Some(0),
                    Some(3),
                    Some(0),
                    Some(3),
                    Some(6),
                    None,
                    Some(3),
                    Some(4),
                    Some(2),
                    Some(2),
                    Some(1),
                    Some(1),
                    Some(3),
                    None,
                    Some(2),
                    Some(7),
                    None,
                    Some(3),
                    None,
                    Some(7),
                    Some(3),
                    Some(7),
                    Some(5),
                    Some(0),
                    Some(3),
                    Some(3),
                    Some(4),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(5),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_qDeathEyeAngles() {
        let prop = (
            "CCSPlayerPawn.m_qDeathEyeAngles",
            PropColumn {
                data: Some(XYZVec(vec![
                    Some([0.0, 0.0, 0.0]),
                    Some([27.125, -71.03125, 0.0]),
                    Some([4.3125, -110.75, 0.0]),
                    Some([-5.71875, -37.53125, 0.0]),
                    Some([13.96875, -1.625, 0.0]),
                    Some([7.6875, 83.9375, 0.0]),
                    Some([-0.65625, -72.9375, 0.0]),
                    Some([6.78125, -108.0625, 0.0]),
                    Some([0.6875, 132.0, 0.0]),
                    Some([0.0, 0.0, 0.0]),
                    Some([3.09375, 27.0, 0.0]),
                    Some([0.625, 70.40625, 0.0]),
                    Some([18.59375, 108.71875, 0.0]),
                    Some([-6.0, 64.09375, 0.0]),
                    Some([38.53125, -64.5625, 0.0]),
                    Some([3.09375, 81.46875, 0.0]),
                    Some([-0.65625, -72.9375, 0.0]),
                    Some([-3.21875, 165.65625, 0.0]),
                    Some([2.5625, 6.1875, 0.0]),
                    Some([8.03125, 125.21875, 0.0]),
                    Some([3.09375, 27.0, 0.0]),
                    Some([0.0, 82.15625, 0.0]),
                    Some([0.96875, -98.0, 0.0]),
                    Some([-6.0, 64.09375, 0.0]),
                    Some([2.0, 80.4375, 0.0]),
                    Some([4.09375, 128.6875, 0.0]),
                    Some([22.21875, 56.15625, 0.0]),
                    Some([-14.8125, 5.5, 0.0]),
                    Some([-0.15625, 109.6875, 0.0]),
                    Some([8.03125, 125.21875, 0.0]),
                    Some([4.34375, 177.90625, 0.0]),
                    Some([0.09375, 62.34375, 0.0]),
                    Some([3.875, 5.5625, 0.0]),
                    Some([-19.78125, 3.96875, 0.0]),
                    Some([0.84375, -52.28125, 0.0]),
                    Some([4.09375, 128.6875, 0.0]),
                    Some([-1.40625, -102.625, 0.0]),
                    Some([5.75, 145.71875, 0.0]),
                    Some([1.96875, 110.34375, 0.0]),
                    Some([6.125, 9.6875, 0.0]),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iKillReward() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKillReward",
            PropColumn {
                data: Some(I32(vec![
                    Some(300),
                    Some(0),
                    Some(300),
                    Some(0),
                    Some(300),
                    Some(0),
                    Some(0),
                    Some(300),
                    Some(600),
                    Some(600),
                    Some(300),
                    Some(300),
                    Some(300),
                    Some(600),
                    Some(600),
                    Some(0),
                    Some(300),
                    Some(600),
                    Some(600),
                    Some(600),
                    Some(600),
                    Some(300),
                    Some(300),
                    Some(300),
                    Some(600),
                    Some(300),
                    Some(600),
                    Some(600),
                    Some(600),
                    Some(300),
                    Some(600),
                    Some(300),
                    Some(300),
                    Some(600),
                    Some(600),
                    Some(300),
                    Some(600),
                    Some(600),
                    Some(300),
                    Some(300),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }

    #[test]
    fn CBodyComponentBaseAnimGraph_m_vecX() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_vecX",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    None,
                    None,
                    None,
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    None,
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn Y() {
        let prop = (
            "Y",
            PropColumn {
                data: Some(F32(vec![
                    Some(-256.0),
                    Some(-1812.0),
                    Some(-304.0),
                    Some(-115.0),
                    Some(-1732.0),
                    Some(-1988.0),
                    Some(-1800.0),
                    Some(-64.0),
                    Some(-1896.0),
                    Some(-211.0),
                    Some(707.73047),
                    Some(284.07272),
                    Some(-895.6396),
                    Some(-434.33887),
                    Some(-391.00195),
                    Some(-2352.6865),
                    Some(-1103.3751),
                    Some(-1711.9717),
                    Some(-1336.3375),
                    Some(-134.1239),
                    Some(-1690.152),
                    Some(-961.5833),
                    Some(-1591.5037),
                    Some(-1459.2153),
                    Some(-471.58636),
                    Some(-1324.3574),
                    Some(-2395.391),
                    Some(-1950.639),
                    Some(-2294.539),
                    Some(-2375.4458),
                    Some(-211.0),
                    Some(-2072.0),
                    Some(32.0),
                    Some(-304.0),
                    Some(-1976.0),
                    Some(-1800.0),
                    Some(-1976.0),
                    Some(-16.0),
                    Some(-1800.0),
                    Some(-115.0),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&PLAYER_Y_ID], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_m_iEnemiesFlashed() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemiesFlashed",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(2),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(3),
                    Some(4),
                    Some(2),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(2),
                    Some(0),
                    Some(2),
                    Some(5),
                    Some(4),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CBodyComponentBaseAnimGraph_m_cellX() {
        let prop = (
            "CBodyComponentBaseAnimGraph.m_cellX",
            PropColumn {
                data: Some(U32(vec![
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    None,
                    None,
                    None,
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    Some(32),
                    None,
                    Some(32),
                    Some(32),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_unActiveQuestId() {
        let prop = ("CCSPlayerController.m_unActiveQuestId", PropColumn { data: None, num_nones: 40 });
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_CCSPlayerController_ActionTrackingServices_CSPerRoundStats_t_m_iEquipmentValue() {
        let prop = (
            "CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEquipmentValue",
            PropColumn {
                data: Some(I32(vec![
                    Some(700),
                    Some(800),
                    Some(200),
                    Some(200),
                    Some(500),
                    Some(850),
                    Some(850),
                    Some(300),
                    Some(650),
                    Some(1000),
                    Some(4300),
                    Some(5900),
                    Some(5100),
                    Some(2750),
                    Some(5600),
                    Some(5300),
                    Some(5700),
                    Some(1950),
                    Some(4200),
                    Some(3100),
                    Some(5100),
                    Some(700),
                    Some(5100),
                    Some(5100),
                    Some(4100),
                    Some(3100),
                    Some(4250),
                    Some(2600),
                    Some(300),
                    Some(4200),
                    Some(5100),
                    Some(700),
                    Some(5100),
                    Some(5100),
                    Some(4100),
                    Some(3100),
                    Some(4250),
                    Some(2600),
                    Some(300),
                    Some(4200),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_hPlayerPawn() {
        let prop = (
            "CCSPlayerController.m_hPlayerPawn",
            PropColumn {
                data: Some(U32(vec![
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                    Some(3522664),
                    Some(13942893),
                    Some(10846322),
                    Some(1146999),
                    Some(15466622),
                    Some(16089222),
                    Some(9863309),
                    Some(14352543),
                    Some(6389962),
                    Some(2998498),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_bSpottedByMask() {
        let prop = (
            "CCSPlayerPawn.m_bSpottedByMask",
            PropColumn {
                data: Some(U64Vec(vec![
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![76561198118803912, 76561198265366770],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn steamid() {
        let prop = (
            "steamid",
            PropColumn {
                data: Some(U64(vec![
                    Some(76561198265366770),
                    Some(76561198324843075),
                    Some(76561198118803912),
                    Some(76561198244754626),
                    Some(76561198194694750),
                    Some(76561198073049527),
                    Some(76561198258044111),
                    Some(76561197964020430),
                    Some(76561198280975787),
                    Some(76561198202353993),
                    Some(76561198265366770),
                    Some(76561198324843075),
                    Some(76561198118803912),
                    Some(76561198244754626),
                    Some(76561198194694750),
                    Some(76561198073049527),
                    Some(76561198258044111),
                    Some(76561197964020430),
                    Some(76561198280975787),
                    Some(76561198202353993),
                    Some(76561198265366770),
                    Some(76561198324843075),
                    Some(76561198118803912),
                    Some(76561198244754626),
                    Some(76561198194694750),
                    Some(76561198073049527),
                    Some(76561198258044111),
                    Some(76561197964020430),
                    Some(76561198280975787),
                    Some(76561198202353993),
                    Some(76561198265366770),
                    Some(76561198324843075),
                    Some(76561198118803912),
                    Some(76561198244754626),
                    Some(76561198194694750),
                    Some(76561198073049527),
                    Some(76561198258044111),
                    Some(76561197964020430),
                    Some(76561198280975787),
                    Some(76561198202353993),
                ])),
                num_nones: 0,
            },
        );
        assert_eq!(out.0.df[&STEAMID_ID], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iScore() {
        let prop = (
            "CCSPlayerController.m_iScore",
            PropColumn {
                data: Some(I32(vec![
                    Some(2),
                    Some(1),
                    Some(3),
                    Some(0),
                    Some(3),
                    Some(0),
                    Some(0),
                    Some(2),
                    Some(4),
                    Some(6),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(4),
                    Some(8),
                    Some(0),
                    Some(5),
                    Some(8),
                    Some(6),
                    Some(6),
                    Some(11),
                    Some(3),
                    Some(3),
                    Some(10),
                    Some(9),
                    Some(2),
                    Some(9),
                    Some(17),
                    Some(6),
                    Some(8),
                    Some(18),
                    Some(3),
                    Some(4),
                    Some(15),
                    Some(13),
                    Some(6),
                    Some(13),
                    Some(22),
                    Some(8),
                    Some(12),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerPawn_m_flFlashDuration() {
        let prop = (
            "CCSPlayerPawn.m_flFlashDuration",
            PropColumn {
                data: Some(F32(vec![
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                    Some(0.0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn m_nDropTick() {
        let prop = (
            "m_nDropTick",
            PropColumn {
                data: Some(U32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(37836),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(51292),
                    Some(0),
                    None,
                    Some(0),
                    None,
                    None,
                    None,
                    Some(82670),
                    Some(88756),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                    Some(0),
                    Some(0),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }
    #[test]
    fn CCSPlayerController_m_iMVPs() {
        let prop = (
            "CCSPlayerController.m_iMVPs",
            PropColumn {
                data: Some(I32(vec![
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(2),
                ])),
                num_nones: 0,
            },
        );
        let prop_id = out.1.name_to_id[prop.0];
        assert_eq!(out.0.df[&prop_id], prop.1);
    }

    #[test]
    fn game_event_hltv_versioninfo() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "hltv_versioninfo".to_string(),
            vec![GameEvent {
                name: "hltv_versioninfo".to_string(),
                fields: vec![
                    EventField {
                        name: "version".to_string(),
                        data: Some(I32(1)),
                    },
                    EventField {
                        name: "tick".to_string(),
                        data: Some(I32(1)),
                    },
                ],
                tick: 1,
            }],
        );
        assert_eq!(out.2["hltv_versioninfo"], prop.1);
    }
    #[test]
    fn game_event_round_freeze_end() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_freeze_end".to_string(),
            vec![
                GameEvent {
                    name: "round_freeze_end".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(1761)),
                    }],
                    tick: 1761,
                },
                GameEvent {
                    name: "round_freeze_end".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(10699)),
                    }],
                    tick: 10699,
                },
            ],
        );
        assert_eq!(out.2["round_freeze_end"], prop.1);
    }
    #[test]
    fn game_event_weapon_reload() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "weapon_reload".to_string(),
            vec![
                GameEvent {
                    name: "weapon_reload".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1991)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Trahun <3 V".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198324843075".to_string())),
                        },
                    ],
                    tick: 1991,
                },
                GameEvent {
                    name: "weapon_reload".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(2016)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 2016,
                },
            ],
        );
        assert_eq!(out.2["weapon_reload"], prop.1);
    }
    #[test]
    fn game_event_cs_pre_restart() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "cs_pre_restart".to_string(),
            vec![
                GameEvent {
                    name: "cs_pre_restart".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(46)),
                    }],
                    tick: 46,
                },
                GameEvent {
                    name: "cs_pre_restart".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(9400)),
                    }],
                    tick: 9400,
                },
            ],
        );
        assert_eq!(out.2["cs_pre_restart"], prop.1);
    }
    #[test]
    fn game_event_weapon_fire() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "weapon_fire".to_string(),
            vec![
                GameEvent {
                    name: "weapon_fire".to_string(),
                    fields: vec![
                        EventField {
                            name: "weapon".to_string(),
                            data: Some(String("weapon_usp_silencer".to_string())),
                        },
                        EventField {
                            name: "silenced".to_string(),
                            data: Some(Bool(true)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1977)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Trahun <3 V".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198324843075".to_string())),
                        },
                    ],
                    tick: 1977,
                },
                GameEvent {
                    name: "weapon_fire".to_string(),
                    fields: vec![
                        EventField {
                            name: "weapon".to_string(),
                            data: Some(String("weapon_hkp2000".to_string())),
                        },
                        EventField {
                            name: "silenced".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1996)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 1996,
                },
            ],
        );
        assert_eq!(out.2["weapon_fire"], prop.1);
    }
    #[test]
    fn game_event_player_death() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "player_death".to_string(),
            vec![
                GameEvent {
                    name: "player_death".to_string(),
                    fields: vec![
                        EventField {
                            name: "assistedflash".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "weapon".to_string(),
                            data: Some(String("p250".to_string())),
                        },
                        EventField {
                            name: "weapon_itemid".to_string(),
                            data: Some(String("0".to_string())),
                        },
                        EventField {
                            name: "weapon_fauxitemid".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "weapon_originalowner_xuid".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "headshot".to_string(),
                            data: Some(Bool(true)),
                        },
                        EventField {
                            name: "dominated".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "revenge".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "wipe".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "penetrated".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "noreplay".to_string(),
                            data: Some(Bool(true)),
                        },
                        EventField {
                            name: "noscope".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "thrusmoke".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "attackerblind".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "distance".to_string(),
                            data: Some(F32(50.464794)),
                        },
                        EventField {
                            name: "dmg_health".to_string(),
                            data: Some(I32(100)),
                        },
                        EventField {
                            name: "dmg_armor".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "hitgroup".to_string(),
                            data: Some(String("head".to_string())),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(3086)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("miu miu".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198073049527".to_string())),
                        },
                        EventField {
                            name: "attacker_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "attacker_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                        EventField {
                            name: "assister_steamid".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "assister_name".to_string(),
                            data: None,
                        },
                    ],
                    tick: 3086,
                },
                GameEvent {
                    name: "player_death".to_string(),
                    fields: vec![
                        EventField {
                            name: "assistedflash".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "weapon".to_string(),
                            data: Some(String("fiveseven".to_string())),
                        },
                        EventField {
                            name: "weapon_itemid".to_string(),
                            data: Some(String("0".to_string())),
                        },
                        EventField {
                            name: "weapon_fauxitemid".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "weapon_originalowner_xuid".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "headshot".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "dominated".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "revenge".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "wipe".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "penetrated".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "noreplay".to_string(),
                            data: Some(Bool(true)),
                        },
                        EventField {
                            name: "noscope".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "thrusmoke".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "attackerblind".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "distance".to_string(),
                            data: Some(F32(6.9499903)),
                        },
                        EventField {
                            name: "dmg_health".to_string(),
                            data: Some(I32(28)),
                        },
                        EventField {
                            name: "dmg_armor".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "hitgroup".to_string(),
                            data: Some(String("right_arm".to_string())),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(4285)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                        EventField {
                            name: "attacker_name".to_string(),
                            data: Some(String("Dog".to_string())),
                        },
                        EventField {
                            name: "attacker_steamid".to_string(),
                            data: Some(String("76561198194694750".to_string())),
                        },
                        EventField {
                            name: "assister_steamid".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "assister_name".to_string(),
                            data: None,
                        },
                    ],
                    tick: 4285,
                },
            ],
        );
        assert_eq!(out.2["player_death"], prop.1);
    }
    #[test]
    fn game_event_smokegrenade_expired() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "smokegrenade_expired".to_string(),
            vec![
                GameEvent {
                    name: "smokegrenade_expired".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(100)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(-1545.4756)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(259.92813)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-166.69106)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(4437)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 4437,
                },
                GameEvent {
                    name: "smokegrenade_expired".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(204)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(253.67014)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(-1489.7667)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-173.96875)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(12694)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("povergo".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198280975787".to_string())),
                        },
                    ],
                    tick: 12694,
                },
            ],
        );
        assert_eq!(out.2["smokegrenade_expired"], prop.1);
    }
    #[test]
    fn game_event_item_equip() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "item_equip".to_string(),
            vec![
                GameEvent {
                    name: "item_equip".to_string(),
                    fields: vec![
                        EventField {
                            name: "item".to_string(),
                            data: Some(String("hkp2000".to_string())),
                        },
                        EventField {
                            name: "defindex".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "canzoom".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "hassilencer".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "issilenced".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "hastracers".to_string(),
                            data: Some(Bool(true)),
                        },
                        EventField {
                            name: "weptype".to_string(),
                            data: Some(I32(1)),
                        },
                        EventField {
                            name: "ispainted".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(23)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 23,
                },
                GameEvent {
                    name: "item_equip".to_string(),
                    fields: vec![
                        EventField {
                            name: "item".to_string(),
                            data: Some(String("hkp2000".to_string())),
                        },
                        EventField {
                            name: "defindex".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "canzoom".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "hassilencer".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "issilenced".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "hastracers".to_string(),
                            data: Some(Bool(true)),
                        },
                        EventField {
                            name: "weptype".to_string(),
                            data: Some(I32(1)),
                        },
                        EventField {
                            name: "ispainted".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(23)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 23,
                },
            ],
        );
        assert_eq!(out.2["item_equip"], prop.1);
    }
    #[test]
    fn game_event_bomb_planted() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "bomb_planted".to_string(),
            vec![
                GameEvent {
                    name: "bomb_planted".to_string(),
                    fields: vec![
                        EventField {
                            name: "site".to_string(),
                            data: Some(I32(185)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(8259)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 8259,
                },
                GameEvent {
                    name: "bomb_planted".to_string(),
                    fields: vec![
                        EventField {
                            name: "site".to_string(),
                            data: Some(I32(184)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(13361)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 13361,
                },
            ],
        );
        assert_eq!(out.2["bomb_planted"], prop.1);
    }
    #[test]
    fn game_event_bomb_exploded() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "bomb_exploded".to_string(),
            vec![
                GameEvent {
                    name: "bomb_exploded".to_string(),
                    fields: vec![
                        EventField {
                            name: "site".to_string(),
                            data: Some(I32(184)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(38820)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 38820,
                },
                GameEvent {
                    name: "bomb_exploded".to_string(),
                    fields: vec![
                        EventField {
                            name: "site".to_string(),
                            data: Some(I32(185)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(51718)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 51718,
                },
            ],
        );
        assert_eq!(out.2["bomb_exploded"], prop.1);
    }
    #[test]
    fn game_event_round_prestart() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_prestart".to_string(),
            vec![
                GameEvent {
                    name: "round_prestart".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(65)),
                    }],
                    tick: 65,
                },
                GameEvent {
                    name: "round_prestart".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(9419)),
                    }],
                    tick: 9419,
                },
            ],
        );
        assert_eq!(out.2["round_prestart"], prop.1);
    }
    #[test]
    fn game_event_cs_round_final_beep() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "cs_round_final_beep".to_string(),
            vec![
                GameEvent {
                    name: "cs_round_final_beep".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(1761)),
                    }],
                    tick: 1761,
                },
                GameEvent {
                    name: "cs_round_final_beep".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(10699)),
                    }],
                    tick: 10699,
                },
            ],
        );
        assert_eq!(out.2["cs_round_final_beep"], prop.1);
    }
    #[test]
    fn game_event_smokegrenade_detonate() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "smokegrenade_detonate".to_string(),
            vec![
                GameEvent {
                    name: "smokegrenade_detonate".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(100)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(-1545.4756)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(259.92813)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-166.69106)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(3025)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 3025,
                },
                GameEvent {
                    name: "smokegrenade_detonate".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(204)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(253.67014)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(-1489.7667)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-173.96875)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(11282)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("povergo".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198280975787".to_string())),
                        },
                    ],
                    tick: 11282,
                },
            ],
        );
        assert_eq!(out.2["smokegrenade_detonate"], prop.1);
    }
    #[test]
    fn game_event_player_footstep() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "player_footstep".to_string(),
            vec![
                GameEvent {
                    name: "player_footstep".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1823)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Trahun <3 V".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198324843075".to_string())),
                        },
                    ],
                    tick: 1823,
                },
                GameEvent {
                    name: "player_footstep".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1843)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("NIGHTSOUL".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198244754626".to_string())),
                        },
                    ],
                    tick: 1843,
                },
            ],
        );
        assert_eq!(out.2["player_footstep"], prop.1);
    }
    #[test]
    fn game_event_buytime_ended() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "buytime_ended".to_string(),
            vec![
                GameEvent {
                    name: "buytime_ended".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(65)),
                    }],
                    tick: 65,
                },
                GameEvent {
                    name: "buytime_ended".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(3042)),
                    }],
                    tick: 3042,
                },
            ],
        );
        assert_eq!(out.2["buytime_ended"], prop.1);
    }
    #[test]
    fn game_event_player_jump() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "player_jump".to_string(),
            vec![
                GameEvent {
                    name: "player_jump".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1823)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Trahun <3 V".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198324843075".to_string())),
                        },
                    ],
                    tick: 1823,
                },
                GameEvent {
                    name: "player_jump".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(2058)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 2058,
                },
            ],
        );
        assert_eq!(out.2["player_jump"], prop.1);
    }
    #[test]
    fn game_event_weapon_zoom() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "weapon_zoom".to_string(),
            vec![
                GameEvent {
                    name: "weapon_zoom".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(11111)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 11111,
                },
                GameEvent {
                    name: "weapon_zoom".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(11130)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 11130,
                },
            ],
        );
        assert_eq!(out.2["weapon_zoom"], prop.1);
    }
    #[test]
    fn game_event_round_poststart() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_poststart".to_string(),
            vec![
                GameEvent {
                    name: "round_poststart".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(65)),
                    }],
                    tick: 65,
                },
                GameEvent {
                    name: "round_poststart".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(9419)),
                    }],
                    tick: 9419,
                },
            ],
        );
        assert_eq!(out.2["round_poststart"], prop.1);
    }
    #[test]
    fn game_event_bomb_pickup() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "bomb_pickup".to_string(),
            vec![
                GameEvent {
                    name: "bomb_pickup".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(65)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 65,
                },
                GameEvent {
                    name: "bomb_pickup".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(5839)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 5839,
                },
            ],
        );
        assert_eq!(out.2["bomb_pickup"], prop.1);
    }
    #[test]
    fn game_event_player_blind() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "player_blind".to_string(),
            vec![
                GameEvent {
                    name: "player_blind".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(138)),
                        },
                        EventField {
                            name: "blind_duration".to_string(),
                            data: Some(F32(0.26377675)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(4213)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                        EventField {
                            name: "attacker_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "attacker_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 4213,
                },
                GameEvent {
                    name: "player_blind".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(138)),
                        },
                        EventField {
                            name: "blind_duration".to_string(),
                            data: Some(F32(3.6708884)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(4213)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Голова, глаза".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198118803912".to_string())),
                        },
                        EventField {
                            name: "attacker_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "attacker_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 4213,
                },
            ],
        );
        assert_eq!(out.2["player_blind"], prop.1);
    }
    #[test]
    fn game_event_bomb_begindefuse() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "bomb_begindefuse".to_string(),
            vec![GameEvent {
                name: "bomb_begindefuse".to_string(),
                fields: vec![
                    EventField {
                        name: "haskit".to_string(),
                        data: Some(Bool(false)),
                    },
                    EventField {
                        name: "tick".to_string(),
                        data: Some(I32(14592)),
                    },
                    EventField {
                        name: "user_name".to_string(),
                        data: Some(String("-ExΩtiC-".to_string())),
                    },
                    EventField {
                        name: "user_steamid".to_string(),
                        data: Some(String("76561198258044111".to_string())),
                    },
                ],
                tick: 14592,
            }],
        );
        assert_eq!(out.2["bomb_begindefuse"], prop.1);
    }
    #[test]
    fn game_event_inferno_startburn() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "inferno_startburn".to_string(),
            vec![
                GameEvent {
                    name: "inferno_startburn".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(421)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(109.69482)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(-1630.7711)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-169.96875)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(17282)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("povergo".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198280975787".to_string())),
                        },
                    ],
                    tick: 17282,
                },
                GameEvent {
                    name: "inferno_startburn".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(429)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(-1029.7075)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(-468.01788)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-309.96875)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(17690)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 17690,
                },
            ],
        );
        assert_eq!(out.2["inferno_startburn"], prop.1);
    }
    #[test]
    fn game_event_player_disconnect() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "player_disconnect".to_string(),
            vec![
                GameEvent {
                    name: "player_disconnect".to_string(),
                    fields: vec![
                        EventField {
                            name: "reason".to_string(),
                            data: Some(I32(2)),
                        },
                        EventField {
                            name: "name".to_string(),
                            data: Some(String("povergo".to_string())),
                        },
                        EventField {
                            name: "networkid".to_string(),
                            data: Some(String("[U:1:320710059]".to_string())),
                        },
                        EventField {
                            name: "xuid".to_string(),
                            data: Some(U64(76561198280975787)),
                        },
                        EventField {
                            name: "PlayerID".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(55752)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("povergo".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198280975787".to_string())),
                        },
                    ],
                    tick: 55752,
                },
                GameEvent {
                    name: "player_disconnect".to_string(),
                    fields: vec![
                        EventField {
                            name: "reason".to_string(),
                            data: Some(I32(2)),
                        },
                        EventField {
                            name: "name".to_string(),
                            data: Some(String("123".to_string())),
                        },
                        EventField {
                            name: "networkid".to_string(),
                            data: Some(String("[U:1:305101042]".to_string())),
                        },
                        EventField {
                            name: "xuid".to_string(),
                            data: Some(U64(76561198265366770)),
                        },
                        EventField {
                            name: "PlayerID".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(57208)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("123".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198265366770".to_string())),
                        },
                    ],
                    tick: 57208,
                },
            ],
        );
        assert_eq!(out.2["player_disconnect"], prop.1);
    }
    #[test]
    fn game_event_player_hurt() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "player_hurt".to_string(),
            vec![
                GameEvent {
                    name: "player_hurt".to_string(),
                    fields: vec![
                        EventField {
                            name: "health".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "armor".to_string(),
                            data: Some(I32(100)),
                        },
                        EventField {
                            name: "weapon".to_string(),
                            data: Some(String("p250".to_string())),
                        },
                        EventField {
                            name: "dmg_health".to_string(),
                            data: Some(I32(100)),
                        },
                        EventField {
                            name: "dmg_armor".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "hitgroup".to_string(),
                            data: Some(String("head".to_string())),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(3086)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("miu miu".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198073049527".to_string())),
                        },
                        EventField {
                            name: "attacker_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "attacker_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 3086,
                },
                GameEvent {
                    name: "player_hurt".to_string(),
                    fields: vec![
                        EventField {
                            name: "health".to_string(),
                            data: Some(I32(72)),
                        },
                        EventField {
                            name: "armor".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "weapon".to_string(),
                            data: Some(String("hkp2000".to_string())),
                        },
                        EventField {
                            name: "dmg_health".to_string(),
                            data: Some(I32(28)),
                        },
                        EventField {
                            name: "dmg_armor".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "hitgroup".to_string(),
                            data: Some(String("chest".to_string())),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(3917)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("NIGHTSOUL".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198244754626".to_string())),
                        },
                        EventField {
                            name: "attacker_name".to_string(),
                            data: Some(String("Trahun <3 V".to_string())),
                        },
                        EventField {
                            name: "attacker_steamid".to_string(),
                            data: Some(String("76561198324843075".to_string())),
                        },
                    ],
                    tick: 3917,
                },
            ],
        );
        assert_eq!(out.2["player_hurt"], prop.1);
    }
    #[test]
    fn game_event_bomb_beginplant() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "bomb_beginplant".to_string(),
            vec![
                GameEvent {
                    name: "bomb_beginplant".to_string(),
                    fields: vec![
                        EventField {
                            name: "site".to_string(),
                            data: Some(I32(185)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(8049)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 8049,
                },
                GameEvent {
                    name: "bomb_beginplant".to_string(),
                    fields: vec![
                        EventField {
                            name: "site".to_string(),
                            data: Some(I32(184)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(13164)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 13164,
                },
            ],
        );
        assert_eq!(out.2["bomb_beginplant"], prop.1);
    }
    #[test]
    fn game_event_round_officially_ended() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_officially_ended".to_string(),
            vec![
                GameEvent {
                    name: "round_officially_ended".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(9419)),
                    }],
                    tick: 9419,
                },
                GameEvent {
                    name: "round_officially_ended".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(15680)),
                    }],
                    tick: 15680,
                },
            ],
        );
        assert_eq!(out.2["round_officially_ended"], prop.1);
    }
    #[test]
    fn game_event_item_pickup() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "item_pickup".to_string(),
            vec![
                GameEvent {
                    name: "item_pickup".to_string(),
                    fields: vec![
                        EventField {
                            name: "item".to_string(),
                            data: Some(String("knife".to_string())),
                        },
                        EventField {
                            name: "silent".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "defindex".to_string(),
                            data: Some(I32(59)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(65)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 65,
                },
                GameEvent {
                    name: "item_pickup".to_string(),
                    fields: vec![
                        EventField {
                            name: "item".to_string(),
                            data: Some(String("glock".to_string())),
                        },
                        EventField {
                            name: "silent".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "defindex".to_string(),
                            data: Some(I32(4)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(65)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 65,
                },
            ],
        );
        assert_eq!(out.2["item_pickup"], prop.1);
    }
    #[test]
    fn game_event_player_spawn() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "player_spawn".to_string(),
            vec![
                GameEvent {
                    name: "player_spawn".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(65)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 65,
                },
                GameEvent {
                    name: "player_spawn".to_string(),
                    fields: vec![
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(65)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("miu miu".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198073049527".to_string())),
                        },
                    ],
                    tick: 65,
                },
            ],
        );
        assert_eq!(out.2["player_spawn"], prop.1);
    }
    #[test]
    fn game_event_other_death() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "other_death".to_string(),
            vec![
                GameEvent {
                    name: "other_death".to_string(),
                    fields: vec![
                        EventField {
                            name: "otherid".to_string(),
                            data: Some(I32(271)),
                        },
                        EventField {
                            name: "othertype".to_string(),
                            data: Some(String("prop_physics_multiplayer".to_string())),
                        },
                        EventField {
                            name: "weapon".to_string(),
                            data: Some(String("usp_silencer".to_string())),
                        },
                        EventField {
                            name: "weapon_itemid".to_string(),
                            data: Some(String("35283237811".to_string())),
                        },
                        EventField {
                            name: "weapon_fauxitemid".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "weapon_originalowner_xuid".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "headshot".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "penetrated".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "noscope".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "thrusmoke".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "attackerblind".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1977)),
                        },
                        EventField {
                            name: "attacker_name".to_string(),
                            data: Some(String("Trahun <3 V".to_string())),
                        },
                        EventField {
                            name: "attacker_steamid".to_string(),
                            data: Some(String("76561198324843075".to_string())),
                        },
                    ],
                    tick: 1977,
                },
                GameEvent {
                    name: "other_death".to_string(),
                    fields: vec![
                        EventField {
                            name: "otherid".to_string(),
                            data: Some(I32(97)),
                        },
                        EventField {
                            name: "othertype".to_string(),
                            data: Some(String("prop_dynamic".to_string())),
                        },
                        EventField {
                            name: "weapon".to_string(),
                            data: Some(String("hkp2000".to_string())),
                        },
                        EventField {
                            name: "weapon_itemid".to_string(),
                            data: Some(String("0".to_string())),
                        },
                        EventField {
                            name: "weapon_fauxitemid".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "weapon_originalowner_xuid".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "headshot".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "penetrated".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "noscope".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "thrusmoke".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "attackerblind".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1996)),
                        },
                        EventField {
                            name: "attacker_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "attacker_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 1996,
                },
            ],
        );
        assert_eq!(out.2["other_death"], prop.1);
    }
    #[test]
    fn game_event_bomb_defused() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "bomb_defused".to_string(),
            vec![GameEvent {
                name: "bomb_defused".to_string(),
                fields: vec![
                    EventField {
                        name: "site".to_string(),
                        data: Some(I32(184)),
                    },
                    EventField {
                        name: "tick".to_string(),
                        data: Some(I32(15232)),
                    },
                    EventField {
                        name: "user_name".to_string(),
                        data: Some(String("-ExΩtiC-".to_string())),
                    },
                    EventField {
                        name: "user_steamid".to_string(),
                        data: Some(String("76561198258044111".to_string())),
                    },
                ],
                tick: 15232,
            }],
        );
        assert_eq!(out.2["bomb_defused"], prop.1);
    }
    #[test]
    fn game_event_begin_new_match() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "begin_new_match".to_string(),
            vec![GameEvent {
                name: "begin_new_match".to_string(),
                fields: vec![EventField {
                    name: "tick".to_string(),
                    data: Some(I32(67)),
                }],
                tick: 67,
            }],
        );
        assert_eq!(out.2["begin_new_match"], prop.1);
    }
    #[test]
    fn game_event_cs_win_panel_round() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "cs_win_panel_round".to_string(),
            vec![
                GameEvent {
                    name: "cs_win_panel_round".to_string(),
                    fields: vec![
                        EventField {
                            name: "show_timer_defend".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "show_timer_attack".to_string(),
                            data: Some(Bool(true)),
                        },
                        EventField {
                            name: "timer_time".to_string(),
                            data: Some(I32(112)),
                        },
                        EventField {
                            name: "final_event".to_string(),
                            data: Some(I32(9)),
                        },
                        EventField {
                            name: "funfact_token".to_string(),
                            data: Some(String("#funfact_kills_headshots".to_string())),
                        },
                        EventField {
                            name: "funfact_player".to_string(),
                            data: Some(I32(4)),
                        },
                        EventField {
                            name: "funfact_data1".to_string(),
                            data: Some(I32(2)),
                        },
                        EventField {
                            name: "funfact_data2".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "funfact_data3".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(8971)),
                        },
                    ],
                    tick: 8971,
                },
                GameEvent {
                    name: "cs_win_panel_round".to_string(),
                    fields: vec![
                        EventField {
                            name: "show_timer_defend".to_string(),
                            data: Some(Bool(false)),
                        },
                        EventField {
                            name: "show_timer_attack".to_string(),
                            data: Some(Bool(true)),
                        },
                        EventField {
                            name: "timer_time".to_string(),
                            data: Some(I32(70)),
                        },
                        EventField {
                            name: "final_event".to_string(),
                            data: Some(I32(7)),
                        },
                        EventField {
                            name: "funfact_token".to_string(),
                            data: Some(String("#funfact_grenades_thrown".to_string())),
                        },
                        EventField {
                            name: "funfact_player".to_string(),
                            data: Some(I32(3)),
                        },
                        EventField {
                            name: "funfact_data1".to_string(),
                            data: Some(I32(3)),
                        },
                        EventField {
                            name: "funfact_data2".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "funfact_data3".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(15232)),
                        },
                    ],
                    tick: 15232,
                },
            ],
        );
        assert_eq!(out.2["cs_win_panel_round"], prop.1);
    }
    #[test]
    fn game_event_cs_win_panel_match() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "cs_win_panel_match".to_string(),
            vec![GameEvent {
                name: "cs_win_panel_match".to_string(),
                fields: vec![EventField {
                    name: "tick".to_string(),
                    data: Some(I32(56898)),
                }],
                tick: 56898,
            }],
        );
        assert_eq!(out.2["cs_win_panel_match"], prop.1);
    }
    #[test]
    fn game_event_cs_round_start_beep() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "cs_round_start_beep".to_string(),
            vec![
                GameEvent {
                    name: "cs_round_start_beep".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(1570)),
                    }],
                    tick: 1570,
                },
                GameEvent {
                    name: "cs_round_start_beep".to_string(),
                    fields: vec![EventField {
                        name: "tick".to_string(),
                        data: Some(I32(1634)),
                    }],
                    tick: 1634,
                },
            ],
        );
        assert_eq!(out.2["cs_round_start_beep"], prop.1);
    }
    #[test]
    fn game_event_bomb_dropped() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "bomb_dropped".to_string(),
            vec![
                GameEvent {
                    name: "bomb_dropped".to_string(),
                    fields: vec![
                        EventField {
                            name: "entindex".to_string(),
                            data: Some(I32(230)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(4285)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Подсосник blick'a".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561197964020430".to_string())),
                        },
                    ],
                    tick: 4285,
                },
                GameEvent {
                    name: "bomb_dropped".to_string(),
                    fields: vec![
                        EventField {
                            name: "entindex".to_string(),
                            data: Some(I32(148)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(15949)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("123".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198265366770".to_string())),
                        },
                    ],
                    tick: 15949,
                },
            ],
        );
        assert_eq!(out.2["bomb_dropped"], prop.1);
    }
    #[test]
    fn game_event_inferno_expire() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "inferno_expire".to_string(),
            vec![
                GameEvent {
                    name: "inferno_expire".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(421)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(109.69482)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(-1630.7711)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-169.96875)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(17732)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("povergo".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198280975787".to_string())),
                        },
                    ],
                    tick: 17732,
                },
                GameEvent {
                    name: "inferno_expire".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(429)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(-1029.7075)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(-468.01788)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-309.96875)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(18140)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 18140,
                },
            ],
        );
        assert_eq!(out.2["inferno_expire"], prop.1);
    }
    #[test]
    fn game_event_round_end() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_end".to_string(),
            vec![
                GameEvent {
                    name: "round_end".to_string(),
                    fields: vec![
                        EventField {
                            name: "winner".to_string(),
                            data: Some(I32(2)),
                        },
                        EventField {
                            name: "reason".to_string(),
                            data: Some(I32(9)),
                        },
                        EventField {
                            name: "message".to_string(),
                            data: Some(String("#SFUI_Notice_Terrorists_Win".to_string())),
                        },
                        EventField {
                            name: "legacy".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "player_count".to_string(),
                            data: Some(I32(20)),
                        },
                        EventField {
                            name: "nomusic".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(8971)),
                        },
                    ],
                    tick: 8971,
                },
                GameEvent {
                    name: "round_end".to_string(),
                    fields: vec![
                        EventField {
                            name: "winner".to_string(),
                            data: Some(I32(3)),
                        },
                        EventField {
                            name: "reason".to_string(),
                            data: Some(I32(7)),
                        },
                        EventField {
                            name: "message".to_string(),
                            data: Some(String("#SFUI_Notice_Bomb_Defused".to_string())),
                        },
                        EventField {
                            name: "legacy".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "player_count".to_string(),
                            data: Some(I32(20)),
                        },
                        EventField {
                            name: "nomusic".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(15232)),
                        },
                    ],
                    tick: 15232,
                },
            ],
        );
        assert_eq!(out.2["round_end"], prop.1);
    }
    #[test]
    fn game_event_round_start() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_start".to_string(),
            vec![
                GameEvent {
                    name: "round_start".to_string(),
                    fields: vec![
                        EventField {
                            name: "timelimit".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "fraglimit".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "objective".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(65)),
                        },
                    ],
                    tick: 65,
                },
                GameEvent {
                    name: "round_start".to_string(),
                    fields: vec![
                        EventField {
                            name: "timelimit".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "fraglimit".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "objective".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(9419)),
                        },
                    ],
                    tick: 9419,
                },
            ],
        );
        assert_eq!(out.2["round_start"], prop.1);
    }
    #[test]
    fn game_event_round_time_warning() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_time_warning".to_string(),
            vec![GameEvent {
                name: "round_time_warning".to_string(),
                fields: vec![EventField {
                    name: "tick".to_string(),
                    data: Some(I32(8482)),
                }],
                tick: 8482,
            }],
        );
        assert_eq!(out.2["round_time_warning"], prop.1);
    }
    #[test]
    fn game_event_item_purchase() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "item_purchase".to_string(),
            vec![
                GameEvent {
                    name: "item_purchase".to_string(),
                    fields: vec![
                        EventField {
                            name: "item_name".to_string(),
                            data: Some(String("item_assaultsuit".to_string())),
                        },
                        EventField {
                            name: "name".to_string(),
                            data: Some(String("123".to_string())),
                        },
                        EventField {
                            name: "steamid".to_string(),
                            data: Some(U64(76561198265366770)),
                        },
                        EventField {
                            name: "inventory_slot".to_string(),
                            data: Some(U32(0)),
                        },
                        EventField {
                            name: "cost".to_string(),
                            data: Some(I32(1000)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1)),
                        },
                        EventField {
                            name: "float".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "skin".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "skin_id".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "paint_seed".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "stickers".to_string(),
                            data: Some(Stickers(vec![])),
                        },
                        EventField {
                            name: "custom_name".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "was_sold".to_string(),
                            data: Some(Bool(false)),
                        },
                    ],
                    tick: 1,
                },
                GameEvent {
                    name: "item_purchase".to_string(),
                    fields: vec![
                        EventField {
                            name: "item_name".to_string(),
                            data: Some(String("AK-47".to_string())),
                        },
                        EventField {
                            name: "name".to_string(),
                            data: Some(String("123".to_string())),
                        },
                        EventField {
                            name: "steamid".to_string(),
                            data: Some(U64(76561198265366770)),
                        },
                        EventField {
                            name: "inventory_slot".to_string(),
                            data: Some(U32(1)),
                        },
                        EventField {
                            name: "cost".to_string(),
                            data: Some(I32(2700)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(1)),
                        },
                        EventField {
                            name: "float".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "skin".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "skin_id".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "paint_seed".to_string(),
                            data: None,
                        },
                        EventField {
                            name: "stickers".to_string(),
                            data: Some(Stickers(vec![])),
                        },
                        EventField {
                            name: "custom_name".to_string(),
                            data: Some(String("".to_string())),
                        },
                        EventField {
                            name: "was_sold".to_string(),
                            data: Some(Bool(false)),
                        },
                    ],
                    tick: 1,
                },
            ],
        );
        assert_eq!(out.2["item_purchase"], prop.1);
    }

    #[test]
    fn game_event_flashbang_detonate() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "flashbang_detonate".to_string(),
            vec![
                GameEvent {
                    name: "flashbang_detonate".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(138)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(-813.8123)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(23.765327)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-65.055046)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(4213)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 4213,
                },
                GameEvent {
                    name: "flashbang_detonate".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(214)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(-662.246)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(-492.32172)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(256.63577)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(11213)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("123".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198265366770".to_string())),
                        },
                    ],
                    tick: 11213,
                },
            ],
        );
        assert_eq!(out.2["flashbang_detonate"], prop.1);
    }
    #[test]
    fn game_event_round_mvp() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_mvp".to_string(),
            vec![
                GameEvent {
                    name: "round_mvp".to_string(),
                    fields: vec![
                        EventField {
                            name: "reason".to_string(),
                            data: Some(I32(1)),
                        },
                        EventField {
                            name: "value".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "musickitmvps".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "nomusic".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "musickitid".to_string(),
                            data: Some(I32(68)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(8971)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 8971,
                },
                GameEvent {
                    name: "round_mvp".to_string(),
                    fields: vec![
                        EventField {
                            name: "reason".to_string(),
                            data: Some(I32(3)),
                        },
                        EventField {
                            name: "value".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "musickitmvps".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "nomusic".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "musickitid".to_string(),
                            data: Some(I32(0)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(15232)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("-ExΩtiC-".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198258044111".to_string())),
                        },
                    ],
                    tick: 15232,
                },
            ],
        );
        assert_eq!(out.2["round_mvp"], prop.1);
    }
    #[test]
    fn game_event_round_announce_match_start() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "round_announce_match_start".to_string(),
            vec![GameEvent {
                name: "round_announce_match_start".to_string(),
                fields: vec![EventField {
                    name: "tick".to_string(),
                    data: Some(I32(1761)),
                }],
                tick: 1761,
            }],
        );
        assert_eq!(out.2["round_announce_match_start"], prop.1);
    }
    #[test]
    fn game_event_hegrenade_detonate() {
        use crate::second_pass::variants::Variant::*;
        let prop = (
            "hegrenade_detonate".to_string(),
            vec![
                GameEvent {
                    name: "hegrenade_detonate".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(151)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(-1405.5868)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(788.1175)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-34.35749)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(3279)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("Dog".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198194694750".to_string())),
                        },
                    ],
                    tick: 3279,
                },
                GameEvent {
                    name: "hegrenade_detonate".to_string(),
                    fields: vec![
                        EventField {
                            name: "entityid".to_string(),
                            data: Some(I32(146)),
                        },
                        EventField {
                            name: "x".to_string(),
                            data: Some(F32(-1302.9464)),
                        },
                        EventField {
                            name: "y".to_string(),
                            data: Some(F32(-1189.3713)),
                        },
                        EventField {
                            name: "z".to_string(),
                            data: Some(F32(-42.400486)),
                        },
                        EventField {
                            name: "tick".to_string(),
                            data: Some(I32(3583)),
                        },
                        EventField {
                            name: "user_name".to_string(),
                            data: Some(String("IMI Negev".to_string())),
                        },
                        EventField {
                            name: "user_steamid".to_string(),
                            data: Some(String("76561198202353993".to_string())),
                        },
                    ],
                    tick: 3583,
                },
            ],
        );
        assert_eq!(out.2["hegrenade_detonate"], prop.1);
    }
}
