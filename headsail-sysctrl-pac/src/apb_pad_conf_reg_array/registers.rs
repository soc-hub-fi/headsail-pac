#[doc = r"Register block"]
#[repr(C)]
pub struct REGISTERS {
    #[doc = "0x00 - "]
    pub i2c_scl_pad_conf_reg: I2C_SCL_PAD_CONF_REG,
    #[doc = "0x04 - "]
    pub i2c_sda_pad_conf_reg: I2C_SDA_PAD_CONF_REG,
    #[doc = "0x08 - "]
    pub sdio_sdclk_pad_conf_reg: SDIO_SDCLK_PAD_CONF_REG,
    #[doc = "0x0c - "]
    pub sdio_sdcmd_pad_conf_reg: SDIO_SDCMD_PAD_CONF_REG,
    #[doc = "0x10 - "]
    pub sdio_sddata0_pad_conf_reg: SDIO_SDDATA0_PAD_CONF_REG,
    #[doc = "0x14 - "]
    pub sdio_sddata1_pad_conf_reg: SDIO_SDDATA1_PAD_CONF_REG,
    #[doc = "0x18 - "]
    pub sdio_sddata2_pad_conf_reg: SDIO_SDDATA2_PAD_CONF_REG,
    #[doc = "0x1c - "]
    pub sdio_sddata3_pad_conf_reg: SDIO_SDDATA3_PAD_CONF_REG,
    #[doc = "0x20 - "]
    pub cpi_clk_pad_conf_reg: CPI_CLK_PAD_CONF_REG,
    #[doc = "0x24 - "]
    pub cpi_data_pad_conf_reg: CPI_DATA_PAD_CONF_REG,
    #[doc = "0x28 - "]
    pub cpi_hsync_pad_conf_reg: CPI_HSYNC_PAD_CONF_REG,
    #[doc = "0x2c - "]
    pub cpi_vsync_pad_conf_reg: CPI_VSYNC_PAD_CONF_REG,
    #[doc = "0x30 - "]
    pub spim0_clk_pad_conf_reg: SPIM0_CLK_PAD_CONF_REG,
    #[doc = "0x34 - "]
    pub spim0_csn0_pad_conf_reg: SPIM0_CSN0_PAD_CONF_REG,
    #[doc = "0x38 - "]
    pub spim0_sdio0_pad_conf_reg: SPIM0_SDIO0_PAD_CONF_REG,
    #[doc = "0x3c - "]
    pub spim0_sdio1_pad_conf_reg: SPIM0_SDIO1_PAD_CONF_REG,
    #[doc = "0x40 - "]
    pub spim0_sdio2_pad_conf_reg: SPIM0_SDIO2_PAD_CONF_REG,
    #[doc = "0x44 - "]
    pub spim0_sdio3_pad_conf_reg: SPIM0_SDIO3_PAD_CONF_REG,
    #[doc = "0x48 - "]
    pub spim1_clk_pad_conf_reg: SPIM1_CLK_PAD_CONF_REG,
    #[doc = "0x4c - "]
    pub spim1_csn0_pad_conf_reg: SPIM1_CSN0_PAD_CONF_REG,
    #[doc = "0x50 - "]
    pub spim1_sdio0_pad_conf_reg: SPIM1_SDIO0_PAD_CONF_REG,
    #[doc = "0x54 - "]
    pub spim1_sdio1_pad_conf_reg: SPIM1_SDIO1_PAD_CONF_REG,
    #[doc = "0x58 - "]
    pub spim1_sdio2_pad_conf_reg: SPIM1_SDIO2_PAD_CONF_REG,
    #[doc = "0x5c - "]
    pub spim1_sdio3_pad_conf_reg: SPIM1_SDIO3_PAD_CONF_REG,
    #[doc = "0x60 - "]
    pub uart0_rx_pad_conf_reg: UART0_RX_PAD_CONF_REG,
    #[doc = "0x64 - "]
    pub uart0_tx_pad_conf_reg: UART0_TX_PAD_CONF_REG,
    #[doc = "0x68 - "]
    pub uart1_rx_pad_conf_reg: UART1_RX_PAD_CONF_REG,
    #[doc = "0x6c - "]
    pub uart1_tx_pad_conf_reg: UART1_TX_PAD_CONF_REG,
    #[doc = "0x70 - "]
    pub spis0_sclk_pad_conf_reg: SPIS0_SCLK_PAD_CONF_REG,
    #[doc = "0x74 - "]
    pub spis0_cs_pad_conf_reg: SPIS0_CS_PAD_CONF_REG,
    #[doc = "0x78 - "]
    pub spis0_sdio0_pad_conf_reg: SPIS0_SDIO0_PAD_CONF_REG,
    #[doc = "0x7c - "]
    pub spis0_sdio0_std_pad_conf_reg: SPIS0_SDIO0_STD_PAD_CONF_REG,
    #[doc = "0x80 - "]
    pub spis0_sdio1_pad_conf_reg: SPIS0_SDIO1_PAD_CONF_REG,
    #[doc = "0x84 - "]
    pub spis0_sdio2_pad_conf_reg: SPIS0_SDIO2_PAD_CONF_REG,
    #[doc = "0x88 - "]
    pub spis0_sdio3_pad_conf_reg: SPIS0_SDIO3_PAD_CONF_REG,
    #[doc = "0x8c - "]
    pub spis1_sclk_pad_conf_reg: SPIS1_SCLK_PAD_CONF_REG,
    #[doc = "0x90 - "]
    pub spis1_cs_pad_conf_reg: SPIS1_CS_PAD_CONF_REG,
    #[doc = "0x94 - "]
    pub spis1_sdio0_pad_conf_reg: SPIS1_SDIO0_PAD_CONF_REG,
    #[doc = "0x98 - "]
    pub spis1_sdio0_std_pad_conf_reg: SPIS1_SDIO0_STD_PAD_CONF_REG,
    #[doc = "0x9c - "]
    pub spis1_sdio1_pad_conf_reg: SPIS1_SDIO1_PAD_CONF_REG,
    #[doc = "0xa0 - "]
    pub spis1_sdio2_pad_conf_reg: SPIS1_SDIO2_PAD_CONF_REG,
    #[doc = "0xa4 - "]
    pub spis1_sdio3_pad_conf_reg: SPIS1_SDIO3_PAD_CONF_REG,
    #[doc = "0xa8 - "]
    pub gpio0_pad_conf_reg: GPIO0_PAD_CONF_REG,
    #[doc = "0xac - "]
    pub gpio1_pad_conf_reg: GPIO1_PAD_CONF_REG,
    #[doc = "0xb0 - "]
    pub gpio2_pad_conf_reg: GPIO2_PAD_CONF_REG,
    #[doc = "0xb4 - "]
    pub gpio3_pad_conf_reg: GPIO3_PAD_CONF_REG,
    #[doc = "0xb8 - "]
    pub gpio4_pad_conf_reg: GPIO4_PAD_CONF_REG,
    #[doc = "0xbc - "]
    pub gpio5_pad_conf_reg: GPIO5_PAD_CONF_REG,
    #[doc = "0xc0 - "]
    pub gpio6_pad_conf_reg: GPIO6_PAD_CONF_REG,
    #[doc = "0xc4 - "]
    pub gpio7_pad_conf_reg: GPIO7_PAD_CONF_REG,
    #[doc = "0xc8 - "]
    pub gpio8_pad_conf_reg: GPIO8_PAD_CONF_REG,
    #[doc = "0xcc - "]
    pub gpio9_pad_conf_reg: GPIO9_PAD_CONF_REG,
    #[doc = "0xd0 - "]
    pub gpio10_pad_conf_reg: GPIO10_PAD_CONF_REG,
    #[doc = "0xd4 - "]
    pub gpio11_pad_conf_reg: GPIO11_PAD_CONF_REG,
    #[doc = "0xd8 - "]
    pub gpio12_pad_conf_reg: GPIO12_PAD_CONF_REG,
    #[doc = "0xdc - "]
    pub gpio13_pad_conf_reg: GPIO13_PAD_CONF_REG,
    #[doc = "0xe0 - "]
    pub gpio14_pad_conf_reg: GPIO14_PAD_CONF_REG,
    #[doc = "0xe4 - "]
    pub gpio15_pad_conf_reg: GPIO15_PAD_CONF_REG,
    #[doc = "0xe8 - "]
    pub gpio16_pad_conf_reg: GPIO16_PAD_CONF_REG,
    #[doc = "0xec - "]
    pub gpio17_pad_conf_reg: GPIO17_PAD_CONF_REG,
    #[doc = "0xf0 - "]
    pub gpio18_pad_conf_reg: GPIO18_PAD_CONF_REG,
    #[doc = "0xf4 - "]
    pub gpio19_pad_conf_reg: GPIO19_PAD_CONF_REG,
    #[doc = "0xf8 - "]
    pub gpio20_pad_conf_reg: GPIO20_PAD_CONF_REG,
    #[doc = "0xfc - "]
    pub gpio21_pad_conf_reg: GPIO21_PAD_CONF_REG,
    #[doc = "0x100 - "]
    pub gpio22_pad_conf_reg: GPIO22_PAD_CONF_REG,
    #[doc = "0x104 - "]
    pub gpio23_pad_conf_reg: GPIO23_PAD_CONF_REG,
    #[doc = "0x108 - "]
    pub gpio24_pad_conf_reg: GPIO24_PAD_CONF_REG,
    #[doc = "0x10c - "]
    pub gpio25_pad_conf_reg: GPIO25_PAD_CONF_REG,
    #[doc = "0x110 - "]
    pub gpio26_pad_conf_reg: GPIO26_PAD_CONF_REG,
    #[doc = "0x114 - "]
    pub gpio27_pad_conf_reg: GPIO27_PAD_CONF_REG,
    #[doc = "0x118 - "]
    pub gpio28_pad_conf_reg: GPIO28_PAD_CONF_REG,
    #[doc = "0x11c - "]
    pub pad_mux0_reg: PAD_MUX0_REG,
    #[doc = "0x120 - "]
    pub pad_mux1_reg: PAD_MUX1_REG,
    #[doc = "0x124 - "]
    pub pad_mux2_reg: PAD_MUX2_REG,
    #[doc = "0x128 - "]
    pub pad_mux3_reg: PAD_MUX3_REG,
    #[doc = "0x12c - "]
    pub dma_event_reg: DMA_EVENT_REG,
    #[doc = "0x130 - "]
    pub int_ack_reg: INT_ACK_REG,
}
#[doc = "i2c_scl_pad_conf_reg (rw) register accessor: an alias for `Reg<I2C_SCL_PAD_CONF_REG_SPEC>`"]
pub type I2C_SCL_PAD_CONF_REG = crate::Reg<i2c_scl_pad_conf_reg::I2C_SCL_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod i2c_scl_pad_conf_reg;
#[doc = "i2c_sda_pad_conf_reg (rw) register accessor: an alias for `Reg<I2C_SDA_PAD_CONF_REG_SPEC>`"]
pub type I2C_SDA_PAD_CONF_REG = crate::Reg<i2c_sda_pad_conf_reg::I2C_SDA_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod i2c_sda_pad_conf_reg;
#[doc = "sdio_sdclk_pad_conf_reg (rw) register accessor: an alias for `Reg<SDIO_SDCLK_PAD_CONF_REG_SPEC>`"]
pub type SDIO_SDCLK_PAD_CONF_REG =
    crate::Reg<sdio_sdclk_pad_conf_reg::SDIO_SDCLK_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod sdio_sdclk_pad_conf_reg;
