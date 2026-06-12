use rustboy_core::debug::{DebugEvent, Debugger};

pub struct NullDebugger;

impl Debugger for NullDebugger {
    fn event(&mut self, _: DebugEvent) {}
}
