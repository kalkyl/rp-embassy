#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::pio::*;
use embassy_rp::pio_instr_util;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use pio_proc::pio_file;
use rp2040_hal as _;
use {defmt_rtt as _, panic_probe as _};

use pio::{Instruction, InstructionOperands, OutDestination};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let program = pio_file!("pwm.pio", select_program("pwm"),).program;
    let origin = program.origin.unwrap_or(0);
    let (_, sm0, _, _, _) = p.PIO0.split();
    sm0.write_instr(origin as usize, &program.code);
    let led = sm0.make_pio_pin(p.PIN_25);
    sm0.set_sideset_base_pin(&led);
    sm0.set_sideset_count(1);
    sm0.set_out_shift_dir(ShiftDirection::Left);
    // sm0.set_side_pindir(true);
    // sm0.set_clkdiv(u32::MAX);

    // sm0.set_out_pins(&[&led]);
    // sm0.set_set_range(25, 1);
    sm0.set_wrap(program.wrap.source + origin, program.wrap.target + origin);
    // sm0.set_in_shift_dir(ShiftDirection::Right);
    // sm0.write_instr(origin as usize, &program.code);
    pio_instr_util::exec_jmp(&sm0, origin);

    sm0.set_enable(true);

    while !sm0.is_tx_empty() {}
    // sm0.set_enable(false);

    let ok = sm0.try_push_tx(u16::MAX as u32 - 1);
    info!("period {}", ok);
    // sm0.set_enable(true);

    while !sm0.is_tx_empty() {}
    info!("cool");
    // sm0.set_enable(false);
    sm0.exec_instr(
        InstructionOperands::PULL {
            if_empty: false,
            block: false,
        }
        .encode(),
    );

    while !sm0.is_tx_empty() {}
    info!("cool");
    sm0.exec_instr(
        InstructionOperands::OUT {
            destination: OutDestination::ISR,
            bit_count: 32,
        }
        .encode(),
    );
    sm0.set_enable(true);
    sm0.push_tx(u16::MAX as u32 / 2);

    while !sm0.is_tx_empty() {}
    info!("cool");
    // sm0.set_autopull(true);

    let mut level = 0;
    loop {
        sm0.wait_push(level * level).await;
        level = (level + 1) % 256;
        Timer::after(Duration::from_millis(100)).await;
    }
}
