#![allow(clippy::unnecessary_lazy_evaluations)]

use crate::definitions::DemoParserError;
use crate::first_pass::parser_settings::FirstPassParser;
use crate::first_pass::read_bits::Bitreader;
use crate::second_pass::parser_settings::SecondPassParser;
use csgoproto::CMsgPlayerInfo;
use csgoproto::CsvcMsgCreateStringTable;
use csgoproto::CsvcMsgUpdateStringTable;
use prost::Message;
use snap::raw::Decoder;

#[derive(Clone, Debug)]
pub struct StringTable {
    name: String,
    user_data_size: i32,
    user_data_fixed: bool,
    #[allow(dead_code)]
    data: Vec<StringTableEntry>,
    flags: i32,
    var_bit_counts: bool,
}

#[derive(Clone, Debug)]
pub struct StringTableEntry {
    pub idx: i32,
    pub key: String,
    pub value: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub steamid: u64,
    pub name: String,
    pub userid: i32,
    pub is_hltv: bool,
}

impl UserInfo {
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<UserInfo, DemoParserError> {
        let player = CMsgPlayerInfo::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        Ok(UserInfo {
            is_hltv: player.ishltv(),
            steamid: player.xuid(),
            name: player.name().to_string(),
            userid: player.userid(),
        })
    }
}

impl<'a> FirstPassParser<'a> {
    #[inline(always)]
    pub fn update_string_table(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let table = CsvcMsgUpdateStringTable::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        let st = self.string_tables.get(table.table_id() as usize).ok_or_else(|| DemoParserError::StringTableNotFound)?;

        self.parse_string_table(
            table.string_data(),
            table.num_changed_entries(),
            st.name.to_owned(),
            st.user_data_fixed,
            st.user_data_size,
            st.flags,
            st.var_bit_counts,
        )
    }

