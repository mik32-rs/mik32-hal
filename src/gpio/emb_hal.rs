use core::convert::Infallible;

use super::*;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

// Implementations for `Pin`
impl<const P: u8, const N: u8, MODE> ErrorType for Pin<P, N, MODE> {
    type Error = Infallible;
}

impl<const P: u8, const N: u8> OutputPin for Pin<P, N, Output> {
    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<const P: u8, const N: u8> StatefulOutputPin for Pin<P, N, Output> {
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_high(self))
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_low(self))
    }
}

impl<const P: u8, const N: u8, MODE> InputPin for Pin<P, N, Input<MODE>> {
    /// Is the input pin high?
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_high(self))
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_low(self))
    }
}

// Implementations for `PartiallyErasedPin`
impl<const P: u8, MODE> ErrorType for PartiallyErasedPin<P, MODE> {
    type Error = Infallible;
}

impl<const P: u8> OutputPin for PartiallyErasedPin<P, Output> {
    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<const P: u8> StatefulOutputPin for PartiallyErasedPin<P, Output> {
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_high(self))
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_low(self))
    }
}

impl<const P: u8, MODE> InputPin for PartiallyErasedPin<P, Input<MODE>> {
    /// Is the input pin high?
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_high(self))
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_low(self))
    }
}

