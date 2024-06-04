#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "ETH_PLL"]
pub struct ETH_PLL {
    eth_pll_loop_ctrl: ETH_PLL_LOOP_CTRL,
    eth_pll_div: ETH_PLL_DIV,
    eth_pll_debug_ctrl: ETH_PLL_DEBUG_CTRL,
    eth_pll_enable: ETH_PLL_ENABLE,
    eth_pll_spare_ctrl: ETH_PLL_SPARE_CTRL,
    eth_pll_tmux_sel: ETH_PLL_TMUX_SEL,
    eth_pll_status1: ETH_PLL_STATUS1,
    eth_pll_status2: ETH_PLL_STATUS2,
}
impl ETH_PLL {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn eth_pll_loop_ctrl(&self) -> &ETH_PLL_LOOP_CTRL {
        &self.eth_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn eth_pll_div(&self) -> &ETH_PLL_DIV {
        &self.eth_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn eth_pll_debug_ctrl(&self) -> &ETH_PLL_DEBUG_CTRL {
        &self.eth_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn eth_pll_enable(&self) -> &ETH_PLL_ENABLE {
        &self.eth_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn eth_pll_spare_ctrl(&self) -> &ETH_PLL_SPARE_CTRL {
        &self.eth_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn eth_pll_tmux_sel(&self) -> &ETH_PLL_TMUX_SEL {
        &self.eth_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn eth_pll_status1(&self) -> &ETH_PLL_STATUS1 {
        &self.eth_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn eth_pll_status2(&self) -> &ETH_PLL_STATUS2 {
        &self.eth_pll_status2
    }
}
#[doc = "ETH_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_loop_ctrl`]
module"]
pub type ETH_PLL_LOOP_CTRL = crate::Reg<eth_pll_loop_ctrl::ETH_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod eth_pll_loop_ctrl;
#[doc = "ETH_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_div`]
module"]
pub type ETH_PLL_DIV = crate::Reg<eth_pll_div::ETH_PLL_DIV_SPEC>;
#[doc = ""]
pub mod eth_pll_div;
#[doc = "ETH_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_debug_ctrl`]
module"]
pub type ETH_PLL_DEBUG_CTRL = crate::Reg<eth_pll_debug_ctrl::ETH_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod eth_pll_debug_ctrl;
#[doc = "ETH_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_enable`]
module"]
pub type ETH_PLL_ENABLE = crate::Reg<eth_pll_enable::ETH_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod eth_pll_enable;
#[doc = "ETH_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_spare_ctrl`]
module"]
pub type ETH_PLL_SPARE_CTRL = crate::Reg<eth_pll_spare_ctrl::ETH_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod eth_pll_spare_ctrl;
#[doc = "ETH_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_tmux_sel`]
module"]
pub type ETH_PLL_TMUX_SEL = crate::Reg<eth_pll_tmux_sel::ETH_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod eth_pll_tmux_sel;
#[doc = "ETH_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_status1`]
module"]
pub type ETH_PLL_STATUS1 = crate::Reg<eth_pll_status1::ETH_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod eth_pll_status1;
#[doc = "ETH_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_status2`]
module"]
pub type ETH_PLL_STATUS2 = crate::Reg<eth_pll_status2::ETH_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod eth_pll_status2;
