#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::pio::*;
use embassy_rp::pio_instr_util;
use embassy_time::{Duration, Timer};
use rp2040_hal as _;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Define some simple PIO program.
    let program = pio_proc::pio_asm!(
        // "set pindirs, 1",
        // "set y, 4"
        ".wrap_target",
        // "countloop:",
        "set pins, 1",
        "mov x, osr",
        "delay_high:",
        "nop",
        "jmp x--, delay_high",
        "set pins, 0",
        "mov x, osr",
        "delay_low:",
        "nop",
        "jmp x--, delay_low",
        // "jmp y--, countloop",
        // "irq 3"
        ".wrap"
    )
    .program;

    let (_, sm0, _, _, _) = p.PIO0.split();
    let led = sm0.make_pio_pin(p.PIN_25);
    sm0.set_set_pins(&[&led]);
    sm0.set_set_range(25, 1);
    sm0.set_clkdiv(u32::MAX);
    sm0.write_instr(program.origin.unwrap_or(0).into(), &program.code);
    sm0.set_wrap(program.wrap.source + 0, program.wrap.target + 0);
    pio_instr_util::set_pindir(&sm0, 1);
    // pio_instr_util::exec_jmp(&sm0, 0);
    sm0.push_tx(500);
    sm0.exec_instr(
        pio::InstructionOperands::PULL {
            if_empty: false,
            block: false,
        }
        .encode(),
    );
    sm0.set_enable(true);

    let mut freq = 10;
    loop {
        sm0.wait_push(freq).await;
        sm0.exec_instr(
            pio::InstructionOperands::PULL {
                if_empty: false,
                block: false,
            }
            .encode(),
        );
        freq += 10;
        // sm0.wait_irq(3).await;
        // info!("tick");
        Timer::after(Duration::from_millis(100)).await;
    }
}
