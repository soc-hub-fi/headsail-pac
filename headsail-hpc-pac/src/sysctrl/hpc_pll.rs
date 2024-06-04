#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "HPC_PLL"]
pub struct HPC_PLL {
    hpc_pll_loop_ctrl: HPC_PLL_LOOP_CTRL,
    hpc_pll_div: HPC_PLL_DIV,
    hpc_pll_debug_ctrl: HPC_PLL_DEBUG_CTRL,
    hpc_pll_enable: HPC_PLL_ENABLE,
    hpc_pll_spare_ctrl: HPC_PLL_SPARE_CTRL,
    hpc_pll_tmux_sel: HPC_PLL_TMUX_SEL,
    hpc_pll_status1: HPC_PLL_STATUS1,
    hpc_pll_status2: HPC_PLL_STATUS2,
}
impl HPC_PLL {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn hpc_pll_loop_ctrl(&self) -> &HPC_PLL_LOOP_CTRL {
        &self.hpc_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn hpc_pll_div(&self) -> &HPC_PLL_DIV {
        &self.hpc_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn hpc_pll_debug_ctrl(&self) -> &HPC_PLL_DEBUG_CTRL {
        &self.hpc_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn hpc_pll_enable(&self) -> &HPC_PLL_ENABLE {
        &self.hpc_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn hpc_pll_spare_ctrl(&self) -> &HPC_PLL_SPARE_CTRL {
        &self.hpc_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn hpc_pll_tmux_sel(&self) -> &HPC_PLL_TMUX_SEL {
        &self.hpc_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn hpc_pll_status1(&self) -> &HPC_PLL_STATUS1 {
        &self.hpc_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn hpc_pll_status2(&self) -> &HPC_PLL_STATUS2 {
        &self.hpc_pll_status2
    }
}
#[doc = "HPC_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_loop_ctrl`]
module"]
pub type HPC_PLL_LOOP_CTRL = crate::Reg<hpc_pll_loop_ctrl::HPC_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod hpc_pll_loop_ctrl;
#[doc = "HPC_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_div`]
module"]
pub type HPC_PLL_DIV = crate::Reg<hpc_pll_div::HPC_PLL_DIV_SPEC>;
#[doc = ""]
pub mod hpc_pll_div;
#[doc = "HPC_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_debug_ctrl`]
module"]
pub type HPC_PLL_DEBUG_CTRL = crate::Reg<hpc_pll_debug_ctrl::HPC_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod hpc_pll_debug_ctrl;
#[doc = "HPC_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_enable`]
module"]
pub type HPC_PLL_ENABLE = crate::Reg<hpc_pll_enable::HPC_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod hpc_pll_enable;
#[doc = "HPC_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_spare_ctrl`]
module"]
pub type HPC_PLL_SPARE_CTRL = crate::Reg<hpc_pll_spare_ctrl::HPC_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod hpc_pll_spare_ctrl;
#[doc = "HPC_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_tmux_sel`]
module"]
pub type HPC_PLL_TMUX_SEL = crate::Reg<hpc_pll_tmux_sel::HPC_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod hpc_pll_tmux_sel;
#[doc = "HPC_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_status1`]
module"]
pub type HPC_PLL_STATUS1 = crate::Reg<hpc_pll_status1::HPC_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod hpc_pll_status1;
#[doc = "HPC_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpc_pll_status2`]
module"]
pub type HPC_PLL_STATUS2 = crate::Reg<hpc_pll_status2::HPC_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod hpc_pll_status2;
