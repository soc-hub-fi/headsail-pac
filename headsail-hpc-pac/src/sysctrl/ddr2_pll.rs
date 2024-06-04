#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "DDR2_PLL"]
pub struct DDR2_PLL {
    ddr2_pll_loop_ctrl: DDR2_PLL_LOOP_CTRL,
    ddr2_pll_div: DDR2_PLL_DIV,
    ddr2_pll_debug_ctrl: DDR2_PLL_DEBUG_CTRL,
    ddr2_pll_enable: DDR2_PLL_ENABLE,
    ddr2_pll_spare_ctrl: DDR2_PLL_SPARE_CTRL,
    ddr2_pll_tmux_sel: DDR2_PLL_TMUX_SEL,
    ddr2_pll_status1: DDR2_PLL_STATUS1,
    ddr2_pll_status2: DDR2_PLL_STATUS2,
}
impl DDR2_PLL {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn ddr2_pll_loop_ctrl(&self) -> &DDR2_PLL_LOOP_CTRL {
        &self.ddr2_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn ddr2_pll_div(&self) -> &DDR2_PLL_DIV {
        &self.ddr2_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ddr2_pll_debug_ctrl(&self) -> &DDR2_PLL_DEBUG_CTRL {
        &self.ddr2_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ddr2_pll_enable(&self) -> &DDR2_PLL_ENABLE {
        &self.ddr2_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn ddr2_pll_spare_ctrl(&self) -> &DDR2_PLL_SPARE_CTRL {
        &self.ddr2_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn ddr2_pll_tmux_sel(&self) -> &DDR2_PLL_TMUX_SEL {
        &self.ddr2_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn ddr2_pll_status1(&self) -> &DDR2_PLL_STATUS1 {
        &self.ddr2_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn ddr2_pll_status2(&self) -> &DDR2_PLL_STATUS2 {
        &self.ddr2_pll_status2
    }
}
#[doc = "DDR2_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_loop_ctrl`]
module"]
pub type DDR2_PLL_LOOP_CTRL = crate::Reg<ddr2_pll_loop_ctrl::DDR2_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod ddr2_pll_loop_ctrl;
#[doc = "DDR2_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_div`]
module"]
pub type DDR2_PLL_DIV = crate::Reg<ddr2_pll_div::DDR2_PLL_DIV_SPEC>;
#[doc = ""]
pub mod ddr2_pll_div;
#[doc = "DDR2_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_debug_ctrl`]
module"]
pub type DDR2_PLL_DEBUG_CTRL = crate::Reg<ddr2_pll_debug_ctrl::DDR2_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod ddr2_pll_debug_ctrl;
#[doc = "DDR2_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_enable`]
module"]
pub type DDR2_PLL_ENABLE = crate::Reg<ddr2_pll_enable::DDR2_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod ddr2_pll_enable;
#[doc = "DDR2_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_spare_ctrl`]
module"]
pub type DDR2_PLL_SPARE_CTRL = crate::Reg<ddr2_pll_spare_ctrl::DDR2_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod ddr2_pll_spare_ctrl;
#[doc = "DDR2_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_tmux_sel`]
module"]
pub type DDR2_PLL_TMUX_SEL = crate::Reg<ddr2_pll_tmux_sel::DDR2_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod ddr2_pll_tmux_sel;
#[doc = "DDR2_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_status1`]
module"]
pub type DDR2_PLL_STATUS1 = crate::Reg<ddr2_pll_status1::DDR2_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod ddr2_pll_status1;
#[doc = "DDR2_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr2_pll_status2`]
module"]
pub type DDR2_PLL_STATUS2 = crate::Reg<ddr2_pll_status2::DDR2_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod ddr2_pll_status2;
