use crate::definitions::DemoParserError;
use crate::first_pass::frameparser::{FrameParser, DemoChunk};
use crate::first_pass::parser::FirstPassOutput;
use crate::first_pass::parser_settings::{FirstPassParser, ParserInputs};
use crate::first_pass::prop_controller::{PropController, NAME_ID, STEAMID_ID, TICK_ID};
use crate::maps::NON_MULTITHREADABLE_PROPS;
use crate::second_pass::collect_data::ProjectileRecord;
use crate::second_pass::game_events::{EventField, GameEvent};
use crate::second_pass::parser::SecondPassOutput;
use crate::second_pass::parser_settings::*;
use crate::second_pass::variants::VarVec;
use crate::second_pass::variants::{PropColumn, Variant};
use ahash::AHashMap;
use ahash::AHashSet;
use csgoproto::CsvcMsgVoiceData;
use itertools::Itertools;
use nohash::IntSet;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use std::mem;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct DemoOutput {
    pub df: AHashMap<u32, PropColumn>,
    pub game_events: Vec<GameEvent>,
    pub skins: Vec<EconItem>,
    pub item_drops: Vec<EconItem>,
    pub header: Option<AHashMap<String, String>>,
    pub player_md: Vec<PlayerEndMetaData>,
    pub game_events_counter: AHashSet<String>,
    pub projectiles: Vec<ProjectileRecord>,
    pub voice_data: Vec<CsvcMsgVoiceData>,
    pub prop_controller: PropController,
    pub df_per_player: AHashMap<u64, AHashMap<u32, PropColumn>>,
}

pub struct Parser {
    input: ParserInputs,
    parsing_mode: ParsingMode,
}

#[derive(PartialEq)]
pub enum ParsingMode {
    ForceSingleThreaded,
    ForceRayonThreaded,
    ForceMultiThreaded,
    Normal,
}

impl Parser {
    pub fn new(input: ParserInputs, parsing_mode: ParsingMode) -> Self {
        Parser {
            input,
            parsing_mode,
        }
    }

    pub fn parse_demo(&mut self, demo_bytes: &[u8]) -> Result<DemoOutput, DemoParserError> {
        use std::time::Instant;
        let now = Instant::now();
    
        {
            let mut first_pass_parser = FirstPassParser::new(&self.input);
            first_pass_parser.parse_demo(demo_bytes)?;
            let _first_pass_output = first_pass_parser.create_output()?;
        }
    
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        let mut first_pass_parser = FirstPassParser::new(&self.input);
        first_pass_parser.parse_demo(demo_bytes)?;
        let first_pass_output = first_pass_parser.create_output()?;

        // Multi threaded second pass
        if self.parsing_mode == ParsingMode::ForceMultiThreaded || self.is_multithreadable() {
            let (sender, receiver) = channel();
            let _ = FrameParser::default().start(demo_bytes, Some(&sender));
            return self.second_pass_threaded_with_channels(demo_bytes, first_pass_output, receiver);
        }

        // Rayon-based multi threaded second pass
        if self.parsing_mode == ParsingMode::ForceRayonThreaded {
            return FrameParser::default()
                .start(demo_bytes, None)
                .and_then(|chunks| self.second_pass_multi_threaded_no_channels(chunks, demo_bytes, first_pass_output));
        }

        // Single threaded second pass
        self.second_pass_single_threaded(demo_bytes, first_pass_output)
    }

    fn second_pass_single_threaded(&self, demo_bytes: &[u8], first_pass_output: FirstPassOutput) -> Result<DemoOutput, DemoParserError> {
        let mut parser = SecondPassParser::new(&self.input, &first_pass_output, None)?;
        parser.start(demo_bytes)?;
        let second_pass_output = parser.create_output();

        let output = self.combine_outputs(vec![second_pass_output], first_pass_output);
        Ok(output)
    }

