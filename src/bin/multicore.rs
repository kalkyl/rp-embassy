#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Executor;
use embassy_executor::_export::StaticCell;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::PIN_25;
use embassy_time::{Duration, Timer};
use hal::multicore::{Multicore, Stack};
use hal::pac;
use rp2040_hal as hal;
use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<4096> = Stack::new();

static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

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
        let executor1 = EXECUTOR0.init(Executor::new());
        executor1.run(|spawner| unwrap!(spawner.spawn(core1_task(led))));
    });

    let executor0 = EXECUTOR1.init(Executor::new());
    executor0.run(|spawner| unwrap!(spawner.spawn(core0_task())));
}

#[embassy_executor::task]
async fn core1_task(mut led: Output<'static, PIN_25>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn core0_task() {
    loop {
        info!("Hello from CORE0");
        Timer::after(Duration::from_secs(1)).await;
    }
}
