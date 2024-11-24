use crate::definitions::DemoParserError;
use bitter::BitReader;
use bitter::LittleEndianReader;

pub struct Bitreader<'a> {
    pub reader: LittleEndianReader<'a>,
    pub bits_left: u32,
    pub bits: u64,
    pub total_bits_left: u32,
}

pub fn read_varint(bytes: &[u8], ptr: &mut usize) -> Result<u32, DemoParserError> {
    let mut result: u32 = 0;
    for i in 0..5 {
        let byte = *bytes.get(*ptr).ok_or_else(|| DemoParserError::OutOfBytesError)? as u32;
        *ptr += 1;
        result |= (byte & 127) << (7 * i as u8);

        if byte & 0x80 == 0 {
            break;
        }
    }
    Ok(result)
}

impl<'a> Bitreader<'a> {
    pub fn new(bytes: &'a [u8]) -> Bitreader<'a> {
        Bitreader {
            reader: LittleEndianReader::new(bytes),
            bits: 0,
            bits_left: 0,
            total_bits_left: 0,
        }
    }

    #[inline(always)]
    pub fn consume(&mut self, n: u32) {
        self.bits_left -= n;
        self.bits >>= n;
        self.reader.consume(n);
    }

    #[inline(always)]
    pub fn peek(&mut self, n: u32) -> u64 {
        self.bits & ((1 << n) - 1)
    }

    #[inline(always)]
    pub fn refill(&mut self) {
        self.reader.refill_lookahead();
        let refilled = self.reader.lookahead_bits();
        if refilled > 0 {
            self.bits = self.reader.peek(refilled);
        }
        self.bits_left = refilled;
    }

    #[inline(always)]
    pub fn bits_remaining(&mut self) -> Option<usize> {
        self.reader.bits_remaining()
    }

    #[inline(always)]
    pub fn read_nbits(&mut self, n: u32) -> Result<u32, DemoParserError> {
        if self.bits_left < n {
            self.refill();
        }
        let b = self.peek(n);
        self.consume(n);
        Ok(b as u32)
    }

    #[inline(always)]
    pub fn read_u_bit_var(&mut self) -> Result<u32, DemoParserError> {
        let bits = self.read_nbits(6)?;
        match bits & 0b110000 {
            0b10000 => Ok((bits & 0b1111) | (self.read_nbits(4)? << 4)),
            0b100000 => Ok((bits & 0b1111) | (self.read_nbits(8)? << 4)),
            0b110000 => Ok((bits & 0b1111) | (self.read_nbits(28)? << 4)),
            _ => Ok(bits),
        }
    }

    #[inline(always)]
    pub fn read_varint32(&mut self) -> Result<i32, DemoParserError> {
        let x = self.read_varint()? as i32;
        let mut y = x >> 1;
        if x & 1 != 0 {
            y = !y;
        }
        Ok(y)
    }

    #[inline(always)]
    pub fn read_varint(&mut self) -> Result<u32, DemoParserError> {
        let mut result: u32 = 0;
        let mut count: i32 = 0;
        let mut b: u32;
        loop {
            if count >= 5 {
                return Ok(result);
            }
            b = self.read_nbits(8)?;
            result |= (b & 127) << (7 * count);
            count += 1;
            if b & 0x80 == 0 {
                break;
            }
        }
        Ok(result)
    }

    #[inline(always)]
    pub fn read_varint_u_64(&mut self) -> Result<u64, DemoParserError> {
        let mut result: u64 = 0;
        let mut count: i32 = 0;
        let mut b: u32;
        let mut s = 0;
        loop {
            b = self.read_nbits(8)?;
            if b < 0x80 {
                if count > 9 || count == 9 && b > 1 {
                    return Err(DemoParserError::MalformedMessage);
                }
                return Ok(result | (b as u64) << s);
            }
            result |= ((b as u64) & 127) << s;
            count += 1;
            if b & 0x80 == 0 {
                break;
            }
            s += 7;
        }
        Ok(result)
    }

    #[inline(always)]
    pub fn read_boolean(&mut self) -> Result<bool, DemoParserError> {
        Ok(self.read_nbits(1)? != 0)
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<Vec<u8>, DemoParserError> {
        let mut bytes = vec![0_u8; n];
        if self.reader.read_bytes(&mut bytes) {
            self.refill();
            Ok(bytes)
        } else {
            Err(DemoParserError::FailedByteRead(
                format!(
                    "Failed to read message/command. bytes left in stream: {}, requested bytes: {n}",
                    self.reader.bits_remaining().unwrap_or(0).checked_div(8).unwrap_or(0)
                )
            ))
        }
    }

    pub fn read_n_bytes_mut(&mut self, n: usize, buf: &mut [u8]) -> Result<(), DemoParserError> {
        if buf.len() < n {
            return Err(DemoParserError::MalformedMessage);
        }
        if self.reader.read_bytes(&mut buf[..n]) {
            self.refill();
            Ok(())
        } else {
            Err(DemoParserError::FailedByteRead(
                format!(
                    "Failed to read message/command. bytes left in stream: {}, requested bytes: {n}",
                    self.reader.bits_remaining().unwrap_or(0).checked_div(8).unwrap_or(0)
                )
            ))
        }
    }

    pub fn read_ubit_var_fp(&mut self) -> Result<u32, DemoParserError> {
        if self.read_boolean()? {
            return self.read_nbits(2);
        }
        if self.read_boolean()? {
            return self.read_nbits(4);
        }
        if self.read_boolean()? {
            return self.read_nbits(10);
        }
        if self.read_boolean()? {
            return self.read_nbits(17);
        }
        self.read_nbits(31)
    }

    #[inline(always)]
    pub fn read_bit_coord(&mut self) -> Result<f32, DemoParserError> {
        let mut int_val = 0;
        let mut frac_val = 0;
        let i2 = self.read_boolean()?;
        let f2 = self.read_boolean()?;
        if !i2 && !f2 {
            return Ok(0.0);
        }
        let is_negative = self.read_boolean()?;
        if i2 {
            int_val = self.read_nbits(14)? + 1;
        }
        if f2 {
            frac_val = self.read_nbits(5)?;
        }
        let resol: f64 = 1.0 / (1 << 5) as f64;
        let result: f32 = (int_val as f64 + (frac_val as f64 * resol)) as f32;
        if is_negative {
            Ok(-result)
        } else {
            Ok(result)
        }
    }
}
