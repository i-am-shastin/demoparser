use crate::definitions::DemoParserError;
use crate::first_pass::read_bits::Bitreader;

#[derive(Clone, Copy, Debug)]
pub struct FieldPath {
    pub path: [i32; 7],
    pub last: usize,
}

impl Default for FieldPath {
    fn default() -> FieldPath {
        FieldPath {
            path: [-1, 0, 0, 0, 0, 0, 0],
            last: 0,
        }
    }
}

impl FieldPath {
    #[inline(always)]
    pub fn pop_special(&mut self, n: usize) -> Result<(), DemoParserError> {
        for _ in 0..n {
            *self.get_entry_mut(self.last)? = 0;
            self.last -= 1;
        }
        Ok(())
    }

    #[inline(always)]
    pub fn get_entry_mut(&mut self, idx: usize) -> Result<&mut i32, DemoParserError> {
        self.path.get_mut(idx).ok_or_else(|| DemoParserError::IllegalPathOp)
    }

    #[inline(always)]
    pub fn do_op(&mut self, opcode: u8, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        // taken directly from here: https://github.com/dotabuff/manta/blob/master/field_path.go
        // Not going to act like I know why exactly these ops are selected, I supposed they provide
        // somewhat good compression.
        match opcode {
            0 => self.plus_one(),
            1 => self.plus_two(),
            2 => self.plus_three(),
            3 => self.plus_four(),
            4 => self.plus_n(bitreader),
            5 => self.push_one_left_delta_zero_right_zero(),
            6 => self.push_one_left_delta_zero_right_non_zero(bitreader),
            7 => self.push_one_left_delta_one_right_zero(),
            8 => self.push_one_left_delta_one_right_non_zero(bitreader),
            9 => self.push_one_left_delta_n_right_zero(bitreader),
            10 => self.push_one_left_delta_n_right_non_zero(bitreader),
            11 => self.push_one_left_delta_n_right_non_zero_pack6_bits(bitreader),
            12 => self.push_one_left_delta_n_right_non_zero_pack8_bits(bitreader),
            13 => self.push_two_left_delta_zero(bitreader),
            14 => self.push_two_pack5_left_delta_zero(bitreader),
            15 => self.push_three_left_delta_zero(bitreader),
            16 => self.push_three_pack5_left_delta_zero(bitreader),
            17 => self.push_two_left_delta_one(bitreader),
            18 => self.push_two_pack5_left_delta_one(bitreader),
            19 => self.push_three_left_delta_one(bitreader),
            20 => self.push_three_pack5_left_delta_one(bitreader),
            21 => self.push_two_left_delta_n(bitreader),
            22 => self.push_two_pack5_left_delta_n(bitreader),
            23 => self.push_three_left_delta_n(bitreader),
            24 => self.push_three_pack5_left_delta_n(bitreader),
            25 => self.push_n(bitreader),
            26 => self.push_n_and_non_topological(bitreader),
            27 => self.pop_one_plus_one(),
            28 => self.pop_one_plus_n(bitreader),
            29 => self.pop_all_but_one_plus_one(),
            30 => self.pop_all_but_one_plus_n(bitreader),
            31 => self.pop_all_but_one_plus_n_pack3_bits(bitreader),
            32 => self.pop_all_but_one_plus_n_pack6_bits(bitreader),
            33 => self.pop_n_plus_one(bitreader),
            34 => self.pop_n_plus_n(bitreader),
            35 => self.pop_n_and_non_topographical(bitreader),
            36 => self.non_topo_complex(bitreader),
            37 => self.non_topo_penultimate_plus_one(),
            38 => self.non_topo_complex_pack4_bits(bitreader),
            _ => Err(DemoParserError::UnknownPathOp),
        }
    }

