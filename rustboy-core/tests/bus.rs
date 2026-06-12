use rustboy_core::bus::Bus;

#[test]
fn reads_and_writes() {
    let mut bus = Bus::new();
    bus.write8(2, 0x12);
    assert_eq!(bus.read8(2), 0x12);
}
