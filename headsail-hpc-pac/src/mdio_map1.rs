#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mdio: MDIO,
}
impl RegisterBlock {
    #[doc = "0x00..0x1c - mdio"]
    #[inline(always)]
    pub const fn mdio(&self) -> &MDIO {
        &self.mdio
    }
}
#[doc = "mdio"]
pub use self::mdio::MDIO;
#[doc = r"Cluster"]
#[doc = "mdio"]
pub mod mdio;