#[doc = "sdio_sdcmd_pad_conf_reg (rw) register accessor: an alias for `Reg<SDIO_SDCMD_PAD_CONF_REG_SPEC>`"]
pub type SDIO_SDCMD_PAD_CONF_REG =
    crate::Reg<sdio_sdcmd_pad_conf_reg::SDIO_SDCMD_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod sdio_sdcmd_pad_conf_reg;
#[doc = "sdio_sddata0_pad_conf_reg (rw) register accessor: an alias for `Reg<SDIO_SDDATA0_PAD_CONF_REG_SPEC>`"]
pub type SDIO_SDDATA0_PAD_CONF_REG =
    crate::Reg<sdio_sddata0_pad_conf_reg::SDIO_SDDATA0_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod sdio_sddata0_pad_conf_reg;
#[doc = "sdio_sddata1_pad_conf_reg (rw) register accessor: an alias for `Reg<SDIO_SDDATA1_PAD_CONF_REG_SPEC>`"]
pub type SDIO_SDDATA1_PAD_CONF_REG =
    crate::Reg<sdio_sddata1_pad_conf_reg::SDIO_SDDATA1_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod sdio_sddata1_pad_conf_reg;
#[doc = "sdio_sddata2_pad_conf_reg (rw) register accessor: an alias for `Reg<SDIO_SDDATA2_PAD_CONF_REG_SPEC>`"]
pub type SDIO_SDDATA2_PAD_CONF_REG =
    crate::Reg<sdio_sddata2_pad_conf_reg::SDIO_SDDATA2_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod sdio_sddata2_pad_conf_reg;