    fn plus_one(&mut self) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 1;
        Ok(())
    }
    
    fn plus_two(&mut self) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 2;
        Ok(())
    }
    
    fn plus_three(&mut self) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 3;
        Ok(())
    }
    
    fn plus_four(&mut self) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 4;
        Ok(())
    }
    
    fn plus_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32 + 5;
        Ok(())
    }
    
    fn push_one_left_delta_zero_right_zero(&mut self) -> Result<(), DemoParserError> {
        self.last += 1;
        *self.get_entry_mut(self.last)? = 0;
        Ok(())
    }
    
    fn push_one_left_delta_zero_right_non_zero(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        Ok(())
    }
    
    fn push_one_left_delta_one_right_zero(&mut self) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 1;
        self.last += 1;
        *self.get_entry_mut(self.last)? = 0;
        Ok(())
    }
    
    fn push_one_left_delta_one_right_non_zero(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 1;
        self.last += 1;
        *self.get_entry_mut(self.last)? = bitreader.read_ubit_var_fp()? as i32;
        Ok(())
    }
    
    fn push_one_left_delta_n_right_zero(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? = 0;
        Ok(())
    }
    
    fn push_one_left_delta_n_right_non_zero(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32 + 2;
        self.last += 1;
        *self.get_entry_mut(self.last)? = bitreader.read_ubit_var_fp()? as i32 + 1;
        Ok(())
    }
    
    fn push_one_left_delta_n_right_non_zero_pack6_bits(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += (bitreader.read_nbits(3)? + 2) as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? = (bitreader.read_nbits(3)? + 1) as i32;
        Ok(())
    }
    
    fn push_one_left_delta_n_right_non_zero_pack8_bits(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += (bitreader.read_nbits(4)? + 2) as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? = (bitreader.read_nbits(4)? + 1) as i32;
        Ok(())
    }
    
    fn push_two_left_delta_zero(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        Ok(())
    }
    
    fn push_two_pack5_left_delta_zero(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.last += 1;
        *self.get_entry_mut(self.last)? = bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? = bitreader.read_nbits(5)? as i32;
        Ok(())
    }
    
    fn push_three_left_delta_zero(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        Ok(())
    }
    
    fn push_three_pack5_left_delta_zero(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.last += 1;
        *self.get_entry_mut(self.last)? = bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? = bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? = bitreader.read_nbits(5)? as i32;
        Ok(())
    }
    
    fn push_two_left_delta_one(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 1;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        Ok(())
    }
    
    fn push_two_pack5_left_delta_one(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 1;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        Ok(())
    }
    
    fn push_three_left_delta_one(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 1;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        Ok(())
    }
    
    fn push_three_pack5_left_delta_one(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += 1;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        Ok(())
    }
    
    fn push_two_left_delta_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += (bitreader.read_u_bit_var()? + 2) as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        Ok(())
    }
    
    fn push_two_pack5_left_delta_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += (bitreader.read_u_bit_var()? + 2) as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        Ok(())
    }
    
    fn push_three_left_delta_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += (bitreader.read_u_bit_var()? + 2) as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        Ok(())
    }
    
    fn push_three_pack5_left_delta_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last)? += (bitreader.read_u_bit_var()? + 2) as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        self.last += 1;
        *self.get_entry_mut(self.last)? += bitreader.read_nbits(5)? as i32;
        Ok(())
    }
    
    fn push_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        let n = bitreader.read_u_bit_var()? as i32;
        *self.get_entry_mut(self.last)? += bitreader.read_u_bit_var()? as i32;
        for _ in 0..n {
            self.last += 1;
            *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32;
        }
        Ok(())
    }
    
    fn push_n_and_non_topological(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        for i in 0..=self.last {
            if bitreader.read_boolean()? {
                *self.get_entry_mut(i)? += bitreader.read_varint32()? + 1;
            }
        }
        let count = bitreader.read_u_bit_var()?;
        for _ in 0..count {
            self.last += 1;
            *self.get_entry_mut(self.last)? = bitreader.read_ubit_var_fp()? as i32;
        }
        Ok(())
    }
    
    fn pop_one_plus_one(&mut self) -> Result<(), DemoParserError> {
        self.pop_special(1)?;
        *self.get_entry_mut(self.last)? += 1;
        Ok(())
    }
    
    fn pop_one_plus_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.pop_special(1)?;
        *self.get_entry_mut(self.last)? += bitreader.read_ubit_var_fp()? as i32 + 1;
        Ok(())
    }
    
    fn pop_all_but_one_plus_one(&mut self) -> Result<(), DemoParserError> {
        self.pop_special(self.last)?;
        *self.get_entry_mut(0)? += 1;
        Ok(())
    }
    
    fn pop_all_but_one_plus_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.pop_special(self.last)?;
        *self.get_entry_mut(0)? += bitreader.read_ubit_var_fp()? as i32 + 1;
        Ok(())
    }
    
    fn pop_all_but_one_plus_n_pack3_bits(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.pop_special(self.last)?;
        *self.get_entry_mut(0)? += bitreader.read_nbits(3)? as i32 + 1;
        Ok(())
    }
    
    fn pop_all_but_one_plus_n_pack6_bits(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.pop_special(self.last)?;
        *self.get_entry_mut(0)? += bitreader.read_nbits(6)? as i32 + 1;
        Ok(())
    }
    
    fn pop_n_plus_one(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.pop_special(bitreader.read_ubit_var_fp()? as usize)?;
        *self.get_entry_mut(self.last)? += 1;
        Ok(())
    }
    
    fn pop_n_plus_n(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.pop_special(bitreader.read_ubit_var_fp()? as usize)?;
        *self.get_entry_mut(self.last)? += bitreader.read_varint32()?;
        Ok(())
    }
    
    fn pop_n_and_non_topographical(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        self.pop_special(bitreader.read_ubit_var_fp()? as usize)?;
        for i in 0..=self.last {
            if bitreader.read_boolean()? {
                *self.get_entry_mut(i)? += bitreader.read_varint32()?;
            }
        }
        Ok(())
    }
    
    fn non_topo_complex(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        for i in 0..=self.last {
            if bitreader.read_boolean()? {
                *self.get_entry_mut(i)? += bitreader.read_varint32()?;
            }
        }
        Ok(())
    }
    
    fn non_topo_penultimate_plus_one(&mut self) -> Result<(), DemoParserError> {
        *self.get_entry_mut(self.last - 1)? += 1;
        Ok(())
    }
    
    fn non_topo_complex_pack4_bits(&mut self, bitreader: &mut Bitreader) -> Result<(), DemoParserError> {
        for i in 0..=self.last {
            if bitreader.read_boolean()? {
                *self.get_entry_mut(i)? += bitreader.read_nbits(4)? as i32 - 7;
            }
        }
        Ok(())
    }
}

