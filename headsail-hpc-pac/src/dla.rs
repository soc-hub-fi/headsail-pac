#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
}
impl RegisterBlock {
    #[doc = "0x00..0x68 - CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
}
#[doc = "CTRL"]
pub use self::ctrl::CTRL;
#[doc = r"Cluster"]
#[doc = "CTRL"]
pub mod ctrl;
