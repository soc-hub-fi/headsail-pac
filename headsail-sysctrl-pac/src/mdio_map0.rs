#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x1c - mdio"]
    pub mdio: MDIO,
}
#[doc = "mdio"]
pub use self::mdio::MDIO;
#[doc = r"Cluster"]
#[doc = "mdio"]
pub mod mdio;
