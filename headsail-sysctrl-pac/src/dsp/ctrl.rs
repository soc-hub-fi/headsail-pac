#[doc = r"Register block"]
#[repr(C)]
pub struct CTRL {
    #[doc = "0x00 - Command (1 = reset, 2 = run, 4 = break)"]
    pub cmd: CMD,
    #[doc = "0x04 - "]
    pub start_addr: START_ADDR,
    #[doc = "0x08 - Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2"]
    pub bp_en: BP_EN,
    #[doc = "0x0c - Cycle count breakpoint"]
    pub cycle_count_bp: CYCLE_COUNT_BP,
    #[doc = "0x10 - Breakpoint 2 address"]
    pub bp2_addr: BP2_ADDR,
    #[doc = "0x14 - Breakpoint 1 address"]
    pub bp1_addr: BP1_ADDR,
}
#[doc = "cmd (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command (1 = reset, 2 = run, 4 = break)"]
pub mod cmd;
#[doc = "start_addr (rw) register accessor: an alias for `Reg<START_ADDR_SPEC>`"]
pub type START_ADDR = crate::Reg<start_addr::START_ADDR_SPEC>;
#[doc = ""]
pub mod start_addr;
#[doc = "bp_en (rw) register accessor: an alias for `Reg<BP_EN_SPEC>`"]
pub type BP_EN = crate::Reg<bp_en::BP_EN_SPEC>;
#[doc = "Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2"]
pub mod bp_en;
#[doc = "bp1_addr (rw) register accessor: an alias for `Reg<BP1_ADDR_SPEC>`"]
pub type BP1_ADDR = crate::Reg<bp1_addr::BP1_ADDR_SPEC>;
#[doc = "Breakpoint 1 address"]
pub mod bp1_addr;
#[doc = "bp2_addr (rw) register accessor: an alias for `Reg<BP2_ADDR_SPEC>`"]
pub type BP2_ADDR = crate::Reg<bp2_addr::BP2_ADDR_SPEC>;
#[doc = "Breakpoint 2 address"]
pub mod bp2_addr;
#[doc = "cycle_count_bp (rw) register accessor: an alias for `Reg<CYCLE_COUNT_BP_SPEC>`"]
pub type CYCLE_COUNT_BP = crate::Reg<cycle_count_bp::CYCLE_COUNT_BP_SPEC>;
#[doc = "Cycle count breakpoint"]
pub mod cycle_count_bp;
