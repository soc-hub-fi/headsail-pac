#[doc = r"Register block"]
#[repr(C)]
pub struct APB_SOFTWARE_IRQ {
    #[doc = "0x00 - "]
    pub read: READ,
    #[doc = "0x04 - "]
    pub set: SET,
    #[doc = "0x08 - "]
    pub clear: CLEAR,
}
#[doc = "read (r) register accessor: an alias for `Reg<READ_SPEC>`"]
pub type READ = crate::Reg<read::READ_SPEC>;
#[doc = ""]
pub mod read;
#[doc = "set (w) register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = ""]
pub mod set;
#[doc = "clear (w) register accessor: an alias for `Reg<CLEAR_SPEC>`"]
pub type CLEAR = crate::Reg<clear::CLEAR_SPEC>;
#[doc = ""]
pub mod clear;
