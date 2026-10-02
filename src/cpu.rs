use crate::bus::Bus;

pub struct Cpu {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn reset(&mut self) {}

    fn inc_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn fetch8(&mut self, bus: &Bus) -> u8 {
        let byte = bus.read8(self.pc);
        self.inc_pc();
        byte
    }

    pub fn fetch16(&mut self, bus: &Bus) -> u16 {
        let low = self.fetch8(bus);
        let high = self.fetch8(bus);
        ((high as u16) << 8) | low as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_pc_correctly() {
        let mut cpu = Cpu::new();
        cpu.pc = u16::MAX - 1;
        assert_eq!(cpu.pc, u16::MAX - 1);
        cpu.inc_pc();
        assert_eq!(cpu.pc, u16::MAX);
        cpu.inc_pc();
        assert_eq!(cpu.pc, 0)
    }

    #[test]
    fn fetches_byte() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        bus.write8(0, 12);
        let res = cpu.fetch8(&bus);
        assert_eq!(res, 12);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn fetches_word() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        bus.write16(0, 1234);
        let res = cpu.fetch16(&bus);
        assert_eq!(res, 1234);
        assert_eq!(cpu.pc, 2);
    }
}
