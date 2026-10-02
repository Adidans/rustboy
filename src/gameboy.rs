use crate::bus::Bus;
use crate::cpu::Cpu;

pub struct Gameboy {
    cpu: Cpu,
    bus: Bus,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
    }
}
