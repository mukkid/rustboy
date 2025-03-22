mod cpu;
mod memory;

use cpu::Cpu;
use cpu::{Register8, Register16, Flag};
use cpu::Register8::*;
use cpu::Register16::*;

use memory::Memory;

fn main() {
    let mut gb = Gameboy::new();
    gb.run();
}

#[allow(non_camel_case_types)]
enum Opcode {
    NOP,
    HALT,
    LD_R_R { target: Register8, source: Register8 },
    LD_R_HL { target: Register8 },
    LD_HL_R { source: Register8 },
    INC_R16 { target: Register16 },
    DEC_R16 {target: Register16 },
    INC_R8 { target: Register8 },
    DEC_R8 { target: Register8 },
    INC_HL,
    DEC_HL,
    ADD_A_R { source: Register8 },
    ADD_A_HL,
    ADC_A_R { source: Register8 },
    ADC_A_HL,
    SUB_A_R { source: Register8 },
    SUB_A_HL,
    SBC_A_R { source: Register8 },
    SBC_A_HL,
    AND_A_R { source: Register8 },
    AND_A_HL,
    XOR_A_R { source: Register8 },
    XOR_A_HL,
    OR_A_R { source: Register8 },
    OR_A_HL,
    CP_A_R { source: Register8 },
    CP_A_HL,
    LD_R_N { target: Register8 },
    LD_R16_N { target: Register16 },
    LD_HL_N,
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
            0x01 => Opcode::LD_R16_N { target: BC },
            0x02 => todo!("LD [BC], A"),
            0x03 => Opcode::INC_R16 { target: BC },
            0x04 => Opcode::INC_R8 { target: B },
            0x05 => Opcode::DEC_R8 { target: B },
            0x06 => Opcode::LD_R_N { target: B },
            0x07 => todo!("RLCA"),
            
            0x08 => todo!("LD [a16], SP"),
            0x09 => todo!("ADD HL, BC"),
            0x0A => todo!("LD A, [BC]"),
            0x0B => Opcode::DEC_R16 { target: BC },
            0x0C => Opcode::INC_R8 { target: C },
            0x0D => Opcode::DEC_R8 { target: C },
            0x0E => Opcode::LD_R_N { target: C },
            0x0F => todo!("RRCA"),
            
            0x10 => todo!("STOP n8"),
            0x11 => Opcode::LD_R16_N { target: DE },
            0x12 => todo!("LD [DE], A"),
            0x13 => Opcode::INC_R16 { target: DE },
            0x14 => Opcode::INC_R8 { target: D },
            0x15 => Opcode::DEC_R8 { target: D },
            0x16 => Opcode::LD_R_N { target: D },
            0x17 => todo!("RLA"),
            
            0x18 => todo!("JR e8"),
            0x19 => todo!("AD HL, DE"),
            0x1A => todo!("LD A, [DE]"),
            0x1B => Opcode::DEC_R16 { target: DE },
            0x1C => Opcode::INC_R8 { target: E },
            0x1D => Opcode::DEC_R8 { target: E },
            0x1E => Opcode::LD_R_N { target: E },
            0x1F => todo!("RRA"),
            
            0x20 => todo!(),
            0x21 => Opcode::LD_R16_N { target: HL },
            0x22 => todo!(),
            0x23 => Opcode::INC_R16 { target: HL },
            0x24 => Opcode::INC_R8 { target: H },
            0x25 => Opcode::DEC_R8 { target: H },
            0x26 => Opcode::LD_R_N { target: H },
            0x27 => todo!(),
            
            0x28 => todo!(),
            0x29 => todo!(),
            0x2A => todo!(),
            0x2B => Opcode::DEC_R16 { target: HL },
            0x2C => Opcode::INC_R8 { target: L },
            0x2D => Opcode::DEC_R8 { target: L },
            0x2E => Opcode::LD_R_N { target: L },
            0x2F => todo!(),
            
            0x30 => todo!(),
            0x31 => Opcode::LD_R16_N { target: SP },
            0x32 => todo!(),
            0x33 => Opcode::INC_R16 { target: SP },
            0x34 => Opcode::INC_HL,
            0x35 => Opcode::DEC_HL,
            0x36 => Opcode::LD_HL_N,
            0x37 => todo!(),
            
