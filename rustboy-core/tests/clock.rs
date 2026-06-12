use rustboy_core::clock::Clock;

#[test]
fn clock_increments() {
    let mut clk = Clock::new();
    assert_eq!(clk.ticks(), 0);
    clk.tick();
    assert_eq!(clk.ticks(), 1);
}
