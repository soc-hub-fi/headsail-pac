#[doc = r"Register block"]
#[repr(C)]
pub struct DDR2_PLL {
    #[doc = "0x00 - "]
    pub ddr2_pll_loop_ctrl: DDR2_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub ddr2_pll_div: DDR2_PLL_DIV,
    #[doc = "0x08 - "]
    pub ddr2_pll_debug_ctrl: DDR2_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub ddr2_pll_enable: DDR2_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub ddr2_pll_spare_ctrl: DDR2_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub ddr2_pll_tmux_sel: DDR2_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub ddr2_pll_status1: DDR2_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub ddr2_pll_status2: DDR2_PLL_STATUS2,
}
#[doc = "DDR2_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<DDR2_PLL_LOOP_CTRL_SPEC>`"]
pub type DDR2_PLL_LOOP_CTRL = crate::Reg<ddr2_pll_loop_ctrl::DDR2_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod ddr2_pll_loop_ctrl;
#[doc = "DDR2_PLL_DIV (rw) register accessor: an alias for `Reg<DDR2_PLL_DIV_SPEC>`"]
pub type DDR2_PLL_DIV = crate::Reg<ddr2_pll_div::DDR2_PLL_DIV_SPEC>;
#[doc = ""]
pub mod ddr2_pll_div;
#[doc = "DDR2_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<DDR2_PLL_DEBUG_CTRL_SPEC>`"]
pub type DDR2_PLL_DEBUG_CTRL = crate::Reg<ddr2_pll_debug_ctrl::DDR2_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod ddr2_pll_debug_ctrl;
#[doc = "DDR2_PLL_ENABLE (rw) register accessor: an alias for `Reg<DDR2_PLL_ENABLE_SPEC>`"]
pub type DDR2_PLL_ENABLE = crate::Reg<ddr2_pll_enable::DDR2_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod ddr2_pll_enable;
#[doc = "DDR2_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<DDR2_PLL_SPARE_CTRL_SPEC>`"]
pub type DDR2_PLL_SPARE_CTRL = crate::Reg<ddr2_pll_spare_ctrl::DDR2_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod ddr2_pll_spare_ctrl;
#[doc = "DDR2_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<DDR2_PLL_TMUX_SEL_SPEC>`"]
pub type DDR2_PLL_TMUX_SEL = crate::Reg<ddr2_pll_tmux_sel::DDR2_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod ddr2_pll_tmux_sel;
#[doc = "DDR2_PLL_STATUS1 (r) register accessor: an alias for `Reg<DDR2_PLL_STATUS1_SPEC>`"]
pub type DDR2_PLL_STATUS1 = crate::Reg<ddr2_pll_status1::DDR2_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod ddr2_pll_status1;
#[doc = "DDR2_PLL_STATUS2 (r) register accessor: an alias for `Reg<DDR2_PLL_STATUS2_SPEC>`"]
pub type DDR2_PLL_STATUS2 = crate::Reg<ddr2_pll_status2::DDR2_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod ddr2_pll_status2;
