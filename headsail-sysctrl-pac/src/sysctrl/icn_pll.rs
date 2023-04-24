#[doc = r"Register block"]
#[repr(C)]
pub struct ICN_PLL {
    #[doc = "0x00 - "]
    pub icn_pll_loop_ctrl: ICN_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub icn_pll_div: ICN_PLL_DIV,
    #[doc = "0x08 - "]
    pub icn_pll_debug_ctrl: ICN_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub icn_pll_enable: ICN_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub icn_pll_spare_ctrl: ICN_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub icn_pll_tmux_sel: ICN_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub icn_pll_status1: ICN_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub icn_pll_status2: ICN_PLL_STATUS2,
}
#[doc = "ICN_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<ICN_PLL_LOOP_CTRL_SPEC>`"]
pub type ICN_PLL_LOOP_CTRL = crate::Reg<icn_pll_loop_ctrl::ICN_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod icn_pll_loop_ctrl;
#[doc = "ICN_PLL_DIV (rw) register accessor: an alias for `Reg<ICN_PLL_DIV_SPEC>`"]
pub type ICN_PLL_DIV = crate::Reg<icn_pll_div::ICN_PLL_DIV_SPEC>;
#[doc = ""]
pub mod icn_pll_div;
#[doc = "ICN_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<ICN_PLL_DEBUG_CTRL_SPEC>`"]
pub type ICN_PLL_DEBUG_CTRL = crate::Reg<icn_pll_debug_ctrl::ICN_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod icn_pll_debug_ctrl;
#[doc = "ICN_PLL_ENABLE (rw) register accessor: an alias for `Reg<ICN_PLL_ENABLE_SPEC>`"]
pub type ICN_PLL_ENABLE = crate::Reg<icn_pll_enable::ICN_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod icn_pll_enable;
#[doc = "ICN_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<ICN_PLL_SPARE_CTRL_SPEC>`"]
pub type ICN_PLL_SPARE_CTRL = crate::Reg<icn_pll_spare_ctrl::ICN_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod icn_pll_spare_ctrl;
#[doc = "ICN_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<ICN_PLL_TMUX_SEL_SPEC>`"]
pub type ICN_PLL_TMUX_SEL = crate::Reg<icn_pll_tmux_sel::ICN_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod icn_pll_tmux_sel;
#[doc = "ICN_PLL_STATUS1 (r) register accessor: an alias for `Reg<ICN_PLL_STATUS1_SPEC>`"]
pub type ICN_PLL_STATUS1 = crate::Reg<icn_pll_status1::ICN_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod icn_pll_status1;
#[doc = "ICN_PLL_STATUS2 (r) register accessor: an alias for `Reg<ICN_PLL_STATUS2_SPEC>`"]
pub type ICN_PLL_STATUS2 = crate::Reg<icn_pll_status2::ICN_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod icn_pll_status2;
