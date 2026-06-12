use rustboy_core::debug::{DebugEvent, Debugger};
use rustboy_debug::recording::RecordingDebugger;

#[test]
fn recording_debugger_saves_event() {
    let mut dbg = RecordingDebugger::new();

    dbg.event(DebugEvent::MemoryWrite { addr: 0, val: 0x12 });

    assert_eq!(dbg.events.len(), 1)
}