#[doc = "sdio_sddata3_pad_conf_reg (rw) register accessor: an alias for `Reg<SDIO_SDDATA3_PAD_CONF_REG_SPEC>`"]
pub type SDIO_SDDATA3_PAD_CONF_REG =
    crate::Reg<sdio_sddata3_pad_conf_reg::SDIO_SDDATA3_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod sdio_sddata3_pad_conf_reg;
#[doc = "cpi_clk_pad_conf_reg (rw) register accessor: an alias for `Reg<CPI_CLK_PAD_CONF_REG_SPEC>`"]
pub type CPI_CLK_PAD_CONF_REG = crate::Reg<cpi_clk_pad_conf_reg::CPI_CLK_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod cpi_clk_pad_conf_reg;
#[doc = "cpi_data_pad_conf_reg (rw) register accessor: an alias for `Reg<CPI_DATA_PAD_CONF_REG_SPEC>`"]
pub type CPI_DATA_PAD_CONF_REG = crate::Reg<cpi_data_pad_conf_reg::CPI_DATA_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod cpi_data_pad_conf_reg;
#[doc = "cpi_hsync_pad_conf_reg (rw) register accessor: an alias for `Reg<CPI_HSYNC_PAD_CONF_REG_SPEC>`"]
pub type CPI_HSYNC_PAD_CONF_REG = crate::Reg<cpi_hsync_pad_conf_reg::CPI_HSYNC_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod cpi_hsync_pad_conf_reg;
#[doc = "spim1_csn0_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM1_CSN0_PAD_CONF_REG_SPEC>`"]
pub type SPIM1_CSN0_PAD_CONF_REG =
    crate::Reg<spim1_csn0_pad_conf_reg::SPIM1_CSN0_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim1_csn0_pad_conf_reg;
