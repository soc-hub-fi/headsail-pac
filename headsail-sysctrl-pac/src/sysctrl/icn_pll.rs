#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "ICN_PLL"]
pub struct ICN_PLL {
    icn_pll_loop_ctrl: ICN_PLL_LOOP_CTRL,
    icn_pll_div: ICN_PLL_DIV,
    icn_pll_debug_ctrl: ICN_PLL_DEBUG_CTRL,
    icn_pll_enable: ICN_PLL_ENABLE,
    icn_pll_spare_ctrl: ICN_PLL_SPARE_CTRL,
    icn_pll_tmux_sel: ICN_PLL_TMUX_SEL,
    icn_pll_status1: ICN_PLL_STATUS1,
    icn_pll_status2: ICN_PLL_STATUS2,
}
impl ICN_PLL {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn icn_pll_loop_ctrl(&self) -> &ICN_PLL_LOOP_CTRL {
        &self.icn_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn icn_pll_div(&self) -> &ICN_PLL_DIV {
        &self.icn_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn icn_pll_debug_ctrl(&self) -> &ICN_PLL_DEBUG_CTRL {
        &self.icn_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn icn_pll_enable(&self) -> &ICN_PLL_ENABLE {
        &self.icn_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn icn_pll_spare_ctrl(&self) -> &ICN_PLL_SPARE_CTRL {
        &self.icn_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn icn_pll_tmux_sel(&self) -> &ICN_PLL_TMUX_SEL {
        &self.icn_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn icn_pll_status1(&self) -> &ICN_PLL_STATUS1 {
        &self.icn_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn icn_pll_status2(&self) -> &ICN_PLL_STATUS2 {
        &self.icn_pll_status2
    }
}
#[doc = "ICN_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_loop_ctrl`]
module"]
pub type ICN_PLL_LOOP_CTRL = crate::Reg<icn_pll_loop_ctrl::ICN_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod icn_pll_loop_ctrl;
#[doc = "ICN_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_div`]
module"]
pub type ICN_PLL_DIV = crate::Reg<icn_pll_div::ICN_PLL_DIV_SPEC>;
#[doc = ""]
pub mod icn_pll_div;
#[doc = "ICN_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_debug_ctrl`]
module"]
pub type ICN_PLL_DEBUG_CTRL = crate::Reg<icn_pll_debug_ctrl::ICN_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod icn_pll_debug_ctrl;
#[doc = "ICN_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_enable`]
module"]
pub type ICN_PLL_ENABLE = crate::Reg<icn_pll_enable::ICN_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod icn_pll_enable;
#[doc = "ICN_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_spare_ctrl`]
module"]
pub type ICN_PLL_SPARE_CTRL = crate::Reg<icn_pll_spare_ctrl::ICN_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod icn_pll_spare_ctrl;
#[doc = "ICN_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icn_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_tmux_sel`]
module"]
pub type ICN_PLL_TMUX_SEL = crate::Reg<icn_pll_tmux_sel::ICN_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod icn_pll_tmux_sel;
#[doc = "ICN_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_status1`]
module"]
pub type ICN_PLL_STATUS1 = crate::Reg<icn_pll_status1::ICN_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod icn_pll_status1;
#[doc = "ICN_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icn_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icn_pll_status2`]
module"]
pub type ICN_PLL_STATUS2 = crate::Reg<icn_pll_status2::ICN_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod icn_pll_status2;
