#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    configuration_registers: CONFIGURATION_REGISTERS,
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - Configuration_Registers"]
    #[inline(always)]
    pub const fn configuration_registers(&self) -> &CONFIGURATION_REGISTERS {
        &self.configuration_registers
    }
}
#[doc = "Configuration_Registers"]
pub use self::configuration_registers::CONFIGURATION_REGISTERS;
#[doc = r"Cluster"]
#[doc = "Configuration_Registers"]
pub mod configuration_registers;