#[doc = "int_ack_reg (rw) register accessor: an alias for `Reg<INT_ACK_REG_SPEC>`"]
pub type INT_ACK_REG = crate::Reg<int_ack_reg::INT_ACK_REG_SPEC>;
#[doc = ""]
pub mod int_ack_reg;
#[doc = "dma_event_reg (rw) register accessor: an alias for `Reg<DMA_EVENT_REG_SPEC>`"]
pub type DMA_EVENT_REG = crate::Reg<dma_event_reg::DMA_EVENT_REG_SPEC>;
#[doc = ""]
pub mod dma_event_reg;
#[doc = "pad_mux3_reg (rw) register accessor: an alias for `Reg<PAD_MUX3_REG_SPEC>`"]
pub type PAD_MUX3_REG = crate::Reg<pad_mux3_reg::PAD_MUX3_REG_SPEC>;
#[doc = ""]
pub mod pad_mux3_reg;
#[doc = "pad_mux2_reg (rw) register accessor: an alias for `Reg<PAD_MUX2_REG_SPEC>`"]
pub type PAD_MUX2_REG = crate::Reg<pad_mux2_reg::PAD_MUX2_REG_SPEC>;
#[doc = ""]
pub mod pad_mux2_reg;
#[doc = "pad_mux1_reg (rw) register accessor: an alias for `Reg<PAD_MUX1_REG_SPEC>`"]
pub type PAD_MUX1_REG = crate::Reg<pad_mux1_reg::PAD_MUX1_REG_SPEC>;
#[doc = ""]
pub mod pad_mux1_reg;
#[doc = "pad_mux0_reg (rw) register accessor: an alias for `Reg<PAD_MUX0_REG_SPEC>`"]
pub type PAD_MUX0_REG = crate::Reg<pad_mux0_reg::PAD_MUX0_REG_SPEC>;
#[doc = ""]
pub mod pad_mux0_reg;
#[doc = "gpio28_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO28_PAD_CONF_REG_SPEC>`"]
pub type GPIO28_PAD_CONF_REG = crate::Reg<gpio28_pad_conf_reg::GPIO28_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio28_pad_conf_reg;
#[doc = "gpio27_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO27_PAD_CONF_REG_SPEC>`"]
pub type GPIO27_PAD_CONF_REG = crate::Reg<gpio27_pad_conf_reg::GPIO27_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio27_pad_conf_reg;
#[doc = "gpio26_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO26_PAD_CONF_REG_SPEC>`"]
pub type GPIO26_PAD_CONF_REG = crate::Reg<gpio26_pad_conf_reg::GPIO26_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio26_pad_conf_reg;
#[doc = "gpio25_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO25_PAD_CONF_REG_SPEC>`"]
pub type GPIO25_PAD_CONF_REG = crate::Reg<gpio25_pad_conf_reg::GPIO25_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio25_pad_conf_reg;
#[doc = "gpio24_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO24_PAD_CONF_REG_SPEC>`"]
pub type GPIO24_PAD_CONF_REG = crate::Reg<gpio24_pad_conf_reg::GPIO24_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio24_pad_conf_reg;
#[doc = "gpio23_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO23_PAD_CONF_REG_SPEC>`"]
pub type GPIO23_PAD_CONF_REG = crate::Reg<gpio23_pad_conf_reg::GPIO23_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio23_pad_conf_reg;
#[doc = "gpio22_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO22_PAD_CONF_REG_SPEC>`"]
pub type GPIO22_PAD_CONF_REG = crate::Reg<gpio22_pad_conf_reg::GPIO22_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio22_pad_conf_reg;
#[doc = "gpio21_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO21_PAD_CONF_REG_SPEC>`"]
pub type GPIO21_PAD_CONF_REG = crate::Reg<gpio21_pad_conf_reg::GPIO21_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio21_pad_conf_reg;
#[doc = "gpio20_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO20_PAD_CONF_REG_SPEC>`"]
pub type GPIO20_PAD_CONF_REG = crate::Reg<gpio20_pad_conf_reg::GPIO20_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio20_pad_conf_reg;
#[doc = "gpio19_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO19_PAD_CONF_REG_SPEC>`"]
pub type GPIO19_PAD_CONF_REG = crate::Reg<gpio19_pad_conf_reg::GPIO19_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio19_pad_conf_reg;
#[doc = "gpio18_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO18_PAD_CONF_REG_SPEC>`"]
pub type GPIO18_PAD_CONF_REG = crate::Reg<gpio18_pad_conf_reg::GPIO18_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio18_pad_conf_reg;
#[doc = "gpio17_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO17_PAD_CONF_REG_SPEC>`"]
pub type GPIO17_PAD_CONF_REG = crate::Reg<gpio17_pad_conf_reg::GPIO17_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio17_pad_conf_reg;
#[doc = "gpio16_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO16_PAD_CONF_REG_SPEC>`"]
pub type GPIO16_PAD_CONF_REG = crate::Reg<gpio16_pad_conf_reg::GPIO16_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio16_pad_conf_reg;
#[doc = "gpio15_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO15_PAD_CONF_REG_SPEC>`"]
pub type GPIO15_PAD_CONF_REG = crate::Reg<gpio15_pad_conf_reg::GPIO15_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio15_pad_conf_reg;
#[doc = "gpio14_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO14_PAD_CONF_REG_SPEC>`"]
pub type GPIO14_PAD_CONF_REG = crate::Reg<gpio14_pad_conf_reg::GPIO14_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio14_pad_conf_reg;
#[doc = "gpio13_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO13_PAD_CONF_REG_SPEC>`"]
pub type GPIO13_PAD_CONF_REG = crate::Reg<gpio13_pad_conf_reg::GPIO13_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio13_pad_conf_reg;
#[doc = "gpio12_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO12_PAD_CONF_REG_SPEC>`"]
pub type GPIO12_PAD_CONF_REG = crate::Reg<gpio12_pad_conf_reg::GPIO12_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio12_pad_conf_reg;
#[doc = "gpio11_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO11_PAD_CONF_REG_SPEC>`"]
pub type GPIO11_PAD_CONF_REG = crate::Reg<gpio11_pad_conf_reg::GPIO11_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio11_pad_conf_reg;
#[doc = "gpio10_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO10_PAD_CONF_REG_SPEC>`"]
pub type GPIO10_PAD_CONF_REG = crate::Reg<gpio10_pad_conf_reg::GPIO10_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio10_pad_conf_reg;
#[doc = "gpio9_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO9_PAD_CONF_REG_SPEC>`"]
pub type GPIO9_PAD_CONF_REG = crate::Reg<gpio9_pad_conf_reg::GPIO9_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio9_pad_conf_reg;
#[doc = "gpio8_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO8_PAD_CONF_REG_SPEC>`"]
pub type GPIO8_PAD_CONF_REG = crate::Reg<gpio8_pad_conf_reg::GPIO8_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio8_pad_conf_reg;
#[doc = "gpio7_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO7_PAD_CONF_REG_SPEC>`"]
pub type GPIO7_PAD_CONF_REG = crate::Reg<gpio7_pad_conf_reg::GPIO7_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio7_pad_conf_reg;
#[doc = "gpio6_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO6_PAD_CONF_REG_SPEC>`"]
pub type GPIO6_PAD_CONF_REG = crate::Reg<gpio6_pad_conf_reg::GPIO6_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio6_pad_conf_reg;
#[doc = "gpio5_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO5_PAD_CONF_REG_SPEC>`"]
pub type GPIO5_PAD_CONF_REG = crate::Reg<gpio5_pad_conf_reg::GPIO5_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio5_pad_conf_reg;
#[doc = "gpio4_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO4_PAD_CONF_REG_SPEC>`"]
pub type GPIO4_PAD_CONF_REG = crate::Reg<gpio4_pad_conf_reg::GPIO4_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio4_pad_conf_reg;
#[doc = "gpio3_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO3_PAD_CONF_REG_SPEC>`"]
pub type GPIO3_PAD_CONF_REG = crate::Reg<gpio3_pad_conf_reg::GPIO3_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio3_pad_conf_reg;
#[doc = "gpio2_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO2_PAD_CONF_REG_SPEC>`"]
pub type GPIO2_PAD_CONF_REG = crate::Reg<gpio2_pad_conf_reg::GPIO2_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio2_pad_conf_reg;
#[doc = "gpio1_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO1_PAD_CONF_REG_SPEC>`"]
pub type GPIO1_PAD_CONF_REG = crate::Reg<gpio1_pad_conf_reg::GPIO1_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio1_pad_conf_reg;
#[doc = "gpio0_pad_conf_reg (rw) register accessor: an alias for `Reg<GPIO0_PAD_CONF_REG_SPEC>`"]
pub type GPIO0_PAD_CONF_REG = crate::Reg<gpio0_pad_conf_reg::GPIO0_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod gpio0_pad_conf_reg;
#[doc = "spis1_sdio3_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS1_SDIO3_PAD_CONF_REG_SPEC>`"]
pub type SPIS1_SDIO3_PAD_CONF_REG =
    crate::Reg<spis1_sdio3_pad_conf_reg::SPIS1_SDIO3_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis1_sdio3_pad_conf_reg;
