use rustboy_core::debug::{DebugEvent, Debugger};

pub struct TraceDebugger;

impl Debugger for TraceDebugger {
    fn event(&mut self, event: DebugEvent) {
        match event {
            DebugEvent::Tick { tick } => {
                println!("[TICK] {}", tick);
            }

            DebugEvent::CpuFetch { pc, opcode } => {
                println!("[FETCH] PC={:04X} OPCODE={:02X}", pc, opcode);
            }

            DebugEvent::MemoryRead { addr, val } => {
                println!("[READ] {:04X} -> {:02X}", addr, val);
            }

            DebugEvent::MemoryWrite { addr, val } => {
                println!("[WRITE] {:04X} <- {:02X}", addr, val);
            }

            DebugEvent::MicroOpExecuted => {
                println!("[MICRO]");
            }
        }
    }
}
