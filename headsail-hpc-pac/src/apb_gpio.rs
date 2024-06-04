#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    address_block: ADDRESS_BLOCK,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - addressBlock"]
    #[inline(always)]
    pub const fn address_block(&self) -> &ADDRESS_BLOCK {
        &self.address_block
    }
}
#[doc = "addressBlock"]
pub use self::address_block::ADDRESS_BLOCK;
#[doc = r"Cluster"]
#[doc = "addressBlock"]
pub mod address_block;
