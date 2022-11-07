#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::pio::*;
use embassy_time::{Duration, Timer};
use rp2040_hal as _;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Define some simple PIO program.
    let program = pio_proc::pio_asm!(
        "set pindirs, 1",
        "loop:",
        "set pins, 1 [31]",
        "set pins, 1 [31]",
        "set pins, 1 [31]",
        "set pins, 1 [31]",
        "set pins, 0 [31]",
        "set pins, 0 [31]",
        "set pins, 0 [31]",
        "set pins, 0 [31]",
        "jmp loop [31]",
    )
    .program;

    let (_, sm0, _, _, _) = p.PIO0.split();
    let led = sm0.make_pio_pin(p.PIN_25);
    sm0.set_set_pins(&[&led]);
    sm0.set_clkdiv(u32::MAX);
    sm0.write_instr(program.origin.unwrap_or(0).into(), &program.code);
    sm0.set_enable(true);

    loop {
        info!("tick");
        Timer::after(Duration::from_secs(1)).await;
    }
}
