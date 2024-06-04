#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    configuration: CONFIGURATION,
}
impl RegisterBlock {
    #[doc = "0x00..0x74 - Configuration"]
    #[inline(always)]
    pub const fn configuration(&self) -> &CONFIGURATION {
        &self.configuration
    }
}
#[doc = "Configuration"]
pub use self::configuration::CONFIGURATION;
#[doc = r"Cluster"]
#[doc = "Configuration"]
pub mod configuration;
