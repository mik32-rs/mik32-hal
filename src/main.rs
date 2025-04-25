#![no_std]
#![no_main]
#![feature(riscv_ext_intrinsics)]

use core::{arch::riscv32::nop, mem::take, panic::PanicInfo};
use gpio::{GpioExt, Input, PinExt};
use mik32v2_pac::{epic::mask_edge_clear::Gpio, pm::ahb_mux::AhbClkMux, spi_0::delay, Peripherals};
use mik32_rt::entry;
mod rcc;
mod time;
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

    let gpio_0 = p.gpio16_0.split();

    let input0 = gpio_0.p16_0_0.into_pull_up_input();
    let input1 = gpio_0.p16_0_1.into_pull_up_input();
    let input2 = gpio_0.p16_0_2.into_pull_down_input();
    let input3 = gpio_0.p16_0_3.into_pull_up_input();
    let input4 = gpio_0.p16_0_4.into_floating_input();
    let mut input5 = gpio_0.p16_0_5.into_pull_up_input();

    input5 = input5.into_floating_input();


    loop {
        input0.is_high();
        input1.is_high();
        input2.is_high();
        input3.is_high();
        input4.is_high();
        input5.is_high();
        led.toggle();
        for _ in 0..100_000_0 {
            nop()
        }
        led.toggle();
        for _ in 0..100_000_0 {
            nop()
        }
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