    fn second_pass_threaded_with_channels(
        &self,
        demo_bytes: &[u8],
        first_pass_output: FirstPassOutput,
        reciever: Receiver<DemoChunk>,
    ) -> Result<DemoOutput, DemoParserError> {
        let output_for_thread = &first_pass_output.clone();
        thread::scope(|s| {
            let mut handles = vec![];
            loop {
                let chunk = reciever.recv_timeout(Duration::from_secs(3)).map_err(|_| DemoParserError::MultithreadingWasNotOk)?;
                if chunk.end_of_demo {
                    break;
                }

                handles.push(s.spawn(move || {
                    let mut parser = SecondPassParser::new(&self.input, output_for_thread, Some(chunk))?;
                    parser.start(demo_bytes)?;
                    Ok(parser.create_output())
                }));
            }

            // Join outputs from threads
            let second_pass_outputs: Result<Vec<SecondPassOutput>, DemoParserError> = handles
                .into_iter()
                .map(|result| result.join().map_err(|_| DemoParserError::MalformedMessage)?)
                .collect();

            let output = self.combine_outputs(second_pass_outputs?, first_pass_output);
            Ok(output)
        })
    }

    fn second_pass_multi_threaded_no_channels(
        &self,
        chunks: Vec<DemoChunk>,
        demo_bytes: &[u8],
        first_pass_output: FirstPassOutput,
    ) -> Result<DemoOutput, DemoParserError> {
        let second_pass_outputs: Result<Vec<SecondPassOutput>, DemoParserError> = chunks
            .into_par_iter()
            .map(|chunk| {
                let mut parser = SecondPassParser::new(&self.input, &first_pass_output, Some(chunk))?;
                parser.start(demo_bytes)?;
                Ok(parser.create_output())
            })
            .collect();

        let output = self.combine_outputs(second_pass_outputs?, first_pass_output);
        Ok(output)
    }

    fn is_multithreadable(&self) -> bool {
        self.parsing_mode == ParsingMode::Normal && !self.input.wanted_player_props.iter().any(|p| NON_MULTITHREADABLE_PROPS.contains(p))
    }

    fn remove_item_sold_events(events: &mut Vec<GameEvent>) {
        events.retain(|x| x.name != "item_sold")
    }

    fn add_item_purchase_sell_column(events: &mut Vec<GameEvent>) {
        // Checks each item_purchase event for if the item was eventually sold

        let purchases = events.iter().filter(|x| x.name == "item_purchase").collect_vec();
        let sells = events.iter().filter(|x| x.name == "item_sold").collect_vec();

        let purchases = purchases
            .iter()
            .filter_map(|event| SellBackHelper::from_event(event))
            .collect_vec();
        let sells = sells
            .iter()
            .filter_map(|event| SellBackHelper::from_event(event))
            .collect_vec();

        let mut was_sold = vec![];
        for purchase in &purchases {
            let wanted_sells = sells.iter().filter(|sell| {
                sell.tick > purchase.tick && sell.steamid == purchase.steamid && sell.inventory_slot == purchase.inventory_slot
            });
            let wanted_buys = purchases.iter().filter(|buy| {
                buy.tick > purchase.tick && buy.steamid == purchase.steamid && buy.inventory_slot == purchase.inventory_slot
            });
            let min_tick_sells = wanted_sells.min_by_key(|x| x.tick);
            let min_tick_buys = wanted_buys.min_by_key(|x| x.tick);
            if let (Some(sell_tick), Some(buy_tick)) = (min_tick_sells, min_tick_buys) {
                if sell_tick.tick < buy_tick.tick {
                    was_sold.push(true);
                } else {
                    was_sold.push(false);
                }
            } else {
                was_sold.push(false);
            }
        }
        let mut idx = 0;
        for event in events {
            if event.name == "item_purchase" {
                event.fields.push(EventField {
                    name: "was_sold".to_string(),
                    data: Some(Variant::Bool(was_sold[idx])),
                });
                idx += 1;
            }
        }
    }

