#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x08 - registers"]
    pub registers: REGISTERS,
}
#[doc = "registers"]
pub use self::registers::REGISTERS;
#[doc = r"Cluster"]
#[doc = "registers"]
pub mod registers;
