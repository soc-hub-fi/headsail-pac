#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "timer"]
#[doc(alias = "timer")]
pub struct Timer {
    timer0_timer: Timer0Timer,
    timer0_ctrl: Timer0Ctrl,
    timer0_cmp: Timer0Cmp,
    timer1_timer: Timer1Timer,
    timer1_ctrl: Timer1Ctrl,
    timer1_cmp: Timer1Cmp,
    timer2_timer: Timer2Timer,
    timer2_ctrl: Timer2Ctrl,
    timer2_cmp: Timer2Cmp,
    _reserved9: [u8; 0x0c],
    timer3_timer: Timer3Timer,
    timer3_ctrl: Timer3Ctrl,
    timer3_cmp: Timer3Cmp,
}
impl Timer {
    #[doc = "0x00 - Monotonically increasing timer register for timer 0"]
    #[inline(always)]
    pub const fn timer0_timer(&self) -> &Timer0Timer {
        &self.timer0_timer
    }
    #[doc = "0x04 - Control register for timer 0"]
    #[inline(always)]
    pub const fn timer0_ctrl(&self) -> &Timer0Ctrl {
        &self.timer0_ctrl
    }
    #[doc = "0x08 - Timer compare register for timer 0. Writing this will zero timer register for timer 0."]
    #[inline(always)]
    pub const fn timer0_cmp(&self) -> &Timer0Cmp {
        &self.timer0_cmp
    }
    #[doc = "0x0c - Monotonically increasing timer register for timer 1"]
    #[inline(always)]
    pub const fn timer1_timer(&self) -> &Timer1Timer {
        &self.timer1_timer
    }
    #[doc = "0x10 - Control register for timer 1"]
    #[inline(always)]
    pub const fn timer1_ctrl(&self) -> &Timer1Ctrl {
        &self.timer1_ctrl
    }
    #[doc = "0x14 - Timer compare register for timer 1. Writing this will zero timer register for timer 1."]
    #[inline(always)]
    pub const fn timer1_cmp(&self) -> &Timer1Cmp {
        &self.timer1_cmp
    }
    #[doc = "0x18 - Monotonically increasing timer register for timer 2"]
    #[inline(always)]
    pub const fn timer2_timer(&self) -> &Timer2Timer {
        &self.timer2_timer
    }
    #[doc = "0x1c - Control register for timer 2"]
    #[inline(always)]
    pub const fn timer2_ctrl(&self) -> &Timer2Ctrl {
        &self.timer2_ctrl
    }
    #[doc = "0x20 - Timer compare register for timer 2. Writing this will zero timer register for timer 2."]
    #[inline(always)]
    pub const fn timer2_cmp(&self) -> &Timer2Cmp {
        &self.timer2_cmp
    }
    #[doc = "0x30 - Monotonically increasing timer register for timer 3"]
    #[inline(always)]
    pub const fn timer3_timer(&self) -> &Timer3Timer {
        &self.timer3_timer
    }
    #[doc = "0x34 - Control register for timer 3"]
    #[inline(always)]
    pub const fn timer3_ctrl(&self) -> &Timer3Ctrl {
        &self.timer3_ctrl
    }
    #[doc = "0x38 - Timer compare register for timer 3. Writing this will zero timer register for timer 3."]
    #[inline(always)]
    pub const fn timer3_cmp(&self) -> &Timer3Cmp {
        &self.timer3_cmp
    }
}
#[doc = "timer3_cmp (rw) register accessor: Timer compare register for timer 3. Writing this will zero timer register for timer 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cmp`]
module"]
#[doc(alias = "timer3_cmp")]
pub type Timer3Cmp = crate::Reg<timer3_cmp::Timer3CmpSpec>;
#[doc = "Timer compare register for timer 3. Writing this will zero timer register for timer 3."]
pub mod timer3_cmp;
#[doc = "timer3_ctrl (rw) register accessor: Control register for timer 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_ctrl`]
module"]
#[doc(alias = "timer3_ctrl")]
pub type Timer3Ctrl = crate::Reg<timer3_ctrl::Timer3CtrlSpec>;
#[doc = "Control register for timer 3"]
pub mod timer3_ctrl;
#[doc = "timer3_timer (r) register accessor: Monotonically increasing timer register for timer 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_timer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_timer`]
module"]
#[doc(alias = "timer3_timer")]
pub type Timer3Timer = crate::Reg<timer3_timer::Timer3TimerSpec>;
#[doc = "Monotonically increasing timer register for timer 3"]
pub mod timer3_timer;
#[doc = "timer2_cmp (rw) register accessor: Timer compare register for timer 2. Writing this will zero timer register for timer 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cmp`]
module"]
#[doc(alias = "timer2_cmp")]
pub type Timer2Cmp = crate::Reg<timer2_cmp::Timer2CmpSpec>;
#[doc = "Timer compare register for timer 2. Writing this will zero timer register for timer 2."]
pub mod timer2_cmp;
#[doc = "timer2_ctrl (rw) register accessor: Control register for timer 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_ctrl`]
module"]
#[doc(alias = "timer2_ctrl")]
pub type Timer2Ctrl = crate::Reg<timer2_ctrl::Timer2CtrlSpec>;
#[doc = "Control register for timer 2"]
pub mod timer2_ctrl;
#[doc = "timer2_timer (r) register accessor: Monotonically increasing timer register for timer 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_timer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_timer`]
module"]
#[doc(alias = "timer2_timer")]
pub type Timer2Timer = crate::Reg<timer2_timer::Timer2TimerSpec>;
#[doc = "Monotonically increasing timer register for timer 2"]
pub mod timer2_timer;
#[doc = "timer1_cmp (rw) register accessor: Timer compare register for timer 1. Writing this will zero timer register for timer 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cmp`]
module"]
#[doc(alias = "timer1_cmp")]
pub type Timer1Cmp = crate::Reg<timer1_cmp::Timer1CmpSpec>;
#[doc = "Timer compare register for timer 1. Writing this will zero timer register for timer 1."]
pub mod timer1_cmp;
#[doc = "timer1_ctrl (rw) register accessor: Control register for timer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_ctrl`]
module"]
#[doc(alias = "timer1_ctrl")]
pub type Timer1Ctrl = crate::Reg<timer1_ctrl::Timer1CtrlSpec>;
#[doc = "Control register for timer 1"]
pub mod timer1_ctrl;
#[doc = "timer1_timer (r) register accessor: Monotonically increasing timer register for timer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_timer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_timer`]
module"]
#[doc(alias = "timer1_timer")]
pub type Timer1Timer = crate::Reg<timer1_timer::Timer1TimerSpec>;
#[doc = "Monotonically increasing timer register for timer 1"]
pub mod timer1_timer;
#[doc = "timer0_cmp (rw) register accessor: Timer compare register for timer 0. Writing this will zero timer register for timer 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cmp`]
module"]
#[doc(alias = "timer0_cmp")]
pub type Timer0Cmp = crate::Reg<timer0_cmp::Timer0CmpSpec>;
#[doc = "Timer compare register for timer 0. Writing this will zero timer register for timer 0."]
pub mod timer0_cmp;
#[doc = "timer0_ctrl (rw) register accessor: Control register for timer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_ctrl`]
module"]
#[doc(alias = "timer0_ctrl")]
pub type Timer0Ctrl = crate::Reg<timer0_ctrl::Timer0CtrlSpec>;
#[doc = "Control register for timer 0"]
pub mod timer0_ctrl;
#[doc = "timer0_timer (r) register accessor: Monotonically increasing timer register for timer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_timer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_timer`]
module"]
#[doc(alias = "timer0_timer")]
pub type Timer0Timer = crate::Reg<timer0_timer::Timer0TimerSpec>;
#[doc = "Monotonically increasing timer register for timer 0"]
pub mod timer0_timer;
