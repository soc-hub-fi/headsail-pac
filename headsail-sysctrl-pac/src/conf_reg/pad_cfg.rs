#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_CFG {
    #[doc = "0x00 - "]
    pub pad_rgmii_1_rx_clk: PAD_RGMII_1_RX_CLK,
    #[doc = "0x04 - "]
    pub pad_rgmii_1_rx_ctl: PAD_RGMII_1_RX_CTL,
    #[doc = "0x08 - "]
    pub pad_rgmii_1_rx_data: PAD_RGMII_1_RX_DATA,
    #[doc = "0x0c - "]
    pub pad_rgmii_1_tx_clk: PAD_RGMII_1_TX_CLK,
    #[doc = "0x10 - "]
    pub pad_rgmii_1_tx_ctl: PAD_RGMII_1_TX_CTL,
    #[doc = "0x14 - "]
    pub pad_rgmii_1_tx_data: PAD_RGMII_1_TX_DATA,
    #[doc = "0x18 - "]
    pub pad_rgmii_2_rx_clk: PAD_RGMII_2_RX_CLK,
    #[doc = "0x1c - "]
    pub pad_rgmii_2_rx_ctl: PAD_RGMII_2_RX_CTL,
    #[doc = "0x20 - "]
    pub pad_rgmii_2_rx_data: PAD_RGMII_2_RX_DATA,
    #[doc = "0x24 - "]
    pub pad_rgmii_2_tx_clk: PAD_RGMII_2_TX_CLK,
    #[doc = "0x28 - "]
    pub pad_rgmii_2_tx_ctl: PAD_RGMII_2_TX_CTL,
    #[doc = "0x2c - "]
    pub pad_rgmii_2_tx_data: PAD_RGMII_2_TX_DATA,
    #[doc = "0x30 - "]
    pub pad_mdio_1_mdc: PAD_MDIO_1_MDC,
    #[doc = "0x34 - "]
    pub pad_mdio_1_mdio: PAD_MDIO_1_MDIO,
    #[doc = "0x38 - "]
    pub pad_mdio_2_mdc: PAD_MDIO_2_MDC,
    #[doc = "0x3c - "]
    pub pad_mdio_2_mdio: PAD_MDIO_2_MDIO,
    #[doc = "0x40 - "]
    pub rgmii_sel: RGMII_SEL,
}
#[doc = "PAD_RGMII_1_RX_CLK (rw) register accessor: an alias for `Reg<PAD_RGMII_1_RX_CLK_SPEC>`"]
pub type PAD_RGMII_1_RX_CLK = crate::Reg<pad_rgmii_1_rx_clk::PAD_RGMII_1_RX_CLK_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_rx_clk;
#[doc = "RGMII_SEL (rw) register accessor: an alias for `Reg<RGMII_SEL_SPEC>`"]
pub type RGMII_SEL = crate::Reg<rgmii_sel::RGMII_SEL_SPEC>;
#[doc = ""]
pub mod rgmii_sel;
#[doc = "PAD_MDIO_2_MDIO (rw) register accessor: an alias for `Reg<PAD_MDIO_2_MDIO_SPEC>`"]
pub type PAD_MDIO_2_MDIO = crate::Reg<pad_mdio_2_mdio::PAD_MDIO_2_MDIO_SPEC>;
#[doc = ""]
pub mod pad_mdio_2_mdio;
#[doc = "PAD_RGMII_1_RX_DATA (rw) register accessor: an alias for `Reg<PAD_RGMII_1_RX_DATA_SPEC>`"]
pub type PAD_RGMII_1_RX_DATA = crate::Reg<pad_rgmii_1_rx_data::PAD_RGMII_1_RX_DATA_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_rx_data;
#[doc = "PAD_RGMII_2_TX_CTL (rw) register accessor: an alias for `Reg<PAD_RGMII_2_TX_CTL_SPEC>`"]
pub type PAD_RGMII_2_TX_CTL = crate::Reg<pad_rgmii_2_tx_ctl::PAD_RGMII_2_TX_CTL_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_tx_ctl;
#[doc = "PAD_RGMII_2_RX_DATA (rw) register accessor: an alias for `Reg<PAD_RGMII_2_RX_DATA_SPEC>`"]
pub type PAD_RGMII_2_RX_DATA = crate::Reg<pad_rgmii_2_rx_data::PAD_RGMII_2_RX_DATA_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_rx_data;
#[doc = "PAD_RGMII_2_RX_CLK (rw) register accessor: an alias for `Reg<PAD_RGMII_2_RX_CLK_SPEC>`"]
pub type PAD_RGMII_2_RX_CLK = crate::Reg<pad_rgmii_2_rx_clk::PAD_RGMII_2_RX_CLK_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_rx_clk;
#[doc = "PAD_RGMII_2_RX_CTL (rw) register accessor: an alias for `Reg<PAD_RGMII_2_RX_CTL_SPEC>`"]
pub type PAD_RGMII_2_RX_CTL = crate::Reg<pad_rgmii_2_rx_ctl::PAD_RGMII_2_RX_CTL_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_rx_ctl;
#[doc = "PAD_RGMII_1_TX_CTL (rw) register accessor: an alias for `Reg<PAD_RGMII_1_TX_CTL_SPEC>`"]
pub type PAD_RGMII_1_TX_CTL = crate::Reg<pad_rgmii_1_tx_ctl::PAD_RGMII_1_TX_CTL_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_tx_ctl;
#[doc = "PAD_RGMII_1_TX_CLK (rw) register accessor: an alias for `Reg<PAD_RGMII_1_TX_CLK_SPEC>`"]
pub type PAD_RGMII_1_TX_CLK = crate::Reg<pad_rgmii_1_tx_clk::PAD_RGMII_1_TX_CLK_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_tx_clk;
#[doc = "PAD_RGMII_1_TX_DATA (rw) register accessor: an alias for `Reg<PAD_RGMII_1_TX_DATA_SPEC>`"]
pub type PAD_RGMII_1_TX_DATA = crate::Reg<pad_rgmii_1_tx_data::PAD_RGMII_1_TX_DATA_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_tx_data;
#[doc = "PAD_RGMII_2_TX_CLK (rw) register accessor: an alias for `Reg<PAD_RGMII_2_TX_CLK_SPEC>`"]
pub type PAD_RGMII_2_TX_CLK = crate::Reg<pad_rgmii_2_tx_clk::PAD_RGMII_2_TX_CLK_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_tx_clk;
#[doc = "PAD_MDIO_1_MDC (rw) register accessor: an alias for `Reg<PAD_MDIO_1_MDC_SPEC>`"]
pub type PAD_MDIO_1_MDC = crate::Reg<pad_mdio_1_mdc::PAD_MDIO_1_MDC_SPEC>;
#[doc = ""]
pub mod pad_mdio_1_mdc;
#[doc = "PAD_MDIO_1_MDIO (rw) register accessor: an alias for `Reg<PAD_MDIO_1_MDIO_SPEC>`"]
pub type PAD_MDIO_1_MDIO = crate::Reg<pad_mdio_1_mdio::PAD_MDIO_1_MDIO_SPEC>;
#[doc = ""]
pub mod pad_mdio_1_mdio;
#[doc = "PAD_RGMII_2_TX_DATA (rw) register accessor: an alias for `Reg<PAD_RGMII_2_TX_DATA_SPEC>`"]
pub type PAD_RGMII_2_TX_DATA = crate::Reg<pad_rgmii_2_tx_data::PAD_RGMII_2_TX_DATA_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_tx_data;
#[doc = "PAD_MDIO_2_MDC (rw) register accessor: an alias for `Reg<PAD_MDIO_2_MDC_SPEC>`"]
pub type PAD_MDIO_2_MDC = crate::Reg<pad_mdio_2_mdc::PAD_MDIO_2_MDC_SPEC>;
#[doc = ""]
pub mod pad_mdio_2_mdc;
#[doc = "PAD_RGMII_1_RX_CTL (rw) register accessor: an alias for `Reg<PAD_RGMII_1_RX_CTL_SPEC>`"]
pub type PAD_RGMII_1_RX_CTL = crate::Reg<pad_rgmii_1_rx_ctl::PAD_RGMII_1_RX_CTL_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_rx_ctl;
