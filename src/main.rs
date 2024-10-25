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
    LD_R_R { target: Register8, source: Register8 },
    LD_R_HL { target: Register8 }
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
            _ => panic!("Unknown opcode {:#X}", byte)
        }
    }

    fn execute(&mut self, opcode: Opcode) -> i32 {
        match opcode {
            Opcode::NOP => {
                // No operation
                self.cpu.pc += 1;
                return 4
            },
            Opcode::LD_R_R {target, source} => {
                let value = match source {
                    A => self.cpu.a,
                    B => self.cpu.b,
                    C => self.cpu.c,
                    D => self.cpu.d,
                    E => self.cpu.e,
                    H => self.cpu.h,
                    L => self.cpu.l,
                    _ => panic!("Cannot write to flag register")
                };

                match target {
                    A => self.cpu.a = value,
                    B => self.cpu.b = value,
                    C => self.cpu.c = value,
                    D => self.cpu.d = value,
                    E => self.cpu.e = value,
                    H => self.cpu.h = value,
                    L => self.cpu.l = value,
                    F => panic!("Cannot write to flag register")
                }
                return 4
            },
            Opcode::LD_R_HL { target } => {
                let value = self.memory.read(self.cpu.read16(HL)).unwrap();
                match target {
                    A => self.cpu.a = value,
                    B => self.cpu.b = value,
                    C => self.cpu.c = value,
                    D => self.cpu.d = value,
                    E => self.cpu.e = value,
                    H => self.cpu.h = value,
                    L => self.cpu.l = value,
                    F => panic!("Cannot write to flag register")
                }
                return 8
            }
            _ => panic!("Opcode not implemented")
        }
    }
}