#[doc = "spis1_sdio2_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS1_SDIO2_PAD_CONF_REG_SPEC>`"]
pub type SPIS1_SDIO2_PAD_CONF_REG =
    crate::Reg<spis1_sdio2_pad_conf_reg::SPIS1_SDIO2_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis1_sdio2_pad_conf_reg;
#[doc = "spis1_sdio1_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS1_SDIO1_PAD_CONF_REG_SPEC>`"]
pub type SPIS1_SDIO1_PAD_CONF_REG =
    crate::Reg<spis1_sdio1_pad_conf_reg::SPIS1_SDIO1_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis1_sdio1_pad_conf_reg;
#[doc = "spis1_sdio0_std_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS1_SDIO0_STD_PAD_CONF_REG_SPEC>`"]
pub type SPIS1_SDIO0_STD_PAD_CONF_REG =
    crate::Reg<spis1_sdio0_std_pad_conf_reg::SPIS1_SDIO0_STD_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis1_sdio0_std_pad_conf_reg;
#[doc = "spis1_sdio0_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS1_SDIO0_PAD_CONF_REG_SPEC>`"]
pub type SPIS1_SDIO0_PAD_CONF_REG =
    crate::Reg<spis1_sdio0_pad_conf_reg::SPIS1_SDIO0_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis1_sdio0_pad_conf_reg;
