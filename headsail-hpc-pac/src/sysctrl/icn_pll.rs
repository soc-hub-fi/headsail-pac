#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "ICN_PLL"]
#[doc(alias = "ICN_PLL")]
pub struct IcnPll {
    icn_pll_loop_ctrl: IcnPllLoopCtrl,
    icn_pll_div: IcnPllDiv,
    icn_pll_debug_ctrl: IcnPllDebugCtrl,
    icn_pll_enable: IcnPllEnable,
    icn_pll_spare_ctrl: IcnPllSpareCtrl,
    icn_pll_tmux_sel: IcnPllTmuxSel,
    icn_pll_status1: IcnPllStatus1,
    icn_pll_status2: IcnPllStatus2,
}
impl IcnPll {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn icn_pll_loop_ctrl(&self) -> &IcnPllLoopCtrl {
        &self.icn_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn icn_pll_div(&self) -> &IcnPllDiv {
        &self.icn_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn icn_pll_debug_ctrl(&self) -> &IcnPllDebugCtrl {
        &self.icn_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn icn_pll_enable(&self) -> &IcnPllEnable {
        &self.icn_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn icn_pll_spare_ctrl(&self) -> &IcnPllSpareCtrl {
        &self.icn_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn icn_pll_tmux_sel(&self) -> &IcnPllTmuxSel {
        &self.icn_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn icn_pll_status1(&self) -> &IcnPllStatus1 {
        &self.icn_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn icn_pll_status2(&self) -> &IcnPllStatus2 {
        &self.icn_pll_status2
    }
}
#[doc = "ICN_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_loop_ctrl`]
module"]
#[doc(alias = "ICN_PLL_LOOP_CTRL")]
pub type IcnPllLoopCtrl = crate::Reg<icn_pll_loop_ctrl::IcnPllLoopCtrlSpec>;
#[doc = ""]
pub mod icn_pll_loop_ctrl;
#[doc = "ICN_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_div`]
module"]
#[doc(alias = "ICN_PLL_DIV")]
pub type IcnPllDiv = crate::Reg<icn_pll_div::IcnPllDivSpec>;
#[doc = ""]
pub mod icn_pll_div;
#[doc = "ICN_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_debug_ctrl`]
module"]
#[doc(alias = "ICN_PLL_DEBUG_CTRL")]
pub type IcnPllDebugCtrl = crate::Reg<icn_pll_debug_ctrl::IcnPllDebugCtrlSpec>;
#[doc = ""]
pub mod icn_pll_debug_ctrl;
#[doc = "ICN_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_enable`]
module"]
#[doc(alias = "ICN_PLL_ENABLE")]
pub type IcnPllEnable = crate::Reg<icn_pll_enable::IcnPllEnableSpec>;
#[doc = ""]
pub mod icn_pll_enable;
#[doc = "ICN_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_spare_ctrl`]
module"]
#[doc(alias = "ICN_PLL_SPARE_CTRL")]
pub type IcnPllSpareCtrl = crate::Reg<icn_pll_spare_ctrl::IcnPllSpareCtrlSpec>;
#[doc = ""]
pub mod icn_pll_spare_ctrl;
#[doc = "ICN_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_tmux_sel`]
module"]
#[doc(alias = "ICN_PLL_TMUX_SEL")]
pub type IcnPllTmuxSel = crate::Reg<icn_pll_tmux_sel::IcnPllTmuxSelSpec>;
#[doc = ""]
pub mod icn_pll_tmux_sel;
#[doc = "ICN_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_status1`]
module"]
#[doc(alias = "ICN_PLL_STATUS1")]
pub type IcnPllStatus1 = crate::Reg<icn_pll_status1::IcnPllStatus1Spec>;
#[doc = ""]
pub mod icn_pll_status1;
#[doc = "ICN_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_status2`]
module"]
#[doc(alias = "ICN_PLL_STATUS2")]
pub type IcnPllStatus2 = crate::Reg<icn_pll_status2::IcnPllStatus2Spec>;
#[doc = ""]
pub mod icn_pll_status2;
