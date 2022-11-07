#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::pio::*;
use embassy_rp::pio_instr_util;
use embassy_time::{Duration, Timer};
use pio_proc::pio_file;
use rp2040_hal as _;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let program = pio_file!("blink.pio", select_program("blink"),).program;
    let origin = program.origin.unwrap_or(0);
    let (_, sm0, _, _, _) = p.PIO0.split();
    let led = sm0.make_pio_pin(p.PIN_25);
    // sm0.set_out_pins(&[&led]);
    sm0.set_set_pins(&[&led]);
    sm0.set_set_range(25, 1);
    sm0.set_wrap(program.wrap.source + origin, program.wrap.target + origin);
    sm0.write_instr(origin as usize, &program.code);
    pio_instr_util::exec_jmp(&sm0, origin);
    // sm0.set_clkdiv((125e6 / 2e3 * 256.0) as u32);
    // sm0.set_autopull(true);
    // sm0.set_out_shift_dir(ShiftDirection::Left);
    // sm0.set_clkdiv(u32::MAX);
    sm0.set_enable(true);
    sm0.wait_push((125_000_000 / (10 * 2)) - 3).await;

    let mut freq = 10;
    loop {
        info!("tick");
        // sm0.push_tx((125_000_000 / (freq * 2)) - 3);
        // freq = if freq == 5 { 10 } else { freq - 1 };
        Timer::after(Duration::from_secs(1)).await;
    }
}
