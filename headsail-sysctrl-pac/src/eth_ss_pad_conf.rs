#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pad_rgmii_1_rx_clk: PadRgmii1RxClk,
    pad_rgmii_1_rx_ctl: PadRgmii1RxCtl,
    pad_rgmii_1_rx_data: PadRgmii1RxData,
    pad_rgmii_1_tx_clk: PadRgmii1TxClk,
    pad_rgmii_1_tx_ctl: PadRgmii1TxCtl,
    pad_rgmii_1_tx_data: PadRgmii1TxData,
    pad_rgmii_2_rx_clk: PadRgmii2RxClk,
    pad_rgmii_2_rx_ctl: PadRgmii2RxCtl,
    pad_rgmii_2_rx_data: PadRgmii2RxData,
    pad_rgmii_2_tx_clk: PadRgmii2TxClk,
    pad_rgmii_2_tx_ctl: PadRgmii2TxCtl,
    pad_rgmii_2_tx_data: PadRgmii2TxData,
    pad_mdio_1_mdc: PadMdio1Mdc,
    pad_mdio_1_mdio: PadMdio1Mdio,
    pad_mdio_2_mdc: PadMdio2Mdc,
    pad_mdio_2_mdio: PadMdio2Mdio,
    rgmii_sel: RgmiiSel,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_rx_clk(&self) -> &PadRgmii1RxClk {
        &self.pad_rgmii_1_rx_clk
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_rx_ctl(&self) -> &PadRgmii1RxCtl {
        &self.pad_rgmii_1_rx_ctl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_rx_data(&self) -> &PadRgmii1RxData {
        &self.pad_rgmii_1_rx_data
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_tx_clk(&self) -> &PadRgmii1TxClk {
        &self.pad_rgmii_1_tx_clk
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_tx_ctl(&self) -> &PadRgmii1TxCtl {
        &self.pad_rgmii_1_tx_ctl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_tx_data(&self) -> &PadRgmii1TxData {
        &self.pad_rgmii_1_tx_data
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_rx_clk(&self) -> &PadRgmii2RxClk {
        &self.pad_rgmii_2_rx_clk
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_rx_ctl(&self) -> &PadRgmii2RxCtl {
        &self.pad_rgmii_2_rx_ctl
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_rx_data(&self) -> &PadRgmii2RxData {
        &self.pad_rgmii_2_rx_data
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_tx_clk(&self) -> &PadRgmii2TxClk {
        &self.pad_rgmii_2_tx_clk
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_tx_ctl(&self) -> &PadRgmii2TxCtl {
        &self.pad_rgmii_2_tx_ctl
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_tx_data(&self) -> &PadRgmii2TxData {
        &self.pad_rgmii_2_tx_data
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn pad_mdio_1_mdc(&self) -> &PadMdio1Mdc {
        &self.pad_mdio_1_mdc
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn pad_mdio_1_mdio(&self) -> &PadMdio1Mdio {
        &self.pad_mdio_1_mdio
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn pad_mdio_2_mdc(&self) -> &PadMdio2Mdc {
        &self.pad_mdio_2_mdc
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn pad_mdio_2_mdio(&self) -> &PadMdio2Mdio {
        &self.pad_mdio_2_mdio
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn rgmii_sel(&self) -> &RgmiiSel {
        &self.rgmii_sel
    }
}
#[doc = "PAD_RGMII_1_RX_CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_rx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_rx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_rx_clk`]
module"]
#[doc(alias = "PAD_RGMII_1_RX_CLK")]
pub type PadRgmii1RxClk = crate::Reg<pad_rgmii_1_rx_clk::PadRgmii1RxClkSpec>;
#[doc = ""]
pub mod pad_rgmii_1_rx_clk;
#[doc = "RGMII_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgmii_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgmii_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgmii_sel`]
module"]
#[doc(alias = "RGMII_SEL")]
pub type RgmiiSel = crate::Reg<rgmii_sel::RgmiiSelSpec>;
#[doc = ""]
pub mod rgmii_sel;
#[doc = "PAD_MDIO_2_MDIO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mdio_2_mdio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mdio_2_mdio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mdio_2_mdio`]
module"]
#[doc(alias = "PAD_MDIO_2_MDIO")]
pub type PadMdio2Mdio = crate::Reg<pad_mdio_2_mdio::PadMdio2MdioSpec>;
#[doc = ""]
pub mod pad_mdio_2_mdio;
#[doc = "PAD_RGMII_1_RX_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_rx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_rx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_rx_data`]
module"]
#[doc(alias = "PAD_RGMII_1_RX_DATA")]
pub type PadRgmii1RxData = crate::Reg<pad_rgmii_1_rx_data::PadRgmii1RxDataSpec>;
#[doc = ""]
pub mod pad_rgmii_1_rx_data;
#[doc = "PAD_RGMII_2_TX_CTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_tx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_tx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_tx_ctl`]
module"]
#[doc(alias = "PAD_RGMII_2_TX_CTL")]
pub type PadRgmii2TxCtl = crate::Reg<pad_rgmii_2_tx_ctl::PadRgmii2TxCtlSpec>;
#[doc = ""]
pub mod pad_rgmii_2_tx_ctl;
#[doc = "PAD_RGMII_2_RX_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_rx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_rx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_rx_data`]
module"]
#[doc(alias = "PAD_RGMII_2_RX_DATA")]
pub type PadRgmii2RxData = crate::Reg<pad_rgmii_2_rx_data::PadRgmii2RxDataSpec>;
#[doc = ""]
pub mod pad_rgmii_2_rx_data;
#[doc = "PAD_RGMII_2_RX_CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_rx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_rx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_rx_clk`]
module"]
#[doc(alias = "PAD_RGMII_2_RX_CLK")]
pub type PadRgmii2RxClk = crate::Reg<pad_rgmii_2_rx_clk::PadRgmii2RxClkSpec>;
#[doc = ""]
pub mod pad_rgmii_2_rx_clk;
#[doc = "PAD_RGMII_2_RX_CTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_rx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_rx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_rx_ctl`]
module"]
#[doc(alias = "PAD_RGMII_2_RX_CTL")]
pub type PadRgmii2RxCtl = crate::Reg<pad_rgmii_2_rx_ctl::PadRgmii2RxCtlSpec>;
#[doc = ""]
pub mod pad_rgmii_2_rx_ctl;
#[doc = "PAD_RGMII_1_TX_CTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_tx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_tx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_tx_ctl`]
module"]
#[doc(alias = "PAD_RGMII_1_TX_CTL")]
pub type PadRgmii1TxCtl = crate::Reg<pad_rgmii_1_tx_ctl::PadRgmii1TxCtlSpec>;
#[doc = ""]
pub mod pad_rgmii_1_tx_ctl;
#[doc = "PAD_RGMII_1_TX_CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_tx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_tx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_tx_clk`]
module"]
#[doc(alias = "PAD_RGMII_1_TX_CLK")]
pub type PadRgmii1TxClk = crate::Reg<pad_rgmii_1_tx_clk::PadRgmii1TxClkSpec>;
#[doc = ""]
pub mod pad_rgmii_1_tx_clk;
#[doc = "PAD_RGMII_1_TX_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_tx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_tx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_tx_data`]
module"]
#[doc(alias = "PAD_RGMII_1_TX_DATA")]
pub type PadRgmii1TxData = crate::Reg<pad_rgmii_1_tx_data::PadRgmii1TxDataSpec>;
#[doc = ""]
pub mod pad_rgmii_1_tx_data;
#[doc = "PAD_RGMII_2_TX_CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_tx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_tx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_tx_clk`]
module"]
#[doc(alias = "PAD_RGMII_2_TX_CLK")]
pub type PadRgmii2TxClk = crate::Reg<pad_rgmii_2_tx_clk::PadRgmii2TxClkSpec>;
#[doc = ""]
pub mod pad_rgmii_2_tx_clk;
#[doc = "PAD_MDIO_1_MDC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mdio_1_mdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mdio_1_mdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mdio_1_mdc`]
module"]
#[doc(alias = "PAD_MDIO_1_MDC")]
pub type PadMdio1Mdc = crate::Reg<pad_mdio_1_mdc::PadMdio1MdcSpec>;
#[doc = ""]
pub mod pad_mdio_1_mdc;
#[doc = "PAD_MDIO_1_MDIO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mdio_1_mdio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mdio_1_mdio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mdio_1_mdio`]
module"]
#[doc(alias = "PAD_MDIO_1_MDIO")]
pub type PadMdio1Mdio = crate::Reg<pad_mdio_1_mdio::PadMdio1MdioSpec>;
#[doc = ""]
pub mod pad_mdio_1_mdio;
#[doc = "PAD_RGMII_2_TX_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_tx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_tx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_tx_data`]
module"]
#[doc(alias = "PAD_RGMII_2_TX_DATA")]
pub type PadRgmii2TxData = crate::Reg<pad_rgmii_2_tx_data::PadRgmii2TxDataSpec>;
#[doc = ""]
pub mod pad_rgmii_2_tx_data;
#[doc = "PAD_MDIO_2_MDC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mdio_2_mdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mdio_2_mdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mdio_2_mdc`]
module"]
#[doc(alias = "PAD_MDIO_2_MDC")]
pub type PadMdio2Mdc = crate::Reg<pad_mdio_2_mdc::PadMdio2MdcSpec>;
#[doc = ""]
pub mod pad_mdio_2_mdc;
#[doc = "PAD_RGMII_1_RX_CTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_rx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_rx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_rx_ctl`]
module"]
#[doc(alias = "PAD_RGMII_1_RX_CTL")]
pub type PadRgmii1RxCtl = crate::Reg<pad_rgmii_1_rx_ctl::PadRgmii1RxCtlSpec>;
#[doc = ""]
pub mod pad_rgmii_1_rx_ctl;
