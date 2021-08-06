pub struct Cpu {
    ram: [u32; 1024],
    r_0: u32,
    r_1: u32,
    r_2: u32,
    r_3: u32,
    r_4: u32,
    r_5: u32,
    r_6: u32,
    r_7: u32,
    r_8: u32,
    r_9: u32,
    r_10: u32,
    r_11: u32,
    r_12: u32,
    r_13: u32,
    r_14: u32,
    r_15: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            ram: [0; 1024],
            r_0: 0,
            r_1: 0,
            r_2: 0,
            r_3: 0,
            r_4: 0,
            r_5: 0,
            r_6: 0,
            r_7: 0,
            r_8: 0,
            r_9: 0,
            r_10: 0,
            r_11: 0,
            r_12: 0,
            r_13: 0,
            r_14: 0,
            r_15: 0,   
        }
    }
    // Just messing around
    pub fn write(&mut self, idx: usize, val: u32) {
        self.ram[idx] = val;
    }

    // Just messing around
    pub fn read(&self, idx: usize) -> u32 {
        self.ram[idx]
    }
}
