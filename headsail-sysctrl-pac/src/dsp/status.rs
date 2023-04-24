#[doc = r"Register block"]
#[repr(C)]
pub struct STATUS {
    #[doc = "0x00 - Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint"]
    pub dsp_status: DSP_STATUS,
    #[doc = "0x04 - Program counter"]
    pub pc: PC,
    #[doc = "0x08 - Low part of Cycle count register"]
    pub cycle_count_lo: CYCLE_COUNT_LO,
    #[doc = "0x0c - High part of Cycle count register"]
    pub cycle_count_hi: CYCLE_COUNT_HI,
    #[doc = "0x10 - Low part of Stall count"]
    pub stall_count_lo: STALL_COUNT_LO,
    #[doc = "0x14 - High part of Stall count"]
    pub stall_count_hi: STALL_COUNT_HI,
}
#[doc = "dsp_status (r) register accessor: an alias for `Reg<DSP_STATUS_SPEC>`"]
pub type DSP_STATUS = crate::Reg<dsp_status::DSP_STATUS_SPEC>;
#[doc = "Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint"]
pub mod dsp_status;
#[doc = "pc (r) register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Program counter"]
pub mod pc;
#[doc = "cycle_count_lo (r) register accessor: an alias for `Reg<CYCLE_COUNT_LO_SPEC>`"]
pub type CYCLE_COUNT_LO = crate::Reg<cycle_count_lo::CYCLE_COUNT_LO_SPEC>;
#[doc = "Low part of Cycle count register"]
pub mod cycle_count_lo;
#[doc = "cycle_count_hi (r) register accessor: an alias for `Reg<CYCLE_COUNT_HI_SPEC>`"]
pub type CYCLE_COUNT_HI = crate::Reg<cycle_count_hi::CYCLE_COUNT_HI_SPEC>;
#[doc = "High part of Cycle count register"]
pub mod cycle_count_hi;
#[doc = "stall_count_lo (r) register accessor: an alias for `Reg<STALL_COUNT_LO_SPEC>`"]
pub type STALL_COUNT_LO = crate::Reg<stall_count_lo::STALL_COUNT_LO_SPEC>;
#[doc = "Low part of Stall count"]
pub mod stall_count_lo;
#[doc = "stall_count_hi (r) register accessor: an alias for `Reg<STALL_COUNT_HI_SPEC>`"]
pub type STALL_COUNT_HI = crate::Reg<stall_count_hi::STALL_COUNT_HI_SPEC>;
#[doc = "High part of Stall count"]
pub mod stall_count_hi;
