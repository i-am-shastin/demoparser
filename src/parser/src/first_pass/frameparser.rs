use crate::definitions::DemoParserError;
use crate::definitions::HEADER_ENDS_AT_BYTE;
use crate::first_pass::read_bits::read_varint;
use csgoproto::EDemoCommands;
use snap::raw::decompress_len;
use snap::raw::Decoder;
use std::sync::mpsc::Sender;

#[derive(Default)]
pub struct FrameParser {
    pub ptr: usize,
    pub fullpacket_offsets: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    pub size: usize,
    pub tick: i32,
    pub starts_at: usize,
    pub ends_at: usize,
    pub is_compressed: bool,
    pub demo_cmd: EDemoCommands,
}

impl Frame {
    #[inline(always)]
    pub fn get_bytes<'b>(&self, buf: &'b mut Vec<u8>, demo_bytes: &'b [u8]) -> Result<&'b [u8], DemoParserError> {
        let packet_bytes = demo_bytes.get(self.ends_at..self.ends_at + self.size).ok_or(DemoParserError::MalformedMessage)?;

        if !self.is_compressed {
            return Ok(packet_bytes);
        }

        let len = decompress_len(packet_bytes).map_err(|e| DemoParserError::DecompressionFailure(format!("{e}")))?;
        if buf.len() < len {
            buf.resize(len, 0)
        }

        Decoder::new().decompress(packet_bytes, buf)
            .map_err(|e| DemoParserError::DecompressionFailure(format!("{e}")))
            .map(|idx| &buf[..idx])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DemoChunk {
    pub start: usize,
    pub end: usize,
    pub end_of_demo: bool,
}

impl FrameParser {
    #[inline(always)]
    pub fn read_frame(demo_bytes: &[u8], ptr: &mut usize) -> Result<Frame, DemoParserError> {
        let starts_at = *ptr;
        let cmd = read_varint(demo_bytes, ptr)? as i32;
        let tick = read_varint(demo_bytes, ptr)? as i32;
        let size = read_varint(demo_bytes, ptr)? as usize;
        let ends_at = *ptr;

        let is_compressed = (cmd & 64) == 64;
        let demo_cmd = EDemoCommands::try_from(cmd & !64).map_err(|_| DemoParserError::UnknownDemoCmd(cmd & !64))?;

        *ptr += size;

        Ok(Frame {
            ends_at,
            size,
            tick,
            starts_at,
            is_compressed,
            demo_cmd,
        })
    }

    pub fn start(&mut self, demo_bytes: &[u8], sender: Option<&Sender<DemoChunk>>) -> Result<Vec<DemoChunk>, DemoParserError> {
        let demo_size = demo_bytes.len();

        let mut chunk_start = HEADER_ENDS_AT_BYTE;
        let mut ptr = chunk_start;
        let mut chunks = vec![];

        loop {
            let frame = FrameParser::read_frame(demo_bytes, &mut ptr)?;
            if ptr >= demo_size {
                break;
            }

            if frame.demo_cmd != EDemoCommands::DemFullPacket {
                continue;
            }

            let chunk = DemoChunk {
                start: chunk_start,
                end: frame.starts_at,
                end_of_demo: false,
            };
            chunk_start = frame.starts_at;

            let _ = sender.map(|s| s.send(chunk));
            chunks.push(chunk);
        }
        
        if let Some(offset) = chunks.last() {
            if offset.end < demo_size {
                let chunk = DemoChunk {
                    start: offset.end,
                    end: demo_size,
                    end_of_demo: false,
                };
                
                let _ = sender.map(|s| s.send(chunk));
                chunks.push(chunk);
            }
        }

        let _ = sender.map(|s| s.send({
            DemoChunk {
                start: 0,
                end: 0,
                end_of_demo: true,
            }
        }));

        Ok(chunks)
    }
}