    #[inline(always)]
    pub fn parse_create_stringtable(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let table = CsvcMsgCreateStringTable::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;

        if !(table.name() == "instancebaseline" || table.name() == "userinfo") {
            return Ok(());
        }
        let bytes = if table.data_compressed() {
            &Decoder::new().decompress_vec(table.string_data()).map_err(|_| DemoParserError::MalformedMessage)?
        } else {
            table.string_data()
        };

        self.parse_string_table(
            bytes,
            table.num_entries(),
            table.name().to_string(),
            table.user_data_fixed_size(),
            table.user_data_size(),
            table.flags(),
            table.using_varint_bitcounts(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn parse_string_table(
        &mut self,
        bytes: &[u8],
        n_updates: i32,
        name: String,
        user_data_fixed: bool,
        user_data_size: i32,
        flags: i32,
        variant_bit_count: bool,
    ) -> Result<(), DemoParserError> {
        let mut bitreader = Bitreader::new(bytes);
        let mut idx = -1;
        let mut keys: Vec<String> = Vec::with_capacity(32);
        let mut items = Vec::with_capacity(n_updates as usize);

        for _ in 0..n_updates {
            // Increment index
            if bitreader.read_boolean()? {
                idx += 1
            } else {
                idx += (bitreader.read_varint()? + 1) as i32
            }

            // Does the value have a key
            if !bitreader.read_boolean()? {
                continue;
            }

            // Should we refer back to history (similar to LZ77)
            let key = if bitreader.read_boolean()? {
                // How far into history we should look
                let position = bitreader.read_nbits(5)? as usize;
                // How many bytes in a row, starting from distance ago, should be copied
                let length = bitreader.read_nbits(5)? as usize;

                if position >= keys.len() {
                    bitreader.read_string()?
                } else if let Some(s) = keys.get(position) {
                    let l = length.min(s.len());
                    s[0..l].to_owned() + &bitreader.read_string()?
                } else {
                    String::new()
                }
            } else {
                bitreader.read_string()?
            };

            if keys.len() >= 32 {
                keys.remove(0);
            }
            keys.push(key.clone());

            let mut value = vec![];
            // Does the entry have a value
            if bitreader.read_boolean()? {
                let is_compressed = if !user_data_fixed && (flags & 0x1) != 0 {
                    bitreader.read_boolean()?
                } else {
                    false
                };
                let size = if user_data_fixed {
                    user_data_size as u32
                } else if variant_bit_count {
                    bitreader.read_u_bit_var()? * 8
                } else {
                    bitreader.read_nbits(17)? * 8
                };

                value = bitreader.read_n_bytes((size.checked_div(8).unwrap_or(0)) as usize)?;
                if is_compressed {
                    value = Decoder::new().decompress_vec(&value).map_err(|_| DemoParserError::MalformedMessage)?
                }
            }

            if name == "userinfo" {
                if let Ok(player) = UserInfo::from_bytes(&value) {
                    if player.steamid != 0 {
                        self.stringtable_players.insert(player.userid, player);
                    }
                }
            } else if name == "instancebaseline" {
                if let Ok(cls_id) = key.parse::<u32>() {
                    self.baselines.insert(cls_id, value.clone());
                }
            }

            items.push(StringTableEntry {
                idx,
                key,
                value,
            });
        }
        self.string_tables.push(StringTable {
            data: items,
            name,
            user_data_size,
            user_data_fixed,
            flags,
            var_bit_counts: variant_bit_count,
        });
        Ok(())
    }
}

impl<'a> SecondPassParser<'a> {
    #[inline(always)]
    pub fn update_string_table(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let table = CsvcMsgUpdateStringTable::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        let Some(st) = self.string_tables.get(table.table_id() as usize) else { return Ok(()) };
        self.parse_string_table(
            table.string_data(),
            table.num_changed_entries(),
            st.name.to_owned(),
            st.user_data_fixed,
            st.user_data_size,
            st.flags,
            st.var_bit_counts,
        )
    }

    #[inline(always)]
    pub fn parse_create_stringtable(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let table = CsvcMsgCreateStringTable::decode(bytes).map_err(|_| DemoParserError::MalformedMessage)?;
        let bytes = if table.data_compressed() {
            &Decoder::new().decompress_vec(table.string_data()).map_err(|_| DemoParserError::MalformedMessage)?
        } else {
            table.string_data()
        };

        self.parse_string_table(
            bytes,
            table.num_entries(),
            table.name().to_string(),
            table.user_data_fixed_size(),
            table.user_data_size(),
            table.flags(),
            table.using_varint_bitcounts(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn parse_string_table(
        &mut self,
        bytes: &[u8],
        n_updates: i32,
        name: String,
        user_data_fixed: bool,
        user_data_size: i32,
        flags: i32,
        variant_bit_count: bool,
    ) -> Result<(), DemoParserError> {
        let mut bitreader = Bitreader::new(bytes);
        let mut idx = -1;
        let mut keys: Vec<String> = Vec::with_capacity(32);
        let mut items = Vec::with_capacity(n_updates as usize);

        for _ in 0..n_updates {
            // Increment index
            if bitreader.read_boolean()? {
                idx += 1
            } else {
                idx += (bitreader.read_varint()? + 1) as i32
            }

            // Does the value have a key
            if !bitreader.read_boolean()? {
                continue;
            }

            // Should we refer back to history (similar to LZ77)
            let key = if bitreader.read_boolean()? {
                // How far into history we should look
                let position = bitreader.read_nbits(5)? as usize;
                // How many bytes in a row, starting from distance ago, should be copied
                let length = bitreader.read_nbits(5)? as usize;

                if position >= keys.len() {
                    bitreader.read_string()?
                } else if let Some(s) = keys.get(position) {
                    let l = length.min(s.len());
                    s[0..l].to_owned() + &bitreader.read_string()?
                } else {
                    String::new()
                }
            } else {
                bitreader.read_string()?
            };

            if keys.len() >= 32 {
                keys.remove(0);
            }
            keys.push(key.clone());

            let mut value = vec![];
            // Does the entry have a value
            if bitreader.read_boolean()? {
                let is_compressed = if !user_data_fixed && (flags & 0x1) != 0 {
                    bitreader.read_boolean()?
                } else {
                    false
                };
                let size = if user_data_fixed {
                    user_data_size as u32
                } else if variant_bit_count {
                    bitreader.read_u_bit_var()? * 8
                } else {
                    bitreader.read_nbits(17)? * 8
                };

                value = bitreader.read_n_bytes((size.checked_div(8).unwrap_or(0)) as usize)?;
                if is_compressed {
                    value = Decoder::new().decompress_vec(&value).map_err(|_| DemoParserError::MalformedMessage)?
                }
            }

            if name == "userinfo" {
                if let Ok(player) = UserInfo::from_bytes(&value) {
                    if player.steamid != 0 {
                        self.stringtable_players.insert(player.userid, player);
                    }
                }
            } else if name == "instancebaseline" {
                if let Ok(cls_id) = key.parse::<u32>() {
                    self.baselines.insert(cls_id, value.clone());
                }
            }

            items.push(StringTableEntry {
                idx,
                key,
                value,
            });
        }
        self.string_tables.push(StringTable {
            data: items,
            name,
            user_data_size,
            user_data_fixed,
            flags,
            var_bit_counts: variant_bit_count,
        });
        Ok(())
    }
}
