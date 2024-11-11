#![cfg(feature = "voice")]
use crate::definitions::DemoParserError;
use csgoproto::{CsvcMsgVoiceData, VoiceDataFormatT::*};
use opus::Decoder;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

#[allow(dead_code)]
#[derive(Debug)]
struct VoicePacket {
    pub length: u16,
    pub voice_type: u8,
}

#[allow(dead_code)]
const FRAME_SIZE: usize = 480;
const AVG_BYTES_PER_PACKET: usize = 1600;

#[allow(dead_code)]
fn parse_voice_chunk_old_format(bytes: &[u8], decoder: &mut Decoder) -> Result<Vec<i16>, DemoParserError> {
    // based on https://github.com/DandrewsDev/CS2VoiceData
    let mut decoded_bytes = vec![];
    let packet = VoicePacket {
        // sample_rate: u16::from_le_bytes(bytes[9..11].try_into().unwrap()),
        voice_type: u8::from_le_bytes([bytes[11]].try_into().unwrap()),
        length: u16::from_le_bytes(bytes[12..14].try_into().unwrap()),
    };
    if packet.voice_type == 6 {
        let mut ptr = 14;

        // read chunks until chunk_len == 65535
        while ptr < packet.length as usize {
            let output = vec![0; FRAME_SIZE];
            let chunk_len = u16::from_le_bytes(bytes[ptr..ptr + 2].try_into().unwrap());
            if chunk_len == 65535 {
                break;
            }
            ptr += 4;
            match decoder.decode(&bytes, &mut decoded_bytes, false) {
                Ok(n) => decoded_bytes.extend(&output[..n]),
                Err(_) => return Err(DemoParserError::MalformedVoicePacket),
            };
            ptr += chunk_len as usize;
        }
    }
    Ok(decoded_bytes)
}

fn parse_voice_chunk_new_format(bytes: &[u8], decoder: &mut Decoder) -> Result<Vec<i16>, DemoParserError> {
    let mut decoded_bytes = vec![0; 1024];
    let len = decoder.decode(&bytes, &mut decoded_bytes, false).map_err(|_| DemoParserError::MalformedVoicePacket)?;
    decoded_bytes.truncate(len);
    Ok(decoded_bytes)
}

fn generate_wav_header(num_channels: u16, sample_rate: u32, bits_per_sample: u16, data_size: u32) -> Vec<u8> {
    let mut header = Vec::new();
    // RIFF header
    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&((36 + data_size) as u32).to_le_bytes());
    header.extend_from_slice(b"WAVE");
    // Format chunk
    header.extend_from_slice(b"fmt ");
    header.extend_from_slice(&(16 as u32).to_le_bytes());
    header.extend_from_slice(&(1 as u16).to_le_bytes());
    header.extend_from_slice(&num_channels.to_le_bytes());
    header.extend_from_slice(&sample_rate.to_le_bytes());
    header.extend_from_slice(&(sample_rate * num_channels as u32 * bits_per_sample as u32 / 8).to_le_bytes());
    header.extend_from_slice(&(num_channels * bits_per_sample / 8).to_le_bytes());
    header.extend_from_slice(&bits_per_sample.to_le_bytes());
    // Data chunk
    header.extend_from_slice(b"data");
    header.extend_from_slice(&data_size.to_le_bytes());
    header
}

pub fn convert_voice_data_to_wav(voice_data: Vec<CsvcMsgVoiceData>) -> Result<Vec<(String, Vec<u8>)>, DemoParserError> {
    // Iterate over data grouped by steamid
    voice_data
        .par_chunk_by(|x, y| x.xuid == y.xuid)
        .map(|data| {
            let mut decoder = Decoder::new(48000, opus::Channels::Mono).map_err(|_| DemoParserError::CantCreateOpusDecoder)?;
            let mut player_voice_data = Vec::with_capacity(AVG_BYTES_PER_PACKET * data.len());

            // add voice data
            for audio in data.iter().filter_map(|c| c.audio.as_ref()) {
                if audio.format() == VoicedataFormatEngine {
                    return Err(DemoParserError::UnknownVoiceFormat)
                }
                
                player_voice_data.extend(
                    parse_voice_chunk_new_format(audio.voice_data(), &mut decoder)?
                        .iter()
                        .flat_map(|x| x.to_le_bytes())
                )
            }

            let mut wav_bytes = generate_wav_header(1, 48000, 16, player_voice_data.len() as u32);
            wav_bytes.extend(player_voice_data);
            Ok((data[0].xuid().to_string(), wav_bytes))
        })
        .collect()
}
