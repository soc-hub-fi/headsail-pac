#[doc = r"Register block"]
#[repr(C)]
pub struct DSP_PLL {
    #[doc = "0x00 - "]
    pub dsp_loop_ctrl: DSP_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub dsp_div: DSP_DIV,
    #[doc = "0x08 - "]
    pub dsp_debug_ctrl: DSP_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub dsp_pll_enable: DSP_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub dsp_pll_spare_ctrl: DSP_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub dsp_pll_tmux_sel: DSP_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub dsp_pll_status1: DSP_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub dsp_pll_status2: DSP_PLL_STATUS2,
}
#[doc = "DSP_LOOP_CTRL (rw) register accessor: an alias for `Reg<DSP_LOOP_CTRL_SPEC>`"]
pub type DSP_LOOP_CTRL = crate::Reg<dsp_loop_ctrl::DSP_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod dsp_loop_ctrl;
#[doc = "DSP_DIV (rw) register accessor: an alias for `Reg<DSP_DIV_SPEC>`"]
pub type DSP_DIV = crate::Reg<dsp_div::DSP_DIV_SPEC>;
#[doc = ""]
pub mod dsp_div;
#[doc = "DSP_DEBUG_CTRL (rw) register accessor: an alias for `Reg<DSP_DEBUG_CTRL_SPEC>`"]
pub type DSP_DEBUG_CTRL = crate::Reg<dsp_debug_ctrl::DSP_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod dsp_debug_ctrl;
#[doc = "DSP_PLL_ENABLE (rw) register accessor: an alias for `Reg<DSP_PLL_ENABLE_SPEC>`"]
pub type DSP_PLL_ENABLE = crate::Reg<dsp_pll_enable::DSP_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod dsp_pll_enable;
#[doc = "DSP_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<DSP_PLL_SPARE_CTRL_SPEC>`"]
pub type DSP_PLL_SPARE_CTRL = crate::Reg<dsp_pll_spare_ctrl::DSP_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod dsp_pll_spare_ctrl;
#[doc = "DSP_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<DSP_PLL_TMUX_SEL_SPEC>`"]
pub type DSP_PLL_TMUX_SEL = crate::Reg<dsp_pll_tmux_sel::DSP_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod dsp_pll_tmux_sel;
#[doc = "DSP_PLL_STATUS1 (r) register accessor: an alias for `Reg<DSP_PLL_STATUS1_SPEC>`"]
pub type DSP_PLL_STATUS1 = crate::Reg<dsp_pll_status1::DSP_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod dsp_pll_status1;
#[doc = "DSP_PLL_STATUS2 (r) register accessor: an alias for `Reg<DSP_PLL_STATUS2_SPEC>`"]
pub type DSP_PLL_STATUS2 = crate::Reg<dsp_pll_status2::DSP_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod dsp_pll_status2;
