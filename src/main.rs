#![no_std]
#![no_main]
#![feature(riscv_ext_intrinsics)]

use core::{arch::riscv32::nop, mem::take, panic::PanicInfo};
use gpio::{GpioExt, Input, PinExt};
use mik32v2_pac::{epic::mask_edge_clear::Gpio, pm::ahb_mux::AhbClkMux, spi_0::delay, Peripherals};
use mik32_rt::entry;
mod rcc;
mod time;
mod serial;
// mod usart;
mod peripheral;
mod gpio;
use riscv::{self as _};

#[derive(Debug)]
enum Error {
    PeripheralsAlreadyTaken
}

struct Config {
    rcc: rcc::Config
}

impl Default for Config {
    fn default() -> Self {
        Self { rcc: Default::default() }
    }
}

fn init(config: Config) -> Result<Peripherals, Error> {
    let mut p = Peripherals::take();
    if p.is_none() {
        return Err(Error::PeripheralsAlreadyTaken);
    }
    let p = p.unwrap();

    rcc::Config::init(config.rcc);


    Ok(p)
}

#[entry]
fn main() -> ! {
    let mut device_config = Config::default();

    let p = init(device_config).unwrap();

    let gpio_2 = p.gpio8_2.split();
    let mut led = gpio_2.p8_2_7.into_output();

    let mut tx = gpio_2.p8_2_0.into_serial_port();

    let gpio_0 = p.gpio16_0.split();

    loop {
        for _ in 0..100_0000 { nop() };
        led.set_high();
        for _ in 0..100_000 { nop() };
        led.set_low();
    }
}


#[unsafe(export_name = "trap_handler")]
fn trap() {
    loop {
        
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
