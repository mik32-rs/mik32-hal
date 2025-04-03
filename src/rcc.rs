use mik32v2_pac::generic::Resettable;
use mik32v2_pac::pm::ahb_mux::AhbClkMux;
use mik32v2_pac::wake_up::clocks_bu::{self, RtcClkMux};
use mik32v2_pac::pm::cpu_rtc_clk_mux::CpuRtcClkMux;
use mik32v2_pac::wake_up::clocks_sys::Force32kClk;
use mik32v2_pac::pm::ahb_mux::ForceMux;
use mik32v2_pac::pm::DivAhb;
use mik32v2_pac::{Pm, WakeUp};
use crate::time::Hertz;

const CLOCKSWITCH_TIMEOUT_VALUE: u32 = 500_000;

pub const HSI32M_FREQ: Hertz = Hertz(32_000_000);
pub const OSC32M_FREQ: Hertz = Hertz(32_000_000);
pub const LSI32K_FREQ: Hertz = Hertz(32_000);
pub const OSC32K_FREQ: Hertz = Hertz(32_000);

pub struct FreqMonitir {
    pub sys: AhbClkMux,
    pub force_osc_sys: ForceMux,
    pub force32k_clk: Force32kClk,
}

impl Default for FreqMonitir {
    fn default() -> Self {
        Self { 
            sys: AhbClkMux::Osc32m,
            force_osc_sys: ForceMux::Unfixed,
            force32k_clk: Force32kClk::Automatic,
        }
    }
}

pub struct Config {
    pub hsi32m: bool,
    pub osc32m: bool,
    pub lsi32k: bool,
    pub osc32k: bool,
    pub freq_monitor: FreqMonitir,

    pub ahb_div: u8,
    pub apb_m_div: u8,
    pub apb_p_div: u8,

    pub hsi32m_calibration_value: u8,
    pub lsi32k_calibration_value: u8,

    pub rtcclk: RtcClkMux,
    pub rtccpuclk: CpuRtcClkMux,
    
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            hsi32m: true,
            osc32m: true,
            lsi32k: true,
            osc32k: true,
            freq_monitor: FreqMonitir::default(),
            ahb_div: 0,
            apb_m_div: 0,
            apb_p_div: 0,
            hsi32m_calibration_value: 128,
            lsi32k_calibration_value: 8,
            rtcclk: RtcClkMux::Automatic,
            rtccpuclk: CpuRtcClkMux::Osc32k,
        }
    }
}

impl Config {
    fn init(config: Config) {
        let wu = unsafe { WakeUp::steal() };
        let pm = unsafe { Pm::steal() };
        wu.clocks_sys().modify(|_, w| w
            .hsi32m_en().enable()
            .osc32m_en().enable()
        );

        wu.clocks_bu().modify(|_, w| w
            .lsi32k_en().enable()
            .osc32k_en().enable()
        );

        wu.clocks_sys().modify(|_, w| unsafe { w.adj_hsi32m().bits(config.hsi32m_calibration_value) });
        wu.clocks_bu().modify(|_, w| unsafe { w.adj_lsi32k().bits(config.lsi32k_calibration_value) });

        wu.clocks_sys().modify(|_, w| match config.freq_monitor.force32k_clk {
            Force32kClk::Automatic => w.force_32k_clk().automatic(),
            Force32kClk::Lsi32k => w.force_32k_clk().lsi32k(),
            Force32kClk::Osc32k => w.force_32k_clk().osc32k(),
        });

        pm.ahb_mux().modify(|_, w| match config.freq_monitor.force_osc_sys {
            ForceMux::Unfixed => w.force_mux().unfixed(),
            ForceMux::Fixed => w.force_mux().fixed(),
        });

        pm.ahb_mux().modify(|_, w| match config.freq_monitor.sys {
            AhbClkMux::Osc32m => w.ahb_clk_mux().osc32m(),
            AhbClkMux::Hsi32m => w.ahb_clk_mux().hsi32m(),
            AhbClkMux::Osc32k => w.ahb_clk_mux().osc32k(),
            AhbClkMux::Lsi32k => w.ahb_clk_mux().lsi32k(),
        });

        wu.clocks_bu().modify(|_, w| match config.rtcclk {
            RtcClkMux::Automatic => w.rtc_clk_mux().automatic(),
            RtcClkMux::Lsi32k => w.rtc_clk_mux().lsi32k(),
            RtcClkMux::Osc32k => w.rtc_clk_mux().osc32k(),
        });
        wu.rtc_control().reset();

        pm.cpu_rtc_clk_mux().modify(|_, w| match config.rtccpuclk{
            CpuRtcClkMux::Osc32k => w.cpu_rtc_clk_mux().osc32k(),
            CpuRtcClkMux::Lsi32k => w.cpu_rtc_clk_mux().osc32k(),
        });

        if !config.osc32m {
            wu.clocks_sys().modify(|_, w| w
                .osc32m_en().disable()
            );
        }

        if !config.hsi32m {
            wu.clocks_sys().modify(|_, w| w
                .hsi32m_en().disable()
            );
        }

        if !config.osc32k {
            wu.clocks_bu().modify(|_, w| w
                .osc32k_en().disable()
            );
        }

        if !config.lsi32k {
            wu.clocks_bu().modify(|_, w| w
                .lsi32k_en().disable()
            );
        }
    }
}