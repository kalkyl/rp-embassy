#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::i2c::{Config, I2c};
use embassy_time::{Duration, Timer};
use embassy_rp::{interrupt};
use {defmt_rtt as _, panic_probe as _};
use pca9685_async::*;
use rp2040_hal as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    
    let irq = interrupt::take!(I2C0_IRQ);
    
    let scl = p.PIN_5;
    let sda = p.PIN_4;
    let mut config = Config::default();
    config.frequency = 400_000;
    
    info!("Hello World!");
    let i2c = I2c::new_async(p.I2C0, scl, sda, irq, config);

    info!("I2c");

    let mut pwm = Pca9685::new(i2c, Address::default()).unwrap();
    info!("Pwm");

    
    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).await.unwrap();
    info!("Prescale");
    
    // It is necessary to enable the device.
    pwm.enable().await.unwrap();
    info!("Enable");
    
    // Turn on channel 0 at 0.
    pwm.set_channel_on(Channel::C0, 0).await.unwrap();    
    loop { 
        info!("0 deg");
        pwm.set_channel_off(Channel::C0, 100).await.unwrap();
        Timer::after(Duration::from_millis(500)).await;

        info!("90 deg");
        pwm.set_channel_off(Channel::C0, 385).await.unwrap();
        Timer::after(Duration::from_millis(500)).await;
         
        info!("180 deg");
        pwm.set_channel_off(Channel::C0, 670).await.unwrap();
        Timer::after(Duration::from_millis(500)).await;
        
        info!("0 deg");
        pwm.set_channel_off(Channel::C0, 384).await.unwrap();
        Timer::after(Duration::from_millis(500)).await;
    }
}