    fn remove_unwanted_ticks(hm: &mut AHashMap<u32, PropColumn>, wanted_ticks: IntSet<i32>) {
        // Used for removing ticks when velocity is needed
        let Some(ticks) = hm.get(&TICK_ID) else { return };
        let Some(VarVec::I32(ticks)) = &ticks.data else { return };

        let wanted_indicies = ticks.iter()
            .enumerate()
            .filter_map(|(idx, tick)| {
                if tick.is_some_and(|t| wanted_ticks.contains(&t)) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect_vec();

        for prop_column in hm.values_mut() {
            prop_column.retain(&wanted_indicies);
        }
    }

    fn combine_outputs(&self, mut second_pass_outputs: Vec<SecondPassOutput>, mut first_pass_output: FirstPassOutput) -> DemoOutput {
        // Combines all inner DemoOutputs into one big output
        second_pass_outputs.sort_by_key(|x| x.ptr);

        // Remove temp props
        for prop in first_pass_output.added_temp_props {
            first_pass_output.prop_controller.wanted_player_props.retain(|x| *x != prop);
            first_pass_output.prop_controller.prop_infos.retain(|x| x.prop_name != prop);
        }

        let dfs = second_pass_outputs.iter_mut().map(|x| mem::take(&mut x.prop_data)).collect_vec();
        let mut all_dfs_combined = self.combine_dfs(dfs, false);
        if !first_pass_output.wanted_ticks.is_empty() {
            Parser::remove_unwanted_ticks(&mut all_dfs_combined, first_pass_output.wanted_ticks);
        }

        let per_players = second_pass_outputs.iter_mut().map(|x| mem::take(&mut x.prop_data_per_player)).collect_vec();
        let mut all_steamids = AHashSet::default();
        for entry in &per_players {
            for k in entry.keys() {
                all_steamids.insert(k);
            }
        }
        let mut pp = AHashMap::default();
        for steamid in all_steamids {
            let mut v = vec![];
            for output in &per_players {
                if let Some(df) = output.get(steamid) {
                    v.push(df.clone());
                }
            }
            let combined = self.combine_dfs(v, true);
            pp.insert(*steamid, combined);
        }

        let mut output = DemoOutput {
            prop_controller: first_pass_output.prop_controller,
            item_drops: second_pass_outputs.iter_mut().flat_map(|x| mem::take(&mut x.item_drops)).collect(),
            player_md: second_pass_outputs.iter_mut().flat_map(|x| mem::take(&mut x.player_md)).collect(),
            game_events: second_pass_outputs.iter_mut().flat_map(|x| mem::take(&mut x.game_events)).collect(),
            skins: second_pass_outputs.iter_mut().flat_map(|x| mem::take(&mut x.skins)).collect(),
            df: all_dfs_combined,
            header: Some(first_pass_output.header),
            game_events_counter: second_pass_outputs.iter_mut().flat_map(|x| mem::take(&mut x.game_events_counter)).collect(),
            projectiles: second_pass_outputs.iter_mut().flat_map(|x| mem::take(&mut x.projectiles)).collect(),
            voice_data: second_pass_outputs.iter_mut().flat_map(|x| mem::take(&mut x.voice_data)).collect_vec(),
            df_per_player: pp,
        };
        
        Parser::add_item_purchase_sell_column(&mut output.game_events);
        Parser::remove_item_sold_events(&mut output.game_events);

        output
    }

    fn combine_dfs(&self, hashmaps: Vec<AHashMap<u32, PropColumn>>, remove_name_and_steamid: bool) -> AHashMap<u32, PropColumn> {
        let mut result = hashmaps.into_iter()
            .reduce(|mut acc, part| {
                for (key, mut value) in part {
                    acc.entry(key)
                        .and_modify(|inner| inner.extend_from(&mut value))
                        .or_insert_with(|| value);
                }
                acc
            })
            .unwrap_or_default();

        if remove_name_and_steamid {
            result.remove(&STEAMID_ID);
            result.remove(&NAME_ID);
        }
        result
    }
}

#[derive(Debug)]
pub struct SellBackHelper {
    pub tick: i32,
    pub steamid: u64,
    pub inventory_slot: u32,
}

impl SellBackHelper {
    pub fn from_event(event: &GameEvent) -> Option<Self> {
        let Some(Variant::I32(tick)) = SellBackHelper::extract_field("tick", &event.fields) else { return None };
        let Some(Variant::U64(steamid)) = SellBackHelper::extract_field("steamid", &event.fields) else { return None };
        let Some(Variant::U32(inventory_slot)) = SellBackHelper::extract_field("inventory_slot", &event.fields) else { return None };
        Some(SellBackHelper {
            tick,
            steamid,
            inventory_slot,
        })
    }

    fn extract_field(name: &str, fields: &[EventField]) -> Option<Variant> {
        for field in fields {
            if field.name == name {
                return field.data.clone();
            }
        }
        None
    }
}
