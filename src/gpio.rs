//! General Purpose Input / Output
 
use core::marker::PhantomData;
mod partially_erased;
pub use partially_erased::{PEPin, PartiallyErasedPin};
 
/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

pub trait PinExt {
    type Mode;
    /// Return pin number
    fn pin_id(&self) -> u8;
    /// Return port number
    fn port_id(&self) -> u8;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PinState {
    /// Low pin state
    Low,
    /// High pin state
    High,
}

/// Generic pin type
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
/// - `N` is pin number: from `0` to `15`.
pub struct Pin<const P: u8, const N: u8, MODE = Input<Floating>> {
    _mode: PhantomData<MODE>,
}

impl<const P: u8, const N: u8, MODE> Pin<P, N, MODE> {
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<const P: u8, const N: u8, MODE> Pin<P, N, MODE> {
    /// Set the output of the pin regardless of its mode.
    /// Primarily used to set the output value of the pin
    /// before changing its mode to an output to avoid
    /// a short spike of an incorrect value
    #[inline(always)]
    fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::High => self._set_high(),
            PinState::Low => self._set_low(),
        }
    }
    #[inline(always)]
    fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).set().write(|w| w.bits(1 << N)); }
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).clear().write(|w| w.bits(1 << N)); }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).output().read().bits() & (1 << N) == 0 }
    }
}

impl<const P: u8, const N: u8, MODE> Pin<P, N, MODE> {
    /// Configures the pin to operate as an push pull output pin
    /// Initial state will be low.
    pub fn into_output(mut self) -> Pin<P, N, Output> {
        self._set_low();

        unsafe {
            (*Gpio::<P>::ptr()).direction_out().write(|w| w.bits(1 << N));
        }
        Pin::new()
    }
}


impl<const P: u8, const N: u8, MODE> PinExt for Pin<P, N, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        N
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        P as u8 - b'A'
    }
}


impl<const P: u8, const N: u8> Pin<P, N, Output> {
    #[inline(always)]
    pub fn set_high(&mut self) {
        self._set_high()
    }

    #[inline(always)]
    pub fn set_low(&mut self) {
        self._set_low()
    }

    #[inline(always)]
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }

    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_low()
    }

    #[inline(always)]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
 
/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output;

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopxenr:ident, $port_id:expr, $PXn:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr $(, $MODE:ty)?),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;
            use mik32v2_pac::$GPIOX;

            use mik32v2_pac::Pm;
            use super::{GpioExt, Input, Floating};

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi $(<$MODE>)?,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    // NOTE(unsafe) This executes only during initialisation
                    let pm = unsafe { &(*Pm::ptr()) };
                    pm.clk_apb_p_set().modify(|_, w| w.$iopxenr().set_bit());

                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            pub type $PXn<MODE> = super::PEPin<$port_id, MODE>;

            $(
                pub type $PXi<MODE = Input<Floating>> = super::Pin<$port_id, $i, MODE>;
            )+
        }

        pub use $gpiox::{ $($PXi,)+ };
    }
}

gpio!(Gpio16_0, gpio16_0, gpio_0, 0, P16_0_n, [
    P16_0_0: (p16_0_0, 0),
    P16_0_1: (p16_0_1, 1),
    P16_0_2: (p16_0_2, 2),
    P16_0_3: (p16_0_3, 3),
    P16_0_4: (p16_0_4, 4),
    P16_0_5: (p16_0_5, 5),
    P16_0_6: (p16_0_6, 6),
    P16_0_7: (p16_0_7, 7),
    P16_0_8: (p16_0_8, 8),
    P16_0_9: (p16_0_9, 9),
    P16_0_10: (p16_0_10, 10),
    P16_0_11: (p16_0_11, 11),
    P16_0_12: (p16_0_12, 12),
    P16_0_13: (p16_0_13, 13),
    P16_0_14: (p16_0_14, 14),
    P16_0_15: (p16_0_15, 15),
]);

gpio!(Gpio8_2, gpio8_2, gpio_2, 2, P8_2_n, [
    P8_0_0: (p8_0_0, 0),
    P8_0_1: (p8_0_1, 1),
    P8_0_2: (p8_0_2, 2),
    P8_0_3: (p8_0_3, 3),
    P8_0_4: (p8_0_4, 4),
    P8_0_5: (p8_0_5, 5),
    P8_0_6: (p8_0_6, 6),
    P8_0_7: (p8_0_7, 7),
]);



struct Gpio<const P: u8>;
impl<const P: u8> Gpio<P> {
    const fn ptr() -> *const mik32v2_pac::gpio16_0::RegisterBlock {
        match P {
            0 => mik32v2_pac::Gpio16_0::ptr(),
            1 => mik32v2_pac::Gpio16_1::ptr() as _,
            2 => mik32v2_pac::Gpio8_2::ptr() as _,
            _ => mik32v2_pac::Gpio16_0::ptr(),
        }
    }
}