#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "DSP_PLL"]
pub struct DSP_PLL {
    dsp_loop_ctrl: DSP_LOOP_CTRL,
    dsp_div: DSP_DIV,
    dsp_debug_ctrl: DSP_DEBUG_CTRL,
    dsp_pll_enable: DSP_PLL_ENABLE,
    dsp_pll_spare_ctrl: DSP_PLL_SPARE_CTRL,
    dsp_pll_tmux_sel: DSP_PLL_TMUX_SEL,
    dsp_pll_status1: DSP_PLL_STATUS1,
    dsp_pll_status2: DSP_PLL_STATUS2,
}
impl DSP_PLL {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn dsp_loop_ctrl(&self) -> &DSP_LOOP_CTRL {
        &self.dsp_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn dsp_div(&self) -> &DSP_DIV {
        &self.dsp_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn dsp_debug_ctrl(&self) -> &DSP_DEBUG_CTRL {
        &self.dsp_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn dsp_pll_enable(&self) -> &DSP_PLL_ENABLE {
        &self.dsp_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn dsp_pll_spare_ctrl(&self) -> &DSP_PLL_SPARE_CTRL {
        &self.dsp_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dsp_pll_tmux_sel(&self) -> &DSP_PLL_TMUX_SEL {
        &self.dsp_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn dsp_pll_status1(&self) -> &DSP_PLL_STATUS1 {
        &self.dsp_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn dsp_pll_status2(&self) -> &DSP_PLL_STATUS2 {
        &self.dsp_pll_status2
    }
}
#[doc = "DSP_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_loop_ctrl`]
module"]
pub type DSP_LOOP_CTRL = crate::Reg<dsp_loop_ctrl::DSP_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod dsp_loop_ctrl;
#[doc = "DSP_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_div`]
module"]
pub type DSP_DIV = crate::Reg<dsp_div::DSP_DIV_SPEC>;
#[doc = ""]
pub mod dsp_div;
#[doc = "DSP_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_debug_ctrl`]
module"]
pub type DSP_DEBUG_CTRL = crate::Reg<dsp_debug_ctrl::DSP_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod dsp_debug_ctrl;
#[doc = "DSP_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_enable`]
module"]
pub type DSP_PLL_ENABLE = crate::Reg<dsp_pll_enable::DSP_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod dsp_pll_enable;
#[doc = "DSP_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_spare_ctrl`]
module"]
pub type DSP_PLL_SPARE_CTRL = crate::Reg<dsp_pll_spare_ctrl::DSP_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod dsp_pll_spare_ctrl;
#[doc = "DSP_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_tmux_sel`]
module"]
pub type DSP_PLL_TMUX_SEL = crate::Reg<dsp_pll_tmux_sel::DSP_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod dsp_pll_tmux_sel;
#[doc = "DSP_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_status1`]
module"]
pub type DSP_PLL_STATUS1 = crate::Reg<dsp_pll_status1::DSP_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod dsp_pll_status1;
#[doc = "DSP_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp_pll_status2`]
module"]
pub type DSP_PLL_STATUS2 = crate::Reg<dsp_pll_status2::DSP_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod dsp_pll_status2;
