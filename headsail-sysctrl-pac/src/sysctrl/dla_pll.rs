#[doc = r"Register block"]
#[repr(C)]
pub struct DLA_PLL {
    #[doc = "0x00 - "]
    pub dla_pll_loop_ctrl: DLA_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub dla_pll_div: DLA_PLL_DIV,
    #[doc = "0x08 - "]
    pub dla_pll_debug_ctrl: DLA_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub dla_pll_enable: DLA_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub dla_pll_spare_ctrl: DLA_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub dla_pll_tmux_sel: DLA_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub dla_pll_status1: DLA_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub dla_pll_status2: DLA_PLL_STATUS2,
}
#[doc = "DLA_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<DLA_PLL_LOOP_CTRL_SPEC>`"]
pub type DLA_PLL_LOOP_CTRL = crate::Reg<dla_pll_loop_ctrl::DLA_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod dla_pll_loop_ctrl;
#[doc = "DLA_PLL_DIV (rw) register accessor: an alias for `Reg<DLA_PLL_DIV_SPEC>`"]
pub type DLA_PLL_DIV = crate::Reg<dla_pll_div::DLA_PLL_DIV_SPEC>;
#[doc = ""]
pub mod dla_pll_div;
#[doc = "DLA_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<DLA_PLL_DEBUG_CTRL_SPEC>`"]
pub type DLA_PLL_DEBUG_CTRL = crate::Reg<dla_pll_debug_ctrl::DLA_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod dla_pll_debug_ctrl;
#[doc = "DLA_PLL_ENABLE (rw) register accessor: an alias for `Reg<DLA_PLL_ENABLE_SPEC>`"]
pub type DLA_PLL_ENABLE = crate::Reg<dla_pll_enable::DLA_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod dla_pll_enable;
#[doc = "DLA_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<DLA_PLL_SPARE_CTRL_SPEC>`"]
pub type DLA_PLL_SPARE_CTRL = crate::Reg<dla_pll_spare_ctrl::DLA_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod dla_pll_spare_ctrl;
#[doc = "DLA_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<DLA_PLL_TMUX_SEL_SPEC>`"]
pub type DLA_PLL_TMUX_SEL = crate::Reg<dla_pll_tmux_sel::DLA_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod dla_pll_tmux_sel;
#[doc = "DLA_PLL_STATUS1 (r) register accessor: an alias for `Reg<DLA_PLL_STATUS1_SPEC>`"]
pub type DLA_PLL_STATUS1 = crate::Reg<dla_pll_status1::DLA_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod dla_pll_status1;
#[doc = "DLA_PLL_STATUS2 (r) register accessor: an alias for `Reg<DLA_PLL_STATUS2_SPEC>`"]
pub type DLA_PLL_STATUS2 = crate::Reg<dla_pll_status2::DLA_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod dla_pll_status2;
