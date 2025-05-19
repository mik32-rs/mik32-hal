use super::*;

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

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(mut self) -> Pin<P, N, Input<Floating>> {
        unsafe {
            (*Gpio::<P>::ptr()).direction_in().write(|w| w.bits(1 << N));
            let mask = 0b11 << 2 * N;
            let value = 0b00 << 2 * N;
            match P {
                0 => (*mik32v2_pac::PadConfig::ptr()).pad0_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                1 => (*mik32v2_pac::PadConfig::ptr()).pad1_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                2 => (*mik32v2_pac::PadConfig::ptr()).pad2_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                _ => panic!("Invalid GPIO port number: {}", P)
            }
        };

        Pin::new()
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(mut self) -> Pin<P, N, Input<Floating>> {
        unsafe {
            (*Gpio::<P>::ptr()).direction_in().write(|w| w.bits(1 << N));
            let mask = 0b11 << 2 * N;
            let value = 0b10 << 2 * N;
            match P {
                0 => (*mik32v2_pac::PadConfig::ptr()).pad0_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                1 => (*mik32v2_pac::PadConfig::ptr()).pad1_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                2 => (*mik32v2_pac::PadConfig::ptr()).pad2_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                _ => panic!("Invalid GPIO port number: {}", P)
            }
        };

        Pin::new()
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(mut self) -> Pin<P, N, Input<Floating>> {
        unsafe {
            (*Gpio::<P>::ptr()).direction_in().write(|w| w.bits(1 << N));
            let mask = 0b11 << 2 * N;
            let value = 0b01 << 2 * N;
            match P {
                0 => (*mik32v2_pac::PadConfig::ptr()).pad0_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                1 => (*mik32v2_pac::PadConfig::ptr()).pad1_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                2 => (*mik32v2_pac::PadConfig::ptr()).pad2_pupd()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                _ => panic!("Invalid GPIO port number: {}", P)
            }
        };

        Pin::new()
    }

    /// Configures the output to work in the serial port
    pub fn into_serial_port(mut self) -> Pin<P, N, SerialMode> {
        unsafe {
            (*Gpio::<P>::ptr()).direction_in().write(|w| w.bits(1 << N));
            let mask = 0b11 << 2 * N;
            let value = 0b01 << 2 * N;
            match P {
                0 => (*mik32v2_pac::PadConfig::ptr()).pad0_cfg()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                1 => (*mik32v2_pac::PadConfig::ptr()).pad1_cfg()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                2 => (*mik32v2_pac::PadConfig::ptr()).pad2_cfg()
                .modify(|r, w| w.bits((r.bits() & !mask) | value)),
                _ => panic!("Invalid GPIO port number: {}", P)
            }
        };

        Pin::new()
    }
}