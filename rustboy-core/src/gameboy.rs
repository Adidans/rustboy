use super::bus::Bus;
use super::clock::Clock;
use super::cpu::Cpu;
use crate::debug::Debugger;

pub struct Gameboy<D: Debugger> {
    cpu: Cpu,
    clock: Clock,
    bus: Bus,
    debugger: D,
}

impl<D: Debugger> Gameboy<D> {
    pub fn new(debugger: D) -> Self {
        Self {
            cpu: Cpu::new(),
            clock: Clock::new(),
            bus: Bus::new(),
            debugger,
        }
    }
}
