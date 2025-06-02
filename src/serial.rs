use core::{error::Error, fmt, marker::PhantomData, ops::Deref};
use mik32v2_pac::{crypto::config, Pm, Usart0, Usart1};
use embedded_hal_nb::serial::{ErrorKind, ErrorType, Read, Write};
use nb::block;
use riscv::register::mcounteren::write;
use core::ptr;

use crate::gpio::{self, Func2Mode};

/// Serial error
#[derive(Debug)]
pub enum SerialError {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
}

pub trait Pins<U> {}
pub trait PinTx<U> {}
pub trait PinRx<U> {}

impl<U, TX, RX> Pins<U> for (TX, RX)
where
    TX: PinTx<U>,
    RX: PinRx<U>,
{
}


impl PinTx<Usart0> for gpio::P16_0_6<Func2Mode> {}
impl PinRx<Usart0> for gpio::P16_0_5<Func2Mode> {}

impl PinTx<Usart1> for gpio::P16_1_9<Func2Mode> {}
impl PinRx<Usart1> for gpio::P16_1_8<Func2Mode> {}



/// Serial abstraction
pub struct Serial<U, PINS> {
    usart: U,
    pins: PINS,
}

impl<U, PINS> Serial<U, PINS>
where
    PINS: Pins<U>,
    U: Instance,
{
    pub fn new(usart: U, pins: PINS, config: Config) -> Self {
        let pm = unsafe { &(*Pm::ptr()) };
        U::enable_clock(&pm);

        // TODO: Calculate correct baudrate divisor on the fly
        usart.divider().modify(|_, w| unsafe { w.brr().bits(0xd05) }); 

        // Enable tx / rx and reset USART
        usart.control1().modify(|_, w| w
            .te().enable()
            .re().enable()
            .ue().enable()
        );

        while usart.flags().read().teack().bit_is_clear() {};

        Serial { usart, pins }
    }

    pub fn split(self) -> (Tx<U>, Rx<U>) {
        (
            Tx {
                _usart: PhantomData,
            },
            Rx {
                _usart: PhantomData,
            },
        )
    }

    pub fn release(self) -> (U, PINS) {
        (self.usart, self.pins)
    }
}

/// USART configuration
pub struct Config {}

/// Serial transmitter
pub struct Tx<U> {
    _usart: PhantomData<U>,
}

impl<U> ErrorType for Tx<U>
where
    U: Instance,
{
    type Error = ErrorKind;
}

impl<U> Write<u8> for Tx<U>
where
    U: Instance,
{
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let isr = unsafe { (*U::ptr()).flags().read() };

        if isr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        unsafe {
            (*U::ptr()).txdata().write(|w| w.tdr().bits(byte as u16));
            while (*U::ptr()).flags().read().tc().bit_is_clear() {};
        }
        Ok(())
    }
}

/// Serial receiver
pub struct Rx<U> {
    _usart: PhantomData<U>,
}

/// Implemented by all USART instances
pub trait Instance: Deref<Target = mik32v2_pac::usart_0::RegisterBlock> {
    fn ptr() -> *const mik32v2_pac::usart_0::RegisterBlock;
    fn enable_clock(pm: &mik32v2_pac::pm::RegisterBlock);
}

macro_rules! impl_instance {
    ($(
        $USARTX:ident: ($usartX:ident, $usartXen:ident),
    )+) => {
        $(
            impl Instance for $USARTX {
                fn ptr() -> *const mik32v2_pac::usart_0::RegisterBlock {
                    $USARTX::ptr()
                }

                fn enable_clock(pm: &mik32v2_pac::pm::RegisterBlock) {
                    pm.clk_apb_p_set().modify(|_, w| w.$usartXen().set_bit());
                }
            }
        )+
    }
}

impl_instance! {
    Usart0: (usart0, uart_0),
    Usart1: (usart1, uart_1),
}

impl<U> fmt::Write for Tx<U>
where
    Tx<U>: embedded_hal_nb::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = s.as_bytes().iter().map(|c| block!(self.write(*c))).last();
        Ok(())
    }
}