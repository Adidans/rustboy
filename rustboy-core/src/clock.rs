pub struct Clock {
    ticks: u64,
}

impl Clock {
    pub fn new() -> Self {
        Self { ticks: 0 }
    }

    pub fn tick(&mut self) {
        self.ticks += 1
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}
