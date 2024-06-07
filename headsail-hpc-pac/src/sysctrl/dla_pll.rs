#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "DLA_PLL"]
#[doc(alias = "DLA_PLL")]
pub struct DlaPll {
    dla_pll_loop_ctrl: DlaPllLoopCtrl,
    dla_pll_div: DlaPllDiv,
    dla_pll_debug_ctrl: DlaPllDebugCtrl,
    dla_pll_enable: DlaPllEnable,
    dla_pll_spare_ctrl: DlaPllSpareCtrl,
    dla_pll_tmux_sel: DlaPllTmuxSel,
    dla_pll_status1: DlaPllStatus1,
    dla_pll_status2: DlaPllStatus2,
}
impl DlaPll {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn dla_pll_loop_ctrl(&self) -> &DlaPllLoopCtrl {
        &self.dla_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn dla_pll_div(&self) -> &DlaPllDiv {
        &self.dla_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn dla_pll_debug_ctrl(&self) -> &DlaPllDebugCtrl {
        &self.dla_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn dla_pll_enable(&self) -> &DlaPllEnable {
        &self.dla_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn dla_pll_spare_ctrl(&self) -> &DlaPllSpareCtrl {
        &self.dla_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dla_pll_tmux_sel(&self) -> &DlaPllTmuxSel {
        &self.dla_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn dla_pll_status1(&self) -> &DlaPllStatus1 {
        &self.dla_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn dla_pll_status2(&self) -> &DlaPllStatus2 {
        &self.dla_pll_status2
    }
}
#[doc = "DLA_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_loop_ctrl`]
module"]
#[doc(alias = "DLA_PLL_LOOP_CTRL")]
pub type DlaPllLoopCtrl = crate::Reg<dla_pll_loop_ctrl::DlaPllLoopCtrlSpec>;
#[doc = ""]
pub mod dla_pll_loop_ctrl;
#[doc = "DLA_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_div`]
module"]
#[doc(alias = "DLA_PLL_DIV")]
pub type DlaPllDiv = crate::Reg<dla_pll_div::DlaPllDivSpec>;
#[doc = ""]
pub mod dla_pll_div;
#[doc = "DLA_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_debug_ctrl`]
module"]
#[doc(alias = "DLA_PLL_DEBUG_CTRL")]
pub type DlaPllDebugCtrl = crate::Reg<dla_pll_debug_ctrl::DlaPllDebugCtrlSpec>;
#[doc = ""]
pub mod dla_pll_debug_ctrl;
#[doc = "DLA_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_enable`]
module"]
#[doc(alias = "DLA_PLL_ENABLE")]
pub type DlaPllEnable = crate::Reg<dla_pll_enable::DlaPllEnableSpec>;
#[doc = ""]
pub mod dla_pll_enable;
#[doc = "DLA_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_spare_ctrl`]
module"]
#[doc(alias = "DLA_PLL_SPARE_CTRL")]
pub type DlaPllSpareCtrl = crate::Reg<dla_pll_spare_ctrl::DlaPllSpareCtrlSpec>;
#[doc = ""]
pub mod dla_pll_spare_ctrl;
#[doc = "DLA_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_tmux_sel`]
module"]
#[doc(alias = "DLA_PLL_TMUX_SEL")]
pub type DlaPllTmuxSel = crate::Reg<dla_pll_tmux_sel::DlaPllTmuxSelSpec>;
#[doc = ""]
pub mod dla_pll_tmux_sel;
#[doc = "DLA_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_status1`]
module"]
#[doc(alias = "DLA_PLL_STATUS1")]
pub type DlaPllStatus1 = crate::Reg<dla_pll_status1::DlaPllStatus1Spec>;
#[doc = ""]
pub mod dla_pll_status1;
#[doc = "DLA_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_status2`]
module"]
#[doc(alias = "DLA_PLL_STATUS2")]
pub type DlaPllStatus2 = crate::Reg<dla_pll_status2::DlaPllStatus2Spec>;
#[doc = ""]
pub mod dla_pll_status2;
