#[derive(Default)]
pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub ime: bool,
}

#[derive(Debug)]
pub enum Register {
    Register8(Register8),
    Register16(Register16),
}

#[derive(Debug)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

#[derive(Debug)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub struct InvalidRegisterOperation;

pub fn split_word(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = (word & 0xFF) as u8;
    (high, low)
}

pub fn join_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

impl Cpu {
    pub fn write8(&mut self, target: Register8, data: u8) {
        match target {
            Register8::A => self.a = data,
            Register8::B => self.b = data,
            Register8::C => self.c = data,
            Register8::D => self.d = data,
            Register8::E => self.e = data,
            Register8::F => self.f = data,
            Register8::H => self.h = data,
            Register8::L => self.l = data,
        }
    }

    pub fn write16(&mut self, target: Register16, data: u16) {
        match target {
            Register16::AF => (self.a, self.f) = split_word(data),
            Register16::BC => (self.b, self.c) = split_word(data),
            Register16::DE => (self.d, self.e) = split_word(data),
            Register16::HL => (self.h, self.l) = split_word(data),
            Register16::PC => self.pc = data,
            Register16::SP => self.sp = data,
        }
    }

    pub fn read8(&self, target: &Register8) -> u8 {
        match target {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => self.f,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    pub fn read16(&self, target: &Register16) -> u16 {
        match target {
            Register16::AF => join_bytes(self.a, self.f),
            Register16::BC => join_bytes(self.b, self.c),
            Register16::DE => join_bytes(self.d, self.e),
            Register16::HL => join_bytes(self.h, self.l),
            Register16::PC => self.pc,
            Register16::SP => self.sp,
        }
    }

    pub fn has_half_carry(a: u8, b: u8) -> bool {
        ((a & 0xF) + (b & 0xF)) > 0xF
    }

    pub fn has_half_borrow(a: u8, b: u8) -> bool {
        (a & 0xF) < (b & 0xF)
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        match flag {
            Flag::Z => {
                if value {
                    self.f |= 0b1000_0000
                } else {
                    self.f &= 0b0111_1111
                }
            }
            Flag::N => {
                if value {
                    self.f |= 0b0100_0000
                } else {
                    self.f &= 0b1011_1111
                }
            }
            Flag::H => {
                if value {
                    self.f |= 0b0010_0000
                } else {
                    self.f &= 0b1101_1111
                }
            }
            Flag::C => {
                if value {
                    self.f |= 0b0001_0000
                } else {
                    self.f &= 0b1110_1111
                }
            }
        }
    }

    pub fn get_flag(&self, flag: Flag) -> u8 {
        match flag {
            Flag::Z => (self.f & 0b1000_0000) >> 7,
            Flag::N => (self.f & 0b0100_0000) >> 6,
            Flag::H => (self.f & 0b0010_0000) >> 5,
            Flag::C => (self.f & 0b0001_0000) >> 4,
        }
    }
}
