pub struct Gpu<'a> {
    pub gpu_mode: GpuMode,
    cycles: i32,
    line: u8,
    vram: &'a [u8],
    oam: &'a [u8],
    frame_buffer: [u8; 160 * 144] // 160x144 screen resolution
}

pub enum GpuMode {
    HBlank,
    VBlank,
    OamScan,
    Drawing,
}

impl<'a> Gpu<'a> {
    pub fn new(vram: &'a [u8], oam: &'a [u8]) -> Self {
        Self {
            vram,
            oam,
            gpu_mode: GpuMode::OamScan,
            cycles: 0,
            line: 0,
            frame_buffer: [0; 160 * 144],
        }
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
