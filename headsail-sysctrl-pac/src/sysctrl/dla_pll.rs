#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "DLA_PLL"]
pub struct DLA_PLL {
    dla_pll_loop_ctrl: DLA_PLL_LOOP_CTRL,
    dla_pll_div: DLA_PLL_DIV,
    dla_pll_debug_ctrl: DLA_PLL_DEBUG_CTRL,
    dla_pll_enable: DLA_PLL_ENABLE,
    dla_pll_spare_ctrl: DLA_PLL_SPARE_CTRL,
    dla_pll_tmux_sel: DLA_PLL_TMUX_SEL,
    dla_pll_status1: DLA_PLL_STATUS1,
    dla_pll_status2: DLA_PLL_STATUS2,
}
impl DLA_PLL {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn dla_pll_loop_ctrl(&self) -> &DLA_PLL_LOOP_CTRL {
        &self.dla_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn dla_pll_div(&self) -> &DLA_PLL_DIV {
        &self.dla_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn dla_pll_debug_ctrl(&self) -> &DLA_PLL_DEBUG_CTRL {
        &self.dla_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn dla_pll_enable(&self) -> &DLA_PLL_ENABLE {
        &self.dla_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn dla_pll_spare_ctrl(&self) -> &DLA_PLL_SPARE_CTRL {
        &self.dla_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dla_pll_tmux_sel(&self) -> &DLA_PLL_TMUX_SEL {
        &self.dla_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn dla_pll_status1(&self) -> &DLA_PLL_STATUS1 {
        &self.dla_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn dla_pll_status2(&self) -> &DLA_PLL_STATUS2 {
        &self.dla_pll_status2
    }
}
#[doc = "DLA_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_loop_ctrl`]
module"]
pub type DLA_PLL_LOOP_CTRL = crate::Reg<dla_pll_loop_ctrl::DLA_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod dla_pll_loop_ctrl;
#[doc = "DLA_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_div`]
module"]
pub type DLA_PLL_DIV = crate::Reg<dla_pll_div::DLA_PLL_DIV_SPEC>;
#[doc = ""]
pub mod dla_pll_div;
#[doc = "DLA_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_debug_ctrl`]
module"]
pub type DLA_PLL_DEBUG_CTRL = crate::Reg<dla_pll_debug_ctrl::DLA_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod dla_pll_debug_ctrl;
#[doc = "DLA_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_enable`]
module"]
pub type DLA_PLL_ENABLE = crate::Reg<dla_pll_enable::DLA_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod dla_pll_enable;
#[doc = "DLA_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_spare_ctrl`]
module"]
pub type DLA_PLL_SPARE_CTRL = crate::Reg<dla_pll_spare_ctrl::DLA_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod dla_pll_spare_ctrl;
#[doc = "DLA_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_tmux_sel`]
module"]
pub type DLA_PLL_TMUX_SEL = crate::Reg<dla_pll_tmux_sel::DLA_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod dla_pll_tmux_sel;
#[doc = "DLA_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_status1`]
module"]
pub type DLA_PLL_STATUS1 = crate::Reg<dla_pll_status1::DLA_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod dla_pll_status1;
#[doc = "DLA_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_pll_status2`]
module"]
pub type DLA_PLL_STATUS2 = crate::Reg<dla_pll_status2::DLA_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod dla_pll_status2;