/*
Huffman tree is this:

value, weight, len(prefix), prefix
0	36271	2	0
39	25474	3	10
8	2942	6	11000
2	1375	7	110010
29	1837	7	110011
4	4128	6	11010
30	149	    10	110110000
38	99	    11	1101100010
35	1	    17	1101100011000000
34	1	    17	1101100011000001
27	2	    16	110110001100001
25	1	    17	1101100011000100
24	1	    17	1101100011000101
33	1	    17	1101100011000110
28	1	    17	1101100011000111
13	1	    17	1101100011001000
15	1	    18	11011000110010010
14	1	    18	11011000110010011
6	3	    16	110110001100101
21	1	    18	11011000110011000
20	1	    18	11011000110011001
23	1	    18	11011000110011010
22	1	    18	11011000110011011
17	1	    18	11011000110011100
16	1	    18	11011000110011101
19	1	    18	11011000110011110
18	1	    18	11011000110011111
5	35	    13	110110001101
36	76	    12	11011000111
10	471	    9	11011001
7	521	    9	11011010
12	251	    10	110110110
37	271	    10	110110111
9	560	    9	11011100
31	300	    10	110111010
26	310	    10	110111011
32	634	    9	11011110
3	646	    9	11011111
1	10334	5	1110
11	10530	5	1111
*/
