use rustboy_core::debug::{DebugEvent, Debugger};

pub struct RecordingDebugger {
    pub events: Vec<DebugEvent>,
}

impl RecordingDebugger {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
}

impl Debugger for RecordingDebugger {
    fn event(&mut self, event: DebugEvent) {
        self.events.push(event);
    }
}
