#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x68 - CTRL"]
    pub ctrl: CTRL,
}
#[doc = "CTRL"]
pub use self::ctrl::CTRL;
#[doc = r"Cluster"]
#[doc = "CTRL"]
pub mod ctrl;
