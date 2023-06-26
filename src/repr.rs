pub struct Repr(pub u32);

impl Repr {
    #[inline]
    pub fn opcode(&self) -> u8 {
        (self.0 & 0b1111111) as u8
    }

    #[inline]
    pub fn rs1(&self) -> u8 {
        (self.0 >> 15 & 0b11111) as u8
    }

    #[inline]
    pub fn rs2(&self) -> u8 {
        (self.0 >> 20 & 0b11111) as u8
    }

    #[inline]
    pub fn rd(&self) -> u8 {
        (self.0 >> 7 & 0b11111) as u8
    }

    #[inline]
    pub fn funct3(&self) -> u8 {
        (self.0 >> 12 & 0b111) as u8
    }

    #[inline]
    pub fn funct7(&self) -> u8 {
        (self.0 >> 25 & 0b1111111) as u8
    }

    #[inline]
    // zero extends the sign bit (only for sltiu)
    pub fn imm_iu(&self) -> i16 {
        (self.0 >> 20 & 0b111111111111) as i16
    }

    #[inline]
    pub fn imm_i(&self) -> i16 {
        make_imm_signed(self.imm_iu() as u32, 12) as i16
    }

    #[inline]
    pub fn imm_s(&self) -> i16 {
        let imm = (self.0 >> 20 & 0b111111100000) | (self.0 >> 7 & 0b11111);
        make_imm_signed(imm, 12) as i16
    }

    #[inline]
    pub fn imm_u(&self) -> i32 {
        self.0 as i32 & -(1 << 12)
    }

    #[inline]
    pub fn imm_b(&self) -> i16 {
        let imm_12 = ((self.0 >> 31) as i16) << 12;
        let imm_10_5 = (self.0 >> 20 & 0b11111100000) as i16;
        let imm_4_1 = (self.0 >> 7 & 0b11110) as i16;
        let imm_11 = ((self.0 >> 7 & 0b1) as i16) << 11;
        (-imm_12) | imm_11 | imm_10_5 | imm_4_1
    }

    #[inline]
    pub fn imm_j(&self) -> i32 {
        let imm_20 = ((self.0 >> 31) as i32) << 20;
        let imm_10_1 = (self.0 >> 20 & 0b11111111110) as i32;
        let imm_11 = ((self.0 >> 20 & 1) << 11) as i32;
        let imm_19_12 = (self.0 & (0b11111111 << 12)) as i32;

        (-imm_20) | imm_19_12 | imm_11 | imm_10_1
    }
}

fn make_imm_signed(imm: u32, bits: u8) -> i32 {
    if imm & (1 << (bits - 1)) != 0 {
        imm as i32 - (1 << bits)
    } else {
        imm as i32
    }
}