            0x38 => todo!(),
            0x39 => todo!(),
            0x3A => todo!(),
            0x3B => Opcode::DEC_R16 { target: SP },
            0x3C => Opcode::INC_R8 { target: A },
            0x3D => Opcode::DEC_R8 { target: A },
            0x3E => Opcode::LD_R_N { target: A },
            0x3F => todo!(),

            0x40 => Opcode::LD_R_R { target: B, source: B },
            0x41 => Opcode::LD_R_R { target: B, source: C },
            0x42 => Opcode::LD_R_R { target: B, source: D },
            0x43 => Opcode::LD_R_R { target: B, source: E },
            0x44 => Opcode::LD_R_R { target: B, source: H },
            0x45 => Opcode::LD_R_R { target: B, source: L },
            0x46 => Opcode::LD_R_HL { target: B },
            0x47 => Opcode::LD_R_R { target: B, source: A },

            0x48 => Opcode::LD_R_R { target: C, source: B },
            0x49 => Opcode::LD_R_R { target: C, source: C },
            0x4A => Opcode::LD_R_R { target: C, source: D },
            0x4B => Opcode::LD_R_R { target: C, source: E },
            0x4C => Opcode::LD_R_R { target: C, source: H },
            0x4D => Opcode::LD_R_R { target: C, source: L },
            0x4E => Opcode::LD_R_HL { target: C },
            0x4F => Opcode::LD_R_R { target: C, source: A },

            0x50 => Opcode::LD_R_R { target: D, source: B },
            0x51 => Opcode::LD_R_R { target: D, source: C },
            0x52 => Opcode::LD_R_R { target: D, source: D },
            0x53 => Opcode::LD_R_R { target: D, source: E },
            0x54 => Opcode::LD_R_R { target: D, source: H },
            0x55 => Opcode::LD_R_R { target: D, source: L },
            0x56 => Opcode::LD_R_HL { target: D },
            0x57 => Opcode::LD_R_R { target: D, source: A },

            0x58 => Opcode::LD_R_R { target: E, source: B },
            0x59 => Opcode::LD_R_R { target: E, source: C },
            0x5A => Opcode::LD_R_R { target: E, source: D },
            0x5B => Opcode::LD_R_R { target: E, source: E },
            0x5C => Opcode::LD_R_R { target: E, source: H },
            0x5D => Opcode::LD_R_R { target: E, source: L },
            0x5E => Opcode::LD_R_HL { target: E },
            0x5F => Opcode::LD_R_R { target: E, source: A },

            0x60 => Opcode::LD_R_R { target: H, source: B },
            0x61 => Opcode::LD_R_R { target: H, source: C },
            0x62 => Opcode::LD_R_R { target: H, source: D },
            0x63 => Opcode::LD_R_R { target: H, source: E },
            0x64 => Opcode::LD_R_R { target: H, source: H },
            0x65 => Opcode::LD_R_R { target: H, source: L },
            0x66 => Opcode::LD_R_HL { target: H },
            0x67 => Opcode::LD_R_R { target: H, source: A },

            0x68 => Opcode::LD_R_R { target: L, source: B },
            0x69 => Opcode::LD_R_R { target: L, source: C },
            0x6A => Opcode::LD_R_R { target: L, source: D },
            0x6B => Opcode::LD_R_R { target: L, source: E },
            0x6C => Opcode::LD_R_R { target: L, source: H },
            0x6D => Opcode::LD_R_R { target: L, source: L },
            0x6E => Opcode::LD_R_HL { target: L },
            0x6F => Opcode::LD_R_R { target: L, source: A },

            0x70 => Opcode::LD_HL_R { source: B },
            0x71 => Opcode::LD_HL_R { source: C },
            0x72 => Opcode::LD_HL_R { source: D },
            0x73 => Opcode::LD_HL_R { source: E },
            0x74 => Opcode::LD_HL_R { source: H },
            0x75 => Opcode::LD_HL_R { source: L },
            0x76 => Opcode::HALT,
            0x77 => Opcode::LD_HL_R {source: A},

