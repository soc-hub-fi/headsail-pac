#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "ETH_PLL"]
#[doc(alias = "ETH_PLL")]
pub struct EthPll {
    eth_pll_loop_ctrl: EthPllLoopCtrl,
    eth_pll_div: EthPllDiv,
    eth_pll_debug_ctrl: EthPllDebugCtrl,
    eth_pll_enable: EthPllEnable,
    eth_pll_spare_ctrl: EthPllSpareCtrl,
    eth_pll_tmux_sel: EthPllTmuxSel,
    eth_pll_status1: EthPllStatus1,
    eth_pll_status2: EthPllStatus2,
}
impl EthPll {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn eth_pll_loop_ctrl(&self) -> &EthPllLoopCtrl {
        &self.eth_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn eth_pll_div(&self) -> &EthPllDiv {
        &self.eth_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn eth_pll_debug_ctrl(&self) -> &EthPllDebugCtrl {
        &self.eth_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn eth_pll_enable(&self) -> &EthPllEnable {
        &self.eth_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn eth_pll_spare_ctrl(&self) -> &EthPllSpareCtrl {
        &self.eth_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn eth_pll_tmux_sel(&self) -> &EthPllTmuxSel {
        &self.eth_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn eth_pll_status1(&self) -> &EthPllStatus1 {
        &self.eth_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn eth_pll_status2(&self) -> &EthPllStatus2 {
        &self.eth_pll_status2
    }
}
#[doc = "ETH_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_loop_ctrl`]
module"]
#[doc(alias = "ETH_PLL_LOOP_CTRL")]
pub type EthPllLoopCtrl = crate::Reg<eth_pll_loop_ctrl::EthPllLoopCtrlSpec>;
#[doc = ""]
pub mod eth_pll_loop_ctrl;
#[doc = "ETH_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_div`]
module"]
#[doc(alias = "ETH_PLL_DIV")]
pub type EthPllDiv = crate::Reg<eth_pll_div::EthPllDivSpec>;
#[doc = ""]
pub mod eth_pll_div;
#[doc = "ETH_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_debug_ctrl`]
module"]
#[doc(alias = "ETH_PLL_DEBUG_CTRL")]
pub type EthPllDebugCtrl = crate::Reg<eth_pll_debug_ctrl::EthPllDebugCtrlSpec>;
#[doc = ""]
pub mod eth_pll_debug_ctrl;
#[doc = "ETH_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_enable`]
module"]
#[doc(alias = "ETH_PLL_ENABLE")]
pub type EthPllEnable = crate::Reg<eth_pll_enable::EthPllEnableSpec>;
#[doc = ""]
pub mod eth_pll_enable;
#[doc = "ETH_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_spare_ctrl`]
module"]
#[doc(alias = "ETH_PLL_SPARE_CTRL")]
pub type EthPllSpareCtrl = crate::Reg<eth_pll_spare_ctrl::EthPllSpareCtrlSpec>;
#[doc = ""]
pub mod eth_pll_spare_ctrl;
#[doc = "ETH_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_tmux_sel`]
module"]
#[doc(alias = "ETH_PLL_TMUX_SEL")]
pub type EthPllTmuxSel = crate::Reg<eth_pll_tmux_sel::EthPllTmuxSelSpec>;
#[doc = ""]
pub mod eth_pll_tmux_sel;
#[doc = "ETH_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_status1`]
module"]
#[doc(alias = "ETH_PLL_STATUS1")]
pub type EthPllStatus1 = crate::Reg<eth_pll_status1::EthPllStatus1Spec>;
#[doc = ""]
pub mod eth_pll_status1;
#[doc = "ETH_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_pll_status2`]
module"]
#[doc(alias = "ETH_PLL_STATUS2")]
pub type EthPllStatus2 = crate::Reg<eth_pll_status2::EthPllStatus2Spec>;
#[doc = ""]
pub mod eth_pll_status2;
