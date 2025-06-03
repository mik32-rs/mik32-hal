pub mod delay;
pub use delay::*;

use crate::time::Hertz;

/// Timer wrapper
pub struct Timer<TIM> {
    pub(crate) tim: TIM,
    pub(crate) clk: Hertz,
}

