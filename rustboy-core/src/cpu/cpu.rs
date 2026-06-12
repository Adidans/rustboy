use super::registers::{Flags, Registers};

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn bc(&self) -> u16 {
        ((self.registers.b as u16) << 8) | (self.registers.c as u16)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.registers.b = (val >> 8) as u8;
        self.registers.c = val as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.registers.d as u16) << 8) | (self.registers.e as u16)
    }

    pub fn set_de(&mut self, val: u16) {
        self.registers.d = (val >> 8) as u8;
        self.registers.e = val as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.registers.h as u16) << 8) | (self.registers.l as u16)
    }

    pub fn set_hl(&mut self, val: u16) {
        self.registers.h = (val >> 8) as u8;
        self.registers.l = val as u8;
    }

    pub fn z(&self) -> bool {
        self.registers.f.contains(Flags::Z)
    }

    pub fn set_z(&mut self, val: bool) {
        self.registers.f.set(Flags::Z, val);
    }

    pub fn n(&self) -> bool {
        self.registers.f.contains(Flags::N)
    }

    pub fn set_n(&mut self, val: bool) {
        self.registers.f.set(Flags::N, val);
    }

    pub fn h(&self) -> bool {
        self.registers.f.contains(Flags::H)
    }

    pub fn set_h(&mut self, val: bool) {
        self.registers.f.set(Flags::H, val);
    }

    pub fn c(&self) -> bool {
        self.registers.f.contains(Flags::C)
    }

    pub fn set_c(&mut self, val: bool) {
        self.registers.f.set(Flags::C, val);
    }
}
