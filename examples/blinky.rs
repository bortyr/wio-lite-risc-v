#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use wio_lite_riscv::hal::{pac, prelude::*};
use wio_lite_riscv::hal::delay::McycleDelay;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.configure().freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let mut delay = McycleDelay::new(&rcu.clocks);
    // PA8 is a "User LED" near USB Type C connector
    let mut led = gpioa.pa8.into_push_pull_output();
    loop {
        led.set_high().unwrap();
        delay.delay_ms(1000_u32);
        led.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }
}
