#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x5c - Configuration_Registers"]
    pub configuration_registers: CONFIGURATION_REGISTERS,
}
#[doc = "Configuration_Registers"]
pub use self::configuration_registers::CONFIGURATION_REGISTERS;
#[doc = r"Cluster"]
#[doc = "Configuration_Registers"]
pub mod configuration_registers;
