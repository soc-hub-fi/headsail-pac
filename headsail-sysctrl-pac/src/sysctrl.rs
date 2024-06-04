#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    gpio: GPIO,
    _reserved1: [u8; 0x0fc8],
    udma: UDMA,
    _reserved2: [u8; 0x1ed4],
    soc_control: SOC_CONTROL,
    _reserved3: [u8; 0x01d0],
    boot_config: BOOT_CONFIG,
    _reserved4: [u8; 0x28],
    eth_pll: ETH_PLL,
    dla_pll: DLA_PLL,
    hpc_pll: HPC_PLL,
    ddr2_pll: DDR2_PLL,
    dsp_pll: DSP_PLL,
    icn_pll: ICN_PLL,
    c2c_ser_pll: C2C_SER_PLL,
    c2c_par_pll: C2C_PAR_PLL,
    _reserved12: [u8; 0x0100],
    ema_ss: EMA_SS,
    _reserved13: [u8; 0x0bf0],
    advanced_timer: ADVANCED_TIMER,
    _reserved14: [u8; 0x0ef8],
    soc_event_generator: SOC_EVENT_GENERATOR,
    _reserved15: [u8; 0x2f74],
    event_interrupt_unit: EVENT_INTERRUPT_UNIT,
    _reserved16: [u8; 0x1fd8],
    timer: TIMER,
    _reserved17: [u8; 0x0001_4fd8],
    sdio: SDIO,
}
impl RegisterBlock {
    #[doc = "0x1000..0x1038 - GPIO"]
    #[inline(always)]
    pub const fn gpio(&self) -> &GPIO {
        &self.gpio
    }
    #[doc = "0x2000..0x212c - UDMA"]
    #[inline(always)]
    pub const fn udma(&self) -> &UDMA {
        &self.udma
    }
    #[doc = "0x4000 - SocControl"]
    #[inline(always)]
    pub const fn soc_control(&self) -> &SOC_CONTROL {
        &self.soc_control
    }
    #[doc = "0x41d0..0x41d8 - BootConfig"]
    #[inline(always)]
    pub const fn boot_config(&self) -> &BOOT_CONFIG {
        &self.boot_config
    }
    #[doc = "0x4200..0x4220 - ETH_PLL"]
    #[inline(always)]
    pub const fn eth_pll(&self) -> &ETH_PLL {
        &self.eth_pll
    }
    #[doc = "0x4220..0x4240 - DLA_PLL"]
    #[inline(always)]
    pub const fn dla_pll(&self) -> &DLA_PLL {
        &self.dla_pll
    }
    #[doc = "0x4240..0x4260 - HPC_PLL"]
    #[inline(always)]
    pub const fn hpc_pll(&self) -> &HPC_PLL {
        &self.hpc_pll
    }
    #[doc = "0x4260..0x4280 - DDR2_PLL"]
    #[inline(always)]
    pub const fn ddr2_pll(&self) -> &DDR2_PLL {
        &self.ddr2_pll
    }
    #[doc = "0x4280..0x42a0 - DSP_PLL"]
    #[inline(always)]
    pub const fn dsp_pll(&self) -> &DSP_PLL {
        &self.dsp_pll
    }
    #[doc = "0x42a0..0x42c0 - ICN_PLL"]
    #[inline(always)]
    pub const fn icn_pll(&self) -> &ICN_PLL {
        &self.icn_pll
    }
    #[doc = "0x42c0..0x42e0 - C2C_SER_PLL"]
    #[inline(always)]
    pub const fn c2c_ser_pll(&self) -> &C2C_SER_PLL {
        &self.c2c_ser_pll
    }
    #[doc = "0x42e0..0x4300 - C2C_PAR_PLL"]
    #[inline(always)]
    pub const fn c2c_par_pll(&self) -> &C2C_PAR_PLL {
        &self.c2c_par_pll
    }
    #[doc = "0x4400..0x4410 - EMA_SS"]
    #[inline(always)]
    pub const fn ema_ss(&self) -> &EMA_SS {
        &self.ema_ss
    }
    #[doc = "0x5000..0x5108 - AdvancedTimer"]
    #[inline(always)]
    pub const fn advanced_timer(&self) -> &ADVANCED_TIMER {
        &self.advanced_timer
    }
    #[doc = "0x6000..0x608c - SocEventGenerator"]
    #[inline(always)]
    pub const fn soc_event_generator(&self) -> &SOC_EVENT_GENERATOR {
        &self.soc_event_generator
    }
    #[doc = "0x9000..0x9028 - EventInterruptUnit"]
    #[inline(always)]
    pub const fn event_interrupt_unit(&self) -> &EVENT_INTERRUPT_UNIT {
        &self.event_interrupt_unit
    }
    #[doc = "0xb000..0xb028 - Timer"]
    #[inline(always)]
    pub const fn timer(&self) -> &TIMER {
        &self.timer
    }
    #[doc = "0x20000..0x20060 - SDIO"]
    #[inline(always)]
    pub const fn sdio(&self) -> &SDIO {
        &self.sdio
    }
}
#[doc = "GPIO"]
pub use self::gpio::GPIO;
#[doc = r"Cluster"]
#[doc = "GPIO"]
pub mod gpio;
#[doc = "UDMA"]
pub use self::udma::UDMA;
#[doc = r"Cluster"]
#[doc = "UDMA"]
pub mod udma;
#[doc = "AdvancedTimer"]
pub use self::advanced_timer::ADVANCED_TIMER;
#[doc = r"Cluster"]
#[doc = "AdvancedTimer"]
pub mod advanced_timer;
#[doc = "SocEventGenerator"]
pub use self::soc_event_generator::SOC_EVENT_GENERATOR;
#[doc = r"Cluster"]
#[doc = "SocEventGenerator"]
pub mod soc_event_generator;
#[doc = "EventInterruptUnit"]
pub use self::event_interrupt_unit::EVENT_INTERRUPT_UNIT;
#[doc = r"Cluster"]
#[doc = "EventInterruptUnit"]
pub mod event_interrupt_unit;
#[doc = "Timer"]
pub use self::timer::TIMER;
#[doc = r"Cluster"]
#[doc = "Timer"]
pub mod timer;
#[doc = "SDIO"]
pub use self::sdio::SDIO;
#[doc = r"Cluster"]
#[doc = "SDIO"]
pub mod sdio;
#[doc = "SocControl"]
pub use self::soc_control::SOC_CONTROL;
#[doc = r"Cluster"]
#[doc = "SocControl"]
pub mod soc_control;
#[doc = "ETH_PLL"]
pub use self::eth_pll::ETH_PLL;
#[doc = r"Cluster"]
#[doc = "ETH_PLL"]
pub mod eth_pll;
#[doc = "DLA_PLL"]
pub use self::dla_pll::DLA_PLL;
#[doc = r"Cluster"]
#[doc = "DLA_PLL"]
pub mod dla_pll;
#[doc = "HPC_PLL"]
pub use self::hpc_pll::HPC_PLL;
#[doc = r"Cluster"]
#[doc = "HPC_PLL"]
pub mod hpc_pll;
#[doc = "DDR2_PLL"]
pub use self::ddr2_pll::DDR2_PLL;
#[doc = r"Cluster"]
#[doc = "DDR2_PLL"]
pub mod ddr2_pll;
#[doc = "ICN_PLL"]
pub use self::icn_pll::ICN_PLL;
#[doc = r"Cluster"]
#[doc = "ICN_PLL"]
pub mod icn_pll;
#[doc = "C2C_PAR_PLL"]
pub use self::c2c_par_pll::C2C_PAR_PLL;
#[doc = r"Cluster"]
#[doc = "C2C_PAR_PLL"]
pub mod c2c_par_pll;
#[doc = "DSP_PLL"]
pub use self::dsp_pll::DSP_PLL;
#[doc = r"Cluster"]
#[doc = "DSP_PLL"]
pub mod dsp_pll;
#[doc = "BootConfig"]
pub use self::boot_config::BOOT_CONFIG;
#[doc = r"Cluster"]
#[doc = "BootConfig"]
pub mod boot_config;
#[doc = "C2C_SER_PLL"]
pub use self::c2c_ser_pll::C2C_SER_PLL;
#[doc = r"Cluster"]
#[doc = "C2C_SER_PLL"]
pub mod c2c_ser_pll;
#[doc = "EMA_SS"]
pub use self::ema_ss::EMA_SS;
#[doc = r"Cluster"]
#[doc = "EMA_SS"]
pub mod ema_ss;