#[doc = "spis1_cs_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS1_CS_PAD_CONF_REG_SPEC>`"]
pub type SPIS1_CS_PAD_CONF_REG = crate::Reg<spis1_cs_pad_conf_reg::SPIS1_CS_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis1_cs_pad_conf_reg;
#[doc = "spis1_sclk_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS1_SCLK_PAD_CONF_REG_SPEC>`"]
pub type SPIS1_SCLK_PAD_CONF_REG =
    crate::Reg<spis1_sclk_pad_conf_reg::SPIS1_SCLK_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis1_sclk_pad_conf_reg;
#[doc = "spis0_sdio3_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS0_SDIO3_PAD_CONF_REG_SPEC>`"]
pub type SPIS0_SDIO3_PAD_CONF_REG =
    crate::Reg<spis0_sdio3_pad_conf_reg::SPIS0_SDIO3_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis0_sdio3_pad_conf_reg;
#[doc = "spis0_sdio2_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS0_SDIO2_PAD_CONF_REG_SPEC>`"]
pub type SPIS0_SDIO2_PAD_CONF_REG =
    crate::Reg<spis0_sdio2_pad_conf_reg::SPIS0_SDIO2_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis0_sdio2_pad_conf_reg;
#[doc = "spis0_sdio1_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS0_SDIO1_PAD_CONF_REG_SPEC>`"]
pub type SPIS0_SDIO1_PAD_CONF_REG =
    crate::Reg<spis0_sdio1_pad_conf_reg::SPIS0_SDIO1_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis0_sdio1_pad_conf_reg;
