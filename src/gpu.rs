use crate::MemoryAddressError;


pub struct Gpu {
    pub gpu_mode: GpuMode,
    cycles: i32,
    line: u8,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    lcdc: u8,
    stat: u8,
    scroll_x: u8,
    scroll_y: u8,
    ly: u8,
    lyc: u8,
    pallettes: [u8; 3],
    window_x_y: [u8; 2],

    frame_buffer: [u8; 160 * 144] // 160x144 screen resolution
}

pub enum GpuMode {
    HBlank,
    VBlank,
    OamScan,
    Drawing,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            gpu_mode: GpuMode::OamScan,
            cycles: 0,
            line: 0,
            frame_buffer: [0; 160 * 144],
            lcdc: 0,
            stat: 0,
            scroll_x: 0,
            scroll_y: 0,
            ly: 0,
            lyc: 0,
            pallettes: [0; 3],
            window_x_y: [0; 2],
             
        }
    }

    pub fn read(&self, address: u16) -> Result<u8, MemoryAddressError> {
        Ok(match address {
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            _ => return Err(MemoryAddressError),
        })
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryAddressError> {
            match address {
                0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
                0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
                _ => return Err(MemoryAddressError)
            }
            Ok(())
    }
    
    pub fn step(&mut self, cycles: i32) {
        self.cycles += cycles;
        match self.gpu_mode {
            GpuMode::OamScan => {
                if self.cycles >= 80 {
                    self.gpu_mode = GpuMode::Drawing;
                    self.cycles = 0
                }
            },
            GpuMode::Drawing => {
                if self.cycles >= 172 {
                    self.gpu_mode = GpuMode::HBlank;
                    self.cycles = 0;
                }
            }
            GpuMode::HBlank => {
                if self.cycles >= 204 {
                    self.line += 1;
                    self.cycles = 0;

                    if self.line == 144 {
                        self.gpu_mode = GpuMode::VBlank;
                    } else {
                        self.gpu_mode = GpuMode::OamScan;
                    }
                }
            },
            GpuMode::VBlank => {
                if self.cycles >= 456 {
                    self.line += 1;
                    self.cycles = 0;

                    if self.line > 153 {
                        self.line = 0;
                        self.gpu_mode = GpuMode::OamScan;
                    }
                }
            },
        }
    }
}
