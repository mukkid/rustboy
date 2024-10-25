use std::{error::Error, net::AddrParseError};

fn main() {
    println!("Hello, world!");
}
#[derive(Default)]
struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16
}

#[derive(Debug)]
enum Register {
    Register8,
    Register16,
}

enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

enum Register16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP
}


struct InvalidRegisterOperation;

fn split_word(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = (word & 0xFF) as u8;
    (high, low)
}

fn join_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

impl Cpu {
    fn write8(&mut self, target: Register8, data: u8) {
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

    fn write16(&mut self, target: Register16, data: u16) {
        match target {
            Register16::AF => (self.a, self.f) = split_word(data),
            Register16::BC => (self.b, self.c) = split_word(data),
            Register16::DE => (self.d, self.e) = split_word(data),
            Register16::HL => (self.h, self.l) = split_word(data),
            Register16::PC => self.pc = data,
            Register16::SP => self.sp = data,
        }
    }

    fn read8(&self, target: Register8) -> u8 {
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

    fn read16(&self, target: Register16) -> u16 {
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

struct Memory {
    rom: [u8; 0x8000],
    vram: [u8; 0x2000],
    wram: [u8; 0x2000],
    echo_ram: [u8; 0x2000],
    oam: [u8; 0xA0],
    io: [u8; 0x80],
    hram: [u8; 0x7F],
    interrupt_enable: u8
}

#[derive(Debug)]
struct MemoryAddressError;

impl Memory {
    fn new() -> Self {
        Self {
            rom: [0; 0x8000],
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            echo_ram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: 0
        }
    }

    fn read(&self, address: u16) -> Result<u8, MemoryAddressError> {
        Ok(
            match address {
                0x0000..=0x7FFF => self.rom[address as usize],
                0x8000..=0x9FFF => self.vram[(address-0x8000) as usize],
                0xC000..=0xCFFF => self.wram[(address-0xC000) as usize],
                0xE000..=0xEFFF => self.echo_ram[(address-0xE000) as usize],
                0xFE00..=0xFE9F => self.oam[(address-0xFE00) as usize],
                0xFF00..=0xFF7F => self.io[(address-0xFF00) as usize],
                0xFF80..=0xFFFE => self.hram[(address-0xFF80) as usize],
                0xFFFF => self.interrupt_enable,
                _ => return Err(MemoryAddressError)
            }
        )
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryAddressError> {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize] = value,
            0x8000..=0x9FFF => self.vram[(address-0x8000) as usize] = value,
            0xC000..=0xCFFF => self.wram[(address-0xC000) as usize] = value,
            0xE000..=0xEFFF => self.echo_ram[(address-0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(address-0xFE00) as usize] = value,
            0xFF00..=0xFF7F => self.io[(address-0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address-0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => return Err(MemoryAddressError)
        }
        Ok(())
    }
}

enum Opcode {
    NOP,
}

struct Gameboy {
    cpu: Cpu,
    memory: Memory
}

impl Gameboy {
    fn new() -> Self{
        Self {
            cpu: Cpu::default(),
            memory: Memory::new()
        }
    }

    fn run(&mut self) {
        loop {
            self.run_single_opcode();
            todo!("timer wait accounting for clock, instruction cycles, and draw buffer");
            if self.cpu.pc > 0xFFFF { break }
        }
    }

    fn run_single_opcode(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute(opcode);
    }

    fn fetch_opcode(&self) -> Opcode {
        let byte = self.memory.read(self.cpu.pc).unwrap();
        match byte {
            0x00 => Opcode::NOP,
            _ => panic!("Unknown opcode {:#X}", byte)
        }
    }

    fn execute(&mut self, opcode: Opcode) -> i32 {
        match opcode {
            Opcode::NOP => {
                // No operation
                self.cpu.pc += 1;
                return 4
            }
            _ => panic!("Opcode not implemented")
        }
    }
}