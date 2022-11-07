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
        "pull block",
        "mov x, osr",
        "pull block",
        "mov y, osr",
        "jmp !x end",
        "loop:",
        "jmp !osre step",
        "mov osr, y",
        "step:",
        "out pins, 4 [31]"
        "jmp x-- loop",
        "end:",
        "irq 0"
    )
    .program;

    let (_, sm0, _, _, _) = p.PIO0.split();
    let pin0 = sm0.make_pio_pin(p.PIN_6);
    let pin1 = sm0.make_pio_pin(p.PIN_7);
    let pin2 = sm0.make_pio_pin(p.PIN_8);
    let pin3 = sm0.make_pio_pin(p.PIN_9);
    // sm0.set_set_pins(&[&pin0, &pin1, &pin2, &pin3]);

    sm0.set_out_pins(&[&pin0, &pin1, &pin2, &pin3]);
    sm0.set_set_range(6, 4);
    pio_instr_util::set_out_pindir(&sm0, 0b0000);
    pio_instr_util::set_pindir(&sm0, 0b1111);
    pio_instr_util::set_out_pin(&sm0, 0b0000);
    // sm0.set_out_shift_dir(ShiftDirection::Right);
    // sm0.set_in_shift_dir(ShiftDirection::Left);

    // sm0.set_clkdiv(u32::MAX);
    sm0.set_clkdiv(200);
    sm0.write_instr(program.origin.unwrap_or(0).into(), &program.code);
    // sm0.set_wrap(program.wrap.source + 0, program.wrap.target + 0);
    // pio_instr_util::set_pindir(&sm0, 0b1111);
    // pio_instr_util::exec_jmp(&sm0, 0);
    // sm0.push_tx(500);
    // sm0.exec_instr(
    //     pio::InstructionOperands::PULL {
    //         if_empty: false,
    //         block: false,
    //     }
    //     .encode(),
    // );

    let fw = [(1, 2, 4, 8), (2, 4, 8, 1), (4, 8, 1, 2), (8, 1, 2, 4)];
    let rw = [(8, 4, 2, 1), (4, 2, 1, 8), (2, 1, 8, 4), (1, 8, 4, 2)];
    let dirs = [
        306713160, 2216789025, 919156425, 2623773795, 321277065, 2563007025,
    ];
    let mut steps = 0;

    // let mut freq = 10;
    sm0.set_enable(true);
    let mut dir = 0;
    loop {
        let data = dirs[dir % 6];
        dir += 1;
        // let idx = steps % 4;

        // let mut a = data[idx].0 | (data[idx].1 << 4) | (data[idx].2 << 8) | (data[idx].3 << 12);
        // a = a << 16 | a;

        // sm0.clkdiv_restart();
        pio_instr_util::exec_jmp(&sm0, 0);
        sm0.wait_push(400).await;
        sm0.wait_push(data).await;

        // info!("tick {:04b}", data[idx]);
        sm0.wait_irq(0).await;
        // sm0.set_enable(false);
        steps += 400;
        info!("done {}", steps);
        // sm0.restart();
        // sm0.wait_push(freq).await;
        // sm0.exec_instr(
        //     pio::InstructionOperands::PULL {
        //         if_empty: false,
        //         block: false,
        //     }
        //     .encode(),
        // );
        // freq += 10;
        // sm0.wait_irq(3).await;
        // info!("tick");
        Timer::after(Duration::from_millis(100)).await;
    }
}
