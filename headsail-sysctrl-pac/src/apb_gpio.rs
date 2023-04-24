#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - addressBlock"]
    pub address_block: ADDRESS_BLOCK,
}
#[doc = "addressBlock"]
pub use self::address_block::ADDRESS_BLOCK;
#[doc = r"Cluster"]
#[doc = "addressBlock"]
pub mod address_block;
