

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
    pub sp: u16
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
    L
}
#[derive(Debug)]

pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP
}


pub struct InvalidRegisterOperation;

fn split_word(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = (word & 0xFF) as u8;
    (high, low)
}

fn join_bytes(high: u8, low: u8) -> u16 {
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
            Register8::L => self.l = data
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

    pub fn read8(&self, target: Register8) -> u8 {
        match target {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => self.f,
            Register8::H => self.h,
            Register8::L => self.l
        }
    }

    pub fn read16(&self, target: Register16) -> u16 {
        match target {
            Register16::AF => join_bytes(self.a, self.f),
            Register16::BC => join_bytes(self.b, self.c),
            Register16::DE => join_bytes(self.d, self.e),
            Register16::HL => join_bytes(self.h, self.l),
            Register16::PC => self.pc,
            Register16::SP => self.sp
        }
    }
}