#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_scl: I2cScl,
    i2c_sda: I2cSda,
    sdio_sdclk: SdioSdclk,
    sdio_sdcmd: SdioSdcmd,
    sdio_sddata0: SdioSddata0,
    sdio_sddata1: SdioSddata1,
    sdio_sddata2: SdioSddata2,
    sdio_sddata3: SdioSddata3,
    cpi_clk: CpiClk,
    cpi_data: CpiData,
    cpi_hsync: CpiHsync,
    cpi_vsync: CpiVsync,
    spim0_clk: Spim0Clk,
    spim0_csn0: Spim0Csn0,
    spim0_sdio0: Spim0Sdio0,
    spim0_sdio1: Spim0Sdio1,
    spim0_sdio2: Spim0Sdio2,
    spim0_sdio3: Spim0Sdio3,
    spim1_clk: Spim1Clk,
    spim1_csn0: Spim1Csn0,
    spim1_sdio0: Spim1Sdio0,
    spim1_sdio1: Spim1Sdio1,
    spim1_sdio2: Spim1Sdio2,
    spim1_sdio3: Spim1Sdio3,
    uart0_rx: Uart0Rx,
    uart0_tx: Uart0Tx,
    uart1_rx: Uart1Rx,
    uart1_tx: Uart1Tx,
    spis0_sclk: Spis0Sclk,
    spis0_cs: Spis0Cs,
    spis0_sdio0: Spis0Sdio0,
    spis0_sdio0_std: Spis0Sdio0Std,
    spis0_sdio1: Spis0Sdio1,
    spis0_sdio2: Spis0Sdio2,
    spis0_sdio3: Spis0Sdio3,
    spis1_sclk: Spis1Sclk,
    spis1_cs: Spis1Cs,
    spis1_sdio0: Spis1Sdio0,
    spis1_sdio0_std: Spis1Sdio0Std,
    spis1_sdio1: Spis1Sdio1,
    spis1_sdio2: Spis1Sdio2,
    spis1_sdio3: Spis1Sdio3,
    gpio0: Gpio0,
    gpio1: Gpio1,
    gpio2: Gpio2,
    gpio3: Gpio3,
    gpio4: Gpio4,
    gpio5: Gpio5,
    gpio6: Gpio6,
    gpio7: Gpio7,
    gpio8: Gpio8,
    gpio9: Gpio9,
    gpio10: Gpio10,
    gpio11: Gpio11,
    gpio12: Gpio12,
    gpio13: Gpio13,
    gpio14: Gpio14,
    gpio15: Gpio15,
    gpio16: Gpio16,
    gpio17: Gpio17,
    gpio18: Gpio18,
    gpio19: Gpio19,
    gpio20: Gpio20,
    gpio21: Gpio21,
    gpio22: Gpio22,
    gpio23: Gpio23,
    gpio24: Gpio24,
    gpio25: Gpio25,
    gpio26: Gpio26,
    gpio27: Gpio27,
    gpio28: Gpio28,
    pad_mux0: PadMux0,
    pad_mux1: PadMux1,
    pad_mux2: PadMux2,
    pad_mux3: PadMux3,
    dma_event: DmaEvent,
    int_ack: IntAck,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn i2c_scl(&self) -> &I2cScl {
        &self.i2c_scl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn i2c_sda(&self) -> &I2cSda {
        &self.i2c_sda
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn sdio_sdclk(&self) -> &SdioSdclk {
        &self.sdio_sdclk
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn sdio_sdcmd(&self) -> &SdioSdcmd {
        &self.sdio_sdcmd
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn sdio_sddata0(&self) -> &SdioSddata0 {
        &self.sdio_sddata0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sdio_sddata1(&self) -> &SdioSddata1 {
        &self.sdio_sddata1
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn sdio_sddata2(&self) -> &SdioSddata2 {
        &self.sdio_sddata2
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn sdio_sddata3(&self) -> &SdioSddata3 {
        &self.sdio_sddata3
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn cpi_clk(&self) -> &CpiClk {
        &self.cpi_clk
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn cpi_data(&self) -> &CpiData {
        &self.cpi_data
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn cpi_hsync(&self) -> &CpiHsync {
        &self.cpi_hsync
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn cpi_vsync(&self) -> &CpiVsync {
        &self.cpi_vsync
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn spim0_clk(&self) -> &Spim0Clk {
        &self.spim0_clk
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn spim0_csn0(&self) -> &Spim0Csn0 {
        &self.spim0_csn0
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn spim0_sdio0(&self) -> &Spim0Sdio0 {
        &self.spim0_sdio0
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn spim0_sdio1(&self) -> &Spim0Sdio1 {
        &self.spim0_sdio1
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn spim0_sdio2(&self) -> &Spim0Sdio2 {
        &self.spim0_sdio2
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn spim0_sdio3(&self) -> &Spim0Sdio3 {
        &self.spim0_sdio3
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn spim1_clk(&self) -> &Spim1Clk {
        &self.spim1_clk
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn spim1_csn0(&self) -> &Spim1Csn0 {
        &self.spim1_csn0
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn spim1_sdio0(&self) -> &Spim1Sdio0 {
        &self.spim1_sdio0
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn spim1_sdio1(&self) -> &Spim1Sdio1 {
        &self.spim1_sdio1
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn spim1_sdio2(&self) -> &Spim1Sdio2 {
        &self.spim1_sdio2
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn spim1_sdio3(&self) -> &Spim1Sdio3 {
        &self.spim1_sdio3
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn uart0_rx(&self) -> &Uart0Rx {
        &self.uart0_rx
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn uart0_tx(&self) -> &Uart0Tx {
        &self.uart0_tx
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn uart1_rx(&self) -> &Uart1Rx {
        &self.uart1_rx
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn uart1_tx(&self) -> &Uart1Tx {
        &self.uart1_tx
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn spis0_sclk(&self) -> &Spis0Sclk {
        &self.spis0_sclk
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn spis0_cs(&self) -> &Spis0Cs {
        &self.spis0_cs
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn spis0_sdio0(&self) -> &Spis0Sdio0 {
        &self.spis0_sdio0
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn spis0_sdio0_std(&self) -> &Spis0Sdio0Std {
        &self.spis0_sdio0_std
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn spis0_sdio1(&self) -> &Spis0Sdio1 {
        &self.spis0_sdio1
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn spis0_sdio2(&self) -> &Spis0Sdio2 {
        &self.spis0_sdio2
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn spis0_sdio3(&self) -> &Spis0Sdio3 {
        &self.spis0_sdio3
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn spis1_sclk(&self) -> &Spis1Sclk {
        &self.spis1_sclk
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn spis1_cs(&self) -> &Spis1Cs {
        &self.spis1_cs
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn spis1_sdio0(&self) -> &Spis1Sdio0 {
        &self.spis1_sdio0
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn spis1_sdio0_std(&self) -> &Spis1Sdio0Std {
        &self.spis1_sdio0_std
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn spis1_sdio1(&self) -> &Spis1Sdio1 {
        &self.spis1_sdio1
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn spis1_sdio2(&self) -> &Spis1Sdio2 {
        &self.spis1_sdio2
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn spis1_sdio3(&self) -> &Spis1Sdio3 {
        &self.spis1_sdio3
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn gpio0(&self) -> &Gpio0 {
        &self.gpio0
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn gpio1(&self) -> &Gpio1 {
        &self.gpio1
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn gpio2(&self) -> &Gpio2 {
        &self.gpio2
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn gpio3(&self) -> &Gpio3 {
        &self.gpio3
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn gpio4(&self) -> &Gpio4 {
        &self.gpio4
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn gpio5(&self) -> &Gpio5 {
        &self.gpio5
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn gpio6(&self) -> &Gpio6 {
        &self.gpio6
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn gpio7(&self) -> &Gpio7 {
        &self.gpio7
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn gpio8(&self) -> &Gpio8 {
        &self.gpio8
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn gpio9(&self) -> &Gpio9 {
        &self.gpio9
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn gpio10(&self) -> &Gpio10 {
        &self.gpio10
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn gpio11(&self) -> &Gpio11 {
        &self.gpio11
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn gpio12(&self) -> &Gpio12 {
        &self.gpio12
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn gpio13(&self) -> &Gpio13 {
        &self.gpio13
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn gpio14(&self) -> &Gpio14 {
        &self.gpio14
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn gpio15(&self) -> &Gpio15 {
        &self.gpio15
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn gpio16(&self) -> &Gpio16 {
        &self.gpio16
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn gpio17(&self) -> &Gpio17 {
        &self.gpio17
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn gpio18(&self) -> &Gpio18 {
        &self.gpio18
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn gpio19(&self) -> &Gpio19 {
        &self.gpio19
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn gpio20(&self) -> &Gpio20 {
        &self.gpio20
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn gpio21(&self) -> &Gpio21 {
        &self.gpio21
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn gpio22(&self) -> &Gpio22 {
        &self.gpio22
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn gpio23(&self) -> &Gpio23 {
        &self.gpio23
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn gpio24(&self) -> &Gpio24 {
        &self.gpio24
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn gpio25(&self) -> &Gpio25 {
        &self.gpio25
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn gpio26(&self) -> &Gpio26 {
        &self.gpio26
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn gpio27(&self) -> &Gpio27 {
        &self.gpio27
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn gpio28(&self) -> &Gpio28 {
        &self.gpio28
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn pad_mux0(&self) -> &PadMux0 {
        &self.pad_mux0
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn pad_mux1(&self) -> &PadMux1 {
        &self.pad_mux1
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn pad_mux2(&self) -> &PadMux2 {
        &self.pad_mux2
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn pad_mux3(&self) -> &PadMux3 {
        &self.pad_mux3
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn dma_event(&self) -> &DmaEvent {
        &self.dma_event
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn int_ack(&self) -> &IntAck {
        &self.int_ack
    }
}
#[doc = "i2c_scl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_scl`]
module"]
#[doc(alias = "i2c_scl")]
pub type I2cScl = crate::Reg<i2c_scl::I2cSclSpec>;
#[doc = ""]
pub mod i2c_scl;
#[doc = "i2c_sda (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_sda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sda`]
module"]
#[doc(alias = "i2c_sda")]
pub type I2cSda = crate::Reg<i2c_sda::I2cSdaSpec>;
#[doc = ""]
pub mod i2c_sda;
#[doc = "sdio_sdclk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sdclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sdclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sdclk`]
module"]
#[doc(alias = "sdio_sdclk")]
pub type SdioSdclk = crate::Reg<sdio_sdclk::SdioSdclkSpec>;
#[doc = ""]
pub mod sdio_sdclk;
#[doc = "sdio_sdcmd (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sdcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sdcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sdcmd`]
module"]
#[doc(alias = "sdio_sdcmd")]
pub type SdioSdcmd = crate::Reg<sdio_sdcmd::SdioSdcmdSpec>;
#[doc = ""]
pub mod sdio_sdcmd;
#[doc = "sdio_sddata0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata0`]
module"]
#[doc(alias = "sdio_sddata0")]
pub type SdioSddata0 = crate::Reg<sdio_sddata0::SdioSddata0Spec>;
#[doc = ""]
pub mod sdio_sddata0;
#[doc = "sdio_sddata1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata1`]
module"]
#[doc(alias = "sdio_sddata1")]
pub type SdioSddata1 = crate::Reg<sdio_sddata1::SdioSddata1Spec>;
#[doc = ""]
pub mod sdio_sddata1;
#[doc = "sdio_sddata2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata2`]
module"]
#[doc(alias = "sdio_sddata2")]
pub type SdioSddata2 = crate::Reg<sdio_sddata2::SdioSddata2Spec>;
#[doc = ""]
pub mod sdio_sddata2;
#[doc = "sdio_sddata3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata3`]
module"]
#[doc(alias = "sdio_sddata3")]
pub type SdioSddata3 = crate::Reg<sdio_sddata3::SdioSddata3Spec>;
#[doc = ""]
pub mod sdio_sddata3;
#[doc = "cpi_clk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_clk`]
module"]
#[doc(alias = "cpi_clk")]
pub type CpiClk = crate::Reg<cpi_clk::CpiClkSpec>;
#[doc = ""]
pub mod cpi_clk;
#[doc = "cpi_data (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_data`]
module"]
#[doc(alias = "cpi_data")]
pub type CpiData = crate::Reg<cpi_data::CpiDataSpec>;
#[doc = ""]
pub mod cpi_data;
#[doc = "cpi_hsync (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_hsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_hsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_hsync`]
module"]
#[doc(alias = "cpi_hsync")]
pub type CpiHsync = crate::Reg<cpi_hsync::CpiHsyncSpec>;
#[doc = ""]
pub mod cpi_hsync;
#[doc = "spim1_csn0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_csn0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_csn0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_csn0`]
module"]
#[doc(alias = "spim1_csn0")]
pub type Spim1Csn0 = crate::Reg<spim1_csn0::Spim1Csn0Spec>;
#[doc = ""]
pub mod spim1_csn0;
#[doc = "int_ack (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ack`]
module"]
#[doc(alias = "int_ack")]
pub type IntAck = crate::Reg<int_ack::IntAckSpec>;
#[doc = ""]
pub mod int_ack;
#[doc = "dma_event (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_event::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_event::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_event`]
module"]
#[doc(alias = "dma_event")]
pub type DmaEvent = crate::Reg<dma_event::DmaEventSpec>;
#[doc = ""]
pub mod dma_event;
#[doc = "pad_mux3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mux3`]
module"]
#[doc(alias = "pad_mux3")]
pub type PadMux3 = crate::Reg<pad_mux3::PadMux3Spec>;
#[doc = ""]
pub mod pad_mux3;
#[doc = "pad_mux2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mux2`]
module"]
#[doc(alias = "pad_mux2")]
pub type PadMux2 = crate::Reg<pad_mux2::PadMux2Spec>;
#[doc = ""]
pub mod pad_mux2;
#[doc = "pad_mux1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mux1`]
module"]
#[doc(alias = "pad_mux1")]
pub type PadMux1 = crate::Reg<pad_mux1::PadMux1Spec>;
#[doc = ""]
pub mod pad_mux1;
#[doc = "pad_mux0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mux0`]
module"]
#[doc(alias = "pad_mux0")]
pub type PadMux0 = crate::Reg<pad_mux0::PadMux0Spec>;
#[doc = ""]
pub mod pad_mux0;
#[doc = "gpio28 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio28`]
module"]
#[doc(alias = "gpio28")]
pub type Gpio28 = crate::Reg<gpio28::Gpio28Spec>;
#[doc = ""]
pub mod gpio28;
#[doc = "gpio27 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio27`]
module"]
#[doc(alias = "gpio27")]
pub type Gpio27 = crate::Reg<gpio27::Gpio27Spec>;
#[doc = ""]
pub mod gpio27;
#[doc = "gpio26 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio26`]
module"]
#[doc(alias = "gpio26")]
pub type Gpio26 = crate::Reg<gpio26::Gpio26Spec>;
#[doc = ""]
pub mod gpio26;
#[doc = "gpio25 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio25`]
module"]
#[doc(alias = "gpio25")]
pub type Gpio25 = crate::Reg<gpio25::Gpio25Spec>;
#[doc = ""]
pub mod gpio25;
#[doc = "gpio24 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio24`]
module"]
#[doc(alias = "gpio24")]
pub type Gpio24 = crate::Reg<gpio24::Gpio24Spec>;
#[doc = ""]
pub mod gpio24;
#[doc = "gpio23 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio23`]
module"]
#[doc(alias = "gpio23")]
pub type Gpio23 = crate::Reg<gpio23::Gpio23Spec>;
#[doc = ""]
pub mod gpio23;
#[doc = "gpio22 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio22`]
module"]
#[doc(alias = "gpio22")]
pub type Gpio22 = crate::Reg<gpio22::Gpio22Spec>;
#[doc = ""]
pub mod gpio22;
#[doc = "gpio21 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio21`]
module"]
#[doc(alias = "gpio21")]
pub type Gpio21 = crate::Reg<gpio21::Gpio21Spec>;
#[doc = ""]
pub mod gpio21;
#[doc = "gpio20 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio20`]
module"]
#[doc(alias = "gpio20")]
pub type Gpio20 = crate::Reg<gpio20::Gpio20Spec>;
#[doc = ""]
pub mod gpio20;
#[doc = "gpio19 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio19`]
module"]
#[doc(alias = "gpio19")]
pub type Gpio19 = crate::Reg<gpio19::Gpio19Spec>;
#[doc = ""]
pub mod gpio19;
#[doc = "gpio18 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio18`]
module"]
#[doc(alias = "gpio18")]
pub type Gpio18 = crate::Reg<gpio18::Gpio18Spec>;
#[doc = ""]
pub mod gpio18;
#[doc = "gpio17 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio17`]
module"]
#[doc(alias = "gpio17")]
pub type Gpio17 = crate::Reg<gpio17::Gpio17Spec>;
#[doc = ""]
pub mod gpio17;
#[doc = "gpio16 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio16`]
module"]
#[doc(alias = "gpio16")]
pub type Gpio16 = crate::Reg<gpio16::Gpio16Spec>;
#[doc = ""]
pub mod gpio16;
#[doc = "gpio15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio15`]
module"]
#[doc(alias = "gpio15")]
pub type Gpio15 = crate::Reg<gpio15::Gpio15Spec>;
#[doc = ""]
pub mod gpio15;
#[doc = "gpio14 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio14`]
module"]
#[doc(alias = "gpio14")]
pub type Gpio14 = crate::Reg<gpio14::Gpio14Spec>;
#[doc = ""]
pub mod gpio14;
#[doc = "gpio13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio13`]
module"]
#[doc(alias = "gpio13")]
pub type Gpio13 = crate::Reg<gpio13::Gpio13Spec>;
#[doc = ""]
pub mod gpio13;
#[doc = "gpio12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio12`]
module"]
#[doc(alias = "gpio12")]
pub type Gpio12 = crate::Reg<gpio12::Gpio12Spec>;
#[doc = ""]
pub mod gpio12;
#[doc = "gpio11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio11`]
module"]
#[doc(alias = "gpio11")]
pub type Gpio11 = crate::Reg<gpio11::Gpio11Spec>;
#[doc = ""]
pub mod gpio11;
#[doc = "gpio10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio10`]
module"]
#[doc(alias = "gpio10")]
pub type Gpio10 = crate::Reg<gpio10::Gpio10Spec>;
#[doc = ""]
pub mod gpio10;
#[doc = "gpio9 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9`]
module"]
#[doc(alias = "gpio9")]
pub type Gpio9 = crate::Reg<gpio9::Gpio9Spec>;
#[doc = ""]
pub mod gpio9;
#[doc = "gpio8 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8`]
module"]
#[doc(alias = "gpio8")]
pub type Gpio8 = crate::Reg<gpio8::Gpio8Spec>;
#[doc = ""]
pub mod gpio8;
#[doc = "gpio7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio7`]
module"]
#[doc(alias = "gpio7")]
pub type Gpio7 = crate::Reg<gpio7::Gpio7Spec>;
#[doc = ""]
pub mod gpio7;
#[doc = "gpio6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio6`]
module"]
#[doc(alias = "gpio6")]
pub type Gpio6 = crate::Reg<gpio6::Gpio6Spec>;
#[doc = ""]
pub mod gpio6;
#[doc = "gpio5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5`]
module"]
#[doc(alias = "gpio5")]
pub type Gpio5 = crate::Reg<gpio5::Gpio5Spec>;
#[doc = ""]
pub mod gpio5;
#[doc = "gpio4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4`]
module"]
#[doc(alias = "gpio4")]
pub type Gpio4 = crate::Reg<gpio4::Gpio4Spec>;
#[doc = ""]
pub mod gpio4;
#[doc = "gpio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3`]
module"]
#[doc(alias = "gpio3")]
pub type Gpio3 = crate::Reg<gpio3::Gpio3Spec>;
#[doc = ""]
pub mod gpio3;
#[doc = "gpio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2`]
module"]
#[doc(alias = "gpio2")]
pub type Gpio2 = crate::Reg<gpio2::Gpio2Spec>;
#[doc = ""]
pub mod gpio2;
#[doc = "gpio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1`]
module"]
#[doc(alias = "gpio1")]
pub type Gpio1 = crate::Reg<gpio1::Gpio1Spec>;
#[doc = ""]
pub mod gpio1;
#[doc = "gpio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0`]
module"]
#[doc(alias = "gpio0")]
pub type Gpio0 = crate::Reg<gpio0::Gpio0Spec>;
#[doc = ""]
pub mod gpio0;
#[doc = "spis1_sdio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio3`]
module"]
#[doc(alias = "spis1_sdio3")]
pub type Spis1Sdio3 = crate::Reg<spis1_sdio3::Spis1Sdio3Spec>;
#[doc = ""]
pub mod spis1_sdio3;
#[doc = "spis1_sdio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio2`]
module"]
#[doc(alias = "spis1_sdio2")]
pub type Spis1Sdio2 = crate::Reg<spis1_sdio2::Spis1Sdio2Spec>;
#[doc = ""]
pub mod spis1_sdio2;
#[doc = "spis1_sdio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio1`]
module"]
#[doc(alias = "spis1_sdio1")]
pub type Spis1Sdio1 = crate::Reg<spis1_sdio1::Spis1Sdio1Spec>;
#[doc = ""]
pub mod spis1_sdio1;
#[doc = "spis1_sdio0_std (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio0_std::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio0_std::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio0_std`]
module"]
#[doc(alias = "spis1_sdio0_std")]
pub type Spis1Sdio0Std = crate::Reg<spis1_sdio0_std::Spis1Sdio0StdSpec>;
#[doc = ""]
pub mod spis1_sdio0_std;
#[doc = "spis1_sdio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio0`]
module"]
#[doc(alias = "spis1_sdio0")]
pub type Spis1Sdio0 = crate::Reg<spis1_sdio0::Spis1Sdio0Spec>;
#[doc = ""]
pub mod spis1_sdio0;
#[doc = "spis1_cs (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_cs`]
module"]
#[doc(alias = "spis1_cs")]
pub type Spis1Cs = crate::Reg<spis1_cs::Spis1CsSpec>;
#[doc = ""]
pub mod spis1_cs;
#[doc = "spis1_sclk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sclk`]
module"]
#[doc(alias = "spis1_sclk")]
pub type Spis1Sclk = crate::Reg<spis1_sclk::Spis1SclkSpec>;
#[doc = ""]
pub mod spis1_sclk;
#[doc = "spis0_sdio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio3`]
module"]
#[doc(alias = "spis0_sdio3")]
pub type Spis0Sdio3 = crate::Reg<spis0_sdio3::Spis0Sdio3Spec>;
#[doc = ""]
pub mod spis0_sdio3;
#[doc = "spis0_sdio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio2`]
module"]
#[doc(alias = "spis0_sdio2")]
pub type Spis0Sdio2 = crate::Reg<spis0_sdio2::Spis0Sdio2Spec>;
#[doc = ""]
pub mod spis0_sdio2;
#[doc = "spis0_sdio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio1`]
module"]
#[doc(alias = "spis0_sdio1")]
pub type Spis0Sdio1 = crate::Reg<spis0_sdio1::Spis0Sdio1Spec>;
#[doc = ""]
pub mod spis0_sdio1;
#[doc = "spis0_sdio0_std (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio0_std::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio0_std::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio0_std`]
module"]
#[doc(alias = "spis0_sdio0_std")]
pub type Spis0Sdio0Std = crate::Reg<spis0_sdio0_std::Spis0Sdio0StdSpec>;
#[doc = ""]
pub mod spis0_sdio0_std;
#[doc = "spis0_sdio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio0`]
module"]
#[doc(alias = "spis0_sdio0")]
pub type Spis0Sdio0 = crate::Reg<spis0_sdio0::Spis0Sdio0Spec>;
#[doc = ""]
pub mod spis0_sdio0;
#[doc = "spis0_cs (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_cs`]
module"]
#[doc(alias = "spis0_cs")]
pub type Spis0Cs = crate::Reg<spis0_cs::Spis0CsSpec>;
#[doc = ""]
pub mod spis0_cs;
#[doc = "spis0_sclk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sclk`]
module"]
#[doc(alias = "spis0_sclk")]
pub type Spis0Sclk = crate::Reg<spis0_sclk::Spis0SclkSpec>;
#[doc = ""]
pub mod spis0_sclk;
#[doc = "uart1_tx (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_tx`]
module"]
#[doc(alias = "uart1_tx")]
pub type Uart1Tx = crate::Reg<uart1_tx::Uart1TxSpec>;
#[doc = ""]
pub mod uart1_tx;
#[doc = "uart1_rx (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_rx`]
module"]
#[doc(alias = "uart1_rx")]
pub type Uart1Rx = crate::Reg<uart1_rx::Uart1RxSpec>;
#[doc = ""]
pub mod uart1_rx;
#[doc = "uart0_tx (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_tx`]
module"]
#[doc(alias = "uart0_tx")]
pub type Uart0Tx = crate::Reg<uart0_tx::Uart0TxSpec>;
#[doc = ""]
pub mod uart0_tx;
#[doc = "uart0_rx (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_rx`]
module"]
#[doc(alias = "uart0_rx")]
pub type Uart0Rx = crate::Reg<uart0_rx::Uart0RxSpec>;
#[doc = ""]
pub mod uart0_rx;
#[doc = "spim1_sdio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio3`]
module"]
#[doc(alias = "spim1_sdio3")]
pub type Spim1Sdio3 = crate::Reg<spim1_sdio3::Spim1Sdio3Spec>;
#[doc = ""]
pub mod spim1_sdio3;
#[doc = "spim1_sdio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio2`]
module"]
#[doc(alias = "spim1_sdio2")]
pub type Spim1Sdio2 = crate::Reg<spim1_sdio2::Spim1Sdio2Spec>;
#[doc = ""]
pub mod spim1_sdio2;
#[doc = "spim1_sdio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio1`]
module"]
#[doc(alias = "spim1_sdio1")]
pub type Spim1Sdio1 = crate::Reg<spim1_sdio1::Spim1Sdio1Spec>;
#[doc = ""]
pub mod spim1_sdio1;
#[doc = "spim1_sdio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio0`]
module"]
#[doc(alias = "spim1_sdio0")]
pub type Spim1Sdio0 = crate::Reg<spim1_sdio0::Spim1Sdio0Spec>;
#[doc = ""]
pub mod spim1_sdio0;
#[doc = "spim1_clk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_clk`]
module"]
#[doc(alias = "spim1_clk")]
pub type Spim1Clk = crate::Reg<spim1_clk::Spim1ClkSpec>;
#[doc = ""]
pub mod spim1_clk;
#[doc = "spim0_sdio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio3`]
module"]
#[doc(alias = "spim0_sdio3")]
pub type Spim0Sdio3 = crate::Reg<spim0_sdio3::Spim0Sdio3Spec>;
#[doc = ""]
pub mod spim0_sdio3;
#[doc = "spim0_sdio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio2`]
module"]
#[doc(alias = "spim0_sdio2")]
pub type Spim0Sdio2 = crate::Reg<spim0_sdio2::Spim0Sdio2Spec>;
#[doc = ""]
pub mod spim0_sdio2;
#[doc = "spim0_sdio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio1`]
module"]
#[doc(alias = "spim0_sdio1")]
pub type Spim0Sdio1 = crate::Reg<spim0_sdio1::Spim0Sdio1Spec>;
#[doc = ""]
pub mod spim0_sdio1;
#[doc = "spim0_sdio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio0`]
module"]
#[doc(alias = "spim0_sdio0")]
pub type Spim0Sdio0 = crate::Reg<spim0_sdio0::Spim0Sdio0Spec>;
#[doc = ""]
pub mod spim0_sdio0;
#[doc = "spim0_csn0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_csn0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_csn0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_csn0`]
module"]
#[doc(alias = "spim0_csn0")]
pub type Spim0Csn0 = crate::Reg<spim0_csn0::Spim0Csn0Spec>;
#[doc = ""]
pub mod spim0_csn0;
#[doc = "spim0_clk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_clk`]
module"]
#[doc(alias = "spim0_clk")]
pub type Spim0Clk = crate::Reg<spim0_clk::Spim0ClkSpec>;
#[doc = ""]
pub mod spim0_clk;
#[doc = "cpi_vsync (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_vsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_vsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_vsync`]
module"]
#[doc(alias = "cpi_vsync")]
pub type CpiVsync = crate::Reg<cpi_vsync::CpiVsyncSpec>;
#[doc = ""]
pub mod cpi_vsync;
