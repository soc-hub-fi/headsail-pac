#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "DSP_PLL"]
#[doc(alias = "DSP_PLL")]
pub struct DspPll {
    dsp_loop_ctrl: DspLoopCtrl,
    dsp_div: DspDiv,
    dsp_debug_ctrl: DspDebugCtrl,
    dsp_pll_enable: DspPllEnable,
    dsp_pll_spare_ctrl: DspPllSpareCtrl,
    dsp_pll_tmux_sel: DspPllTmuxSel,
    dsp_pll_status1: DspPllStatus1,
    dsp_pll_status2: DspPllStatus2,
}
impl DspPll {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn dsp_loop_ctrl(&self) -> &DspLoopCtrl {
        &self.dsp_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn dsp_div(&self) -> &DspDiv {
        &self.dsp_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn dsp_debug_ctrl(&self) -> &DspDebugCtrl {
        &self.dsp_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn dsp_pll_enable(&self) -> &DspPllEnable {
        &self.dsp_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn dsp_pll_spare_ctrl(&self) -> &DspPllSpareCtrl {
        &self.dsp_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dsp_pll_tmux_sel(&self) -> &DspPllTmuxSel {
        &self.dsp_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn dsp_pll_status1(&self) -> &DspPllStatus1 {
        &self.dsp_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn dsp_pll_status2(&self) -> &DspPllStatus2 {
        &self.dsp_pll_status2
    }
}
#[doc = "DSP_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_loop_ctrl`]
module"]
#[doc(alias = "DSP_LOOP_CTRL")]
pub type DspLoopCtrl = crate::Reg<dsp_loop_ctrl::DspLoopCtrlSpec>;
#[doc = ""]
pub mod dsp_loop_ctrl;
#[doc = "DSP_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_div`]
module"]
#[doc(alias = "DSP_DIV")]
pub type DspDiv = crate::Reg<dsp_div::DspDivSpec>;
#[doc = ""]
pub mod dsp_div;
#[doc = "DSP_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_debug_ctrl`]
module"]
#[doc(alias = "DSP_DEBUG_CTRL")]
pub type DspDebugCtrl = crate::Reg<dsp_debug_ctrl::DspDebugCtrlSpec>;
#[doc = ""]
pub mod dsp_debug_ctrl;
#[doc = "DSP_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_enable`]
module"]
#[doc(alias = "DSP_PLL_ENABLE")]
pub type DspPllEnable = crate::Reg<dsp_pll_enable::DspPllEnableSpec>;
#[doc = ""]
pub mod dsp_pll_enable;
#[doc = "DSP_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_spare_ctrl`]
module"]
#[doc(alias = "DSP_PLL_SPARE_CTRL")]
pub type DspPllSpareCtrl = crate::Reg<dsp_pll_spare_ctrl::DspPllSpareCtrlSpec>;
#[doc = ""]
pub mod dsp_pll_spare_ctrl;
#[doc = "DSP_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_tmux_sel`]
module"]
#[doc(alias = "DSP_PLL_TMUX_SEL")]
pub type DspPllTmuxSel = crate::Reg<dsp_pll_tmux_sel::DspPllTmuxSelSpec>;
#[doc = ""]
pub mod dsp_pll_tmux_sel;
#[doc = "DSP_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_status1`]
module"]
#[doc(alias = "DSP_PLL_STATUS1")]
pub type DspPllStatus1 = crate::Reg<dsp_pll_status1::DspPllStatus1Spec>;
#[doc = ""]
pub mod dsp_pll_status1;
#[doc = "DSP_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_status2`]
module"]
#[doc(alias = "DSP_PLL_STATUS2")]
pub type DspPllStatus2 = crate::Reg<dsp_pll_status2::DspPllStatus2Spec>;
#[doc = ""]
pub mod dsp_pll_status2;
