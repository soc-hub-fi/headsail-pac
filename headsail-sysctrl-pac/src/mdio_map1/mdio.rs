#[doc = r"Register block"]
#[repr(C)]
pub struct MDIO {
    #[doc = "0x00 - "]
    pub mdio_ctrl: MDIO_CTRL,
    #[doc = "0x04 - "]
    pub mdio_div: MDIO_DIV,
    #[doc = "0x08 - "]
    pub mdio_wr_data: MDIO_WR_DATA,
    #[doc = "0x0c - "]
    pub mdio_rd_data: MDIO_RD_DATA,
    #[doc = "0x10 - "]
    pub mdio_status: MDIO_STATUS,
    #[doc = "0x14 - "]
    pub mdio_addr_on_phy: MDIO_ADDR_ON_PHY,
    #[doc = "0x18 - "]
    pub mdio_addr_phy: MDIO_ADDR_PHY,
}
#[doc = "MDIO_CTRL (rw) register accessor: an alias for `Reg<MDIO_CTRL_SPEC>`"]
pub type MDIO_CTRL = crate::Reg<mdio_ctrl::MDIO_CTRL_SPEC>;
#[doc = ""]
pub mod mdio_ctrl;
#[doc = "MDIO_WR_DATA (rw) register accessor: an alias for `Reg<MDIO_WR_DATA_SPEC>`"]
pub type MDIO_WR_DATA = crate::Reg<mdio_wr_data::MDIO_WR_DATA_SPEC>;
#[doc = ""]
pub mod mdio_wr_data;
#[doc = "MDIO_DIV (rw) register accessor: an alias for `Reg<MDIO_DIV_SPEC>`"]
pub type MDIO_DIV = crate::Reg<mdio_div::MDIO_DIV_SPEC>;
#[doc = ""]
pub mod mdio_div;
#[doc = "MDIO_ADDR_PHY (rw) register accessor: an alias for `Reg<MDIO_ADDR_PHY_SPEC>`"]
pub type MDIO_ADDR_PHY = crate::Reg<mdio_addr_phy::MDIO_ADDR_PHY_SPEC>;
#[doc = ""]
pub mod mdio_addr_phy;
#[doc = "MDIO_ADDR_ON_PHY (rw) register accessor: an alias for `Reg<MDIO_ADDR_ON_PHY_SPEC>`"]
pub type MDIO_ADDR_ON_PHY = crate::Reg<mdio_addr_on_phy::MDIO_ADDR_ON_PHY_SPEC>;
#[doc = ""]
pub mod mdio_addr_on_phy;
#[doc = "MDIO_STATUS (r) register accessor: an alias for `Reg<MDIO_STATUS_SPEC>`"]
pub type MDIO_STATUS = crate::Reg<mdio_status::MDIO_STATUS_SPEC>;
#[doc = ""]
pub mod mdio_status;
#[doc = "MDIO_RD_DATA (r) register accessor: an alias for `Reg<MDIO_RD_DATA_SPEC>`"]
pub type MDIO_RD_DATA = crate::Reg<mdio_rd_data::MDIO_RD_DATA_SPEC>;
#[doc = ""]
pub mod mdio_rd_data;
