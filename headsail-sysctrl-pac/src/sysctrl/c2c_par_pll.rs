#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "C2C_PAR_PLL"]
#[doc(alias = "C2C_PAR_PLL")]
pub struct C2cParPll {
    c2c_par_pll_loop_ctrl: C2cParPllLoopCtrl,
    c2c_par_pll_div: C2cParPllDiv,
    c2c_par_pll_debug_ctrl: C2cParPllDebugCtrl,
    c2c_par_pll_enable: C2cParPllEnable,
    c2c_par_pll_spare_ctrl: C2cParPllSpareCtrl,
    c2c_par_pll_tmux_sel: C2cParPllTmuxSel,
    c2c_par_pll_status1: C2cParPllStatus1,
    c2c_par_pll_status2: C2cParPllStatus2,
}
impl C2cParPll {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn c2c_par_pll_loop_ctrl(&self) -> &C2cParPllLoopCtrl {
        &self.c2c_par_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn c2c_par_pll_div(&self) -> &C2cParPllDiv {
        &self.c2c_par_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn c2c_par_pll_debug_ctrl(&self) -> &C2cParPllDebugCtrl {
        &self.c2c_par_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn c2c_par_pll_enable(&self) -> &C2cParPllEnable {
        &self.c2c_par_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn c2c_par_pll_spare_ctrl(&self) -> &C2cParPllSpareCtrl {
        &self.c2c_par_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn c2c_par_pll_tmux_sel(&self) -> &C2cParPllTmuxSel {
        &self.c2c_par_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn c2c_par_pll_status1(&self) -> &C2cParPllStatus1 {
        &self.c2c_par_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn c2c_par_pll_status2(&self) -> &C2cParPllStatus2 {
        &self.c2c_par_pll_status2
    }
}
#[doc = "C2C_PAR_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_par_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_par_pll_loop_ctrl`]
module"]
#[doc(alias = "C2C_PAR_PLL_LOOP_CTRL")]
pub type C2cParPllLoopCtrl = crate::Reg<c2c_par_pll_loop_ctrl::C2cParPllLoopCtrlSpec>;
#[doc = ""]
pub mod c2c_par_pll_loop_ctrl;
#[doc = "C2C_PAR_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_par_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_par_pll_div`]
module"]
#[doc(alias = "C2C_PAR_PLL_DIV")]
pub type C2cParPllDiv = crate::Reg<c2c_par_pll_div::C2cParPllDivSpec>;
#[doc = ""]
pub mod c2c_par_pll_div;
#[doc = "C2C_PAR_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_par_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_par_pll_debug_ctrl`]
module"]
#[doc(alias = "C2C_PAR_PLL_DEBUG_CTRL")]
pub type C2cParPllDebugCtrl = crate::Reg<c2c_par_pll_debug_ctrl::C2cParPllDebugCtrlSpec>;
#[doc = ""]
pub mod c2c_par_pll_debug_ctrl;
#[doc = "C2C_PAR_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_par_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_par_pll_enable`]
module"]
#[doc(alias = "C2C_PAR_PLL_ENABLE")]
pub type C2cParPllEnable = crate::Reg<c2c_par_pll_enable::C2cParPllEnableSpec>;
#[doc = ""]
pub mod c2c_par_pll_enable;
#[doc = "C2C_PAR_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_par_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_par_pll_spare_ctrl`]
module"]
#[doc(alias = "C2C_PAR_PLL_SPARE_CTRL")]
pub type C2cParPllSpareCtrl = crate::Reg<c2c_par_pll_spare_ctrl::C2cParPllSpareCtrlSpec>;
#[doc = ""]
pub mod c2c_par_pll_spare_ctrl;
#[doc = "C2C_PAR_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_par_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_par_pll_tmux_sel`]
module"]
#[doc(alias = "C2C_PAR_PLL_TMUX_SEL")]
pub type C2cParPllTmuxSel = crate::Reg<c2c_par_pll_tmux_sel::C2cParPllTmuxSelSpec>;
#[doc = ""]
pub mod c2c_par_pll_tmux_sel;
#[doc = "C2C_PAR_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_par_pll_status1`]
module"]
#[doc(alias = "C2C_PAR_PLL_STATUS1")]
pub type C2cParPllStatus1 = crate::Reg<c2c_par_pll_status1::C2cParPllStatus1Spec>;
#[doc = ""]
pub mod c2c_par_pll_status1;
#[doc = "C2C_PAR_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_par_pll_status2`]
module"]
#[doc(alias = "C2C_PAR_PLL_STATUS2")]
pub type C2cParPllStatus2 = crate::Reg<c2c_par_pll_status2::C2cParPllStatus2Spec>;
#[doc = ""]
pub mod c2c_par_pll_status2;