#[doc = "spis0_sdio0_std_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS0_SDIO0_STD_PAD_CONF_REG_SPEC>`"]
pub type SPIS0_SDIO0_STD_PAD_CONF_REG =
    crate::Reg<spis0_sdio0_std_pad_conf_reg::SPIS0_SDIO0_STD_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis0_sdio0_std_pad_conf_reg;
#[doc = "spis0_sdio0_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS0_SDIO0_PAD_CONF_REG_SPEC>`"]
pub type SPIS0_SDIO0_PAD_CONF_REG =
    crate::Reg<spis0_sdio0_pad_conf_reg::SPIS0_SDIO0_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis0_sdio0_pad_conf_reg;
#[doc = "spis0_cs_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS0_CS_PAD_CONF_REG_SPEC>`"]
pub type SPIS0_CS_PAD_CONF_REG = crate::Reg<spis0_cs_pad_conf_reg::SPIS0_CS_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis0_cs_pad_conf_reg;
#[doc = "spis0_sclk_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIS0_SCLK_PAD_CONF_REG_SPEC>`"]
pub type SPIS0_SCLK_PAD_CONF_REG =
    crate::Reg<spis0_sclk_pad_conf_reg::SPIS0_SCLK_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spis0_sclk_pad_conf_reg;
#[doc = "uart1_tx_pad_conf_reg (rw) register accessor: an alias for `Reg<UART1_TX_PAD_CONF_REG_SPEC>`"]
pub type UART1_TX_PAD_CONF_REG = crate::Reg<uart1_tx_pad_conf_reg::UART1_TX_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod uart1_tx_pad_conf_reg;
#[doc = "uart1_rx_pad_conf_reg (rw) register accessor: an alias for `Reg<UART1_RX_PAD_CONF_REG_SPEC>`"]
pub type UART1_RX_PAD_CONF_REG = crate::Reg<uart1_rx_pad_conf_reg::UART1_RX_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod uart1_rx_pad_conf_reg;
#[doc = "uart0_tx_pad_conf_reg (rw) register accessor: an alias for `Reg<UART0_TX_PAD_CONF_REG_SPEC>`"]
pub type UART0_TX_PAD_CONF_REG = crate::Reg<uart0_tx_pad_conf_reg::UART0_TX_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod uart0_tx_pad_conf_reg;
#[doc = "uart0_rx_pad_conf_reg (rw) register accessor: an alias for `Reg<UART0_RX_PAD_CONF_REG_SPEC>`"]
pub type UART0_RX_PAD_CONF_REG = crate::Reg<uart0_rx_pad_conf_reg::UART0_RX_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod uart0_rx_pad_conf_reg;
#[doc = "spim1_sdio3_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM1_SDIO3_PAD_CONF_REG_SPEC>`"]
pub type SPIM1_SDIO3_PAD_CONF_REG =
    crate::Reg<spim1_sdio3_pad_conf_reg::SPIM1_SDIO3_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim1_sdio3_pad_conf_reg;
