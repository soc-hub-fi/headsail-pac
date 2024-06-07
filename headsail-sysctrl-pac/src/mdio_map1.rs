#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mdio_ctrl: MDIO_CTRL,
    mdio_div: MDIO_DIV,
    mdio_wr_data: MDIO_WR_DATA,
    mdio_rd_data: MDIO_RD_DATA,
    mdio_status: MDIO_STATUS,
    mdio_addr_on_phy: MDIO_ADDR_ON_PHY,
    mdio_addr_phy: MDIO_ADDR_PHY,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn mdio_ctrl(&self) -> &MDIO_CTRL {
        &self.mdio_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn mdio_div(&self) -> &MDIO_DIV {
        &self.mdio_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn mdio_wr_data(&self) -> &MDIO_WR_DATA {
        &self.mdio_wr_data
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn mdio_rd_data(&self) -> &MDIO_RD_DATA {
        &self.mdio_rd_data
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn mdio_status(&self) -> &MDIO_STATUS {
        &self.mdio_status
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn mdio_addr_on_phy(&self) -> &MDIO_ADDR_ON_PHY {
        &self.mdio_addr_on_phy
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn mdio_addr_phy(&self) -> &MDIO_ADDR_PHY {
        &self.mdio_addr_phy
    }
}
#[doc = "MDIO_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_ctrl`]
module"]
pub type MDIO_CTRL = crate::Reg<mdio_ctrl::MDIO_CTRL_SPEC>;
#[doc = ""]
pub mod mdio_ctrl;
#[doc = "MDIO_WR_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_wr_data`]
module"]
pub type MDIO_WR_DATA = crate::Reg<mdio_wr_data::MDIO_WR_DATA_SPEC>;
#[doc = ""]
pub mod mdio_wr_data;
#[doc = "MDIO_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_div`]
module"]
pub type MDIO_DIV = crate::Reg<mdio_div::MDIO_DIV_SPEC>;
#[doc = ""]
pub mod mdio_div;
#[doc = "MDIO_ADDR_PHY (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_addr_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_addr_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_addr_phy`]
module"]
pub type MDIO_ADDR_PHY = crate::Reg<mdio_addr_phy::MDIO_ADDR_PHY_SPEC>;
#[doc = ""]
pub mod mdio_addr_phy;
#[doc = "MDIO_ADDR_ON_PHY (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_addr_on_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_addr_on_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_addr_on_phy`]
module"]
pub type MDIO_ADDR_ON_PHY = crate::Reg<mdio_addr_on_phy::MDIO_ADDR_ON_PHY_SPEC>;
#[doc = ""]
pub mod mdio_addr_on_phy;
#[doc = "MDIO_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_status`]
module"]
pub type MDIO_STATUS = crate::Reg<mdio_status::MDIO_STATUS_SPEC>;
#[doc = ""]
pub mod mdio_status;
#[doc = "MDIO_RD_DATA (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_rd_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_rd_data`]
module"]
pub type MDIO_RD_DATA = crate::Reg<mdio_rd_data::MDIO_RD_DATA_SPEC>;
#[doc = ""]
pub mod mdio_rd_data;
