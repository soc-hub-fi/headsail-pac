#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "DDR2_PLL"]
#[doc(alias = "DDR2_PLL")]
pub struct Ddr2Pll {
    ddr2_pll_loop_ctrl: Ddr2PllLoopCtrl,
    ddr2_pll_div: Ddr2PllDiv,
    ddr2_pll_debug_ctrl: Ddr2PllDebugCtrl,
    ddr2_pll_enable: Ddr2PllEnable,
    ddr2_pll_spare_ctrl: Ddr2PllSpareCtrl,
    ddr2_pll_tmux_sel: Ddr2PllTmuxSel,
    ddr2_pll_status1: Ddr2PllStatus1,
    ddr2_pll_status2: Ddr2PllStatus2,
}
impl Ddr2Pll {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn ddr2_pll_loop_ctrl(&self) -> &Ddr2PllLoopCtrl {
        &self.ddr2_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn ddr2_pll_div(&self) -> &Ddr2PllDiv {
        &self.ddr2_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ddr2_pll_debug_ctrl(&self) -> &Ddr2PllDebugCtrl {
        &self.ddr2_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ddr2_pll_enable(&self) -> &Ddr2PllEnable {
        &self.ddr2_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn ddr2_pll_spare_ctrl(&self) -> &Ddr2PllSpareCtrl {
        &self.ddr2_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn ddr2_pll_tmux_sel(&self) -> &Ddr2PllTmuxSel {
        &self.ddr2_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn ddr2_pll_status1(&self) -> &Ddr2PllStatus1 {
        &self.ddr2_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn ddr2_pll_status2(&self) -> &Ddr2PllStatus2 {
        &self.ddr2_pll_status2
    }
}
#[doc = "DDR2_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_loop_ctrl`]
module"]
#[doc(alias = "DDR2_PLL_LOOP_CTRL")]
pub type Ddr2PllLoopCtrl = crate::Reg<ddr2_pll_loop_ctrl::Ddr2PllLoopCtrlSpec>;
#[doc = ""]
pub mod ddr2_pll_loop_ctrl;
#[doc = "DDR2_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_div`]
module"]
#[doc(alias = "DDR2_PLL_DIV")]
pub type Ddr2PllDiv = crate::Reg<ddr2_pll_div::Ddr2PllDivSpec>;
#[doc = ""]
pub mod ddr2_pll_div;
#[doc = "DDR2_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_debug_ctrl`]
module"]
#[doc(alias = "DDR2_PLL_DEBUG_CTRL")]
pub type Ddr2PllDebugCtrl = crate::Reg<ddr2_pll_debug_ctrl::Ddr2PllDebugCtrlSpec>;
#[doc = ""]
pub mod ddr2_pll_debug_ctrl;
#[doc = "DDR2_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_enable`]
module"]
#[doc(alias = "DDR2_PLL_ENABLE")]
pub type Ddr2PllEnable = crate::Reg<ddr2_pll_enable::Ddr2PllEnableSpec>;
#[doc = ""]
pub mod ddr2_pll_enable;
#[doc = "DDR2_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_spare_ctrl`]
module"]
#[doc(alias = "DDR2_PLL_SPARE_CTRL")]
pub type Ddr2PllSpareCtrl = crate::Reg<ddr2_pll_spare_ctrl::Ddr2PllSpareCtrlSpec>;
#[doc = ""]
pub mod ddr2_pll_spare_ctrl;
#[doc = "DDR2_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_tmux_sel`]
module"]
#[doc(alias = "DDR2_PLL_TMUX_SEL")]
pub type Ddr2PllTmuxSel = crate::Reg<ddr2_pll_tmux_sel::Ddr2PllTmuxSelSpec>;
#[doc = ""]
pub mod ddr2_pll_tmux_sel;
#[doc = "DDR2_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_status1`]
module"]
#[doc(alias = "DDR2_PLL_STATUS1")]
pub type Ddr2PllStatus1 = crate::Reg<ddr2_pll_status1::Ddr2PllStatus1Spec>;
#[doc = ""]
pub mod ddr2_pll_status1;
#[doc = "DDR2_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_status2`]
module"]
#[doc(alias = "DDR2_PLL_STATUS2")]
pub type Ddr2PllStatus2 = crate::Reg<ddr2_pll_status2::Ddr2PllStatus2Spec>;
#[doc = ""]
pub mod ddr2_pll_status2;
