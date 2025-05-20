use core::ops::Deref;
use mik32v2_pac::{Usart0, Usart1};
pub trait Pins<U> {}

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

}

/// Implemented by all USART instances
pub trait Instance: Deref<Target = mik32v2_pac::usart_0::RegisterBlock> {
    fn ptr() -> *const mik32v2_pac::usart_0::RegisterBlock;
    fn enable_clock(rcc: &mik32v2_pac::pm::RegisterBlock);
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

                fn enable_clock(rcc: &mik32v2_pac::pm::RegisterBlock) {
                    rcc.clk_apb_p_set().modify(|_, w| w.$usartXen().set_bit());
                }
            }
        )+
    }
}

impl_instance! {
    Usart0: (usart0, uart_0),
    Usart1: (usart1, uart_1),
}