#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x74 - Configuration"]
    pub configuration: CONFIGURATION,
}
#[doc = "Configuration"]
pub use self::configuration::CONFIGURATION;
#[doc = r"Cluster"]
#[doc = "Configuration"]
pub mod configuration;
