#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "C2C_SER_PLL"]
pub struct C2C_SER_PLL {
    c2c_ser_pll_loop_ctrl: C2C_SER_PLL_LOOP_CTRL,
    c2c_ser_pll_div: C2C_SER_PLL_DIV,
    c2c_ser_pll_debug_ctrl: C2C_SER_PLL_DEBUG_CTRL,
    c2c_ser_pll_enable: C2C_SER_PLL_ENABLE,
    c2c_ser_pll_spare_ctrl: C2C_SER_PLL_SPARE_CTRL,
    c2c_ser_pll_tmux_sel: C2C_SER_PLL_TMUX_SEL,
    c2c_ser_pll_status1: C2C_SER_PLL_STATUS1,
    c2c_ser_pll_status2: C2C_SER_PLL_STATUS2,
}
impl C2C_SER_PLL {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn c2c_ser_pll_loop_ctrl(&self) -> &C2C_SER_PLL_LOOP_CTRL {
        &self.c2c_ser_pll_loop_ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn c2c_ser_pll_div(&self) -> &C2C_SER_PLL_DIV {
        &self.c2c_ser_pll_div
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn c2c_ser_pll_debug_ctrl(&self) -> &C2C_SER_PLL_DEBUG_CTRL {
        &self.c2c_ser_pll_debug_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn c2c_ser_pll_enable(&self) -> &C2C_SER_PLL_ENABLE {
        &self.c2c_ser_pll_enable
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn c2c_ser_pll_spare_ctrl(&self) -> &C2C_SER_PLL_SPARE_CTRL {
        &self.c2c_ser_pll_spare_ctrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn c2c_ser_pll_tmux_sel(&self) -> &C2C_SER_PLL_TMUX_SEL {
        &self.c2c_ser_pll_tmux_sel
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn c2c_ser_pll_status1(&self) -> &C2C_SER_PLL_STATUS1 {
        &self.c2c_ser_pll_status1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn c2c_ser_pll_status2(&self) -> &C2C_SER_PLL_STATUS2 {
        &self.c2c_ser_pll_status2
    }
}
#[doc = "C2C_SER_PLL_LOOP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_ser_pll_loop_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_ser_pll_loop_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_ser_pll_loop_ctrl`]
module"]
pub type C2C_SER_PLL_LOOP_CTRL = crate::Reg<c2c_ser_pll_loop_ctrl::C2C_SER_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod c2c_ser_pll_loop_ctrl;
#[doc = "C2C_SER_PLL_DIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_ser_pll_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_ser_pll_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_ser_pll_div`]
module"]
pub type C2C_SER_PLL_DIV = crate::Reg<c2c_ser_pll_div::C2C_SER_PLL_DIV_SPEC>;
#[doc = ""]
pub mod c2c_ser_pll_div;
#[doc = "C2C_SER_PLL_DEBUG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_ser_pll_debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_ser_pll_debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_ser_pll_debug_ctrl`]
module"]
pub type C2C_SER_PLL_DEBUG_CTRL = crate::Reg<c2c_ser_pll_debug_ctrl::C2C_SER_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod c2c_ser_pll_debug_ctrl;
#[doc = "C2C_SER_PLL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_ser_pll_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_ser_pll_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_ser_pll_enable`]
module"]
pub type C2C_SER_PLL_ENABLE = crate::Reg<c2c_ser_pll_enable::C2C_SER_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod c2c_ser_pll_enable;
#[doc = "C2C_SER_PLL_SPARE_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_ser_pll_spare_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_ser_pll_spare_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_ser_pll_spare_ctrl`]
module"]
pub type C2C_SER_PLL_SPARE_CTRL = crate::Reg<c2c_ser_pll_spare_ctrl::C2C_SER_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod c2c_ser_pll_spare_ctrl;
#[doc = "C2C_SER_PLL_TMUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_ser_pll_tmux_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_ser_pll_tmux_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_ser_pll_tmux_sel`]
module"]
pub type C2C_SER_PLL_TMUX_SEL = crate::Reg<c2c_ser_pll_tmux_sel::C2C_SER_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod c2c_ser_pll_tmux_sel;
#[doc = "C2C_SER_PLL_STATUS1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_ser_pll_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_ser_pll_status1`]
module"]
pub type C2C_SER_PLL_STATUS1 = crate::Reg<c2c_ser_pll_status1::C2C_SER_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod c2c_ser_pll_status1;
#[doc = "C2C_SER_PLL_STATUS2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_ser_pll_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2c_ser_pll_status2`]
module"]
pub type C2C_SER_PLL_STATUS2 = crate::Reg<c2c_ser_pll_status2::C2C_SER_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod c2c_ser_pll_status2;
