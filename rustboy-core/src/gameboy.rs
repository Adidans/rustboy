use super::bus::Bus;
use super::clock::Clock;

pub struct Gameboy {
    clock: Clock,
    bus: Bus,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
            bus: Bus::new(),
        }
    }
}
