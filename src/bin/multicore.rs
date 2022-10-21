#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::raw::TaskPool;
use embassy_executor::Executor;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::PIN_25;
use embassy_time::{Duration, Timer};
use futures::Future;
use hal::multicore::{Multicore, Stack};
use hal::pac;
use rp2040_hal as hal;
use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<4096> = Stack::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    let led = Output::new(p.PIN_25, Level::Low);

    let mut pac = unsafe { pac::Peripherals::steal() };
    let mut sio = hal::sio::Sio::new(pac.SIO);
    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);

    let cores = mc.cores();
    let core1 = &mut cores[1];
    let _core1 = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || {
        run_executor(1, core1_task(led))
    });

    run_executor(0, core0_task())
}

fn run_executor<F: Future + 'static>(id: u8, f: F) -> ! {
    let mut task_pool = TaskPool::<F, 1>::new();
    let task_pool: &mut TaskPool<F, 1> = unsafe { core::mem::transmute(&mut task_pool) };

    let mut executor: Executor = Executor::new();
    let executor: &mut Executor = unsafe { core::mem::transmute(&mut executor) };

    info!("Starting Core {} executor...", id);

    executor.run(|spawner| {
        let token = task_pool.spawn(move || f);
        spawner.must_spawn(token);
    });
}

async fn core1_task(mut led: Output<'static, PIN_25>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}

async fn core0_task() {
    loop {
        info!("Hello from CORE0");
        Timer::after(Duration::from_secs(1)).await;
    }
}
