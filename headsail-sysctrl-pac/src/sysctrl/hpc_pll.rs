#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "HPC_PLL"]
#[doc(alias = "HPC_PLL")]
pub struct HpcPll {
    hpc_pll_loop_ctrl: HpcPllLoopCtrl,
    hpc_pll_div: HpcPllDiv,
    hpc_pll_debug_ctrl: HpcPllDebugCtrl,
    hpc_pll_enable: HpcPllEnable,
    hpc_pll_spare_ctrl: HpcPllSpareCtrl,
    hpc_pll_tmux_sel: HpcPllTmuxSel,
    hpc_pll_status1: HpcPllStatus1,
    hpc_pll_status2: HpcPllStatus2,
}
impl HpcPll {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn hpc_pll_loop_ctrl(&self) -> &HpcPllLoopCtrl {
        &self.hpc_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn hpc_pll_div(&self) -> &HpcPllDiv {
        &self.hpc_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn hpc_pll_debug_ctrl(&self) -> &HpcPllDebugCtrl {
        &self.hpc_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn hpc_pll_enable(&self) -> &HpcPllEnable {
        &self.hpc_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn hpc_pll_spare_ctrl(&self) -> &HpcPllSpareCtrl {
        &self.hpc_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn hpc_pll_tmux_sel(&self) -> &HpcPllTmuxSel {
        &self.hpc_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn hpc_pll_status1(&self) -> &HpcPllStatus1 {
        &self.hpc_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn hpc_pll_status2(&self) -> &HpcPllStatus2 {
        &self.hpc_pll_status2
    }
}
#[doc = "HPC_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_loop_ctrl`]
module"]
#[doc(alias = "HPC_PLL_LOOP_CTRL")]
pub type HpcPllLoopCtrl = crate::Reg<hpc_pll_loop_ctrl::HpcPllLoopCtrlSpec>;
#[doc = ""]
pub mod hpc_pll_loop_ctrl;
#[doc = "HPC_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_div`]
module"]
#[doc(alias = "HPC_PLL_DIV")]
pub type HpcPllDiv = crate::Reg<hpc_pll_div::HpcPllDivSpec>;
#[doc = ""]
pub mod hpc_pll_div;
#[doc = "HPC_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_debug_ctrl`]
module"]
#[doc(alias = "HPC_PLL_DEBUG_CTRL")]
pub type HpcPllDebugCtrl = crate::Reg<hpc_pll_debug_ctrl::HpcPllDebugCtrlSpec>;
#[doc = ""]
pub mod hpc_pll_debug_ctrl;
#[doc = "HPC_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_enable`]
module"]
#[doc(alias = "HPC_PLL_ENABLE")]
pub type HpcPllEnable = crate::Reg<hpc_pll_enable::HpcPllEnableSpec>;
#[doc = ""]
pub mod hpc_pll_enable;
#[doc = "HPC_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_spare_ctrl`]
module"]
#[doc(alias = "HPC_PLL_SPARE_CTRL")]
pub type HpcPllSpareCtrl = crate::Reg<hpc_pll_spare_ctrl::HpcPllSpareCtrlSpec>;
#[doc = ""]
pub mod hpc_pll_spare_ctrl;
#[doc = "HPC_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_tmux_sel`]
module"]
#[doc(alias = "HPC_PLL_TMUX_SEL")]
pub type HpcPllTmuxSel = crate::Reg<hpc_pll_tmux_sel::HpcPllTmuxSelSpec>;
#[doc = ""]
pub mod hpc_pll_tmux_sel;
#[doc = "HPC_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_status1`]
module"]
#[doc(alias = "HPC_PLL_STATUS1")]
pub type HpcPllStatus1 = crate::Reg<hpc_pll_status1::HpcPllStatus1Spec>;
#[doc = ""]
pub mod hpc_pll_status1;
#[doc = "HPC_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_status2`]
module"]
#[doc(alias = "HPC_PLL_STATUS2")]
pub type HpcPllStatus2 = crate::Reg<hpc_pll_status2::HpcPllStatus2Spec>;
#[doc = ""]
pub mod hpc_pll_status2;
