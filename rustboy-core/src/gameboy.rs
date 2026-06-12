use super::clock::Clock;

pub struct Gameboy {
    clock: Clock,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
        }
    }
}
