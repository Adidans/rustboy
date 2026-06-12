use rustboy_core::cpu::Cpu;

#[test]
fn set_and_read_bc() {
    let mut cpu = Cpu::new();
    cpu.set_bc(0x1234);
    assert_eq!(cpu.bc(), 0x1234);
}
#[test]
fn set_and_read_de() {
    let mut cpu = Cpu::new();
    cpu.set_de(0x1234);
    assert_eq!(cpu.de(), 0x1234);
}
#[test]
fn set_and_read_hl() {
    let mut cpu = Cpu::new();
    cpu.set_hl(0x1234);
    assert_eq!(cpu.hl(), 0x1234);
}
