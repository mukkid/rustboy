mod cpu;
mod gpu;
mod memory;
mod screen;

use cpu::Cpu;
use cpu::{Register8, Register16, Flag};
use cpu::Register8::*;
use cpu::Register16::*;
use gpu::Gpu;
use memory::Memory;

use thiserror::Error;
use anyhow::Result;

fn main() {
    todo!("load ROM");
    let mut gb = Gameboy {
        cpu: Cpu::default(),
        memory: Memory::new(),
        gpu: Gpu::new(),
    };
    gb.gpu.assemble_tiles();
    screen::render();
    let _ = gb.run();
}

#[derive(Debug, Error)]
#[error("Invalid memory reached")]
pub struct MemoryAddressError;


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
    SET_u3_R8 { bit: u8, target: Register8 },
    SET_u3_HL { bit: u8 },
    RES_u3_R8 { bit: u8, target: Register8 },
    RES_u3_HL { bit: u8 },
    BIT_u3_R8 { bit: u8, target: Register8 },
    BIT_u3_HL { bit: u8 },
    SWAP_R8 { target: Register8 },
    SWAP_HL,
    RLC_R8 { target: Register8 },
    RLC_HL,
    RRC_R8 { target: Register8 },
    RRC_HL,
    RL_R8 { target: Register8 },
    RL_HL,
    RR_R8 { target: Register8 },
    RR_HL,
    SLA_R8 { target: Register8 },
    SLA_HL,
    SRA_R8 { target: Register8 },
    SRA_HL,
    SRL_R8 { target: Register8 },
    SRL_HL,
    POP_R16 { target: Register16 },
    PUSH_R16 { target: Register16 },
    RET,
    RET_cc { condition: Flag, set: bool },
    RST { vct: u16 },
    CALL_n16,
    CALL_cc_n16 { condition: Flag, set: bool },
    JP_n16,
    JP_cc_n16 { condition: Flag, set: bool },
    JP_HL,
    ADD_A_n8,
    SUB_A_n8,
    AND_A_n8,
    OR_A_n8,
    ADC_A_n8,
    SBC_A_n8,
    XOR_A_n8,
    CP_A_n8,
    JR_e8,
    JR_cc_e8 { condition: Flag, set: bool },
    LD_R16_A { target: Register16 },
    LD_HLID_A { inc: bool }, // Combine HLI and HLD. if `inc` is true, run HLI. HLD otherwise
    ADD_HL_R16 { target: Register16 },
    ADD_HL_SP,
    LD_A_R16 { target: Register16 },
    LD_A_HLID { inc: bool },
    LD_a16_SP,
    LDH_a8_A,
    LDH_c_A,
    LDH_A_a8,
    LDH_A_c,
    LD_a16_A,
    LD_A_a16,
    ADD_SP_e8,
    LD_HL_SPe8,
    LD_SP_HL,
    RLCA,
    RRCA,
    RLA,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    RETI,
    DI,
    EI,
}

struct Gameboy {
    cpu: Cpu,
    memory: Memory,
    gpu: Gpu,
}

impl Gameboy {

    fn push(&mut self, value: u8) -> Result<()> {
        self.cpu.sp -= 1;
        self.memory.write(self.cpu.sp, value)?;
        Ok(())
    }

    fn pop(&mut self) -> Result<u8> {
        let value = self.memory.read(self.cpu.sp)?;
        self.cpu.sp += 1;
        Ok(value)
    }

