#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "ctrl"]
#[doc(alias = "ctrl")]
pub struct CTRL {
    cmd: CMD,
    start_addr: START_ADDR,
    bp_en: BP_EN,
    cycle_count_bp: CYCLE_COUNT_BP,
    bp2_addr: BP2_ADDR,
    bp1_addr: BP1_ADDR,
}
impl CTRL {
    #[doc = "0x00 - Command (1 = reset, 2 = run, 4 = break)"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn start_addr(&self) -> &START_ADDR {
        &self.start_addr
    }
    #[doc = "0x08 - Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2"]
    #[inline(always)]
    pub const fn bp_en(&self) -> &BP_EN {
        &self.bp_en
    }
    #[doc = "0x0c - Cycle count breakpoint"]
    #[inline(always)]
    pub const fn cycle_count_bp(&self) -> &CYCLE_COUNT_BP {
        &self.cycle_count_bp
    }
    #[doc = "0x10 - Breakpoint 2 address"]
    #[inline(always)]
    pub const fn bp2_addr(&self) -> &BP2_ADDR {
        &self.bp2_addr
    }
    #[doc = "0x14 - Breakpoint 1 address"]
    #[inline(always)]
    pub const fn bp1_addr(&self) -> &BP1_ADDR {
        &self.bp1_addr
    }
}
#[doc = "cmd (w) register accessor: Command (1 = reset, 2 = run, 4 = break)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "cmd")]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command (1 = reset, 2 = run, 4 = break)"]
pub mod cmd;
#[doc = "start_addr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_addr`]
module"]
#[doc(alias = "start_addr")]
pub type START_ADDR = crate::Reg<start_addr::START_ADDR_SPEC>;
#[doc = ""]
pub mod start_addr;
#[doc = "bp_en (rw) register accessor: Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp_en`]
module"]
#[doc(alias = "bp_en")]
pub type BP_EN = crate::Reg<bp_en::BP_EN_SPEC>;
#[doc = "Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2"]
pub mod bp_en;
#[doc = "bp1_addr (rw) register accessor: Breakpoint 1 address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp1_addr`]
module"]
#[doc(alias = "bp1_addr")]
pub type BP1_ADDR = crate::Reg<bp1_addr::BP1_ADDR_SPEC>;
#[doc = "Breakpoint 1 address"]
pub mod bp1_addr;
#[doc = "bp2_addr (rw) register accessor: Breakpoint 2 address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp2_addr`]
module"]
#[doc(alias = "bp2_addr")]
pub type BP2_ADDR = crate::Reg<bp2_addr::BP2_ADDR_SPEC>;
#[doc = "Breakpoint 2 address"]
pub mod bp2_addr;
#[doc = "cycle_count_bp (rw) register accessor: Cycle count breakpoint\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle_count_bp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle_count_bp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle_count_bp`]
module"]
#[doc(alias = "cycle_count_bp")]
pub type CYCLE_COUNT_BP = crate::Reg<cycle_count_bp::CYCLE_COUNT_BP_SPEC>;
#[doc = "Cycle count breakpoint"]
pub mod cycle_count_bp;
