use crate::definitions::DemoParserError;
use crate::maps::PAINTKITS;
use crate::maps::WEAPINDICIES;
use crate::second_pass::parser_settings::EconItem;
use crate::second_pass::parser_settings::PlayerEndMetaData;
use crate::second_pass::parser_settings::SecondPassParser;
use csgoproto::CEconItemPreviewDataBlock;
use csgoproto::CcsUsrMsgEndOfMatchAllPlayersData;
use csgoproto::CcsUsrMsgSendPlayerItemDrops;
use prost::Message;

impl<'a> SecondPassParser<'a> {
    pub fn parse_item_drops(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let drops = CcsUsrMsgSendPlayerItemDrops::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        for item in &drops.entity_updates {
            let econ_item = create_econ_item(item, None);
            self.item_drops.push(econ_item);
        }
        Ok(())
    }

    pub fn parse_player_end_msg(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let end_data = CcsUsrMsgEndOfMatchAllPlayersData::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        /*
        Todo parse "accolade", seems to be the awards at the end like "most mvps in game"
        But seems to only have integers so need to figure out what they mean
        example:

        Accolade {
            eaccolade: Some(
                21,
            ),
            value: Some(
                5100.0,
            ),
            position: Some(
                1,
            ),
        }
        */
        for player in &end_data.allplayerdata {
            self.player_end_data.push(PlayerEndMetaData {
                name: player.name.to_owned(),
                steamid: player.xuid,
                team_number: player.teamnumber
            });

            for item in &player.items {
                if item.itemid() == 0 {
                    continue;
                }
                let econ_item = create_econ_item(item, player.xuid);
                self.skins.push(econ_item);
            }
        }
        Ok(())
    }

    pub fn parse_player_stats_update(&mut self, _bytes: &[u8]) -> Result<(), DemoParserError> {
        // let upd: CCSUsrMsg_PlayerStatsUpdate = Message::parse_from_bytes(bytes);
        Ok(())
    }

    pub fn parse_file_info(&mut self, _bytes: &[u8]) -> Result<(), DemoParserError> {
        // let _info: CDemoFileInfo = Message::parse_from_bytes(bytes);
        Ok(())
    }
}

#[inline(always)]
fn create_econ_item(input: &CEconItemPreviewDataBlock, steamid: Option<u64>) -> EconItem {
    let item_name = input.defindex.and_then(|idx| WEAPINDICIES.get(&idx).map(|name| name.to_string()));
    let skin_name = input.paintindex.and_then(|idx| PAINTKITS.get(&idx).map(|name| name.to_string()));

    EconItem {
        account_id: input.accountid,
        item_id: input.itemid,
        def_index: input.defindex,
        paint_index: input.paintindex,
        rarity: input.rarity,
        quality: input.quality,
        paint_seed: input.paintseed,
        paint_wear: input.paintwear,
        quest_id: input.questid,
        dropreason: input.dropreason,
        custom_name: input.customname.to_owned(),
        inventory: input.inventory,
        ent_idx: input.entindex,
        steamid,
        item_name,
        skin_name
    }
}
