#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    registers: REGISTERS,
}
impl RegisterBlock {
    #[doc = "0x00..0x2c - registers"]
    #[inline(always)]
    pub const fn registers(&self) -> &REGISTERS {
        &self.registers
    }
}
#[doc = "registers"]
pub use self::registers::REGISTERS;
#[doc = r"Cluster"]
#[doc = "registers"]
pub mod registers;