            0x78 => Opcode::LD_R_R { target: A, source: B },
            0x79 => Opcode::LD_R_R { target: A, source: C },
            0x7A => Opcode::LD_R_R { target: A, source: D },
            0x7B => Opcode::LD_R_R { target: A, source: E },
            0x7C => Opcode::LD_R_R { target: A, source: H },
            0x7D => Opcode::LD_R_R { target: A, source: L },
            0x7E => Opcode::LD_R_HL { target: A },
            0x7F => Opcode::LD_R_R { target: A, source: A },

            0x80 => Opcode::ADD_A_R { source: B },
            0x81 => Opcode::ADD_A_R { source: C },
            0x82 => Opcode::ADD_A_R { source: D },
            0x83 => Opcode::ADD_A_R { source: E },
            0x84 => Opcode::ADD_A_R { source: H },
            0x85 => Opcode::ADD_A_R { source: L },
            0x86 => Opcode::ADD_A_HL,
            0x87 => Opcode::ADD_A_R { source: A },

            0x88 => Opcode::ADC_A_R { source: B },
            0x89 => Opcode::ADC_A_R { source: C },
            0x8A => Opcode::ADC_A_R { source: D },
            0x8B => Opcode::ADC_A_R { source: E },
            0x8C => Opcode::ADC_A_R { source: H },
            0x8D => Opcode::ADC_A_R { source: L },
            0x8E => Opcode::ADC_A_HL,
            0x8F => Opcode::ADC_A_R { source: A },

            0x90 => Opcode::SUB_A_R { source: B },
            0x91 => Opcode::SUB_A_R { source: C },
            0x92 => Opcode::SUB_A_R { source: D },
            0x93 => Opcode::SUB_A_R { source: E },
            0x94 => Opcode::SUB_A_R { source: H },
            0x95 => Opcode::SUB_A_R { source: L },
            0x96 => Opcode::SUB_A_HL,
            0x97 => Opcode::SUB_A_R { source: A },

            0x98 => Opcode::SBC_A_R { source: B },
            0x99 => Opcode::SBC_A_R { source: C },
            0x9A => Opcode::SBC_A_R { source: D },
            0x9B => Opcode::SBC_A_R { source: E },
            0x9C => Opcode::SBC_A_R { source: H },
            0x9D => Opcode::SBC_A_R { source: L },
            0x9E => Opcode::SBC_A_HL,
            0x9F => Opcode::SBC_A_R { source: A },

            0xA0 => Opcode::AND_A_R { source: B },
            0xA1 => Opcode::AND_A_R { source: C },
            0xA2 => Opcode::AND_A_R { source: D },
            0xA3 => Opcode::AND_A_R { source: E },
            0xA4 => Opcode::AND_A_R { source: H },
            0xA5 => Opcode::AND_A_R { source: L },
            0xA6 => Opcode::AND_A_HL,
            0xA7 => Opcode::AND_A_R { source: A },

            0xA8 => Opcode::XOR_A_R { source: B },
            0xA9 => Opcode::XOR_A_R { source: C },
            0xAA => Opcode::XOR_A_R { source: D },
            0xAB => Opcode::XOR_A_R { source: E },
            0xAC => Opcode::XOR_A_R { source: H },
            0xAD => Opcode::XOR_A_R { source: L },
            0xAE => Opcode::XOR_A_HL,
            0xAF => Opcode::XOR_A_R { source: A },

            0xB0 => Opcode::OR_A_R { source: B },
            0xB1 => Opcode::OR_A_R { source: C },
            0xB2 => Opcode::OR_A_R { source: D },
            0xB3 => Opcode::OR_A_R { source: E },
            0xB4 => Opcode::OR_A_R { source: H },
            0xB5 => Opcode::OR_A_R { source: L },
            0xB6 => Opcode::OR_A_HL,
            0xB7 => Opcode::OR_A_R { source: A },

            0xB8 => Opcode::CP_A_R { source: B },
            0xB9 => Opcode::CP_A_R { source: C },
            0xBA => Opcode::CP_A_R { source: D },
            0xBB => Opcode::CP_A_R { source: E },
            0xBC => Opcode::CP_A_R { source: H },
            0xBD => Opcode::CP_A_R { source: L },
            0xBE => Opcode::CP_A_HL,
            0xBF => Opcode::CP_A_R { source: A },

