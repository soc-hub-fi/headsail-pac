#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pad_rgmii_1_rx_clk: PAD_RGMII_1_RX_CLK,
    pad_rgmii_1_rx_ctl: PAD_RGMII_1_RX_CTL,
    pad_rgmii_1_rx_data: PAD_RGMII_1_RX_DATA,
    pad_rgmii_1_tx_clk: PAD_RGMII_1_TX_CLK,
    pad_rgmii_1_tx_ctl: PAD_RGMII_1_TX_CTL,
    pad_rgmii_1_tx_data: PAD_RGMII_1_TX_DATA,
    pad_rgmii_2_rx_clk: PAD_RGMII_2_RX_CLK,
    pad_rgmii_2_rx_ctl: PAD_RGMII_2_RX_CTL,
    pad_rgmii_2_rx_data: PAD_RGMII_2_RX_DATA,
    pad_rgmii_2_tx_clk: PAD_RGMII_2_TX_CLK,
    pad_rgmii_2_tx_ctl: PAD_RGMII_2_TX_CTL,
    pad_rgmii_2_tx_data: PAD_RGMII_2_TX_DATA,
    pad_mdio_1_mdc: PAD_MDIO_1_MDC,
    pad_mdio_1_mdio: PAD_MDIO_1_MDIO,
    pad_mdio_2_mdc: PAD_MDIO_2_MDC,
    pad_mdio_2_mdio: PAD_MDIO_2_MDIO,
    rgmii_sel: RGMII_SEL,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_rx_clk(&self) -> &PAD_RGMII_1_RX_CLK {
        &self.pad_rgmii_1_rx_clk
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_rx_ctl(&self) -> &PAD_RGMII_1_RX_CTL {
        &self.pad_rgmii_1_rx_ctl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_rx_data(&self) -> &PAD_RGMII_1_RX_DATA {
        &self.pad_rgmii_1_rx_data
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_tx_clk(&self) -> &PAD_RGMII_1_TX_CLK {
        &self.pad_rgmii_1_tx_clk
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_tx_ctl(&self) -> &PAD_RGMII_1_TX_CTL {
        &self.pad_rgmii_1_tx_ctl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn pad_rgmii_1_tx_data(&self) -> &PAD_RGMII_1_TX_DATA {
        &self.pad_rgmii_1_tx_data
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_rx_clk(&self) -> &PAD_RGMII_2_RX_CLK {
        &self.pad_rgmii_2_rx_clk
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_rx_ctl(&self) -> &PAD_RGMII_2_RX_CTL {
        &self.pad_rgmii_2_rx_ctl
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_rx_data(&self) -> &PAD_RGMII_2_RX_DATA {
        &self.pad_rgmii_2_rx_data
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_tx_clk(&self) -> &PAD_RGMII_2_TX_CLK {
        &self.pad_rgmii_2_tx_clk
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_tx_ctl(&self) -> &PAD_RGMII_2_TX_CTL {
        &self.pad_rgmii_2_tx_ctl
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn pad_rgmii_2_tx_data(&self) -> &PAD_RGMII_2_TX_DATA {
        &self.pad_rgmii_2_tx_data
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn pad_mdio_1_mdc(&self) -> &PAD_MDIO_1_MDC {
        &self.pad_mdio_1_mdc
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn pad_mdio_1_mdio(&self) -> &PAD_MDIO_1_MDIO {
        &self.pad_mdio_1_mdio
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn pad_mdio_2_mdc(&self) -> &PAD_MDIO_2_MDC {
        &self.pad_mdio_2_mdc
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn pad_mdio_2_mdio(&self) -> &PAD_MDIO_2_MDIO {
        &self.pad_mdio_2_mdio
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn rgmii_sel(&self) -> &RGMII_SEL {
        &self.rgmii_sel
    }
}
#[doc = "PAD_RGMII_1_RX_CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_rx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_rx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_rx_clk`]
module"]
pub type PAD_RGMII_1_RX_CLK = crate::Reg<pad_rgmii_1_rx_clk::PAD_RGMII_1_RX_CLK_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_rx_clk;
#[doc = "RGMII_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgmii_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgmii_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgmii_sel`]
module"]
pub type RGMII_SEL = crate::Reg<rgmii_sel::RGMII_SEL_SPEC>;
#[doc = ""]
pub mod rgmii_sel;
#[doc = "PAD_MDIO_2_MDIO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mdio_2_mdio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mdio_2_mdio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mdio_2_mdio`]
module"]
pub type PAD_MDIO_2_MDIO = crate::Reg<pad_mdio_2_mdio::PAD_MDIO_2_MDIO_SPEC>;
#[doc = ""]
pub mod pad_mdio_2_mdio;
#[doc = "PAD_RGMII_1_RX_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_rx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_rx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_rx_data`]
module"]
pub type PAD_RGMII_1_RX_DATA = crate::Reg<pad_rgmii_1_rx_data::PAD_RGMII_1_RX_DATA_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_rx_data;
#[doc = "PAD_RGMII_2_TX_CTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_tx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_tx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_tx_ctl`]
module"]
pub type PAD_RGMII_2_TX_CTL = crate::Reg<pad_rgmii_2_tx_ctl::PAD_RGMII_2_TX_CTL_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_tx_ctl;
#[doc = "PAD_RGMII_2_RX_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_rx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_rx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_rx_data`]
module"]
pub type PAD_RGMII_2_RX_DATA = crate::Reg<pad_rgmii_2_rx_data::PAD_RGMII_2_RX_DATA_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_rx_data;
#[doc = "PAD_RGMII_2_RX_CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_rx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_rx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_rx_clk`]
module"]
pub type PAD_RGMII_2_RX_CLK = crate::Reg<pad_rgmii_2_rx_clk::PAD_RGMII_2_RX_CLK_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_rx_clk;
#[doc = "PAD_RGMII_2_RX_CTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_rx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_rx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_rx_ctl`]
module"]
pub type PAD_RGMII_2_RX_CTL = crate::Reg<pad_rgmii_2_rx_ctl::PAD_RGMII_2_RX_CTL_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_rx_ctl;
#[doc = "PAD_RGMII_1_TX_CTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_tx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_tx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_tx_ctl`]
module"]
pub type PAD_RGMII_1_TX_CTL = crate::Reg<pad_rgmii_1_tx_ctl::PAD_RGMII_1_TX_CTL_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_tx_ctl;
#[doc = "PAD_RGMII_1_TX_CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_tx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_tx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_tx_clk`]
module"]
pub type PAD_RGMII_1_TX_CLK = crate::Reg<pad_rgmii_1_tx_clk::PAD_RGMII_1_TX_CLK_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_tx_clk;
#[doc = "PAD_RGMII_1_TX_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_tx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_tx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_tx_data`]
module"]
pub type PAD_RGMII_1_TX_DATA = crate::Reg<pad_rgmii_1_tx_data::PAD_RGMII_1_TX_DATA_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_tx_data;
#[doc = "PAD_RGMII_2_TX_CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_tx_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_tx_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_tx_clk`]
module"]
pub type PAD_RGMII_2_TX_CLK = crate::Reg<pad_rgmii_2_tx_clk::PAD_RGMII_2_TX_CLK_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_tx_clk;
#[doc = "PAD_MDIO_1_MDC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mdio_1_mdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mdio_1_mdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mdio_1_mdc`]
module"]
pub type PAD_MDIO_1_MDC = crate::Reg<pad_mdio_1_mdc::PAD_MDIO_1_MDC_SPEC>;
#[doc = ""]
pub mod pad_mdio_1_mdc;
#[doc = "PAD_MDIO_1_MDIO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mdio_1_mdio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mdio_1_mdio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mdio_1_mdio`]
module"]
pub type PAD_MDIO_1_MDIO = crate::Reg<pad_mdio_1_mdio::PAD_MDIO_1_MDIO_SPEC>;
#[doc = ""]
pub mod pad_mdio_1_mdio;
#[doc = "PAD_RGMII_2_TX_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_tx_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_tx_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_2_tx_data`]
module"]
pub type PAD_RGMII_2_TX_DATA = crate::Reg<pad_rgmii_2_tx_data::PAD_RGMII_2_TX_DATA_SPEC>;
#[doc = ""]
pub mod pad_rgmii_2_tx_data;
#[doc = "PAD_MDIO_2_MDC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mdio_2_mdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mdio_2_mdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_mdio_2_mdc`]
module"]
pub type PAD_MDIO_2_MDC = crate::Reg<pad_mdio_2_mdc::PAD_MDIO_2_MDC_SPEC>;
#[doc = ""]
pub mod pad_mdio_2_mdc;
#[doc = "PAD_RGMII_1_RX_CTL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_1_rx_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_1_rx_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_rgmii_1_rx_ctl`]
module"]
pub type PAD_RGMII_1_RX_CTL = crate::Reg<pad_rgmii_1_rx_ctl::PAD_RGMII_1_RX_CTL_SPEC>;
#[doc = ""]
pub mod pad_rgmii_1_rx_ctl;
