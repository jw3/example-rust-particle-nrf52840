#![no_main]
#![no_std]

use cortex_m::asm;
use defmt::dbg;
use defmt_rtt as _;
use embedded_graphics::fonts::{Font12x16, Font6x12, Font6x8, Font8x16};
// global logger
use nrf52840_hal as _;
use nrf52840_hal::gpio::{Level, p0, p1};

use nrf52840_hal::pac::Peripherals;
use nrf52840_hal::prelude::OutputPin;

use embedded_graphics::prelude::*;
use nrf52840_hal::{twim, Twim};
use ssd1306::Builder;
use ssd1306::prelude::*;


#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let port0 = p0::Parts::new(p.P0);
    let scl = port0.p0_27.into_floating_input().degrade();
    let sda = port0.p0_26.into_floating_input().degrade();

    let pins = twim::Pins { scl, sda };
    let i2c = Twim::new(p.TWIM1, pins, twim::Frequency::K100);

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().expect("Display initialization");
    disp.flush().expect("Cleans the display");

    disp.draw(
        Font8x16::render_str("Hello Rust!")
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new(10, 40))
            .into_iter(),
    );

    disp.flush().expect("Render display");

    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked, {}", info.location().unwrap().line());
    exit()
}

pub fn exit() -> ! {
    loop {
        asm::bkpt();
    }
}
