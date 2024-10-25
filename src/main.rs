mod cpu;
mod memory;

use cpu::Cpu;
use cpu::{Register, Register8, Register16};
use cpu::Register8::*;
use cpu::Register16::*;

use memory::Memory;

fn main() {
    let mut gb = Gameboy::new();
    gb.run();
}

enum Opcode {
    NOP,
    HALT,
    LD_R_R { target: Register8, source: Register8 },
    LD_R_HL { target: Register8 },
    LD_HL_R { source: Register8 },
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
            0x40 => Opcode::LD_R_R {target: B, source: B},
            0x41 => Opcode::LD_R_R {target: B, source: C},
            0x42 => Opcode::LD_R_R {target: B, source: D},
            0x43 => Opcode::LD_R_R {target: B, source: E},
            0x44 => Opcode::LD_R_R {target: B, source: H},
            0x45 => Opcode::LD_R_R {target: B, source: L},
            0x46 => Opcode::LD_R_HL {target: B},
            0x47 => Opcode::LD_R_R {target: B, source: A},

            0x48 => Opcode::LD_R_R {target: C, source: B},
            0x49 => Opcode::LD_R_R {target: C, source: C},
            0x4A => Opcode::LD_R_R {target: C, source: D},
            0x4B => Opcode::LD_R_R {target: C, source: E},
            0x4C => Opcode::LD_R_R {target: C, source: H},
            0x4D => Opcode::LD_R_R {target: C, source: L},
            0x4E => Opcode::LD_R_HL {target: C},
            0x4F => Opcode::LD_R_R {target: C, source: A},

            0x50 => Opcode::LD_R_R {target: D, source: B},
            0x51 => Opcode::LD_R_R {target: D, source: C},
            0x52 => Opcode::LD_R_R {target: D, source: D},
            0x53 => Opcode::LD_R_R {target: D, source: E},
            0x54 => Opcode::LD_R_R {target: D, source: H},
            0x55 => Opcode::LD_R_R {target: D, source: L},
            0x56 => Opcode::LD_R_HL {target: D},
            0x57 => Opcode::LD_R_R {target: D, source: A},

            0x58 => Opcode::LD_R_R {target: E, source: B},
            0x59 => Opcode::LD_R_R {target: E, source: C},
            0x5A => Opcode::LD_R_R {target: E, source: D},
            0x5B => Opcode::LD_R_R {target: E, source: E},
            0x5C => Opcode::LD_R_R {target: E, source: H},
            0x5D => Opcode::LD_R_R {target: E, source: L},
            0x5E => Opcode::LD_R_HL {target: E},
            0x5F => Opcode::LD_R_R {target: E, source: A},

            0x60 => Opcode::LD_R_R {target: H, source: B},
            0x61 => Opcode::LD_R_R {target: H, source: C},
            0x62 => Opcode::LD_R_R {target: H, source: D},
            0x63 => Opcode::LD_R_R {target: H, source: E},
            0x64 => Opcode::LD_R_R {target: H, source: H},
            0x65 => Opcode::LD_R_R {target: H, source: L},
            0x66 => Opcode::LD_R_HL {target: H},
            0x67 => Opcode::LD_R_R {target: H, source: A},

            0x68 => Opcode::LD_R_R {target: L, source: B},
            0x69 => Opcode::LD_R_R {target: L, source: C},
            0x6A => Opcode::LD_R_R {target: L, source: D},
            0x6B => Opcode::LD_R_R {target: L, source: E},
            0x6C => Opcode::LD_R_R {target: L, source: H},
            0x6D => Opcode::LD_R_R {target: L, source: L},
            0x6E => Opcode::LD_R_HL {target: L},
            0x6F => Opcode::LD_R_R {target: L, source: A},

            0x70 => Opcode::LD_HL_R {source: B},
            0x71 => Opcode::LD_HL_R {source: C},
            0x72 => Opcode::LD_HL_R {source: D},
            0x73 => Opcode::LD_HL_R {source: E},
            0x74 => Opcode::LD_HL_R {source: H},
            0x75 => Opcode::LD_HL_R {source: L},
            0x76 => Opcode::HALT,
            0x77 => Opcode::LD_HL_R {source: A},

            0x78 => Opcode::LD_R_R {target: A, source: B},
            0x79 => Opcode::LD_R_R {target: A, source: C},
            0x7A => Opcode::LD_R_R {target: A, source: D},
            0x7B => Opcode::LD_R_R {target: A, source: E},
            0x7C => Opcode::LD_R_R {target: A, source: H},
            0x7D => Opcode::LD_R_R {target: A, source: L},
            0x7E => Opcode::LD_R_HL {target: A},
            0x7F => Opcode::LD_R_R {target: A, source: A},
            _ => panic!("Unknown opcode {:#X}", byte)
        }
    }

    fn execute(&mut self, opcode: Opcode) -> i32 {
        match opcode {
            Opcode::NOP => {
                self.cpu.pc += 1;
                return 4
            },
            Opcode::LD_R_R {target, source} => {
                let value = self.cpu.read8(source);
                self.cpu.write8(target, value);
                self.cpu.pc += 1;
                return 4
            },
            Opcode::LD_R_HL { target } => {
                let value = self.memory.read(self.cpu.read16(HL)).unwrap();
                self.cpu.write8(target, value);
                self.cpu.pc += 1;
                return 8
            },
            Opcode::LD_HL_R {source } => {
                let value = self.cpu.read8(source);
                self.memory.write(self.cpu.read16(HL), value).unwrap();
                self.cpu.pc += 1;
                return 8
            },
            Opcode::HALT => {
                panic!("Received HALT Opcode");
            },
            _ => panic!("Opcode not implemented")
        }
    }
}