#![no_main]
#![no_std]

use cortex_m::asm;
use defmt_rtt as _; // global logger
use nrf52840_hal as _;
use nrf52840_hal::gpio::{Level, p1};

use nrf52840_hal::pac::Peripherals;
use nrf52840_hal::prelude::OutputPin;

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let gpio = p1::Parts::new(peripherals.P1);
    let mut led = gpio.p1_12.into_push_pull_output(Level::Low).degrade();

    loop {
        defmt::info!("blinky");
        led.set_high().unwrap();
        asm::delay(136_000_000);
        led.set_low().unwrap();
        asm::delay(136_000_000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked");
    exit()
}

pub fn exit() -> ! {
    loop {
        asm::bkpt();
    }
}
