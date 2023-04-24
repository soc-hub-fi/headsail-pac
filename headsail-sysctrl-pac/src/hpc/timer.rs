#[doc = r"Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - Monotonically increasing timer register for timer 0"]
    pub timer0_timer: TIMER0_TIMER,
    #[doc = "0x04 - Control register for timer 0"]
    pub timer0_ctrl: TIMER0_CTRL,
    #[doc = "0x08 - Timer compare register for timer 0. Writing this will zero timer register for timer 0."]
    pub timer0_cmp: TIMER0_CMP,
    #[doc = "0x0c - Monotonically increasing timer register for timer 1"]
    pub timer1_timer: TIMER1_TIMER,
    #[doc = "0x10 - Control register for timer 1"]
    pub timer1_ctrl: TIMER1_CTRL,
    #[doc = "0x14 - Timer compare register for timer 1. Writing this will zero timer register for timer 1."]
    pub timer1_cmp: TIMER1_CMP,
    #[doc = "0x18 - Monotonically increasing timer register for timer 2"]
    pub timer2_timer: TIMER2_TIMER,
    #[doc = "0x1c - Control register for timer 2"]
    pub timer2_ctrl: TIMER2_CTRL,
    #[doc = "0x20 - Timer compare register for timer 2. Writing this will zero timer register for timer 2."]
    pub timer2_cmp: TIMER2_CMP,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - Monotonically increasing timer register for timer 3"]
    pub timer3_timer: TIMER3_TIMER,
    #[doc = "0x34 - Control register for timer 3"]
    pub timer3_ctrl: TIMER3_CTRL,
    #[doc = "0x38 - Timer compare register for timer 3. Writing this will zero timer register for timer 3."]
    pub timer3_cmp: TIMER3_CMP,
}
#[doc = "timer3_cmp (rw) register accessor: an alias for `Reg<TIMER3_CMP_SPEC>`"]
pub type TIMER3_CMP = crate::Reg<timer3_cmp::TIMER3_CMP_SPEC>;
#[doc = "Timer compare register for timer 3. Writing this will zero timer register for timer 3."]
pub mod timer3_cmp;
#[doc = "timer3_ctrl (rw) register accessor: an alias for `Reg<TIMER3_CTRL_SPEC>`"]
pub type TIMER3_CTRL = crate::Reg<timer3_ctrl::TIMER3_CTRL_SPEC>;
#[doc = "Control register for timer 3"]
pub mod timer3_ctrl;
#[doc = "timer3_timer (r) register accessor: an alias for `Reg<TIMER3_TIMER_SPEC>`"]
pub type TIMER3_TIMER = crate::Reg<timer3_timer::TIMER3_TIMER_SPEC>;
#[doc = "Monotonically increasing timer register for timer 3"]
pub mod timer3_timer;
#[doc = "timer2_cmp (rw) register accessor: an alias for `Reg<TIMER2_CMP_SPEC>`"]
pub type TIMER2_CMP = crate::Reg<timer2_cmp::TIMER2_CMP_SPEC>;
#[doc = "Timer compare register for timer 2. Writing this will zero timer register for timer 2."]
pub mod timer2_cmp;
#[doc = "timer2_ctrl (rw) register accessor: an alias for `Reg<TIMER2_CTRL_SPEC>`"]
pub type TIMER2_CTRL = crate::Reg<timer2_ctrl::TIMER2_CTRL_SPEC>;
#[doc = "Control register for timer 2"]
pub mod timer2_ctrl;
#[doc = "timer2_timer (r) register accessor: an alias for `Reg<TIMER2_TIMER_SPEC>`"]
pub type TIMER2_TIMER = crate::Reg<timer2_timer::TIMER2_TIMER_SPEC>;
#[doc = "Monotonically increasing timer register for timer 2"]
pub mod timer2_timer;
#[doc = "timer1_cmp (rw) register accessor: an alias for `Reg<TIMER1_CMP_SPEC>`"]
pub type TIMER1_CMP = crate::Reg<timer1_cmp::TIMER1_CMP_SPEC>;
#[doc = "Timer compare register for timer 1. Writing this will zero timer register for timer 1."]
pub mod timer1_cmp;
#[doc = "timer1_ctrl (rw) register accessor: an alias for `Reg<TIMER1_CTRL_SPEC>`"]
pub type TIMER1_CTRL = crate::Reg<timer1_ctrl::TIMER1_CTRL_SPEC>;
#[doc = "Control register for timer 1"]
pub mod timer1_ctrl;
#[doc = "timer1_timer (r) register accessor: an alias for `Reg<TIMER1_TIMER_SPEC>`"]
pub type TIMER1_TIMER = crate::Reg<timer1_timer::TIMER1_TIMER_SPEC>;
#[doc = "Monotonically increasing timer register for timer 1"]
pub mod timer1_timer;
#[doc = "timer0_cmp (rw) register accessor: an alias for `Reg<TIMER0_CMP_SPEC>`"]
pub type TIMER0_CMP = crate::Reg<timer0_cmp::TIMER0_CMP_SPEC>;
#[doc = "Timer compare register for timer 0. Writing this will zero timer register for timer 0."]
pub mod timer0_cmp;
#[doc = "timer0_ctrl (rw) register accessor: an alias for `Reg<TIMER0_CTRL_SPEC>`"]
pub type TIMER0_CTRL = crate::Reg<timer0_ctrl::TIMER0_CTRL_SPEC>;
#[doc = "Control register for timer 0"]
pub mod timer0_ctrl;
#[doc = "timer0_timer (r) register accessor: an alias for `Reg<TIMER0_TIMER_SPEC>`"]
pub type TIMER0_TIMER = crate::Reg<timer0_timer::TIMER0_TIMER_SPEC>;
#[doc = "Monotonically increasing timer register for timer 0"]
pub mod timer0_timer;