#[doc = "spim1_sdio2_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM1_SDIO2_PAD_CONF_REG_SPEC>`"]
pub type SPIM1_SDIO2_PAD_CONF_REG =
    crate::Reg<spim1_sdio2_pad_conf_reg::SPIM1_SDIO2_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim1_sdio2_pad_conf_reg;
#[doc = "spim1_sdio1_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM1_SDIO1_PAD_CONF_REG_SPEC>`"]
pub type SPIM1_SDIO1_PAD_CONF_REG =
    crate::Reg<spim1_sdio1_pad_conf_reg::SPIM1_SDIO1_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim1_sdio1_pad_conf_reg;
#[doc = "spim1_sdio0_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM1_SDIO0_PAD_CONF_REG_SPEC>`"]
pub type SPIM1_SDIO0_PAD_CONF_REG =
    crate::Reg<spim1_sdio0_pad_conf_reg::SPIM1_SDIO0_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim1_sdio0_pad_conf_reg;
#[doc = "spim1_clk_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM1_CLK_PAD_CONF_REG_SPEC>`"]
pub type SPIM1_CLK_PAD_CONF_REG = crate::Reg<spim1_clk_pad_conf_reg::SPIM1_CLK_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim1_clk_pad_conf_reg;
#[doc = "spim0_sdio3_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM0_SDIO3_PAD_CONF_REG_SPEC>`"]
pub type SPIM0_SDIO3_PAD_CONF_REG =
    crate::Reg<spim0_sdio3_pad_conf_reg::SPIM0_SDIO3_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim0_sdio3_pad_conf_reg;
#[doc = "spim0_sdio2_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM0_SDIO2_PAD_CONF_REG_SPEC>`"]
pub type SPIM0_SDIO2_PAD_CONF_REG =
    crate::Reg<spim0_sdio2_pad_conf_reg::SPIM0_SDIO2_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim0_sdio2_pad_conf_reg;
#[doc = "spim0_sdio1_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM0_SDIO1_PAD_CONF_REG_SPEC>`"]
pub type SPIM0_SDIO1_PAD_CONF_REG =
    crate::Reg<spim0_sdio1_pad_conf_reg::SPIM0_SDIO1_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim0_sdio1_pad_conf_reg;
#[doc = "spim0_sdio0_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM0_SDIO0_PAD_CONF_REG_SPEC>`"]
pub type SPIM0_SDIO0_PAD_CONF_REG =
    crate::Reg<spim0_sdio0_pad_conf_reg::SPIM0_SDIO0_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim0_sdio0_pad_conf_reg;
#[doc = "spim0_csn0_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM0_CSN0_PAD_CONF_REG_SPEC>`"]
pub type SPIM0_CSN0_PAD_CONF_REG =
    crate::Reg<spim0_csn0_pad_conf_reg::SPIM0_CSN0_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim0_csn0_pad_conf_reg;
#[doc = "spim0_clk_pad_conf_reg (rw) register accessor: an alias for `Reg<SPIM0_CLK_PAD_CONF_REG_SPEC>`"]
pub type SPIM0_CLK_PAD_CONF_REG = crate::Reg<spim0_clk_pad_conf_reg::SPIM0_CLK_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod spim0_clk_pad_conf_reg;
#[doc = "cpi_vsync_pad_conf_reg (rw) register accessor: an alias for `Reg<CPI_VSYNC_PAD_CONF_REG_SPEC>`"]
pub type CPI_VSYNC_PAD_CONF_REG = crate::Reg<cpi_vsync_pad_conf_reg::CPI_VSYNC_PAD_CONF_REG_SPEC>;
#[doc = ""]
pub mod cpi_vsync_pad_conf_reg;
