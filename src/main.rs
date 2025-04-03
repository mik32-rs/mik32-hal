#![no_std]
#![no_main]
#![feature(riscv_ext_intrinsics)]

use core::{panic::PanicInfo};
use mik32v2_pac::{spi_0::delay, Peripherals};
use mik32_rt::entry;
mod rcc;
mod time;
use riscv::{self as _};


#[entry]
fn main() -> ! {
    loop {
        
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
