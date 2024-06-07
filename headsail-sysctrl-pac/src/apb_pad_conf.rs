#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_scl: I2C_SCL,
    i2c_sda: I2C_SDA,
    sdio_sdclk: SDIO_SDCLK,
    sdio_sdcmd: SDIO_SDCMD,
    sdio_sddata0: SDIO_SDDATA0,
    sdio_sddata1: SDIO_SDDATA1,
    sdio_sddata2: SDIO_SDDATA2,
    sdio_sddata3: SDIO_SDDATA3,
    cpi_clk: CPI_CLK,
    cpi_data: CPI_DATA,
    cpi_hsync: CPI_HSYNC,
    cpi_vsync: CPI_VSYNC,
    spim0_clk: SPIM0_CLK,
    spim0_csn0: SPIM0_CSN0,
    spim0_sdio0: SPIM0_SDIO0,
    spim0_sdio1: SPIM0_SDIO1,
    spim0_sdio2: SPIM0_SDIO2,
    spim0_sdio3: SPIM0_SDIO3,
    spim1_clk: SPIM1_CLK,
    spim1_csn0: SPIM1_CSN0,
    spim1_sdio0: SPIM1_SDIO0,
    spim1_sdio1: SPIM1_SDIO1,
    spim1_sdio2: SPIM1_SDIO2,
    spim1_sdio3: SPIM1_SDIO3,
    uart0_rx: UART0_RX,
    uart0_tx: UART0_TX,
    uart1_rx: UART1_RX,
    uart1_tx: UART1_TX,
    spis0_sclk: SPIS0_SCLK,
    spis0_cs: SPIS0_CS,
    spis0_sdio0: SPIS0_SDIO0,
    spis0_sdio0_std: SPIS0_SDIO0_STD,
    spis0_sdio1: SPIS0_SDIO1,
    spis0_sdio2: SPIS0_SDIO2,
    spis0_sdio3: SPIS0_SDIO3,
    spis1_sclk: SPIS1_SCLK,
    spis1_cs: SPIS1_CS,
    spis1_sdio0: SPIS1_SDIO0,
    spis1_sdio0_std: SPIS1_SDIO0_STD,
    spis1_sdio1: SPIS1_SDIO1,
    spis1_sdio2: SPIS1_SDIO2,
    spis1_sdio3: SPIS1_SDIO3,
    gpio0: GPIO0,
    gpio1: GPIO1,
    gpio2: GPIO2,
    gpio3: GPIO3,
    gpio4: GPIO4,
    gpio5: GPIO5,
    gpio6: GPIO6,
    gpio7: GPIO7,
    gpio8: GPIO8,
    gpio9: GPIO9,
    gpio10: GPIO10,
    gpio11: GPIO11,
    gpio12: GPIO12,
    gpio13: GPIO13,
    gpio14: GPIO14,
    gpio15: GPIO15,
    gpio16: GPIO16,
    gpio17: GPIO17,
    gpio18: GPIO18,
    gpio19: GPIO19,
    gpio20: GPIO20,
    gpio21: GPIO21,
    gpio22: GPIO22,
    gpio23: GPIO23,
    gpio24: GPIO24,
    gpio25: GPIO25,
    gpio26: GPIO26,
    gpio27: GPIO27,
    gpio28: GPIO28,
    pad_mux0: PAD_MUX0,
    pad_mux1: PAD_MUX1,
    pad_mux2: PAD_MUX2,
    pad_mux3: PAD_MUX3,
    dma_event: DMA_EVENT,
    int_ack: INT_ACK,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn i2c_scl(&self) -> &I2C_SCL {
        &self.i2c_scl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn i2c_sda(&self) -> &I2C_SDA {
        &self.i2c_sda
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn sdio_sdclk(&self) -> &SDIO_SDCLK {
        &self.sdio_sdclk
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn sdio_sdcmd(&self) -> &SDIO_SDCMD {
        &self.sdio_sdcmd
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn sdio_sddata0(&self) -> &SDIO_SDDATA0 {
        &self.sdio_sddata0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sdio_sddata1(&self) -> &SDIO_SDDATA1 {
        &self.sdio_sddata1
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn sdio_sddata2(&self) -> &SDIO_SDDATA2 {
        &self.sdio_sddata2
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn sdio_sddata3(&self) -> &SDIO_SDDATA3 {
        &self.sdio_sddata3
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn cpi_clk(&self) -> &CPI_CLK {
        &self.cpi_clk
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn cpi_data(&self) -> &CPI_DATA {
        &self.cpi_data
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn cpi_hsync(&self) -> &CPI_HSYNC {
        &self.cpi_hsync
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn cpi_vsync(&self) -> &CPI_VSYNC {
        &self.cpi_vsync
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn spim0_clk(&self) -> &SPIM0_CLK {
        &self.spim0_clk
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn spim0_csn0(&self) -> &SPIM0_CSN0 {
        &self.spim0_csn0
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn spim0_sdio0(&self) -> &SPIM0_SDIO0 {
        &self.spim0_sdio0
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn spim0_sdio1(&self) -> &SPIM0_SDIO1 {
        &self.spim0_sdio1
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn spim0_sdio2(&self) -> &SPIM0_SDIO2 {
        &self.spim0_sdio2
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn spim0_sdio3(&self) -> &SPIM0_SDIO3 {
        &self.spim0_sdio3
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn spim1_clk(&self) -> &SPIM1_CLK {
        &self.spim1_clk
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn spim1_csn0(&self) -> &SPIM1_CSN0 {
        &self.spim1_csn0
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn spim1_sdio0(&self) -> &SPIM1_SDIO0 {
        &self.spim1_sdio0
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn spim1_sdio1(&self) -> &SPIM1_SDIO1 {
        &self.spim1_sdio1
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn spim1_sdio2(&self) -> &SPIM1_SDIO2 {
        &self.spim1_sdio2
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn spim1_sdio3(&self) -> &SPIM1_SDIO3 {
        &self.spim1_sdio3
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn uart0_rx(&self) -> &UART0_RX {
        &self.uart0_rx
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn uart0_tx(&self) -> &UART0_TX {
        &self.uart0_tx
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn uart1_rx(&self) -> &UART1_RX {
        &self.uart1_rx
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn uart1_tx(&self) -> &UART1_TX {
        &self.uart1_tx
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn spis0_sclk(&self) -> &SPIS0_SCLK {
        &self.spis0_sclk
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn spis0_cs(&self) -> &SPIS0_CS {
        &self.spis0_cs
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn spis0_sdio0(&self) -> &SPIS0_SDIO0 {
        &self.spis0_sdio0
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn spis0_sdio0_std(&self) -> &SPIS0_SDIO0_STD {
        &self.spis0_sdio0_std
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn spis0_sdio1(&self) -> &SPIS0_SDIO1 {
        &self.spis0_sdio1
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn spis0_sdio2(&self) -> &SPIS0_SDIO2 {
        &self.spis0_sdio2
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn spis0_sdio3(&self) -> &SPIS0_SDIO3 {
        &self.spis0_sdio3
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn spis1_sclk(&self) -> &SPIS1_SCLK {
        &self.spis1_sclk
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn spis1_cs(&self) -> &SPIS1_CS {
        &self.spis1_cs
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn spis1_sdio0(&self) -> &SPIS1_SDIO0 {
        &self.spis1_sdio0
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn spis1_sdio0_std(&self) -> &SPIS1_SDIO0_STD {
        &self.spis1_sdio0_std
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn spis1_sdio1(&self) -> &SPIS1_SDIO1 {
        &self.spis1_sdio1
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn spis1_sdio2(&self) -> &SPIS1_SDIO2 {
        &self.spis1_sdio2
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn spis1_sdio3(&self) -> &SPIS1_SDIO3 {
        &self.spis1_sdio3
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn gpio0(&self) -> &GPIO0 {
        &self.gpio0
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn gpio1(&self) -> &GPIO1 {
        &self.gpio1
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn gpio2(&self) -> &GPIO2 {
        &self.gpio2
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn gpio3(&self) -> &GPIO3 {
        &self.gpio3
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn gpio4(&self) -> &GPIO4 {
        &self.gpio4
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn gpio5(&self) -> &GPIO5 {
        &self.gpio5
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn gpio6(&self) -> &GPIO6 {
        &self.gpio6
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn gpio7(&self) -> &GPIO7 {
        &self.gpio7
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn gpio8(&self) -> &GPIO8 {
        &self.gpio8
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn gpio9(&self) -> &GPIO9 {
        &self.gpio9
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn gpio10(&self) -> &GPIO10 {
        &self.gpio10
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn gpio11(&self) -> &GPIO11 {
        &self.gpio11
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn gpio12(&self) -> &GPIO12 {
        &self.gpio12
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn gpio13(&self) -> &GPIO13 {
        &self.gpio13
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn gpio14(&self) -> &GPIO14 {
        &self.gpio14
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn gpio15(&self) -> &GPIO15 {
        &self.gpio15
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn gpio16(&self) -> &GPIO16 {
        &self.gpio16
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn gpio17(&self) -> &GPIO17 {
        &self.gpio17
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn gpio18(&self) -> &GPIO18 {
        &self.gpio18
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn gpio19(&self) -> &GPIO19 {
        &self.gpio19
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn gpio20(&self) -> &GPIO20 {
        &self.gpio20
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn gpio21(&self) -> &GPIO21 {
        &self.gpio21
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn gpio22(&self) -> &GPIO22 {
        &self.gpio22
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn gpio23(&self) -> &GPIO23 {
        &self.gpio23
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn gpio24(&self) -> &GPIO24 {
        &self.gpio24
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn gpio25(&self) -> &GPIO25 {
        &self.gpio25
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn gpio26(&self) -> &GPIO26 {
        &self.gpio26
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn gpio27(&self) -> &GPIO27 {
        &self.gpio27
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn gpio28(&self) -> &GPIO28 {
        &self.gpio28
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn pad_mux0(&self) -> &PAD_MUX0 {
        &self.pad_mux0
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn pad_mux1(&self) -> &PAD_MUX1 {
        &self.pad_mux1
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn pad_mux2(&self) -> &PAD_MUX2 {
        &self.pad_mux2
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn pad_mux3(&self) -> &PAD_MUX3 {
        &self.pad_mux3
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn dma_event(&self) -> &DMA_EVENT {
        &self.dma_event
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn int_ack(&self) -> &INT_ACK {
        &self.int_ack
    }
}
#[doc = "i2c_scl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_scl`]
module"]
#[doc(alias = "i2c_scl")]
pub type I2C_SCL = crate::Reg<i2c_scl::I2C_SCL_SPEC>;
#[doc = ""]
pub mod i2c_scl;
#[doc = "i2c_sda (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_sda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sda`]
module"]
#[doc(alias = "i2c_sda")]
pub type I2C_SDA = crate::Reg<i2c_sda::I2C_SDA_SPEC>;
#[doc = ""]
pub mod i2c_sda;
#[doc = "sdio_sdclk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sdclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sdclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sdclk`]
module"]
#[doc(alias = "sdio_sdclk")]
pub type SDIO_SDCLK = crate::Reg<sdio_sdclk::SDIO_SDCLK_SPEC>;
#[doc = ""]
pub mod sdio_sdclk;
#[doc = "sdio_sdcmd (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sdcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sdcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sdcmd`]
module"]
#[doc(alias = "sdio_sdcmd")]
pub type SDIO_SDCMD = crate::Reg<sdio_sdcmd::SDIO_SDCMD_SPEC>;
#[doc = ""]
pub mod sdio_sdcmd;
#[doc = "sdio_sddata0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata0`]
module"]
#[doc(alias = "sdio_sddata0")]
pub type SDIO_SDDATA0 = crate::Reg<sdio_sddata0::SDIO_SDDATA0_SPEC>;
#[doc = ""]
pub mod sdio_sddata0;
#[doc = "sdio_sddata1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata1`]
module"]
#[doc(alias = "sdio_sddata1")]
pub type SDIO_SDDATA1 = crate::Reg<sdio_sddata1::SDIO_SDDATA1_SPEC>;
#[doc = ""]
pub mod sdio_sddata1;
#[doc = "sdio_sddata2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata2`]
module"]
#[doc(alias = "sdio_sddata2")]
pub type SDIO_SDDATA2 = crate::Reg<sdio_sddata2::SDIO_SDDATA2_SPEC>;
#[doc = ""]
pub mod sdio_sddata2;
#[doc = "sdio_sddata3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata3`]
module"]
#[doc(alias = "sdio_sddata3")]
pub type SDIO_SDDATA3 = crate::Reg<sdio_sddata3::SDIO_SDDATA3_SPEC>;
#[doc = ""]
pub mod sdio_sddata3;
#[doc = "cpi_clk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_clk`]
module"]
#[doc(alias = "cpi_clk")]
pub type CPI_CLK = crate::Reg<cpi_clk::CPI_CLK_SPEC>;
#[doc = ""]
pub mod cpi_clk;
#[doc = "cpi_data (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_data`]
module"]
#[doc(alias = "cpi_data")]
pub type CPI_DATA = crate::Reg<cpi_data::CPI_DATA_SPEC>;
#[doc = ""]
pub mod cpi_data;
#[doc = "cpi_hsync (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_hsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_hsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_hsync`]
module"]
#[doc(alias = "cpi_hsync")]
pub type CPI_HSYNC = crate::Reg<cpi_hsync::CPI_HSYNC_SPEC>;
#[doc = ""]
pub mod cpi_hsync;
#[doc = "spim1_csn0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_csn0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_csn0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_csn0`]
module"]
#[doc(alias = "spim1_csn0")]
pub type SPIM1_CSN0 = crate::Reg<spim1_csn0::SPIM1_CSN0_SPEC>;
#[doc = ""]
pub mod spim1_csn0;
#[doc = "int_ack (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ack`]
module"]
#[doc(alias = "int_ack")]
pub type INT_ACK = crate::Reg<int_ack::INT_ACK_SPEC>;
#[doc = ""]
pub mod int_ack;
#[doc = "dma_event (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_event::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_event::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_event`]
module"]
#[doc(alias = "dma_event")]
pub type DMA_EVENT = crate::Reg<dma_event::DMA_EVENT_SPEC>;
#[doc = ""]
pub mod dma_event;
#[doc = "pad_mux3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mux3`]
module"]
#[doc(alias = "pad_mux3")]
pub type PAD_MUX3 = crate::Reg<pad_mux3::PAD_MUX3_SPEC>;
#[doc = ""]
pub mod pad_mux3;
#[doc = "pad_mux2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mux2`]
module"]
#[doc(alias = "pad_mux2")]
pub type PAD_MUX2 = crate::Reg<pad_mux2::PAD_MUX2_SPEC>;
#[doc = ""]
pub mod pad_mux2;
#[doc = "pad_mux1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mux1`]
module"]
#[doc(alias = "pad_mux1")]
pub type PAD_MUX1 = crate::Reg<pad_mux1::PAD_MUX1_SPEC>;
#[doc = ""]
pub mod pad_mux1;
#[doc = "pad_mux0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mux0`]
module"]
#[doc(alias = "pad_mux0")]
pub type PAD_MUX0 = crate::Reg<pad_mux0::PAD_MUX0_SPEC>;
#[doc = ""]
pub mod pad_mux0;
#[doc = "gpio28 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio28`]
module"]
#[doc(alias = "gpio28")]
pub type GPIO28 = crate::Reg<gpio28::GPIO28_SPEC>;
#[doc = ""]
pub mod gpio28;
#[doc = "gpio27 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio27`]
module"]
#[doc(alias = "gpio27")]
pub type GPIO27 = crate::Reg<gpio27::GPIO27_SPEC>;
#[doc = ""]
pub mod gpio27;
#[doc = "gpio26 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio26`]
module"]
#[doc(alias = "gpio26")]
pub type GPIO26 = crate::Reg<gpio26::GPIO26_SPEC>;
#[doc = ""]
pub mod gpio26;
#[doc = "gpio25 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio25`]
module"]
#[doc(alias = "gpio25")]
pub type GPIO25 = crate::Reg<gpio25::GPIO25_SPEC>;
#[doc = ""]
pub mod gpio25;
#[doc = "gpio24 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio24`]
module"]
#[doc(alias = "gpio24")]
pub type GPIO24 = crate::Reg<gpio24::GPIO24_SPEC>;
#[doc = ""]
pub mod gpio24;
#[doc = "gpio23 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio23`]
module"]
#[doc(alias = "gpio23")]
pub type GPIO23 = crate::Reg<gpio23::GPIO23_SPEC>;
#[doc = ""]
pub mod gpio23;
#[doc = "gpio22 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio22`]
module"]
#[doc(alias = "gpio22")]
pub type GPIO22 = crate::Reg<gpio22::GPIO22_SPEC>;
#[doc = ""]
pub mod gpio22;
#[doc = "gpio21 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio21`]
module"]
#[doc(alias = "gpio21")]
pub type GPIO21 = crate::Reg<gpio21::GPIO21_SPEC>;
#[doc = ""]
pub mod gpio21;
#[doc = "gpio20 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio20`]
module"]
#[doc(alias = "gpio20")]
pub type GPIO20 = crate::Reg<gpio20::GPIO20_SPEC>;
#[doc = ""]
pub mod gpio20;
#[doc = "gpio19 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio19`]
module"]
#[doc(alias = "gpio19")]
pub type GPIO19 = crate::Reg<gpio19::GPIO19_SPEC>;
#[doc = ""]
pub mod gpio19;
#[doc = "gpio18 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio18`]
module"]
#[doc(alias = "gpio18")]
pub type GPIO18 = crate::Reg<gpio18::GPIO18_SPEC>;
#[doc = ""]
pub mod gpio18;
#[doc = "gpio17 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio17`]
module"]
#[doc(alias = "gpio17")]
pub type GPIO17 = crate::Reg<gpio17::GPIO17_SPEC>;
#[doc = ""]
pub mod gpio17;
#[doc = "gpio16 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio16`]
module"]
#[doc(alias = "gpio16")]
pub type GPIO16 = crate::Reg<gpio16::GPIO16_SPEC>;
#[doc = ""]
pub mod gpio16;
#[doc = "gpio15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio15`]
module"]
#[doc(alias = "gpio15")]
pub type GPIO15 = crate::Reg<gpio15::GPIO15_SPEC>;
#[doc = ""]
pub mod gpio15;
#[doc = "gpio14 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio14`]
module"]
#[doc(alias = "gpio14")]
pub type GPIO14 = crate::Reg<gpio14::GPIO14_SPEC>;
#[doc = ""]
pub mod gpio14;
#[doc = "gpio13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio13`]
module"]
#[doc(alias = "gpio13")]
pub type GPIO13 = crate::Reg<gpio13::GPIO13_SPEC>;
#[doc = ""]
pub mod gpio13;
#[doc = "gpio12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio12`]
module"]
#[doc(alias = "gpio12")]
pub type GPIO12 = crate::Reg<gpio12::GPIO12_SPEC>;
#[doc = ""]
pub mod gpio12;
#[doc = "gpio11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio11`]
module"]
#[doc(alias = "gpio11")]
pub type GPIO11 = crate::Reg<gpio11::GPIO11_SPEC>;
#[doc = ""]
pub mod gpio11;
#[doc = "gpio10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio10`]
module"]
#[doc(alias = "gpio10")]
pub type GPIO10 = crate::Reg<gpio10::GPIO10_SPEC>;
#[doc = ""]
pub mod gpio10;
#[doc = "gpio9 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9`]
module"]
#[doc(alias = "gpio9")]
pub type GPIO9 = crate::Reg<gpio9::GPIO9_SPEC>;
#[doc = ""]
pub mod gpio9;
#[doc = "gpio8 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8`]
module"]
#[doc(alias = "gpio8")]
pub type GPIO8 = crate::Reg<gpio8::GPIO8_SPEC>;
#[doc = ""]
pub mod gpio8;
#[doc = "gpio7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio7`]
module"]
#[doc(alias = "gpio7")]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = ""]
pub mod gpio7;
#[doc = "gpio6 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio6`]
module"]
#[doc(alias = "gpio6")]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = ""]
pub mod gpio6;
#[doc = "gpio5 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5`]
module"]
#[doc(alias = "gpio5")]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = ""]
pub mod gpio5;
#[doc = "gpio4 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4`]
module"]
#[doc(alias = "gpio4")]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = ""]
pub mod gpio4;
#[doc = "gpio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3`]
module"]
#[doc(alias = "gpio3")]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = ""]
pub mod gpio3;
#[doc = "gpio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2`]
module"]
#[doc(alias = "gpio2")]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = ""]
pub mod gpio2;
#[doc = "gpio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1`]
module"]
#[doc(alias = "gpio1")]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = ""]
pub mod gpio1;
#[doc = "gpio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0`]
module"]
#[doc(alias = "gpio0")]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = ""]
pub mod gpio0;
#[doc = "spis1_sdio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio3`]
module"]
#[doc(alias = "spis1_sdio3")]
pub type SPIS1_SDIO3 = crate::Reg<spis1_sdio3::SPIS1_SDIO3_SPEC>;
#[doc = ""]
pub mod spis1_sdio3;
#[doc = "spis1_sdio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio2`]
module"]
#[doc(alias = "spis1_sdio2")]
pub type SPIS1_SDIO2 = crate::Reg<spis1_sdio2::SPIS1_SDIO2_SPEC>;
#[doc = ""]
pub mod spis1_sdio2;
#[doc = "spis1_sdio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio1`]
module"]
#[doc(alias = "spis1_sdio1")]
pub type SPIS1_SDIO1 = crate::Reg<spis1_sdio1::SPIS1_SDIO1_SPEC>;
#[doc = ""]
pub mod spis1_sdio1;
#[doc = "spis1_sdio0_std (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio0_std::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio0_std::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio0_std`]
module"]
#[doc(alias = "spis1_sdio0_std")]
pub type SPIS1_SDIO0_STD = crate::Reg<spis1_sdio0_std::SPIS1_SDIO0_STD_SPEC>;
#[doc = ""]
pub mod spis1_sdio0_std;
#[doc = "spis1_sdio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio0`]
module"]
#[doc(alias = "spis1_sdio0")]
pub type SPIS1_SDIO0 = crate::Reg<spis1_sdio0::SPIS1_SDIO0_SPEC>;
#[doc = ""]
pub mod spis1_sdio0;
#[doc = "spis1_cs (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_cs`]
module"]
#[doc(alias = "spis1_cs")]
pub type SPIS1_CS = crate::Reg<spis1_cs::SPIS1_CS_SPEC>;
#[doc = ""]
pub mod spis1_cs;
#[doc = "spis1_sclk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sclk`]
module"]
#[doc(alias = "spis1_sclk")]
pub type SPIS1_SCLK = crate::Reg<spis1_sclk::SPIS1_SCLK_SPEC>;
#[doc = ""]
pub mod spis1_sclk;
#[doc = "spis0_sdio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio3`]
module"]
#[doc(alias = "spis0_sdio3")]
pub type SPIS0_SDIO3 = crate::Reg<spis0_sdio3::SPIS0_SDIO3_SPEC>;
#[doc = ""]
pub mod spis0_sdio3;
#[doc = "spis0_sdio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio2`]
module"]
#[doc(alias = "spis0_sdio2")]
pub type SPIS0_SDIO2 = crate::Reg<spis0_sdio2::SPIS0_SDIO2_SPEC>;
#[doc = ""]
pub mod spis0_sdio2;
#[doc = "spis0_sdio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio1`]
module"]
#[doc(alias = "spis0_sdio1")]
pub type SPIS0_SDIO1 = crate::Reg<spis0_sdio1::SPIS0_SDIO1_SPEC>;
#[doc = ""]
pub mod spis0_sdio1;
#[doc = "spis0_sdio0_std (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio0_std::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio0_std::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio0_std`]
module"]
#[doc(alias = "spis0_sdio0_std")]
pub type SPIS0_SDIO0_STD = crate::Reg<spis0_sdio0_std::SPIS0_SDIO0_STD_SPEC>;
#[doc = ""]
pub mod spis0_sdio0_std;
#[doc = "spis0_sdio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio0`]
module"]
#[doc(alias = "spis0_sdio0")]
pub type SPIS0_SDIO0 = crate::Reg<spis0_sdio0::SPIS0_SDIO0_SPEC>;
#[doc = ""]
pub mod spis0_sdio0;
#[doc = "spis0_cs (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_cs`]
module"]
#[doc(alias = "spis0_cs")]
pub type SPIS0_CS = crate::Reg<spis0_cs::SPIS0_CS_SPEC>;
#[doc = ""]
pub mod spis0_cs;
#[doc = "spis0_sclk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sclk`]
module"]
#[doc(alias = "spis0_sclk")]
pub type SPIS0_SCLK = crate::Reg<spis0_sclk::SPIS0_SCLK_SPEC>;
#[doc = ""]
pub mod spis0_sclk;
#[doc = "uart1_tx (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_tx`]
module"]
#[doc(alias = "uart1_tx")]
pub type UART1_TX = crate::Reg<uart1_tx::UART1_TX_SPEC>;
#[doc = ""]
pub mod uart1_tx;
#[doc = "uart1_rx (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_rx`]
module"]
#[doc(alias = "uart1_rx")]
pub type UART1_RX = crate::Reg<uart1_rx::UART1_RX_SPEC>;
#[doc = ""]
pub mod uart1_rx;
#[doc = "uart0_tx (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_tx`]
module"]
#[doc(alias = "uart0_tx")]
pub type UART0_TX = crate::Reg<uart0_tx::UART0_TX_SPEC>;
#[doc = ""]
pub mod uart0_tx;
#[doc = "uart0_rx (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_rx`]
module"]
#[doc(alias = "uart0_rx")]
pub type UART0_RX = crate::Reg<uart0_rx::UART0_RX_SPEC>;
#[doc = ""]
pub mod uart0_rx;
#[doc = "spim1_sdio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio3`]
module"]
#[doc(alias = "spim1_sdio3")]
pub type SPIM1_SDIO3 = crate::Reg<spim1_sdio3::SPIM1_SDIO3_SPEC>;
#[doc = ""]
pub mod spim1_sdio3;
#[doc = "spim1_sdio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio2`]
module"]
#[doc(alias = "spim1_sdio2")]
pub type SPIM1_SDIO2 = crate::Reg<spim1_sdio2::SPIM1_SDIO2_SPEC>;
#[doc = ""]
pub mod spim1_sdio2;
#[doc = "spim1_sdio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio1`]
module"]
#[doc(alias = "spim1_sdio1")]
pub type SPIM1_SDIO1 = crate::Reg<spim1_sdio1::SPIM1_SDIO1_SPEC>;
#[doc = ""]
pub mod spim1_sdio1;
#[doc = "spim1_sdio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio0`]
module"]
#[doc(alias = "spim1_sdio0")]
pub type SPIM1_SDIO0 = crate::Reg<spim1_sdio0::SPIM1_SDIO0_SPEC>;
#[doc = ""]
pub mod spim1_sdio0;
#[doc = "spim1_clk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_clk`]
module"]
#[doc(alias = "spim1_clk")]
pub type SPIM1_CLK = crate::Reg<spim1_clk::SPIM1_CLK_SPEC>;
#[doc = ""]
pub mod spim1_clk;
#[doc = "spim0_sdio3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio3`]
module"]
#[doc(alias = "spim0_sdio3")]
pub type SPIM0_SDIO3 = crate::Reg<spim0_sdio3::SPIM0_SDIO3_SPEC>;
#[doc = ""]
pub mod spim0_sdio3;
#[doc = "spim0_sdio2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio2`]
module"]
#[doc(alias = "spim0_sdio2")]
pub type SPIM0_SDIO2 = crate::Reg<spim0_sdio2::SPIM0_SDIO2_SPEC>;
#[doc = ""]
pub mod spim0_sdio2;
#[doc = "spim0_sdio1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio1`]
module"]
#[doc(alias = "spim0_sdio1")]
pub type SPIM0_SDIO1 = crate::Reg<spim0_sdio1::SPIM0_SDIO1_SPEC>;
#[doc = ""]
pub mod spim0_sdio1;
#[doc = "spim0_sdio0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio0`]
module"]
#[doc(alias = "spim0_sdio0")]
pub type SPIM0_SDIO0 = crate::Reg<spim0_sdio0::SPIM0_SDIO0_SPEC>;
#[doc = ""]
pub mod spim0_sdio0;
#[doc = "spim0_csn0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_csn0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_csn0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_csn0`]
module"]
#[doc(alias = "spim0_csn0")]
pub type SPIM0_CSN0 = crate::Reg<spim0_csn0::SPIM0_CSN0_SPEC>;
#[doc = ""]
pub mod spim0_csn0;
#[doc = "spim0_clk (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_clk`]
module"]
#[doc(alias = "spim0_clk")]
pub type SPIM0_CLK = crate::Reg<spim0_clk::SPIM0_CLK_SPEC>;
#[doc = ""]
pub mod spim0_clk;
#[doc = "cpi_vsync (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_vsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_vsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_vsync`]
module"]
#[doc(alias = "cpi_vsync")]
pub type CPI_VSYNC = crate::Reg<cpi_vsync::CPI_VSYNC_SPEC>;
#[doc = ""]
pub mod cpi_vsync;