    fn read(&self, address: u16) -> Result<u8, MemoryAddressError> {
        match address {
            0x0000..=0x7FFF => self.memory.read(address),
            0x8000..=0x9FFF => self.gpu.read(address),
            0xC000..=0xCFFF => self.memory.read(address),
            0xE000..=0xFDFF => self.memory.read(address),
            0xFE00..=0xFE9F => self.gpu.read(address),
            0xFF80..=0xFFFE => self.memory.read(address),
            _ => Err(MemoryAddressError)
        }
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryAddressError> {
        match address {
            0x0000..=0x7FFF => self.memory.write(address, value),
            0x8000..=0x9FFF => self.gpu.write(address, value),
            0xC000..=0xCFFF => self.memory.write(address, value),
            0xE000..=0xFDFF => self.memory.write(address, value),
            0xFE00..=0xFE9F => self.gpu.write(address, value),
            0xFF80..=0xFFFE => self.memory.write(address, value),
            _ => Err(MemoryAddressError)
        }
    }

    fn run(&mut self) -> Result<()> {
        loop {
            let cycles_elapsed = self.run_single_opcode()?;
            self.gpu.step(cycles_elapsed);
            todo!("timer wait accounting for clock, instruction cycles, and draw buffer");
        }
    }

    fn run_single_opcode(&mut self) -> Result<i32> {
        let opcode = self.fetch_opcode();
        self.execute(opcode)
    }

    fn fetch_opcode(&self) -> Opcode {
        let byte = self.read(self.cpu.pc).unwrap();
        match byte {
            0x00 => Opcode::NOP,
            0x01 => Opcode::LD_R16_N { target: BC },
            0x02 => Opcode::LD_R16_A { target: BC },
            0x03 => Opcode::INC_R16 { target: BC },
            0x04 => Opcode::INC_R8 { target: B },
            0x05 => Opcode::DEC_R8 { target: B },
            0x06 => Opcode::LD_R_N { target: B },
            0x07 => Opcode::RLCA,
            
            0x08 => Opcode::LD_a16_SP,
            0x09 => Opcode::ADD_HL_R16 { target: BC },
            0x0A => Opcode::LD_A_R16 { target: BC },
            0x0B => Opcode::DEC_R16 { target: BC },
            0x0C => Opcode::INC_R8 { target: C },
            0x0D => Opcode::DEC_R8 { target: C },
            0x0E => Opcode::LD_R_N { target: C },
            0x0F => Opcode::RRCA,
            
            0x10 => panic!("I stopped"), // todo!("STOP n8")
            0x11 => Opcode::LD_R16_N { target: DE },
            0x12 => Opcode::LD_R16_A { target: DE },
            0x13 => Opcode::INC_R16 { target: DE },
            0x14 => Opcode::INC_R8 { target: D },
            0x15 => Opcode::DEC_R8 { target: D },
            0x16 => Opcode::LD_R_N { target: D },
            0x17 => Opcode::RLA,
            
            0x18 => Opcode::JR_e8,
            0x19 => Opcode::ADD_HL_R16 { target: DE },
            0x1A => Opcode::LD_A_R16 { target: DE },
            0x1B => Opcode::DEC_R16 { target: DE },
            0x1C => Opcode::INC_R8 { target: E },
            0x1D => Opcode::DEC_R8 { target: E },
            0x1E => Opcode::LD_R_N { target: E },
            0x1F => Opcode::RRA,
            
            0x20 => Opcode::JR_cc_e8 { condition: Flag::Z, set: false },
            0x21 => Opcode::LD_R16_N { target: HL },
            0x22 => Opcode::LD_HLID_A { inc: true },
            0x23 => Opcode::INC_R16 { target: HL },
            0x24 => Opcode::INC_R8 { target: H },
            0x25 => Opcode::DEC_R8 { target: H },
            0x26 => Opcode::LD_R_N { target: H },
            0x27 => Opcode::DAA,
            
            0x28 => Opcode::JR_cc_e8 { condition: Flag::Z, set: true },
            0x29 => Opcode::ADD_HL_R16 { target: DE },
            0x2A => Opcode::LD_A_HLID { inc: true },
            0x2B => Opcode::DEC_R16 { target: HL },
            0x2C => Opcode::INC_R8 { target: L },
            0x2D => Opcode::DEC_R8 { target: L },
            0x2E => Opcode::LD_R_N { target: L },
            0x2F => Opcode::CPL,
            
            0x30 => Opcode::JR_cc_e8 { condition: Flag::C, set: false },
            0x31 => Opcode::LD_R16_N { target: SP },
            0x32 => Opcode::LD_HLID_A { inc: false },
            0x33 => Opcode::INC_R16 { target: SP },
            0x34 => Opcode::INC_HL,
            0x35 => Opcode::DEC_HL,
            0x36 => Opcode::LD_HL_N,
            0x37 => Opcode::SCF,
            
            0x38 => Opcode::JR_cc_e8 { condition: Flag::C, set: true },
            0x39 => Opcode::ADD_HL_SP,
            0x3A => Opcode::LD_A_HLID { inc: false },
            0x3B => Opcode::DEC_R16 { target: SP },
            0x3C => Opcode::INC_R8 { target: A },
            0x3D => Opcode::DEC_R8 { target: A },
            0x3E => Opcode::LD_R_N { target: A },
            0x3F => Opcode::CCF,

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

            0xC0 => Opcode::RET_cc { condition: Flag::Z, set: false },
            0xC1 => Opcode::POP_R16 { target: BC },
            0xC2 => Opcode::JP_cc_n16 { condition: Flag::Z, set: false },
            0xC3 => Opcode::JP_n16,
            0xC4 => Opcode::CALL_cc_n16 { condition: Flag::Z, set: false },
            0xC5 => Opcode::PUSH_R16 { target: BC },
            0xC6 => Opcode::ADD_A_n8,
            0xC7 => Opcode::RST { vct: 0x00 },

            0xC8 => Opcode::RET_cc { condition: Flag::Z, set: true },
            0xC9 => Opcode::RET,
            0xCA => Opcode::JP_cc_n16 { condition: Flag::Z, set: true },
            0xCB => self.fetch_prefixed_opcode(),
            0xCC => Opcode::CALL_cc_n16 { condition: Flag::Z, set: true },
            0xCD => Opcode::CALL_n16,
            0xCE => Opcode::ADC_A_n8,
            0xCF => Opcode::RST { vct: 0x08 },

            0xD0 => Opcode::RET_cc { condition: Flag::C, set: false },
            0xD1 => Opcode::POP_R16 { target: DE },
            0xD2 => Opcode::JP_cc_n16 { condition: Flag::C, set: false },
            0xD3 => panic!("Unknown opcode {:#X}", byte), 
            0xD4 => Opcode::CALL_cc_n16 { condition: Flag::C, set: false },
            0xD5 => Opcode::PUSH_R16 { target: DE },
            0xD6 => Opcode::SUB_A_n8,
            0xD7 => Opcode::RST { vct: 0x10 },

            0xD8 => Opcode::RET_cc { condition: Flag::C, set: true },
            0xD9 => Opcode::RETI,
            0xDA => Opcode::JP_cc_n16 { condition: Flag::C, set: true },
            0xDB => panic!("Unknown opcode {:#X}", byte),
            0xDC => Opcode::CALL_cc_n16 { condition: Flag::C, set: true },
            0xDD => panic!("Unknown opcode {:#X}", byte),
            0xDE => Opcode::SBC_A_n8,
            0xDF => Opcode::RST { vct: 0x18 },

            0xE0 => Opcode::LDH_a8_A,
            0xE1 => Opcode::POP_R16 { target: HL },
            0xE2 => Opcode::LDH_c_A,
            0xE3 => panic!("Unknown opcode {:#X}", byte),
            0xE4 => panic!("Unknown opcode {:#X}", byte),
            0xE5 => Opcode::PUSH_R16 { target: HL },
            0xE6 => Opcode::AND_A_n8,
            0xE7 => Opcode::RST { vct: 0x20 },

            0xE8 => Opcode::ADD_SP_e8,
            0xE9 => Opcode::JP_HL,
            0xEA => Opcode::LD_a16_A,
            0xEB => panic!("Unknown opcode {:#X}", byte),
            0xEC => panic!("Unknown opcode {:#X}", byte),
            0xED => panic!("Unknown opcode {:#X}", byte),
            0xEE => Opcode::XOR_A_n8,
            0xEF => Opcode::RST { vct: 0x28 },

            0xF0 => Opcode::LDH_A_a8,
            0xF1 => Opcode::POP_R16 { target: AF },
            0xF2 => Opcode::LDH_A_c,
            0xF3 => Opcode::DI,
            0xF4 => panic!("Unknown opcode {:#X}", byte),
            0xF5 => Opcode::PUSH_R16 { target: AF },
            0xF6 => Opcode::OR_A_n8,
            0xF7 => Opcode::RST { vct: 0x30 },

            0xF8 => Opcode::LD_HL_SPe8,
            0xF9 => Opcode::LD_SP_HL,
            0xFA => Opcode::LD_A_a16,
            0xFB => Opcode::EI,
            0xFC => panic!("Unknown opcode {:#X}", byte),
            0xFD => panic!("Unknown opcode {:#X}", byte),
            0xFE => Opcode::CP_A_n8,
            0xFF => Opcode::RST { vct: 0x38 },
        }
    }

    fn fetch_prefixed_opcode(&self) -> Opcode {
        let byte = self.read(self.cpu.pc + 1).unwrap();
        match byte {
            0x00 => Opcode::RLC_R8 { target: B },
            0x01 => Opcode::RLC_R8 { target: C },
            0x02 => Opcode::RLC_R8 { target: D },
            0x03 => Opcode::RLC_R8 { target: E },
            0x04 => Opcode::RLC_R8 { target: H },
            0x05 => Opcode::RLC_R8 { target: L },
            0x06 => Opcode::RLC_HL,
            0x07 => Opcode::RLC_R8 { target: A },

            0x08 => Opcode::RRC_R8 { target: B },
            0x09 => Opcode::RRC_R8 { target: C },
            0x0A => Opcode::RRC_R8 { target: D },
            0x0B => Opcode::RRC_R8 { target: E },
            0x0C => Opcode::RRC_R8 { target: H },
            0x0D => Opcode::RRC_R8 { target: L },
            0x0E => Opcode::RRC_HL,
            0x0F => Opcode::RRC_R8 { target: A },

            0x10 => Opcode::RL_R8 { target: B },
            0x11 => Opcode::RL_R8 { target: C },
            0x12 => Opcode::RL_R8 { target: D },
            0x13 => Opcode::RL_R8 { target: E },
            0x14 => Opcode::RL_R8 { target: H },
            0x15 => Opcode::RL_R8 { target: L },
            0x16 => Opcode::RL_HL,
            0x17 => Opcode::RL_R8 { target: A },

            0x18 => Opcode::RR_R8 { target: B },
            0x19 => Opcode::RR_R8 { target: C },
            0x1A => Opcode::RR_R8 { target: D },
            0x1B => Opcode::RR_R8 { target: E },
            0x1C => Opcode::RR_R8 { target: H },
            0x1D => Opcode::RR_R8 { target: L },
            0x1E => Opcode::RR_HL,
            0x1F => Opcode::RR_R8 { target: A },

            0x20 => Opcode::SLA_R8 { target: B },
            0x21 => Opcode::SLA_R8 { target: C },
            0x22 => Opcode::SLA_R8 { target: D },
            0x23 => Opcode::SLA_R8 { target: E },
            0x24 => Opcode::SLA_R8 { target: H },
            0x25 => Opcode::SLA_R8 { target: L },
            0x26 => Opcode::SLA_HL,
            0x27 => Opcode::SLA_R8 { target: A },

            0x28 => Opcode::SRA_R8 { target: B },
            0x29 => Opcode::SRA_R8 { target: C },
            0x2A => Opcode::SRA_R8 { target: D },
            0x2B => Opcode::SRA_R8 { target: E },
            0x2C => Opcode::SRA_R8 { target: H },
            0x2D => Opcode::SRA_R8 { target: L },
            0x2E => Opcode::SRA_HL,
            0x2F => Opcode::SRA_R8 { target: A },

            0x30 => Opcode::SWAP_R8 { target: B },
            0x31 => Opcode::SWAP_R8 { target: C },
            0x32 => Opcode::SWAP_R8 { target: D },
            0x33 => Opcode::SWAP_R8 { target: E },
            0x34 => Opcode::SWAP_R8 { target: H },
            0x35 => Opcode::SWAP_R8 { target: L },
            0x36 => Opcode::SWAP_HL,
            0x37 => Opcode::SWAP_R8 { target: A },

            0x38 => Opcode::SRL_R8 { target: B },
            0x39 => Opcode::SRL_R8 { target: C },
            0x3A => Opcode::SRL_R8 { target: D },
            0x3B => Opcode::SRL_R8 { target: E },
            0x3C => Opcode::SRL_R8 { target: H },
            0x3D => Opcode::SRL_R8 { target: L },
            0x3E => Opcode::SRL_HL,
            0x3F => Opcode::SRL_R8 { target: A },

            0x40 => Opcode::BIT_u3_R8 { bit: 0, target: B },
            0x41 => Opcode::BIT_u3_R8 { bit: 0, target: C },
            0x42 => Opcode::BIT_u3_R8 { bit: 0, target: D },
            0x43 => Opcode::BIT_u3_R8 { bit: 0, target: E },
            0x44 => Opcode::BIT_u3_R8 { bit: 0, target: H },
            0x45 => Opcode::BIT_u3_R8 { bit: 0, target: L },
            0x46 => Opcode::BIT_u3_HL { bit: 0 },
            0x47 => Opcode::BIT_u3_R8 { bit: 0, target: A },
            
            0x48 => Opcode::BIT_u3_R8 { bit: 1, target: B },
            0x49 => Opcode::BIT_u3_R8 { bit: 1, target: C },
            0x4A => Opcode::BIT_u3_R8 { bit: 1, target: D },
            0x4B => Opcode::BIT_u3_R8 { bit: 1, target: E },
            0x4C => Opcode::BIT_u3_R8 { bit: 1, target: H },
            0x4D => Opcode::BIT_u3_R8 { bit: 1, target: L },
            0x4E => Opcode::BIT_u3_HL { bit: 1 },
            0x4F => Opcode::BIT_u3_R8 { bit: 1, target: A },
            
            0x50 => Opcode::BIT_u3_R8 { bit: 2, target: B },
            0x51 => Opcode::BIT_u3_R8 { bit: 2, target: C },
            0x52 => Opcode::BIT_u3_R8 { bit: 2, target: D },
            0x53 => Opcode::BIT_u3_R8 { bit: 2, target: E },
            0x54 => Opcode::BIT_u3_R8 { bit: 2, target: H },
            0x55 => Opcode::BIT_u3_R8 { bit: 2, target: L },
            0x56 => Opcode::BIT_u3_HL { bit: 2 },
            0x57 => Opcode::BIT_u3_R8 { bit: 2, target: A },
            
            0x58 => Opcode::BIT_u3_R8 { bit: 3, target: B },
            0x59 => Opcode::BIT_u3_R8 { bit: 3, target: C },
            0x5A => Opcode::BIT_u3_R8 { bit: 3, target: D },
            0x5B => Opcode::BIT_u3_R8 { bit: 3, target: E },
            0x5C => Opcode::BIT_u3_R8 { bit: 3, target: H },
            0x5D => Opcode::BIT_u3_R8 { bit: 3, target: L },
            0x5E => Opcode::BIT_u3_HL { bit: 3 },
            0x5F => Opcode::BIT_u3_R8 { bit: 3, target: A },
            
            0x60 => Opcode::BIT_u3_R8 { bit: 4, target: B },
            0x61 => Opcode::BIT_u3_R8 { bit: 4, target: C },
            0x62 => Opcode::BIT_u3_R8 { bit: 4, target: D },
            0x63 => Opcode::BIT_u3_R8 { bit: 4, target: E },
            0x64 => Opcode::BIT_u3_R8 { bit: 4, target: H },
            0x65 => Opcode::BIT_u3_R8 { bit: 4, target: L },
            0x66 => Opcode::BIT_u3_HL { bit: 4 },
            0x67 => Opcode::BIT_u3_R8 { bit: 4, target: A },
            
            0x68 => Opcode::BIT_u3_R8 { bit: 5, target: B },
            0x69 => Opcode::BIT_u3_R8 { bit: 5, target: C },
            0x6A => Opcode::BIT_u3_R8 { bit: 5, target: D },
            0x6B => Opcode::BIT_u3_R8 { bit: 5, target: E },
            0x6C => Opcode::BIT_u3_R8 { bit: 5, target: H },
            0x6D => Opcode::BIT_u3_R8 { bit: 5, target: L },
            0x6E => Opcode::BIT_u3_HL { bit: 5 },
            0x6F => Opcode::BIT_u3_R8 { bit: 5, target: A },
            
            0x70 => Opcode::BIT_u3_R8 { bit: 6, target: B },
            0x71 => Opcode::BIT_u3_R8 { bit: 6, target: C },
            0x72 => Opcode::BIT_u3_R8 { bit: 6, target: D },
            0x73 => Opcode::BIT_u3_R8 { bit: 6, target: E },
            0x74 => Opcode::BIT_u3_R8 { bit: 6, target: H },
            0x75 => Opcode::BIT_u3_R8 { bit: 6, target: L },
            0x76 => Opcode::BIT_u3_HL { bit: 6 },
            0x77 => Opcode::BIT_u3_R8 { bit: 6, target: A },
            
            0x78 => Opcode::BIT_u3_R8 { bit: 7, target: B },
            0x79 => Opcode::BIT_u3_R8 { bit: 7, target: C },
            0x7A => Opcode::BIT_u3_R8 { bit: 7, target: D },
            0x7B => Opcode::BIT_u3_R8 { bit: 7, target: E },
            0x7C => Opcode::BIT_u3_R8 { bit: 7, target: H },
            0x7D => Opcode::BIT_u3_R8 { bit: 7, target: L },
            0x7E => Opcode::BIT_u3_HL { bit: 7 },
            0x7F => Opcode::BIT_u3_R8 { bit: 7, target: A },

            0x80 => Opcode::RES_u3_R8 { bit: 0, target: B },
            0x81 => Opcode::RES_u3_R8 { bit: 0, target: C },
            0x82 => Opcode::RES_u3_R8 { bit: 0, target: D },
            0x83 => Opcode::RES_u3_R8 { bit: 0, target: E },
            0x84 => Opcode::RES_u3_R8 { bit: 0, target: H },
            0x85 => Opcode::RES_u3_R8 { bit: 0, target: L },
            0x86 => Opcode::RES_u3_HL { bit: 0 },
            0x87 => Opcode::RES_u3_R8 { bit: 0, target: A },
            
            0x88 => Opcode::RES_u3_R8 { bit: 1, target: B },
            0x89 => Opcode::RES_u3_R8 { bit: 1, target: C },
            0x8A => Opcode::RES_u3_R8 { bit: 1, target: D },
            0x8B => Opcode::RES_u3_R8 { bit: 1, target: E },
            0x8C => Opcode::RES_u3_R8 { bit: 1, target: H },
            0x8D => Opcode::RES_u3_R8 { bit: 1, target: L },
            0x8E => Opcode::RES_u3_HL { bit: 1 },
            0x8F => Opcode::RES_u3_R8 { bit: 1, target: A },
            
            0x90 => Opcode::RES_u3_R8 { bit: 2, target: B },
            0x91 => Opcode::RES_u3_R8 { bit: 2, target: C },
            0x92 => Opcode::RES_u3_R8 { bit: 2, target: D },
            0x93 => Opcode::RES_u3_R8 { bit: 2, target: E },
            0x94 => Opcode::RES_u3_R8 { bit: 2, target: H },
            0x95 => Opcode::RES_u3_R8 { bit: 2, target: L },
            0x96 => Opcode::RES_u3_HL { bit: 2 },
            0x97 => Opcode::RES_u3_R8 { bit: 2, target: A },
            
            0x98 => Opcode::RES_u3_R8 { bit: 3, target: B },
            0x99 => Opcode::RES_u3_R8 { bit: 3, target: C },
            0x9A => Opcode::RES_u3_R8 { bit: 3, target: D },
            0x9B => Opcode::RES_u3_R8 { bit: 3, target: E },
            0x9C => Opcode::RES_u3_R8 { bit: 3, target: H },
            0x9D => Opcode::RES_u3_R8 { bit: 3, target: L },
            0x9E => Opcode::RES_u3_HL { bit: 3 },
            0x9F => Opcode::RES_u3_R8 { bit: 3, target: A },
            
            0xA0 => Opcode::RES_u3_R8 { bit: 4, target: B },
            0xA1 => Opcode::RES_u3_R8 { bit: 4, target: C },
            0xA2 => Opcode::RES_u3_R8 { bit: 4, target: D },
            0xA3 => Opcode::RES_u3_R8 { bit: 4, target: E },
            0xA4 => Opcode::RES_u3_R8 { bit: 4, target: H },
            0xA5 => Opcode::RES_u3_R8 { bit: 4, target: L },
            0xA6 => Opcode::RES_u3_HL { bit: 4 },
            0xA7 => Opcode::RES_u3_R8 { bit: 4, target: A },
            
            0xA8 => Opcode::RES_u3_R8 { bit: 5, target: B },
            0xA9 => Opcode::RES_u3_R8 { bit: 5, target: C },
            0xAA => Opcode::RES_u3_R8 { bit: 5, target: D },
            0xAB => Opcode::RES_u3_R8 { bit: 5, target: E },
            0xAC => Opcode::RES_u3_R8 { bit: 5, target: H },
            0xAD => Opcode::RES_u3_R8 { bit: 5, target: L },
            0xAE => Opcode::RES_u3_HL { bit: 5 },
            0xAF => Opcode::RES_u3_R8 { bit: 5, target: A },
            
            0xB0 => Opcode::RES_u3_R8 { bit: 6, target: B },
            0xB1 => Opcode::RES_u3_R8 { bit: 6, target: C },
            0xB2 => Opcode::RES_u3_R8 { bit: 6, target: D },
            0xB3 => Opcode::RES_u3_R8 { bit: 6, target: E },
            0xB4 => Opcode::RES_u3_R8 { bit: 6, target: H },
            0xB5 => Opcode::RES_u3_R8 { bit: 6, target: L },
            0xB6 => Opcode::RES_u3_HL { bit: 6 },
            0xB7 => Opcode::RES_u3_R8 { bit: 6, target: A },
            
            0xB8 => Opcode::RES_u3_R8 { bit: 7, target: B },
            0xB9 => Opcode::RES_u3_R8 { bit: 7, target: C },
            0xBA => Opcode::RES_u3_R8 { bit: 7, target: D },
            0xBB => Opcode::RES_u3_R8 { bit: 7, target: E },
            0xBC => Opcode::RES_u3_R8 { bit: 7, target: H },
            0xBD => Opcode::RES_u3_R8 { bit: 7, target: L },
            0xBE => Opcode::RES_u3_HL { bit: 7 },
            0xBF => Opcode::RES_u3_R8 { bit: 7, target: A },
            
            0xC0 => Opcode::SET_u3_R8 { bit: 0, target: B },
            0xC1 => Opcode::SET_u3_R8 { bit: 0, target: C },
            0xC2 => Opcode::SET_u3_R8 { bit: 0, target: D },
            0xC3 => Opcode::SET_u3_R8 { bit: 0, target: E },
            0xC4 => Opcode::SET_u3_R8 { bit: 0, target: H },
            0xC5 => Opcode::SET_u3_R8 { bit: 0, target: L },
            0xC6 => Opcode::SET_u3_HL { bit: 0 },
            0xC7 => Opcode::SET_u3_R8 { bit: 0, target: A },
            
            0xC8 => Opcode::SET_u3_R8 { bit: 1, target: B },
            0xC9 => Opcode::SET_u3_R8 { bit: 1, target: C },
            0xCA => Opcode::SET_u3_R8 { bit: 1, target: D },
            0xCB => Opcode::SET_u3_R8 { bit: 1, target: E },
            0xCC => Opcode::SET_u3_R8 { bit: 1, target: H },
            0xCD => Opcode::SET_u3_R8 { bit: 1, target: L },
            0xCE => Opcode::SET_u3_HL { bit: 1 },
            0xCF => Opcode::SET_u3_R8 { bit: 1, target: A },
            
            0xD0 => Opcode::SET_u3_R8 { bit: 2, target: B },
            0xD1 => Opcode::SET_u3_R8 { bit: 2, target: C },
            0xD2 => Opcode::SET_u3_R8 { bit: 2, target: D },
            0xD3 => Opcode::SET_u3_R8 { bit: 2, target: E },
            0xD4 => Opcode::SET_u3_R8 { bit: 2, target: H },
            0xD5 => Opcode::SET_u3_R8 { bit: 2, target: L },
            0xD6 => Opcode::SET_u3_HL { bit: 2 },
            0xD7 => Opcode::SET_u3_R8 { bit: 2, target: A },
            
            0xD8 => Opcode::SET_u3_R8 { bit: 3, target: B },
            0xD9 => Opcode::SET_u3_R8 { bit: 3, target: C },
            0xDA => Opcode::SET_u3_R8 { bit: 3, target: D },
            0xDB => Opcode::SET_u3_R8 { bit: 3, target: E },
            0xDC => Opcode::SET_u3_R8 { bit: 3, target: H },
            0xDD => Opcode::SET_u3_R8 { bit: 3, target: L },
            0xDE => Opcode::SET_u3_HL { bit: 3 },
            0xDF => Opcode::SET_u3_R8 { bit: 3, target: A },
            
            0xE0 => Opcode::SET_u3_R8 { bit: 4, target: B },
            0xE1 => Opcode::SET_u3_R8 { bit: 4, target: C },
            0xE2 => Opcode::SET_u3_R8 { bit: 4, target: D },
            0xE3 => Opcode::SET_u3_R8 { bit: 4, target: E },
            0xE4 => Opcode::SET_u3_R8 { bit: 4, target: H },
            0xE5 => Opcode::SET_u3_R8 { bit: 4, target: L },
            0xE6 => Opcode::SET_u3_HL { bit: 4 },
            0xE7 => Opcode::SET_u3_R8 { bit: 4, target: A },
            
            0xE8 => Opcode::SET_u3_R8 { bit: 5, target: B },
            0xE9 => Opcode::SET_u3_R8 { bit: 5, target: C },
            0xEA => Opcode::SET_u3_R8 { bit: 5, target: D },
            0xEB => Opcode::SET_u3_R8 { bit: 5, target: E },
            0xEC => Opcode::SET_u3_R8 { bit: 5, target: H },
            0xED => Opcode::SET_u3_R8 { bit: 5, target: L },
            0xEE => Opcode::SET_u3_HL { bit: 5 },
            0xEF => Opcode::SET_u3_R8 { bit: 5, target: A },
            
            0xF0 => Opcode::SET_u3_R8 { bit: 6, target: B },
            0xF1 => Opcode::SET_u3_R8 { bit: 6, target: C },
            0xF2 => Opcode::SET_u3_R8 { bit: 6, target: D },
            0xF3 => Opcode::SET_u3_R8 { bit: 6, target: E },
            0xF4 => Opcode::SET_u3_R8 { bit: 6, target: H },
            0xF5 => Opcode::SET_u3_R8 { bit: 6, target: L },
            0xF6 => Opcode::SET_u3_HL { bit: 6 },
            0xF7 => Opcode::SET_u3_R8 { bit: 6, target: A },
            
            0xF8 => Opcode::SET_u3_R8 { bit: 7, target: B },
            0xF9 => Opcode::SET_u3_R8 { bit: 7, target: C },
            0xFA => Opcode::SET_u3_R8 { bit: 7, target: D },
            0xFB => Opcode::SET_u3_R8 { bit: 7, target: E },
            0xFC => Opcode::SET_u3_R8 { bit: 7, target: H },
            0xFD => Opcode::SET_u3_R8 { bit: 7, target: L },
            0xFE => Opcode::SET_u3_HL { bit: 7 },
            0xFF => Opcode::SET_u3_R8 { bit: 7, target: A },
        }
    }

    fn execute(&mut self, opcode: Opcode) -> Result<i32> {
        match opcode {
            Opcode::NOP => {
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::HALT => {
                panic!("Received HALT Opcode");
            },
            Opcode::LD_R_R {target, source} => {
                let value = self.cpu.read8(&source);
                self.cpu.write8(target, value);
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::LD_R_HL { target } => {
                let value = self.read(self.cpu.read16(&HL))?;
                self.cpu.write8(target, value);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::LD_HL_R {source } => {
                let value = self.cpu.read8(&source);
                self.write(self.cpu.read16(&HL), value)?;
                self.cpu.pc += 1;
                Ok(8)
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
                Ok(4)
            },
            Opcode::ADD_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.read16(&HL))?;
                let (sum, c) = n1.overflowing_add(n2);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                Ok(8)
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
                Ok(4)
            },
            Opcode::ADC_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.read16(&HL))?;
                let carry_flag = self.cpu.get_flag(Flag::C);
                let (partial_sum, partial_c) = n1.overflowing_add(n2);
                let (sum, c) = partial_sum.overflowing_add(carry_flag);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, n2) || Cpu::has_half_carry(partial_sum, carry_flag));
                self.cpu.set_flag(Flag::C, c || partial_c);
                self.cpu.pc += 1;
                Ok(8)
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
                Ok(4)
            },
            Opcode::SUB_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.read16(&HL))?;
                let (sum, c) = n1.overflowing_sub(n2);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                Ok(8)
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
                Ok(4)
            },
            Opcode::SBC_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.read16(&HL))?;
                let carry_flag = self.cpu.get_flag(Flag::C);
                let (partial_sum, partial_c) = n1.overflowing_sub(n2);
                let (sum, c) = partial_sum.overflowing_sub(carry_flag);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c || partial_c);
                self.cpu.pc += 1;
                Ok(8)
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
                Ok(4)
            },
            Opcode::AND_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.read16(&HL))?;
                let value = n1 & n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, true);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                Ok(8)
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
                Ok(4)
            },
            Opcode::XOR_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.read16(&HL))?;
                let value = n1 ^ n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                Ok(8)
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
                Ok(4)
            },
            Opcode::OR_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.read16(&HL))?;
                let value = n1 | n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 1;
                Ok(8)
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
                Ok(4)
            },
            Opcode::CP_A_HL => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.read16(&HL))?;
                let (sum, c) = n1.overflowing_sub(n2);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::LD_R16_N { target } => {
                let lsb = self.read(self.cpu.pc+1)?;
                let msb = self.read(self.cpu.pc+2)?;
                let value = cpu::join_bytes(msb, lsb);
                self.cpu.write16(target, value);
                self.cpu.pc += 3;
                Ok(12)
            },
            Opcode::LD_R_N { target } => {
                let value = self.read(self.cpu.pc+1)?;
                self.cpu.write8(target, value);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::LD_HL_N => {
                let value = self.read(self.cpu.pc+1)?;
                self.write(self.cpu.read16(&HL), value)?;
                self.cpu.pc += 2;
                Ok(12)
            },
            Opcode::INC_R16 { target } => {
                let value = self.cpu.read16(&target).wrapping_add(1);
                self.cpu.write16(target, value);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::DEC_R16 { target } => {
                let value = self.cpu.read16(&target).wrapping_sub(1);
                self.cpu.write16(target, value);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::INC_R8 { target } => {
                let n1 = self.cpu.read8(&target);
                let value = n1.wrapping_add(1);
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, 1));
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::DEC_R8 { target } => {
                let n1 = self.cpu.read8(&target);
                let value = n1.wrapping_sub(1);
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, 1));
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::INC_HL => {
                let n1 = self.read(self.cpu.read16(&HL))?;  
                let value = n1.wrapping_add(1);
                self.write(self.cpu.read16(&HL), value)?;

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, 1));
                self.cpu.pc += 1;
                Ok(12)
            },
            Opcode::DEC_HL => {
                let n1 = self.read(self.cpu.read16(&HL))?;  
                let value = n1.wrapping_sub(1);
                self.write(self.cpu.read16(&HL), value)?;

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, 1));
                self.cpu.pc += 1;
                Ok(12)
            },
            Opcode::SET_u3_R8 { bit, target } => {
                let mut value = self.cpu.read8(&target);
                let mask = 1 << bit;
                value |= mask;
                self.cpu.write8(target, value);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::SET_u3_HL { bit } => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let mask = 1 << bit;
                value |= mask;
                self.write(self.cpu.read16(&HL), value)?;
                self.cpu.pc += 2;
                Ok(16)
            }
            Opcode::RES_u3_R8 { bit, target } => {
                let mut value = self.cpu.read8(&target);
                let mask = !(1 << bit);
                value &= mask;
                self.cpu.write8(target, value);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::RES_u3_HL { bit } => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let mask = !(1 << bit);
                value &= mask;
                self.write(self.cpu.read16(&HL), value)?;
                self.cpu.pc += 2;
                Ok(16)
            }
            Opcode::BIT_u3_R8 { bit, target } => {
                let value = self.cpu.read8(&target);
                self.cpu.set_flag(Flag::Z, ((value >> bit) & 1) == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, true);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::BIT_u3_HL { bit } => {
                let value = self.read(self.cpu.read16(&HL))?;
                self.cpu.set_flag(Flag::Z, ((value >> bit) & 1) == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, true);
                self.cpu.pc += 2;
                Ok(12)
            },
            Opcode::SWAP_R8 { target } => {
                let mut value = self.cpu.read8(&target);
                let initial_upper_byte = value >> 4;
                let initial_lower_byte = value & 15;
                value = initial_upper_byte | (initial_lower_byte << 4);
                self.cpu.write8(target, value);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::SWAP_HL => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let initial_upper_byte = value >> 4;
                let initial_lower_byte = value & 15;
                value = initial_upper_byte | (initial_lower_byte << 4);
                self.write(self.cpu.read16(&HL), value)?;
                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::RLC_R8 { target } => {
                let mut value = self.cpu.read8(&target);
                let leading_bit = value >> 7;
                value <<= 1;
                value |= leading_bit;
                self.cpu.write8(target, value);
                
                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, leading_bit == 1);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::RLC_HL => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let leading_bit = value >> 7;
                value <<= 1;
                value |= leading_bit;
                self.write(self.cpu.read16(&HL), value)?;
                
                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, leading_bit == 1);
                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::RRC_R8 { target } => {
                let mut value = self.cpu.read8(&target);
                let trailing_bit = value & 1;
                value >>= 1;
                value |= trailing_bit << 7;
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::RRC_HL => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let trailing_bit = value & 1;
                value >>= 1;
                value |= trailing_bit << 7;
                self.write(self.cpu.read16(&HL), value)?;

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::RL_R8 { target } => {
                let mut value = self.cpu.read8(&target);
                let leading_bit = value >> 7;
                value <<= 1;
                value |= self.cpu.get_flag(Flag::C);
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, leading_bit == 1);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::RL_HL => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let leading_bit = value >> 7;
                value <<= 1;
                value |= self.cpu.get_flag(Flag::C);
                self.write(self.cpu.read16(&HL), value)?;

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, leading_bit == 1);
                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::RR_R8 { target } => {
                let mut value = self.cpu.read8(&target);
                let trailing_bit = value & 1;
                value >>= 1;
                value |= self.cpu.get_flag(Flag::C) << 7;
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::RR_HL => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let trailing_bit = value & 1;
                value >>= 1;
                value |= self.cpu.get_flag(Flag::C) << 7;
                self.write(self.cpu.read16(&HL), value)?;
                
                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::SLA_R8 { target } => {
                let mut value = self.cpu.read8(&target);
                let leading_bit = value >> 7;
                value <<= 1;
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, leading_bit == 1);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::SLA_HL => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let leading_bit = value >> 7;
                value <<= 1;
                self.write(self.cpu.read16(&HL), value)?;
                
                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, leading_bit == 1);
                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::SRA_R8 { target } => {
                let mut value = self.cpu.read8(&target);
                let leading_bit = value & 0b1000_0000;
                let trailing_bit = value & 1;
                value >>= 1;
                value |= leading_bit;
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::SRA_HL => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let leading_bit = value & 0b1000_0000;
                let trailing_bit = value & 1;
                value >>= 1;
                value |= leading_bit;
                self.write(self.cpu.read16(&HL), value)?;

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::SRL_R8 { target } => {
                let mut value = self.cpu.read8(&target);
                let trailing_bit = value & 1;
                value >>= 1;
                self.cpu.write8(target, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::SRL_HL => {
                let mut value = self.read(self.cpu.read16(&HL))?;
                let trailing_bit = value & 1;
                value >>= 1;
                self.write(self.cpu.read16(&HL), value)?;
    
                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::POP_R16 { target } => {
                let lower = self.pop()?;
                let upper = self.pop()?;
                self.cpu.write16(target, cpu::join_bytes(upper, lower));
                self.cpu.pc += 1;
                Ok(12)
            },
            Opcode::PUSH_R16 { target } => {
                let (upper, lower) = cpu::split_word(self.cpu.read16(&target));
                self.push(upper)?;
                self.push(lower)?;
                self.cpu.pc += 1;
                Ok(16)
            },
            Opcode::RET => {
                // RET does not increment the PC. CALL will instead store the PC + 1;
                let lower = self.pop()?;
                let upper = self.pop()?;
                self.cpu.sp += 1;
                self.cpu.pc = cpu::join_bytes(upper, lower);
                Ok(16)
            },
            Opcode::RET_cc { condition, set } => {
                // RET_cc does not increment the PC. CALL will instead store the PC + 1;
                let cc = self.cpu.get_flag(condition);
                if (cc != 0) == set {
                    let lower = self.pop()?;
                    let upper = self.pop()?;
                    self.cpu.pc = cpu::join_bytes(upper, lower);
                    return Ok(20)
                }
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::CALL_n16 => {
                let addr_lower = self.read(self.cpu.pc + 1)?;
                let addr_upper = self.read(self.cpu.pc + 2)?;
                let (upper, lower) = cpu::split_word(self.cpu.pc + 1);
                self.push(upper)?;
                self.push(lower)?;
                self.cpu.pc = cpu::join_bytes(addr_upper, addr_lower);
                Ok(24)
            },
            Opcode::CALL_cc_n16 { condition, set } => {
                let addr_lower = self.read(self.cpu.pc + 1)?;
                let addr_upper = self.read(self.cpu.pc + 2)?;
                let (upper, lower) = cpu::split_word(self.cpu.pc + 1);
                let cc = self.cpu.get_flag(condition);
                if (cc != 0) == set {
                    self.push(upper)?;
                    self.push(lower)?;
                    self.cpu.pc = cpu::join_bytes(addr_upper, addr_lower);
                    return Ok(24)
                }
                self.cpu.pc += 3;
                Ok(12)
            },
            Opcode::JP_n16 => {
                let addr_lower = self.read(self.cpu.pc + 1)?;
                let addr_upper = self.read(self.cpu.pc + 2)?;
                self.cpu.pc = cpu::join_bytes(addr_upper, addr_lower);
                Ok(16)
            },
            Opcode::JP_cc_n16 { condition, set } => {
                let addr_lower = self.read(self.cpu.pc + 1)?;
                let addr_upper = self.read(self.cpu.pc + 2)?;
                let cc = self.cpu.get_flag(condition);
                if (cc != 0) == set {
                    self.cpu.pc = cpu::join_bytes(addr_upper, addr_lower);
                    return Ok(16)
                }
                self.cpu.pc += 3;
                Ok(12)
                
            },
            Opcode::JP_HL => {
                self.cpu.pc = self.cpu.read16(&HL);
                Ok(4)
            },
            Opcode::RST { vct } => {
                let (upper, lower) = cpu::split_word(self.cpu.pc + 1);
                self.push(upper)?;
                self.push(lower)?;
                self.cpu.pc = vct;
                Ok(16)
            },
            Opcode::ADD_A_n8 => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.pc + 1)?;
                let (sum, c) = n1.overflowing_add(n2);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::ADC_A_n8 => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.pc + 1)?;
                let carry_flag = self.cpu.get_flag(Flag::C);
                let (partial_sum, partial_c) = n1.overflowing_add(n2);
                let (sum, c) = partial_sum.overflowing_add(carry_flag);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1, n2));
                self.cpu.set_flag(Flag::C, c || partial_c);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::SUB_A_n8 => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.pc + 1)?;
                let (sum, c) = n1.overflowing_sub(n2);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::SBC_A_n8 => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.pc + 1)?;
                let carry_flag = self.cpu.get_flag(Flag::C);
                let (partial_sum, partial_c) = n1.overflowing_sub(n2);
                let (sum, c) = partial_sum.overflowing_sub(carry_flag);
                self.cpu.write8(A, sum);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2) || Cpu::has_half_borrow(partial_sum, carry_flag));
                self.cpu.set_flag(Flag::C, c || partial_c);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::AND_A_n8 => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.pc + 1)?;
                let value = n1 & n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, true);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::XOR_A_n8 => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.pc + 1)?;
                let value = n1 ^ n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::OR_A_n8 => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.pc + 1)?;
                let value = n1 | n2;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, value == 0);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, false);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::CP_A_n8 => {
                let n1 = self.cpu.read8(&A);
                let n2 = self.read(self.cpu.pc + 1)?;
                let (sum, c) = n1.overflowing_sub(n2);

                self.cpu.set_flag(Flag::Z, sum == 0);
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, Cpu::has_half_borrow(n1, n2));
                self.cpu.set_flag(Flag::C, c);
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::JR_e8 => {
                let rel_addr: i16 = self.read(self.cpu.pc)? as i16;
                let temp_pc = self.cpu.pc as i16;
                let new_addr = temp_pc + rel_addr;
                self.cpu.pc = new_addr as u16;
                Ok(12) 
            },
            Opcode::JR_cc_e8 { condition, set } => {
                let cc = self.cpu.get_flag(condition);
                if (cc != 0) == set {
                    let rel_addr: i16 = self.read(self.cpu.pc)? as i16;
                    let temp_pc = self.cpu.pc as i16;
                    let new_addr = temp_pc + rel_addr;
                    self.cpu.pc = new_addr as u16;
                    return Ok(12)                     
                }
                self.cpu.pc += 2;
                Ok(8)
            },
            Opcode::LD_R16_A { target } => {
                let value = self.cpu.read8(&A);
                self.write(self.cpu.read16(&target), value)?;
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::LD_HLID_A { inc } => {
                let value = self.cpu.read8(&A);
                let target = self.cpu.read16(&HL);
                self.write(target, value)?;
                if inc {
                    self.cpu.write16(HL, target.wrapping_add(1));
                } else {
                    self.cpu.write16(HL, target.wrapping_sub(1));
                }
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::ADD_HL_R16 { target } => {
                let n1 = self.cpu.read16(&HL);
                let n2 = self.cpu.read16(&target);
                let (value, carry) = n1.overflowing_add(n2);
                self.cpu.write16(HL, value);

                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1 as u8, n2 as u8));
                self.cpu.set_flag(Flag::C, carry);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::ADD_HL_SP => {
                let n1 = self.cpu.read16(&HL);
                let (value, carry) = n1.overflowing_add(self.cpu.sp);
                self.cpu.write16(HL, value);
                
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, Cpu::has_half_carry(n1 as u8, self.cpu.sp as u8));
                self.cpu.set_flag(Flag::C, carry);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::LD_A_R16 { target } => {
                let value = self.read(self.cpu.read16(&target))?;
                self.cpu.write8(A, value);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::LD_A_HLID { inc } => {
                let target = self.cpu.read16(&HL);
                let value = self.read(target)?;
                self.cpu.write8(A, value);
                if inc {
                    self.cpu.write16(HL, target.wrapping_add(1));
                } else {
                    self.cpu.write16(HL, target.wrapping_sub(1));
                }
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::LD_a16_SP => {
                let low = self.read(self.cpu.pc + 1)?;
                let high = self.read(self.cpu.pc + 2)?;
                let target = cpu::join_bytes(high, low);
                self.write(target, self.cpu.sp as u8)?;
                self.write(target + 1, (self.cpu.sp >> 8) as u8)?;
                self.cpu.pc += 3;
                Ok(20)
            }
            Opcode::LDH_a8_A => {
                let value = self.cpu.read8(&A);
                let target = self.read(self.cpu.pc + 1)? as u16;
                self.write(target + 0xFF, value)?;
                self.cpu.pc += 2;
                Ok(12)
            },
            Opcode::LDH_A_a8 => {
                let target = self.read(self.cpu.pc + 1)? as u16;
                let value = self.read(target + 0xFF)?;
                self.cpu.write8(A, value);
                self.cpu.pc += 2;
                Ok(12)
            },
            Opcode::LDH_c_A => {
                let value = self.cpu.read8(&A);
                let target = self.cpu.read8(&C) as u16 + 0xFF;
                self.write(target, value)?;
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::LDH_A_c => {
                let target = self.cpu.read8(&C) as u16 + 0xFF;   
                let value = self.read(target)?;
                self.cpu.write8(A, value);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::LD_a16_A => {
                let low = self.read(self.cpu.pc + 1)?;
                let high = self.read(self.cpu.pc + 2)?;
                let target = cpu::join_bytes(high, low);
                let value = self.cpu.read8(&A);
                self.write(target, value)?;
                self.cpu.pc += 3;
                Ok(16)
            },
            Opcode::LD_A_a16 => {
                let low = self.read(self.cpu.pc + 1)?;
                let high = self.read(self.cpu.pc + 2)?;
                let source = cpu::join_bytes(high, low);
                let value = self.read(source)?;
                self.cpu.write8(A, value);
                self.cpu.pc += 3;
                Ok(16)
            },
            Opcode::ADD_SP_e8 => {
                let n1 = self.read(self.cpu.pc + 1)?;
                let n1_3b = n1 & 0x08;
                let n2_3b = self.cpu.sp as u8 & 0x08;
                
                let n1_7b = n1 & 0x80;
                let n2_7b = self.cpu.sp as u8 & 0x80;
                self.cpu.sp = (self.cpu.sp as i16 + n1 as i16) as u16;
                
                let target_3b = self.cpu.sp as u8 & 0x08;
                let target_7b = self.cpu.sp as u8 & 0x80;
                let overflow3 = (n1_3b == n2_3b) && (target_3b != n1_3b);
                let overflow7 = (n1_7b == n2_7b) && (target_7b != n1_7b);

                self.cpu.set_flag(Flag::Z, false);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, overflow3);
                self.cpu.set_flag(Flag::C, overflow7);

                self.cpu.pc += 2;
                Ok(16)
            },
            Opcode::LD_HL_SPe8 => {
                let n1 = self.read(self.cpu.pc + 1)?;
                let target = (self.cpu.sp as i16 + n1 as i16) as u16;
                
                let n1_3b = n1 & 0x08;
                let n2_3b = self.cpu.sp as u8 & 0x08;
                let n1_7b = n1 & 0x80;
                let n2_7b = self.cpu.sp as u8 & 0x80;
                let target_3b = target as u8 & 0x08;
                let target_7b = target as u8 & 0x80;
                
                let overflow3 = (n1_3b == n2_3b) && (target_3b != n1_3b);
                let overflow7 = (n1_7b == n2_7b) && (target_7b != n1_7b);
                
                self.cpu.set_flag(Flag::Z, false);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, overflow3);
                self.cpu.set_flag(Flag::C, overflow7);
                self.cpu.pc += 2;
                Ok(12)
            }
            Opcode::LD_SP_HL => {
                self.cpu.sp = self.cpu.read16(&HL);
                self.cpu.pc += 1;
                Ok(8)
            },
            Opcode::RLCA => {
                let mut value = self.cpu.read8(&A);
                let leading_bit = value >> 7;
                value <<= 1;
                value |= leading_bit;
                self.cpu.write8(A, value);
                
                self.cpu.set_flag(Flag::Z, false);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, leading_bit == 1);
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::RRCA => {
                let mut value = self.cpu.read8(&A);
                let trailing_bit = value & 1;
                value >>= 1;
                value |= trailing_bit << 7;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, false);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::RLA => {
                let mut value = self.cpu.read8(&A);
                let leading_bit = value >> 7;
                value <<= 1;
                value |= self.cpu.get_flag(Flag::C);
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, false);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, leading_bit == 1);
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::RRA => {
                let mut value = self.cpu.read8(&A);
                let trailing_bit = value & 1;
                value >>= 1;
                value |= self.cpu.get_flag(Flag::C) << 7;
                self.cpu.write8(A, value);

                self.cpu.set_flag(Flag::Z, false);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, trailing_bit == 1);
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::DAA => {
                let target = self.cpu.read8(&A);
                let mut adj = 0;
                if self.cpu.get_flag(Flag::N) == 1 {
                    if self.cpu.get_flag(Flag::H) == 1 {
                        adj += 0x6;
                    }
                    if self.cpu.get_flag(Flag::C) == 1{
                        adj += 0x60;
                    }
                    adj = target - adj;
                } else {
                    if self.cpu.get_flag(Flag::H) == 1 || target & 0xF > 0x9 {
                        adj += 0x6;
                    }
                    if self.cpu.get_flag(Flag::C) == 1 || target > 0x99 {
                        adj += 0x60;
                        self.cpu.set_flag(Flag::C, true);
                    }
                    adj = target + adj;
                }
                self.cpu.write8(A, adj);
                self.cpu.set_flag(Flag::Z, adj == 0);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::CPL => {
                self.cpu.write8(A, !self.cpu.read8(&A));
                self.cpu.set_flag(Flag::N, true);
                self.cpu.set_flag(Flag::H, true);
                self.cpu.pc += 1;
                Ok(4)
            }
            Opcode::SCF => {
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, true);
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::CCF => {
                let c = self.cpu.get_flag(Flag::C);
                self.cpu.set_flag(Flag::N, false);
                self.cpu.set_flag(Flag::H, false);
                self.cpu.set_flag(Flag::C, c == 0);
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::RETI => {
                let lower = self.pop()?;
                let upper = self.pop()?;
                self.cpu.sp += 1;
                self.cpu.pc = cpu::join_bytes(upper, lower);
                self.cpu.ime = true;
                Ok(16)
            },
            Opcode::DI => {
                self.cpu.ime = false;
                self.cpu.pc += 1;
                Ok(4)
            },
            Opcode::EI => {
                self.cpu.ime = true;
                self.cpu.pc += 1;
                Ok(4)
            },
        }
    }
}
