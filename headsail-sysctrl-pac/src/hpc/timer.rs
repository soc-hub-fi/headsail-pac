#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "timer"]
#[doc(alias = "timer")]
pub struct TIMER {
    timer0_timer: TIMER0_TIMER,
    timer0_ctrl: TIMER0_CTRL,
    timer0_cmp: TIMER0_CMP,
    timer1_timer: TIMER1_TIMER,
    timer1_ctrl: TIMER1_CTRL,
    timer1_cmp: TIMER1_CMP,
    timer2_timer: TIMER2_TIMER,
    timer2_ctrl: TIMER2_CTRL,
    timer2_cmp: TIMER2_CMP,
    _reserved9: [u8; 0x0c],
    timer3_timer: TIMER3_TIMER,
    timer3_ctrl: TIMER3_CTRL,
    timer3_cmp: TIMER3_CMP,
}
impl TIMER {
    #[doc = "0x00 - Monotonically increasing timer register for timer 0"]
    #[inline(always)]
    pub const fn timer0_timer(&self) -> &TIMER0_TIMER {
        &self.timer0_timer
    }
    #[doc = "0x04 - Control register for timer 0"]
    #[inline(always)]
    pub const fn timer0_ctrl(&self) -> &TIMER0_CTRL {
        &self.timer0_ctrl
    }
    #[doc = "0x08 - Timer compare register for timer 0. Writing this will zero timer register for timer 0."]
    #[inline(always)]
    pub const fn timer0_cmp(&self) -> &TIMER0_CMP {
        &self.timer0_cmp
    }
    #[doc = "0x0c - Monotonically increasing timer register for timer 1"]
    #[inline(always)]
    pub const fn timer1_timer(&self) -> &TIMER1_TIMER {
        &self.timer1_timer
    }
    #[doc = "0x10 - Control register for timer 1"]
    #[inline(always)]
    pub const fn timer1_ctrl(&self) -> &TIMER1_CTRL {
        &self.timer1_ctrl
    }
    #[doc = "0x14 - Timer compare register for timer 1. Writing this will zero timer register for timer 1."]
    #[inline(always)]
    pub const fn timer1_cmp(&self) -> &TIMER1_CMP {
        &self.timer1_cmp
    }
    #[doc = "0x18 - Monotonically increasing timer register for timer 2"]
    #[inline(always)]
    pub const fn timer2_timer(&self) -> &TIMER2_TIMER {
        &self.timer2_timer
    }
    #[doc = "0x1c - Control register for timer 2"]
    #[inline(always)]
    pub const fn timer2_ctrl(&self) -> &TIMER2_CTRL {
        &self.timer2_ctrl
    }
    #[doc = "0x20 - Timer compare register for timer 2. Writing this will zero timer register for timer 2."]
    #[inline(always)]
    pub const fn timer2_cmp(&self) -> &TIMER2_CMP {
        &self.timer2_cmp
    }
    #[doc = "0x30 - Monotonically increasing timer register for timer 3"]
    #[inline(always)]
    pub const fn timer3_timer(&self) -> &TIMER3_TIMER {
        &self.timer3_timer
    }
    #[doc = "0x34 - Control register for timer 3"]
    #[inline(always)]
    pub const fn timer3_ctrl(&self) -> &TIMER3_CTRL {
        &self.timer3_ctrl
    }
    #[doc = "0x38 - Timer compare register for timer 3. Writing this will zero timer register for timer 3."]
    #[inline(always)]
    pub const fn timer3_cmp(&self) -> &TIMER3_CMP {
        &self.timer3_cmp
    }
}
#[doc = "timer3_cmp (rw) register accessor: Timer compare register for timer 3. Writing this will zero timer register for timer 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cmp`]
module"]
#[doc(alias = "timer3_cmp")]
pub type TIMER3_CMP = crate::Reg<timer3_cmp::TIMER3_CMP_SPEC>;
#[doc = "Timer compare register for timer 3. Writing this will zero timer register for timer 3."]
pub mod timer3_cmp;
#[doc = "timer3_ctrl (rw) register accessor: Control register for timer 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_ctrl`]
module"]
#[doc(alias = "timer3_ctrl")]
pub type TIMER3_CTRL = crate::Reg<timer3_ctrl::TIMER3_CTRL_SPEC>;
#[doc = "Control register for timer 3"]
pub mod timer3_ctrl;
#[doc = "timer3_timer (r) register accessor: Monotonically increasing timer register for timer 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_timer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_timer`]
module"]
#[doc(alias = "timer3_timer")]
pub type TIMER3_TIMER = crate::Reg<timer3_timer::TIMER3_TIMER_SPEC>;
#[doc = "Monotonically increasing timer register for timer 3"]
pub mod timer3_timer;
#[doc = "timer2_cmp (rw) register accessor: Timer compare register for timer 2. Writing this will zero timer register for timer 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cmp`]
module"]
#[doc(alias = "timer2_cmp")]
pub type TIMER2_CMP = crate::Reg<timer2_cmp::TIMER2_CMP_SPEC>;
#[doc = "Timer compare register for timer 2. Writing this will zero timer register for timer 2."]
pub mod timer2_cmp;
#[doc = "timer2_ctrl (rw) register accessor: Control register for timer 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_ctrl`]
module"]
#[doc(alias = "timer2_ctrl")]
pub type TIMER2_CTRL = crate::Reg<timer2_ctrl::TIMER2_CTRL_SPEC>;
#[doc = "Control register for timer 2"]
pub mod timer2_ctrl;
#[doc = "timer2_timer (r) register accessor: Monotonically increasing timer register for timer 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_timer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_timer`]
module"]
#[doc(alias = "timer2_timer")]
pub type TIMER2_TIMER = crate::Reg<timer2_timer::TIMER2_TIMER_SPEC>;
#[doc = "Monotonically increasing timer register for timer 2"]
pub mod timer2_timer;
#[doc = "timer1_cmp (rw) register accessor: Timer compare register for timer 1. Writing this will zero timer register for timer 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cmp`]
module"]
#[doc(alias = "timer1_cmp")]
pub type TIMER1_CMP = crate::Reg<timer1_cmp::TIMER1_CMP_SPEC>;
#[doc = "Timer compare register for timer 1. Writing this will zero timer register for timer 1."]
pub mod timer1_cmp;
#[doc = "timer1_ctrl (rw) register accessor: Control register for timer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_ctrl`]
module"]
#[doc(alias = "timer1_ctrl")]
pub type TIMER1_CTRL = crate::Reg<timer1_ctrl::TIMER1_CTRL_SPEC>;
#[doc = "Control register for timer 1"]
pub mod timer1_ctrl;
#[doc = "timer1_timer (r) register accessor: Monotonically increasing timer register for timer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_timer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_timer`]
module"]
#[doc(alias = "timer1_timer")]
pub type TIMER1_TIMER = crate::Reg<timer1_timer::TIMER1_TIMER_SPEC>;
#[doc = "Monotonically increasing timer register for timer 1"]
pub mod timer1_timer;
#[doc = "timer0_cmp (rw) register accessor: Timer compare register for timer 0. Writing this will zero timer register for timer 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cmp`]
module"]
#[doc(alias = "timer0_cmp")]
pub type TIMER0_CMP = crate::Reg<timer0_cmp::TIMER0_CMP_SPEC>;
#[doc = "Timer compare register for timer 0. Writing this will zero timer register for timer 0."]
pub mod timer0_cmp;
#[doc = "timer0_ctrl (rw) register accessor: Control register for timer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_ctrl`]
module"]
#[doc(alias = "timer0_ctrl")]
pub type TIMER0_CTRL = crate::Reg<timer0_ctrl::TIMER0_CTRL_SPEC>;
#[doc = "Control register for timer 0"]
pub mod timer0_ctrl;
#[doc = "timer0_timer (r) register accessor: Monotonically increasing timer register for timer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_timer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_timer`]
module"]
#[doc(alias = "timer0_timer")]
pub type TIMER0_TIMER = crate::Reg<timer0_timer::TIMER0_TIMER_SPEC>;
#[doc = "Monotonically increasing timer register for timer 0"]
pub mod timer0_timer;