            0xC0 => todo!(),
            0xC1 => todo!(),
            0xC2 => todo!(),
            0xC3 => todo!(),
            0xC4 => todo!(),
            0xC5 => todo!(),
            0xC6 => todo!(),
            0xC7 => todo!(),

            0xC8 => todo!(),
            0xC9 => todo!(),
            0xCA => todo!(),
            0xCB => todo!(),
            0xCC => todo!(),
            0xCD => todo!(),
            0xCE => todo!(),
            0xCF => todo!(),

            0xD0 => todo!(),
            0xD1 => todo!(),
            0xD2 => todo!(),
            0xD3 => panic!("Unknown opcode {:#X}", byte), 
            0xD4 => todo!(),
            0xD5 => todo!(),
            0xD6 => todo!(),
            0xD7 => todo!(),

            0xD8 => todo!(),
            0xD9 => todo!(),
            0xDA => todo!(),
            0xDB => panic!("Unknown opcode {:#X}", byte),
            0xDC => todo!(),
            0xDD => panic!("Unknown opcode {:#X}", byte),
            0xDE => todo!(),
            0xDF => todo!(),

            0xE0 => todo!(),
            0xE1 => todo!(),
            0xE2 => todo!(),
            0xE3 => panic!("Unknown opcode {:#X}", byte),
            0xE4 => panic!("Unknown opcode {:#X}", byte),
            0xE5 => todo!(),
            0xE6 => todo!(),
            0xE7 => todo!(),

            0xE8 => todo!(),
            0xE9 => todo!(),
            0xEA => todo!(),
            0xEB => panic!("Unknown opcode {:#X}", byte),
            0xEC => panic!("Unknown opcode {:#X}", byte),
            0xED => panic!("Unknown opcode {:#X}", byte),
            0xEE => todo!(),
            0xEF => todo!(),

            0xF0 => todo!(),
            0xF1 => todo!(),
            0xF2 => todo!(),
            0xF3 => todo!(),
            0xF4 => panic!("Unknown opcode {:#X}", byte),
            0xF5 => todo!(),
            0xF6 => todo!(),
            0xF7 => todo!(),

