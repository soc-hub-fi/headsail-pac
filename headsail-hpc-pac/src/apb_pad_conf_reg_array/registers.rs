#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "registers"]
#[doc(alias = "registers")]
pub struct REGISTERS {
    i2c_scl_pad_conf: I2C_SCL_PAD_CONF,
    i2c_sda_pad_conf: I2C_SDA_PAD_CONF,
    sdio_sdclk_pad_conf: SDIO_SDCLK_PAD_CONF,
    sdio_sdcmd_pad_conf: SDIO_SDCMD_PAD_CONF,
    sdio_sddata0_pad_conf: SDIO_SDDATA0_PAD_CONF,
    sdio_sddata1_pad_conf: SDIO_SDDATA1_PAD_CONF,
    sdio_sddata2_pad_conf: SDIO_SDDATA2_PAD_CONF,
    sdio_sddata3_pad_conf: SDIO_SDDATA3_PAD_CONF,
    cpi_clk_pad_conf: CPI_CLK_PAD_CONF,
    cpi_data_pad_conf: CPI_DATA_PAD_CONF,
    cpi_hsync_pad_conf: CPI_HSYNC_PAD_CONF,
    cpi_vsync_pad_conf: CPI_VSYNC_PAD_CONF,
    spim0_clk_pad_conf: SPIM0_CLK_PAD_CONF,
    spim0_csn0_pad_conf: SPIM0_CSN0_PAD_CONF,
    spim0_sdio0_pad_conf: SPIM0_SDIO0_PAD_CONF,
    spim0_sdio1_pad_conf: SPIM0_SDIO1_PAD_CONF,
    spim0_sdio2_pad_conf: SPIM0_SDIO2_PAD_CONF,
    spim0_sdio3_pad_conf: SPIM0_SDIO3_PAD_CONF,
    spim1_clk_pad_conf: SPIM1_CLK_PAD_CONF,
    spim1_csn0_pad_conf: SPIM1_CSN0_PAD_CONF,
    spim1_sdio0_pad_conf: SPIM1_SDIO0_PAD_CONF,
    spim1_sdio1_pad_conf: SPIM1_SDIO1_PAD_CONF,
    spim1_sdio2_pad_conf: SPIM1_SDIO2_PAD_CONF,
    spim1_sdio3_pad_conf: SPIM1_SDIO3_PAD_CONF,
    uart0_rx_pad_conf: UART0_RX_PAD_CONF,
    uart0_tx_pad_conf: UART0_TX_PAD_CONF,
    uart1_rx_pad_conf: UART1_RX_PAD_CONF,
    uart1_tx_pad_conf: UART1_TX_PAD_CONF,
    spis0_sclk_pad_conf: SPIS0_SCLK_PAD_CONF,
    spis0_cs_pad_conf: SPIS0_CS_PAD_CONF,
    spis0_sdio0_pad_conf: SPIS0_SDIO0_PAD_CONF,
    spis0_sdio0_std_pad_conf: SPIS0_SDIO0_STD_PAD_CONF,
    spis0_sdio1_pad_conf: SPIS0_SDIO1_PAD_CONF,
    spis0_sdio2_pad_conf: SPIS0_SDIO2_PAD_CONF,
    spis0_sdio3_pad_conf: SPIS0_SDIO3_PAD_CONF,
    spis1_sclk_pad_conf: SPIS1_SCLK_PAD_CONF,
    spis1_cs_pad_conf: SPIS1_CS_PAD_CONF,
    spis1_sdio0_pad_conf: SPIS1_SDIO0_PAD_CONF,
    spis1_sdio0_std_pad_conf: SPIS1_SDIO0_STD_PAD_CONF,
    spis1_sdio1_pad_conf: SPIS1_SDIO1_PAD_CONF,
    spis1_sdio2_pad_conf: SPIS1_SDIO2_PAD_CONF,
    spis1_sdio3_pad_conf: SPIS1_SDIO3_PAD_CONF,
    gpio0_pad_conf: GPIO0_PAD_CONF,
    gpio1_pad_conf: GPIO1_PAD_CONF,
    gpio2_pad_conf: GPIO2_PAD_CONF,
    gpio3_pad_conf: GPIO3_PAD_CONF,
    gpio4_pad_conf: GPIO4_PAD_CONF,
    gpio5_pad_conf: GPIO5_PAD_CONF,
    gpio6_pad_conf: GPIO6_PAD_CONF,
    gpio7_pad_conf: GPIO7_PAD_CONF,
    gpio8_pad_conf: GPIO8_PAD_CONF,
    gpio9_pad_conf: GPIO9_PAD_CONF,
    gpio10_pad_conf: GPIO10_PAD_CONF,
    gpio11_pad_conf: GPIO11_PAD_CONF,
    gpio12_pad_conf: GPIO12_PAD_CONF,
    gpio13_pad_conf: GPIO13_PAD_CONF,
    gpio14_pad_conf: GPIO14_PAD_CONF,
    gpio15_pad_conf: GPIO15_PAD_CONF,
    gpio16_pad_conf: GPIO16_PAD_CONF,
    gpio17_pad_conf: GPIO17_PAD_CONF,
    gpio18_pad_conf: GPIO18_PAD_CONF,
    gpio19_pad_conf: GPIO19_PAD_CONF,
    gpio20_pad_conf: GPIO20_PAD_CONF,
    gpio21_pad_conf: GPIO21_PAD_CONF,
    gpio22_pad_conf: GPIO22_PAD_CONF,
    gpio23_pad_conf: GPIO23_PAD_CONF,
    gpio24_pad_conf: GPIO24_PAD_CONF,
    gpio25_pad_conf: GPIO25_PAD_CONF,
    gpio26_pad_conf: GPIO26_PAD_CONF,
    gpio27_pad_conf: GPIO27_PAD_CONF,
    gpio28_pad_conf: GPIO28_PAD_CONF,
    pad_mux0: PAD_MUX0,
    pad_mux1: PAD_MUX1,
    pad_mux2: PAD_MUX2,
    pad_mux3: PAD_MUX3,
    dma_event: DMA_EVENT,
    int_ack: INT_ACK,
}
impl REGISTERS {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn i2c_scl_pad_conf(&self) -> &I2C_SCL_PAD_CONF {
        &self.i2c_scl_pad_conf
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn i2c_sda_pad_conf(&self) -> &I2C_SDA_PAD_CONF {
        &self.i2c_sda_pad_conf
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn sdio_sdclk_pad_conf(&self) -> &SDIO_SDCLK_PAD_CONF {
        &self.sdio_sdclk_pad_conf
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn sdio_sdcmd_pad_conf(&self) -> &SDIO_SDCMD_PAD_CONF {
        &self.sdio_sdcmd_pad_conf
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn sdio_sddata0_pad_conf(&self) -> &SDIO_SDDATA0_PAD_CONF {
        &self.sdio_sddata0_pad_conf
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sdio_sddata1_pad_conf(&self) -> &SDIO_SDDATA1_PAD_CONF {
        &self.sdio_sddata1_pad_conf
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn sdio_sddata2_pad_conf(&self) -> &SDIO_SDDATA2_PAD_CONF {
        &self.sdio_sddata2_pad_conf
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn sdio_sddata3_pad_conf(&self) -> &SDIO_SDDATA3_PAD_CONF {
        &self.sdio_sddata3_pad_conf
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn cpi_clk_pad_conf(&self) -> &CPI_CLK_PAD_CONF {
        &self.cpi_clk_pad_conf
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn cpi_data_pad_conf(&self) -> &CPI_DATA_PAD_CONF {
        &self.cpi_data_pad_conf
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn cpi_hsync_pad_conf(&self) -> &CPI_HSYNC_PAD_CONF {
        &self.cpi_hsync_pad_conf
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn cpi_vsync_pad_conf(&self) -> &CPI_VSYNC_PAD_CONF {
        &self.cpi_vsync_pad_conf
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn spim0_clk_pad_conf(&self) -> &SPIM0_CLK_PAD_CONF {
        &self.spim0_clk_pad_conf
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn spim0_csn0_pad_conf(&self) -> &SPIM0_CSN0_PAD_CONF {
        &self.spim0_csn0_pad_conf
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn spim0_sdio0_pad_conf(&self) -> &SPIM0_SDIO0_PAD_CONF {
        &self.spim0_sdio0_pad_conf
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn spim0_sdio1_pad_conf(&self) -> &SPIM0_SDIO1_PAD_CONF {
        &self.spim0_sdio1_pad_conf
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn spim0_sdio2_pad_conf(&self) -> &SPIM0_SDIO2_PAD_CONF {
        &self.spim0_sdio2_pad_conf
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn spim0_sdio3_pad_conf(&self) -> &SPIM0_SDIO3_PAD_CONF {
        &self.spim0_sdio3_pad_conf
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn spim1_clk_pad_conf(&self) -> &SPIM1_CLK_PAD_CONF {
        &self.spim1_clk_pad_conf
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn spim1_csn0_pad_conf(&self) -> &SPIM1_CSN0_PAD_CONF {
        &self.spim1_csn0_pad_conf
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn spim1_sdio0_pad_conf(&self) -> &SPIM1_SDIO0_PAD_CONF {
        &self.spim1_sdio0_pad_conf
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn spim1_sdio1_pad_conf(&self) -> &SPIM1_SDIO1_PAD_CONF {
        &self.spim1_sdio1_pad_conf
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn spim1_sdio2_pad_conf(&self) -> &SPIM1_SDIO2_PAD_CONF {
        &self.spim1_sdio2_pad_conf
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn spim1_sdio3_pad_conf(&self) -> &SPIM1_SDIO3_PAD_CONF {
        &self.spim1_sdio3_pad_conf
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn uart0_rx_pad_conf(&self) -> &UART0_RX_PAD_CONF {
        &self.uart0_rx_pad_conf
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn uart0_tx_pad_conf(&self) -> &UART0_TX_PAD_CONF {
        &self.uart0_tx_pad_conf
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn uart1_rx_pad_conf(&self) -> &UART1_RX_PAD_CONF {
        &self.uart1_rx_pad_conf
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn uart1_tx_pad_conf(&self) -> &UART1_TX_PAD_CONF {
        &self.uart1_tx_pad_conf
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn spis0_sclk_pad_conf(&self) -> &SPIS0_SCLK_PAD_CONF {
        &self.spis0_sclk_pad_conf
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn spis0_cs_pad_conf(&self) -> &SPIS0_CS_PAD_CONF {
        &self.spis0_cs_pad_conf
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn spis0_sdio0_pad_conf(&self) -> &SPIS0_SDIO0_PAD_CONF {
        &self.spis0_sdio0_pad_conf
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn spis0_sdio0_std_pad_conf(&self) -> &SPIS0_SDIO0_STD_PAD_CONF {
        &self.spis0_sdio0_std_pad_conf
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn spis0_sdio1_pad_conf(&self) -> &SPIS0_SDIO1_PAD_CONF {
        &self.spis0_sdio1_pad_conf
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn spis0_sdio2_pad_conf(&self) -> &SPIS0_SDIO2_PAD_CONF {
        &self.spis0_sdio2_pad_conf
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn spis0_sdio3_pad_conf(&self) -> &SPIS0_SDIO3_PAD_CONF {
        &self.spis0_sdio3_pad_conf
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn spis1_sclk_pad_conf(&self) -> &SPIS1_SCLK_PAD_CONF {
        &self.spis1_sclk_pad_conf
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn spis1_cs_pad_conf(&self) -> &SPIS1_CS_PAD_CONF {
        &self.spis1_cs_pad_conf
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn spis1_sdio0_pad_conf(&self) -> &SPIS1_SDIO0_PAD_CONF {
        &self.spis1_sdio0_pad_conf
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn spis1_sdio0_std_pad_conf(&self) -> &SPIS1_SDIO0_STD_PAD_CONF {
        &self.spis1_sdio0_std_pad_conf
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn spis1_sdio1_pad_conf(&self) -> &SPIS1_SDIO1_PAD_CONF {
        &self.spis1_sdio1_pad_conf
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn spis1_sdio2_pad_conf(&self) -> &SPIS1_SDIO2_PAD_CONF {
        &self.spis1_sdio2_pad_conf
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn spis1_sdio3_pad_conf(&self) -> &SPIS1_SDIO3_PAD_CONF {
        &self.spis1_sdio3_pad_conf
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn gpio0_pad_conf(&self) -> &GPIO0_PAD_CONF {
        &self.gpio0_pad_conf
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn gpio1_pad_conf(&self) -> &GPIO1_PAD_CONF {
        &self.gpio1_pad_conf
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn gpio2_pad_conf(&self) -> &GPIO2_PAD_CONF {
        &self.gpio2_pad_conf
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn gpio3_pad_conf(&self) -> &GPIO3_PAD_CONF {
        &self.gpio3_pad_conf
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn gpio4_pad_conf(&self) -> &GPIO4_PAD_CONF {
        &self.gpio4_pad_conf
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn gpio5_pad_conf(&self) -> &GPIO5_PAD_CONF {
        &self.gpio5_pad_conf
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn gpio6_pad_conf(&self) -> &GPIO6_PAD_CONF {
        &self.gpio6_pad_conf
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn gpio7_pad_conf(&self) -> &GPIO7_PAD_CONF {
        &self.gpio7_pad_conf
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn gpio8_pad_conf(&self) -> &GPIO8_PAD_CONF {
        &self.gpio8_pad_conf
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn gpio9_pad_conf(&self) -> &GPIO9_PAD_CONF {
        &self.gpio9_pad_conf
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn gpio10_pad_conf(&self) -> &GPIO10_PAD_CONF {
        &self.gpio10_pad_conf
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn gpio11_pad_conf(&self) -> &GPIO11_PAD_CONF {
        &self.gpio11_pad_conf
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn gpio12_pad_conf(&self) -> &GPIO12_PAD_CONF {
        &self.gpio12_pad_conf
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn gpio13_pad_conf(&self) -> &GPIO13_PAD_CONF {
        &self.gpio13_pad_conf
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn gpio14_pad_conf(&self) -> &GPIO14_PAD_CONF {
        &self.gpio14_pad_conf
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn gpio15_pad_conf(&self) -> &GPIO15_PAD_CONF {
        &self.gpio15_pad_conf
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn gpio16_pad_conf(&self) -> &GPIO16_PAD_CONF {
        &self.gpio16_pad_conf
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn gpio17_pad_conf(&self) -> &GPIO17_PAD_CONF {
        &self.gpio17_pad_conf
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn gpio18_pad_conf(&self) -> &GPIO18_PAD_CONF {
        &self.gpio18_pad_conf
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn gpio19_pad_conf(&self) -> &GPIO19_PAD_CONF {
        &self.gpio19_pad_conf
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn gpio20_pad_conf(&self) -> &GPIO20_PAD_CONF {
        &self.gpio20_pad_conf
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn gpio21_pad_conf(&self) -> &GPIO21_PAD_CONF {
        &self.gpio21_pad_conf
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn gpio22_pad_conf(&self) -> &GPIO22_PAD_CONF {
        &self.gpio22_pad_conf
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn gpio23_pad_conf(&self) -> &GPIO23_PAD_CONF {
        &self.gpio23_pad_conf
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn gpio24_pad_conf(&self) -> &GPIO24_PAD_CONF {
        &self.gpio24_pad_conf
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn gpio25_pad_conf(&self) -> &GPIO25_PAD_CONF {
        &self.gpio25_pad_conf
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn gpio26_pad_conf(&self) -> &GPIO26_PAD_CONF {
        &self.gpio26_pad_conf
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn gpio27_pad_conf(&self) -> &GPIO27_PAD_CONF {
        &self.gpio27_pad_conf
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn gpio28_pad_conf(&self) -> &GPIO28_PAD_CONF {
        &self.gpio28_pad_conf
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
#[doc = "i2c_scl_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_scl_pad_conf`]
module"]
#[doc(alias = "i2c_scl_pad_conf")]
pub type I2C_SCL_PAD_CONF = crate::Reg<i2c_scl_pad_conf::I2C_SCL_PAD_CONF_SPEC>;
#[doc = ""]
pub mod i2c_scl_pad_conf;
#[doc = "i2c_sda_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sda_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_sda_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sda_pad_conf`]
module"]
#[doc(alias = "i2c_sda_pad_conf")]
pub type I2C_SDA_PAD_CONF = crate::Reg<i2c_sda_pad_conf::I2C_SDA_PAD_CONF_SPEC>;
#[doc = ""]
pub mod i2c_sda_pad_conf;
#[doc = "sdio_sdclk_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sdclk_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sdclk_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sdclk_pad_conf`]
module"]
#[doc(alias = "sdio_sdclk_pad_conf")]
pub type SDIO_SDCLK_PAD_CONF = crate::Reg<sdio_sdclk_pad_conf::SDIO_SDCLK_PAD_CONF_SPEC>;
#[doc = ""]
pub mod sdio_sdclk_pad_conf;
#[doc = "sdio_sdcmd_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sdcmd_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sdcmd_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sdcmd_pad_conf`]
module"]
#[doc(alias = "sdio_sdcmd_pad_conf")]
pub type SDIO_SDCMD_PAD_CONF = crate::Reg<sdio_sdcmd_pad_conf::SDIO_SDCMD_PAD_CONF_SPEC>;
#[doc = ""]
pub mod sdio_sdcmd_pad_conf;
#[doc = "sdio_sddata0_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata0_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata0_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata0_pad_conf`]
module"]
#[doc(alias = "sdio_sddata0_pad_conf")]
pub type SDIO_SDDATA0_PAD_CONF = crate::Reg<sdio_sddata0_pad_conf::SDIO_SDDATA0_PAD_CONF_SPEC>;
#[doc = ""]
pub mod sdio_sddata0_pad_conf;
#[doc = "sdio_sddata1_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata1_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata1_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata1_pad_conf`]
module"]
#[doc(alias = "sdio_sddata1_pad_conf")]
pub type SDIO_SDDATA1_PAD_CONF = crate::Reg<sdio_sddata1_pad_conf::SDIO_SDDATA1_PAD_CONF_SPEC>;
#[doc = ""]
pub mod sdio_sddata1_pad_conf;
#[doc = "sdio_sddata2_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata2_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata2_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata2_pad_conf`]
module"]
#[doc(alias = "sdio_sddata2_pad_conf")]
pub type SDIO_SDDATA2_PAD_CONF = crate::Reg<sdio_sddata2_pad_conf::SDIO_SDDATA2_PAD_CONF_SPEC>;
#[doc = ""]
pub mod sdio_sddata2_pad_conf;
#[doc = "sdio_sddata3_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata3_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata3_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_sddata3_pad_conf`]
module"]
#[doc(alias = "sdio_sddata3_pad_conf")]
pub type SDIO_SDDATA3_PAD_CONF = crate::Reg<sdio_sddata3_pad_conf::SDIO_SDDATA3_PAD_CONF_SPEC>;
#[doc = ""]
pub mod sdio_sddata3_pad_conf;
#[doc = "cpi_clk_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_clk_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_clk_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_clk_pad_conf`]
module"]
#[doc(alias = "cpi_clk_pad_conf")]
pub type CPI_CLK_PAD_CONF = crate::Reg<cpi_clk_pad_conf::CPI_CLK_PAD_CONF_SPEC>;
#[doc = ""]
pub mod cpi_clk_pad_conf;
#[doc = "cpi_data_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_data_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_data_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_data_pad_conf`]
module"]
#[doc(alias = "cpi_data_pad_conf")]
pub type CPI_DATA_PAD_CONF = crate::Reg<cpi_data_pad_conf::CPI_DATA_PAD_CONF_SPEC>;
#[doc = ""]
pub mod cpi_data_pad_conf;
#[doc = "cpi_hsync_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_hsync_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_hsync_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_hsync_pad_conf`]
module"]
#[doc(alias = "cpi_hsync_pad_conf")]
pub type CPI_HSYNC_PAD_CONF = crate::Reg<cpi_hsync_pad_conf::CPI_HSYNC_PAD_CONF_SPEC>;
#[doc = ""]
pub mod cpi_hsync_pad_conf;
#[doc = "spim1_csn0_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_csn0_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_csn0_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_csn0_pad_conf`]
module"]
#[doc(alias = "spim1_csn0_pad_conf")]
pub type SPIM1_CSN0_PAD_CONF = crate::Reg<spim1_csn0_pad_conf::SPIM1_CSN0_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim1_csn0_pad_conf;
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
#[doc = "gpio28_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio28_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio28_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio28_pad_conf`]
module"]
#[doc(alias = "gpio28_pad_conf")]
pub type GPIO28_PAD_CONF = crate::Reg<gpio28_pad_conf::GPIO28_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio28_pad_conf;
#[doc = "gpio27_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio27_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio27_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio27_pad_conf`]
module"]
#[doc(alias = "gpio27_pad_conf")]
pub type GPIO27_PAD_CONF = crate::Reg<gpio27_pad_conf::GPIO27_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio27_pad_conf;
#[doc = "gpio26_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio26_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio26_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio26_pad_conf`]
module"]
#[doc(alias = "gpio26_pad_conf")]
pub type GPIO26_PAD_CONF = crate::Reg<gpio26_pad_conf::GPIO26_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio26_pad_conf;
#[doc = "gpio25_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio25_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio25_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio25_pad_conf`]
module"]
#[doc(alias = "gpio25_pad_conf")]
pub type GPIO25_PAD_CONF = crate::Reg<gpio25_pad_conf::GPIO25_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio25_pad_conf;
#[doc = "gpio24_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio24_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio24_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio24_pad_conf`]
module"]
#[doc(alias = "gpio24_pad_conf")]
pub type GPIO24_PAD_CONF = crate::Reg<gpio24_pad_conf::GPIO24_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio24_pad_conf;
#[doc = "gpio23_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio23_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio23_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio23_pad_conf`]
module"]
#[doc(alias = "gpio23_pad_conf")]
pub type GPIO23_PAD_CONF = crate::Reg<gpio23_pad_conf::GPIO23_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio23_pad_conf;
#[doc = "gpio22_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio22_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio22_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio22_pad_conf`]
module"]
#[doc(alias = "gpio22_pad_conf")]
pub type GPIO22_PAD_CONF = crate::Reg<gpio22_pad_conf::GPIO22_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio22_pad_conf;
#[doc = "gpio21_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio21_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio21_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio21_pad_conf`]
module"]
#[doc(alias = "gpio21_pad_conf")]
pub type GPIO21_PAD_CONF = crate::Reg<gpio21_pad_conf::GPIO21_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio21_pad_conf;
#[doc = "gpio20_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio20_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio20_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio20_pad_conf`]
module"]
#[doc(alias = "gpio20_pad_conf")]
pub type GPIO20_PAD_CONF = crate::Reg<gpio20_pad_conf::GPIO20_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio20_pad_conf;
#[doc = "gpio19_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio19_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio19_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio19_pad_conf`]
module"]
#[doc(alias = "gpio19_pad_conf")]
pub type GPIO19_PAD_CONF = crate::Reg<gpio19_pad_conf::GPIO19_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio19_pad_conf;
#[doc = "gpio18_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio18_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio18_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio18_pad_conf`]
module"]
#[doc(alias = "gpio18_pad_conf")]
pub type GPIO18_PAD_CONF = crate::Reg<gpio18_pad_conf::GPIO18_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio18_pad_conf;
#[doc = "gpio17_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio17_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio17_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio17_pad_conf`]
module"]
#[doc(alias = "gpio17_pad_conf")]
pub type GPIO17_PAD_CONF = crate::Reg<gpio17_pad_conf::GPIO17_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio17_pad_conf;
#[doc = "gpio16_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio16_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio16_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio16_pad_conf`]
module"]
#[doc(alias = "gpio16_pad_conf")]
pub type GPIO16_PAD_CONF = crate::Reg<gpio16_pad_conf::GPIO16_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio16_pad_conf;
#[doc = "gpio15_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio15_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio15_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio15_pad_conf`]
module"]
#[doc(alias = "gpio15_pad_conf")]
pub type GPIO15_PAD_CONF = crate::Reg<gpio15_pad_conf::GPIO15_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio15_pad_conf;
#[doc = "gpio14_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio14_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio14_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio14_pad_conf`]
module"]
#[doc(alias = "gpio14_pad_conf")]
pub type GPIO14_PAD_CONF = crate::Reg<gpio14_pad_conf::GPIO14_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio14_pad_conf;
#[doc = "gpio13_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio13_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio13_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio13_pad_conf`]
module"]
#[doc(alias = "gpio13_pad_conf")]
pub type GPIO13_PAD_CONF = crate::Reg<gpio13_pad_conf::GPIO13_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio13_pad_conf;
#[doc = "gpio12_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio12_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio12_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio12_pad_conf`]
module"]
#[doc(alias = "gpio12_pad_conf")]
pub type GPIO12_PAD_CONF = crate::Reg<gpio12_pad_conf::GPIO12_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio12_pad_conf;
#[doc = "gpio11_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio11_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio11_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio11_pad_conf`]
module"]
#[doc(alias = "gpio11_pad_conf")]
pub type GPIO11_PAD_CONF = crate::Reg<gpio11_pad_conf::GPIO11_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio11_pad_conf;
#[doc = "gpio10_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio10_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio10_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio10_pad_conf`]
module"]
#[doc(alias = "gpio10_pad_conf")]
pub type GPIO10_PAD_CONF = crate::Reg<gpio10_pad_conf::GPIO10_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio10_pad_conf;
#[doc = "gpio9_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio9_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio9_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9_pad_conf`]
module"]
#[doc(alias = "gpio9_pad_conf")]
pub type GPIO9_PAD_CONF = crate::Reg<gpio9_pad_conf::GPIO9_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio9_pad_conf;
#[doc = "gpio8_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio8_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio8_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8_pad_conf`]
module"]
#[doc(alias = "gpio8_pad_conf")]
pub type GPIO8_PAD_CONF = crate::Reg<gpio8_pad_conf::GPIO8_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio8_pad_conf;
#[doc = "gpio7_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio7_pad_conf`]
module"]
#[doc(alias = "gpio7_pad_conf")]
pub type GPIO7_PAD_CONF = crate::Reg<gpio7_pad_conf::GPIO7_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio7_pad_conf;
#[doc = "gpio6_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio6_pad_conf`]
module"]
#[doc(alias = "gpio6_pad_conf")]
pub type GPIO6_PAD_CONF = crate::Reg<gpio6_pad_conf::GPIO6_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio6_pad_conf;
#[doc = "gpio5_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5_pad_conf`]
module"]
#[doc(alias = "gpio5_pad_conf")]
pub type GPIO5_PAD_CONF = crate::Reg<gpio5_pad_conf::GPIO5_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio5_pad_conf;
#[doc = "gpio4_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4_pad_conf`]
module"]
#[doc(alias = "gpio4_pad_conf")]
pub type GPIO4_PAD_CONF = crate::Reg<gpio4_pad_conf::GPIO4_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio4_pad_conf;
#[doc = "gpio3_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3_pad_conf`]
module"]
#[doc(alias = "gpio3_pad_conf")]
pub type GPIO3_PAD_CONF = crate::Reg<gpio3_pad_conf::GPIO3_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio3_pad_conf;
#[doc = "gpio2_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2_pad_conf`]
module"]
#[doc(alias = "gpio2_pad_conf")]
pub type GPIO2_PAD_CONF = crate::Reg<gpio2_pad_conf::GPIO2_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio2_pad_conf;
#[doc = "gpio1_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1_pad_conf`]
module"]
#[doc(alias = "gpio1_pad_conf")]
pub type GPIO1_PAD_CONF = crate::Reg<gpio1_pad_conf::GPIO1_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio1_pad_conf;
#[doc = "gpio0_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0_pad_conf`]
module"]
#[doc(alias = "gpio0_pad_conf")]
pub type GPIO0_PAD_CONF = crate::Reg<gpio0_pad_conf::GPIO0_PAD_CONF_SPEC>;
#[doc = ""]
pub mod gpio0_pad_conf;
#[doc = "spis1_sdio3_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio3_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio3_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio3_pad_conf`]
module"]
#[doc(alias = "spis1_sdio3_pad_conf")]
pub type SPIS1_SDIO3_PAD_CONF = crate::Reg<spis1_sdio3_pad_conf::SPIS1_SDIO3_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis1_sdio3_pad_conf;
#[doc = "spis1_sdio2_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio2_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio2_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio2_pad_conf`]
module"]
#[doc(alias = "spis1_sdio2_pad_conf")]
pub type SPIS1_SDIO2_PAD_CONF = crate::Reg<spis1_sdio2_pad_conf::SPIS1_SDIO2_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis1_sdio2_pad_conf;
#[doc = "spis1_sdio1_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio1_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio1_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio1_pad_conf`]
module"]
#[doc(alias = "spis1_sdio1_pad_conf")]
pub type SPIS1_SDIO1_PAD_CONF = crate::Reg<spis1_sdio1_pad_conf::SPIS1_SDIO1_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis1_sdio1_pad_conf;
#[doc = "spis1_sdio0_std_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio0_std_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio0_std_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio0_std_pad_conf`]
module"]
#[doc(alias = "spis1_sdio0_std_pad_conf")]
pub type SPIS1_SDIO0_STD_PAD_CONF =
    crate::Reg<spis1_sdio0_std_pad_conf::SPIS1_SDIO0_STD_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis1_sdio0_std_pad_conf;
#[doc = "spis1_sdio0_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sdio0_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sdio0_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sdio0_pad_conf`]
module"]
#[doc(alias = "spis1_sdio0_pad_conf")]
pub type SPIS1_SDIO0_PAD_CONF = crate::Reg<spis1_sdio0_pad_conf::SPIS1_SDIO0_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis1_sdio0_pad_conf;
#[doc = "spis1_cs_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_cs_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_cs_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_cs_pad_conf`]
module"]
#[doc(alias = "spis1_cs_pad_conf")]
pub type SPIS1_CS_PAD_CONF = crate::Reg<spis1_cs_pad_conf::SPIS1_CS_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis1_cs_pad_conf;
#[doc = "spis1_sclk_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis1_sclk_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis1_sclk_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis1_sclk_pad_conf`]
module"]
#[doc(alias = "spis1_sclk_pad_conf")]
pub type SPIS1_SCLK_PAD_CONF = crate::Reg<spis1_sclk_pad_conf::SPIS1_SCLK_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis1_sclk_pad_conf;
#[doc = "spis0_sdio3_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio3_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio3_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio3_pad_conf`]
module"]
#[doc(alias = "spis0_sdio3_pad_conf")]
pub type SPIS0_SDIO3_PAD_CONF = crate::Reg<spis0_sdio3_pad_conf::SPIS0_SDIO3_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis0_sdio3_pad_conf;
#[doc = "spis0_sdio2_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio2_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio2_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio2_pad_conf`]
module"]
#[doc(alias = "spis0_sdio2_pad_conf")]
pub type SPIS0_SDIO2_PAD_CONF = crate::Reg<spis0_sdio2_pad_conf::SPIS0_SDIO2_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis0_sdio2_pad_conf;
#[doc = "spis0_sdio1_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio1_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio1_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio1_pad_conf`]
module"]
#[doc(alias = "spis0_sdio1_pad_conf")]
pub type SPIS0_SDIO1_PAD_CONF = crate::Reg<spis0_sdio1_pad_conf::SPIS0_SDIO1_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis0_sdio1_pad_conf;
#[doc = "spis0_sdio0_std_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio0_std_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio0_std_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio0_std_pad_conf`]
module"]
#[doc(alias = "spis0_sdio0_std_pad_conf")]
pub type SPIS0_SDIO0_STD_PAD_CONF =
    crate::Reg<spis0_sdio0_std_pad_conf::SPIS0_SDIO0_STD_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis0_sdio0_std_pad_conf;
#[doc = "spis0_sdio0_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sdio0_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sdio0_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sdio0_pad_conf`]
module"]
#[doc(alias = "spis0_sdio0_pad_conf")]
pub type SPIS0_SDIO0_PAD_CONF = crate::Reg<spis0_sdio0_pad_conf::SPIS0_SDIO0_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis0_sdio0_pad_conf;
#[doc = "spis0_cs_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_cs_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_cs_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_cs_pad_conf`]
module"]
#[doc(alias = "spis0_cs_pad_conf")]
pub type SPIS0_CS_PAD_CONF = crate::Reg<spis0_cs_pad_conf::SPIS0_CS_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis0_cs_pad_conf;
#[doc = "spis0_sclk_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spis0_sclk_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spis0_sclk_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spis0_sclk_pad_conf`]
module"]
#[doc(alias = "spis0_sclk_pad_conf")]
pub type SPIS0_SCLK_PAD_CONF = crate::Reg<spis0_sclk_pad_conf::SPIS0_SCLK_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spis0_sclk_pad_conf;
#[doc = "uart1_tx_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_tx_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_tx_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_tx_pad_conf`]
module"]
#[doc(alias = "uart1_tx_pad_conf")]
pub type UART1_TX_PAD_CONF = crate::Reg<uart1_tx_pad_conf::UART1_TX_PAD_CONF_SPEC>;
#[doc = ""]
pub mod uart1_tx_pad_conf;
#[doc = "uart1_rx_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_rx_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_rx_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_rx_pad_conf`]
module"]
#[doc(alias = "uart1_rx_pad_conf")]
pub type UART1_RX_PAD_CONF = crate::Reg<uart1_rx_pad_conf::UART1_RX_PAD_CONF_SPEC>;
#[doc = ""]
pub mod uart1_rx_pad_conf;
#[doc = "uart0_tx_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_tx_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_tx_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_tx_pad_conf`]
module"]
#[doc(alias = "uart0_tx_pad_conf")]
pub type UART0_TX_PAD_CONF = crate::Reg<uart0_tx_pad_conf::UART0_TX_PAD_CONF_SPEC>;
#[doc = ""]
pub mod uart0_tx_pad_conf;
#[doc = "uart0_rx_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_rx_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_rx_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_rx_pad_conf`]
module"]
#[doc(alias = "uart0_rx_pad_conf")]
pub type UART0_RX_PAD_CONF = crate::Reg<uart0_rx_pad_conf::UART0_RX_PAD_CONF_SPEC>;
#[doc = ""]
pub mod uart0_rx_pad_conf;
#[doc = "spim1_sdio3_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio3_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio3_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio3_pad_conf`]
module"]
#[doc(alias = "spim1_sdio3_pad_conf")]
pub type SPIM1_SDIO3_PAD_CONF = crate::Reg<spim1_sdio3_pad_conf::SPIM1_SDIO3_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim1_sdio3_pad_conf;
#[doc = "spim1_sdio2_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio2_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio2_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio2_pad_conf`]
module"]
#[doc(alias = "spim1_sdio2_pad_conf")]
pub type SPIM1_SDIO2_PAD_CONF = crate::Reg<spim1_sdio2_pad_conf::SPIM1_SDIO2_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim1_sdio2_pad_conf;
#[doc = "spim1_sdio1_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio1_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio1_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio1_pad_conf`]
module"]
#[doc(alias = "spim1_sdio1_pad_conf")]
pub type SPIM1_SDIO1_PAD_CONF = crate::Reg<spim1_sdio1_pad_conf::SPIM1_SDIO1_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim1_sdio1_pad_conf;
#[doc = "spim1_sdio0_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_sdio0_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_sdio0_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_sdio0_pad_conf`]
module"]
#[doc(alias = "spim1_sdio0_pad_conf")]
pub type SPIM1_SDIO0_PAD_CONF = crate::Reg<spim1_sdio0_pad_conf::SPIM1_SDIO0_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim1_sdio0_pad_conf;
#[doc = "spim1_clk_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim1_clk_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim1_clk_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim1_clk_pad_conf`]
module"]
#[doc(alias = "spim1_clk_pad_conf")]
pub type SPIM1_CLK_PAD_CONF = crate::Reg<spim1_clk_pad_conf::SPIM1_CLK_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim1_clk_pad_conf;
#[doc = "spim0_sdio3_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio3_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio3_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio3_pad_conf`]
module"]
#[doc(alias = "spim0_sdio3_pad_conf")]
pub type SPIM0_SDIO3_PAD_CONF = crate::Reg<spim0_sdio3_pad_conf::SPIM0_SDIO3_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim0_sdio3_pad_conf;
#[doc = "spim0_sdio2_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio2_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio2_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio2_pad_conf`]
module"]
#[doc(alias = "spim0_sdio2_pad_conf")]
pub type SPIM0_SDIO2_PAD_CONF = crate::Reg<spim0_sdio2_pad_conf::SPIM0_SDIO2_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim0_sdio2_pad_conf;
#[doc = "spim0_sdio1_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio1_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio1_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio1_pad_conf`]
module"]
#[doc(alias = "spim0_sdio1_pad_conf")]
pub type SPIM0_SDIO1_PAD_CONF = crate::Reg<spim0_sdio1_pad_conf::SPIM0_SDIO1_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim0_sdio1_pad_conf;
#[doc = "spim0_sdio0_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_sdio0_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_sdio0_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_sdio0_pad_conf`]
module"]
#[doc(alias = "spim0_sdio0_pad_conf")]
pub type SPIM0_SDIO0_PAD_CONF = crate::Reg<spim0_sdio0_pad_conf::SPIM0_SDIO0_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim0_sdio0_pad_conf;
#[doc = "spim0_csn0_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_csn0_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_csn0_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_csn0_pad_conf`]
module"]
#[doc(alias = "spim0_csn0_pad_conf")]
pub type SPIM0_CSN0_PAD_CONF = crate::Reg<spim0_csn0_pad_conf::SPIM0_CSN0_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim0_csn0_pad_conf;
#[doc = "spim0_clk_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim0_clk_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim0_clk_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim0_clk_pad_conf`]
module"]
#[doc(alias = "spim0_clk_pad_conf")]
pub type SPIM0_CLK_PAD_CONF = crate::Reg<spim0_clk_pad_conf::SPIM0_CLK_PAD_CONF_SPEC>;
#[doc = ""]
pub mod spim0_clk_pad_conf;
#[doc = "cpi_vsync_pad_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpi_vsync_pad_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpi_vsync_pad_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpi_vsync_pad_conf`]
module"]
#[doc(alias = "cpi_vsync_pad_conf")]
pub type CPI_VSYNC_PAD_CONF = crate::Reg<cpi_vsync_pad_conf::CPI_VSYNC_PAD_CONF_SPEC>;
#[doc = ""]
pub mod cpi_vsync_pad_conf;
