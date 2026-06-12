pub struct Bus {
    mem: [u8; 65536],
}

impl Bus {
    pub fn new() -> Self {
        Self { mem: [0; 65536] }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val
    }
}