            0xF8 => todo!(),
            0xF9 => todo!(),
            0xFA => todo!(),
            0xFB => todo!(),
            0xFC => panic!("Unknown opcode {:#X}", byte),
            0xFD => panic!("Unknown opcode {:#X}", byte),
            0xFE => todo!(),
            0xFF => todo!(),
        }
    }

    fn execute(&mut self, opcode: Opcode) -> i32 {
        match opcode {
            Opcode::NOP => {
                self.cpu.pc += 1;
                return 4
            },
            Opcode::HALT => {
                panic!("Received HALT Opcode");
            },
            Opcode::LD_R_R {target, source} => {
                let value = self.cpu.read8(&source);
                self.cpu.write8(target, value);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::LD_R_HL { target } => {
                let value = self.memory.read(self.cpu.read16(&HL)).unwrap();
                self.cpu.write8(target, value);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::LD_HL_R {source } => {
                let value = self.cpu.read8(&source);
                self.memory.write(self.cpu.read16(&HL), value).unwrap();
                self.cpu.pc += 1;
                return 8
            },
            Opcode::ADD_A_R { source } => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.cpu.read8(&source);
                let (sum, c) = n1.overflowing_add(n2);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::ADD_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.memory.read(self.cpu.read16(&HL)).unwrap();
                let (sum, c) = n1.overflowing_add(n2);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::ADC_A_R { source } => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.cpu.read8(&source);
                let carry_flag = self.cpu.get_flag(Flag::C);
                let (partial_sum, partial_c) = n1.overflowing_add(n2);
                let (sum, c) = partial_sum.overflowing_add(carry_flag);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, n2));
                self.cpu.set_flag(Flag::C, c || partial_c);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::ADC_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.memory.read(self.cpu.read16(&HL)).unwrap();
                let carry_flag = self.cpu.get_flag(Flag::C);
                let (partial_sum, partial_c) = n1.overflowing_add(n2);
                let (sum, c) = partial_sum.overflowing_add(carry_flag);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, n2) || Cpu::has_half_carry(partial_sum, carry_flag));
                self.cpu.set_flag(Flag::C, c || partial_c);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::SUB_A_R { source } => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.cpu.read8(&source);
                let (sum, c) = n1.overflowing_sub(n2);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::SUB_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.memory.read(self.cpu.read16(&HL)).unwrap();
                let (sum, c) = n1.overflowing_sub(n2);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::SBC_A_R { source } => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.cpu.read8(&source);
                let carry_flag = self.cpu.get_flag(Flag::C);
                let (partial_sum, partial_c) = n1.overflowing_sub(n2);
                let (sum, c) = partial_sum.overflowing_sub(carry_flag);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2) || Cpu::has_half_borrow(partial_sum, carry_flag));
                self.cpu.set_flag(Flag::C, c || partial_c);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::SBC_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.memory.read(self.cpu.read16(&HL)).unwrap();
                let carry_flag = self.cpu.get_flag(Flag::C);
                let (partial_sum, partial_c) = n1.overflowing_sub(n2);
                let (sum, c) = partial_sum.overflowing_sub(carry_flag);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c || partial_c);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::AND_A_R { source } => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.cpu.read8(&source);
                let value = n1 & n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, true);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::AND_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.memory.read(self.cpu.read16(&HL)).unwrap();
                let value = n1 & n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, true);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::XOR_A_R { source } => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.cpu.read8(&source);
                let value = n1 ^ n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::XOR_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.memory.read(self.cpu.read16(&HL)).unwrap();
                let value = n1 ^ n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::OR_A_R { source } => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.cpu.read8(&source);
                let value = n1 | n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::OR_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.memory.read(self.cpu.read16(&HL)).unwrap();
                let value = n1 | n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::CP_A_R { source } => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.cpu.read8(&source);
                let (sum, c) = n1.overflowing_sub(n2);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::CP_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.memory.read(self.cpu.read16(&HL)).unwrap();
                let (sum, c) = n1.overflowing_sub(n2);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::LD_R16_N { target } => {
                let lsb = self.memory.read(self.cpu.pc+1).unwrap();
                let msb = self.memory.read(self.cpu.pc+2).unwrap();
                let value = cpu::join_bytes(msb, lsb);
                self.cpu.write16(target, value);
                self.cpu.pc += 3;
                return 12
            },
            Opcode::LD_R_N { target } => {
                let value = self.memory.read(self.cpu.pc+1).unwrap();
                self.cpu.write8(target, value);
                self.cpu.pc += 2;
                return 8
            },
            Opcode::LD_HL_N => {
                let value = self.memory.read(self.cpu.pc+1).unwrap();
                self.memory.write(self.cpu.read16(&HL), value).unwrap();
                self.cpu.pc += 2;
                return 12
            },
            Opcode::INC_R16 { target } => {
                let value = self.cpu.read16(&target).wrapping_add(1);
                self.cpu.write16(target, value);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::DEC_R16 { target } => {
                let value = self.cpu.read16(&target).wrapping_sub(1);
                self.cpu.write16(target, value);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::INC_R8 { target } => {
                let n1 = self.cpu.read8(&target);
                let value = n1.wrapping_add(1);
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, 1));
                self.cpu.pc += 1;
                return 4
            },
            Opcode::DEC_R8 { target } => {
                let n1 = self.cpu.read8(&target);
                let value = n1.wrapping_sub(1);
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, 1));
                self.cpu.pc += 1;
                return 4
            },
            Opcode::INC_HL => {
                let n1 = self.memory.read(self.cpu.read16(&HL)).unwrap();  
                let value = n1.wrapping_add(1);
                self.memory.write(self.cpu.read16(&HL), value).unwrap();

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, 1));
                self.cpu.pc += 1;
                return 12
            },
            Opcode::DEC_HL => {
                let n1 = self.memory.read(self.cpu.read16(&HL)).unwrap();  
                let value = n1.wrapping_sub(1);
                self.memory.write(self.cpu.read16(&HL), value).unwrap();

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, 1));
                self.cpu.pc += 1;
                return 12
            },
        }
    }
}
