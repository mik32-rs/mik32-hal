use crate::timer::Timer;
riscv

/// Timer as a delay provider (SysTick by default)
pub struct SysDelay(Timer<SYST>);