pub struct Bus {
    memory: [u8; 0x10000],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; 0x10000]
    }
}
