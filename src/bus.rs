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

    pub fn read8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let low = self.read8(addr);
        let high = self.read8(addr + 1);
        ((high as u16) << 8) | low as u16
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        self.write8(addr, val as u8);
        self.write8(addr + 1, (val >> 8) as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_writes_bytes_to_memory() {
        let mut bus = Bus::new();
        assert_eq!(bus.read8(2), 0);
        bus.write8(2, 3);
        assert_eq!(bus.read8(2), 3);
    }

    #[test]
    fn reads_and_writes_words_to_memory() {
        let mut bus = Bus::new();
        assert_eq!(bus.read8(2), 0);
        assert_eq!(bus.read8(3), 0);
        bus.write16(2, 1234);
        assert_eq!(bus.read8(2), 0xD2);
        assert_eq!(bus.read8(3), 0x04);
        assert_eq!(bus.read16(2), 0x04D2);
    }
}
