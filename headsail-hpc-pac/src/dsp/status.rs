#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "status"]
#[doc(alias = "status")]
pub struct Status {
    dsp_status: DspStatus,
    pc: Pc,
    cycle_count_lo: CycleCountLo,
    cycle_count_hi: CycleCountHi,
    stall_count_lo: StallCountLo,
    stall_count_hi: StallCountHi,
}
impl Status {
    #[doc = "0x00 - Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint"]
    #[inline(always)]
    pub const fn dsp_status(&self) -> &DspStatus {
        &self.dsp_status
    }
    #[doc = "0x04 - Program counter"]
    #[inline(always)]
    pub const fn pc(&self) -> &Pc {
        &self.pc
    }
    #[doc = "0x08 - Low part of Cycle count register"]
    #[inline(always)]
    pub const fn cycle_count_lo(&self) -> &CycleCountLo {
        &self.cycle_count_lo
    }
    #[doc = "0x0c - High part of Cycle count register"]
    #[inline(always)]
    pub const fn cycle_count_hi(&self) -> &CycleCountHi {
        &self.cycle_count_hi
    }
    #[doc = "0x10 - Low part of Stall count"]
    #[inline(always)]
    pub const fn stall_count_lo(&self) -> &StallCountLo {
        &self.stall_count_lo
    }
    #[doc = "0x14 - High part of Stall count"]
    #[inline(always)]
    pub const fn stall_count_hi(&self) -> &StallCountHi {
        &self.stall_count_hi
    }
}
#[doc = "dsp_status (r) register accessor: Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_status`]
module"]
#[doc(alias = "dsp_status")]
pub type DspStatus = crate::Reg<dsp_status::DspStatusSpec>;
#[doc = "Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint"]
pub mod dsp_status;
#[doc = "pc (r) register accessor: Program counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`]
module"]
#[doc(alias = "pc")]
pub type Pc = crate::Reg<pc::PcSpec>;
#[doc = "Program counter"]
pub mod pc;
#[doc = "cycle_count_lo (r) register accessor: Low part of Cycle count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle_count_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle_count_lo`]
module"]
#[doc(alias = "cycle_count_lo")]
pub type CycleCountLo = crate::Reg<cycle_count_lo::CycleCountLoSpec>;
#[doc = "Low part of Cycle count register"]
pub mod cycle_count_lo;
#[doc = "cycle_count_hi (r) register accessor: High part of Cycle count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle_count_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle_count_hi`]
module"]
#[doc(alias = "cycle_count_hi")]
pub type CycleCountHi = crate::Reg<cycle_count_hi::CycleCountHiSpec>;
#[doc = "High part of Cycle count register"]
pub mod cycle_count_hi;
#[doc = "stall_count_lo (r) register accessor: Low part of Stall count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall_count_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stall_count_lo`]
module"]
#[doc(alias = "stall_count_lo")]
pub type StallCountLo = crate::Reg<stall_count_lo::StallCountLoSpec>;
#[doc = "Low part of Stall count"]
pub mod stall_count_lo;
#[doc = "stall_count_hi (r) register accessor: High part of Stall count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall_count_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stall_count_hi`]
module"]
#[doc(alias = "stall_count_hi")]
pub type StallCountHi = crate::Reg<stall_count_hi::StallCountHiSpec>;
#[doc = "High part of Stall count"]
pub mod stall_count_hi;
