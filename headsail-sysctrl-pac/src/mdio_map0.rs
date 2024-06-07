#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mdio_ctrl: MdioCtrl,
    mdio_div: MdioDiv,
    mdio_wr_data: MdioWrData,
    mdio_rd_data: MdioRdData,
    mdio_status: MdioStatus,
    mdio_addr_on_phy: MdioAddrOnPhy,
    mdio_addr_phy: MdioAddrPhy,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn mdio_ctrl(&self) -> &MdioCtrl {
        &self.mdio_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn mdio_div(&self) -> &MdioDiv {
        &self.mdio_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn mdio_wr_data(&self) -> &MdioWrData {
        &self.mdio_wr_data
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn mdio_rd_data(&self) -> &MdioRdData {
        &self.mdio_rd_data
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn mdio_status(&self) -> &MdioStatus {
        &self.mdio_status
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn mdio_addr_on_phy(&self) -> &MdioAddrOnPhy {
        &self.mdio_addr_on_phy
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn mdio_addr_phy(&self) -> &MdioAddrPhy {
        &self.mdio_addr_phy
    }
}
#[doc = "MDIO_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_ctrl`]
module"]
#[doc(alias = "MDIO_CTRL")]
pub type MdioCtrl = crate::Reg<mdio_ctrl::MdioCtrlSpec>;
#[doc = ""]
pub mod mdio_ctrl;
#[doc = "MDIO_WR_DATA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_wr_data`]
module"]
#[doc(alias = "MDIO_WR_DATA")]
pub type MdioWrData = crate::Reg<mdio_wr_data::MdioWrDataSpec>;
#[doc = ""]
pub mod mdio_wr_data;
#[doc = "MDIO_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_div`]
module"]
#[doc(alias = "MDIO_DIV")]
pub type MdioDiv = crate::Reg<mdio_div::MdioDivSpec>;
#[doc = ""]
pub mod mdio_div;
#[doc = "MDIO_ADDR_PHY (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_addr_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_addr_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_addr_phy`]
module"]
#[doc(alias = "MDIO_ADDR_PHY")]
pub type MdioAddrPhy = crate::Reg<mdio_addr_phy::MdioAddrPhySpec>;
#[doc = ""]
pub mod mdio_addr_phy;
#[doc = "MDIO_ADDR_ON_PHY (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_addr_on_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_addr_on_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_addr_on_phy`]
module"]
#[doc(alias = "MDIO_ADDR_ON_PHY")]
pub type MdioAddrOnPhy = crate::Reg<mdio_addr_on_phy::MdioAddrOnPhySpec>;
#[doc = ""]
pub mod mdio_addr_on_phy;
#[doc = "MDIO_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_status`]
module"]
#[doc(alias = "MDIO_STATUS")]
pub type MdioStatus = crate::Reg<mdio_status::MdioStatusSpec>;
#[doc = ""]
pub mod mdio_status;
#[doc = "MDIO_RD_DATA (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_rd_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdio_rd_data`]
module"]
#[doc(alias = "MDIO_RD_DATA")]
pub type MdioRdData = crate::Reg<mdio_rd_data::MdioRdDataSpec>;
#[doc = ""]
pub mod mdio_rd_data;
