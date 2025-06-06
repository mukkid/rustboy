use crate::MemoryAddressError;


#[derive(Clone)]
pub struct Memory {
    pub rom: [u8; 0x8000],
    pub ram: [u8; 0x4000],
    pub wram: [u8; 0x2000],
    pub wram2: [u8; 0x2000],
    pub echo_ram: [u8; 0x2000],
    pub io: [u8; 0x80],
    pub hram: [u8; 0x7F],
    pub interrupt_enable: u8,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rom: [0; 0x8000],
            ram: [0; 0x4000],
            wram: [0; 0x2000],
            wram2: [0; 0x2000],
            echo_ram: [0; 0x2000],
            io: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable: 0,
        }
    }

    pub fn read(&self, address: u16) -> Result<u8, MemoryAddressError> {
        Ok(match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0xA000..=0xBFFF => self.ram[(address - 0xA000) as usize],
            0xC000..=0xCFFF => self.wram[(address - 0xC000) as usize],
            0xD000..=0xDFFF => self.wram2[(address - 0xD000) as usize],
            0xE000..=0xEFFF => self.echo_ram[(address - 0xE000) as usize],
            0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
            _ => return Err(MemoryAddressError),
        })
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryAddressError> {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize] = value,
            0xA000..=0xBFFF => self.ram[(address - 0xA000) as usize] = value,
            0xC000..=0xCFFF => self.wram[(address - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.wram[(address - 0xD000) as usize] = value,
            0xE000..=0xEFFF => self.echo_ram[(address - 0xE000) as usize] = value,
            0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => return Err(MemoryAddressError),
        }
        Ok(())
    }
}
