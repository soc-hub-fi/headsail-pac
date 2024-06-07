#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "SocEventGenerator"]
pub struct SocEventGenerator {
    sw_event: SwEvent,
    fc_mask0: FcMask0,
    fc_mask1: FcMask1,
    fc_mask2: FcMask2,
    fc_mask3: FcMask3,
    fc_mask4: FcMask4,
    fc_mask5: FcMask5,
    fc_mask6: FcMask6,
    fc_mask7: FcMask7,
    _reserved9: [u8; 0x20],
    pr_mask0: PrMask0,
    pr_mask1: PrMask1,
    pr_mask2: PrMask2,
    pr_mask3: PrMask3,
    pr_mask4: PrMask4,
    pr_mask5: PrMask5,
    pr_mask6: PrMask6,
    pr_mask7: PrMask7,
    err0: Err0,
    err1: Err1,
    err2: Err2,
    err3: Err3,
    err4: Err4,
    err5: Err5,
    err6: Err6,
    err7: Err7,
    timer_lo: TimerLo,
    timer_hi: TimerHi,
}
impl SocEventGenerator {
    #[doc = "0x00 - SoC software events trigger register"]
    #[inline(always)]
    pub const fn sw_event(&self) -> &SwEvent {
        &self.sw_event
    }
    #[doc = "0x04 - Events 0-31 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask0(&self) -> &FcMask0 {
        &self.fc_mask0
    }
    #[doc = "0x08 - Events 32-63 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask1(&self) -> &FcMask1 {
        &self.fc_mask1
    }
    #[doc = "0x0c - Events 64-95 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask2(&self) -> &FcMask2 {
        &self.fc_mask2
    }
    #[doc = "0x10 - Events 96-127 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask3(&self) -> &FcMask3 {
        &self.fc_mask3
    }
    #[doc = "0x14 - Events 128-159 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask4(&self) -> &FcMask4 {
        &self.fc_mask4
    }
    #[doc = "0x18 - Events 160-191 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask5(&self) -> &FcMask5 {
        &self.fc_mask5
    }
    #[doc = "0x1c - Events 191-223 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask6(&self) -> &FcMask6 {
        &self.fc_mask6
    }
    #[doc = "0x20 - F Events 224-255 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn fc_mask7(&self) -> &FcMask7 {
        &self.fc_mask7
    }
    #[doc = "0x44 - Events 0-31 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask0(&self) -> &PrMask0 {
        &self.pr_mask0
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn pr_mask1(&self) -> &PrMask1 {
        &self.pr_mask1
    }
    #[doc = "0x4c - Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask2(&self) -> &PrMask2 {
        &self.pr_mask2
    }
    #[doc = "0x50 - Events 96-127 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask3(&self) -> &PrMask3 {
        &self.pr_mask3
    }
    #[doc = "0x54 - Events 128-159 dispatch mask to peripheral"]
    #[inline(always)]
    pub const fn pr_mask4(&self) -> &PrMask4 {
        &self.pr_mask4
    }
    #[doc = "0x58 - Events 160-191 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask5(&self) -> &PrMask5 {
        &self.pr_mask5
    }
    #[doc = "0x5c - Events 191-223 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask6(&self) -> &PrMask6 {
        &self.pr_mask6
    }
    #[doc = "0x60 - Events 224-255 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask7(&self) -> &PrMask7 {
        &self.pr_mask7
    }
    #[doc = "0x64 - Events 0-31 event queue overflow"]
    #[inline(always)]
    pub const fn err0(&self) -> &Err0 {
        &self.err0
    }
    #[doc = "0x68 - Events 32-63 event queue overflow"]
    #[inline(always)]
    pub const fn err1(&self) -> &Err1 {
        &self.err1
    }
    #[doc = "0x6c - Events 64-95 event queue overflow"]
    #[inline(always)]
    pub const fn err2(&self) -> &Err2 {
        &self.err2
    }
    #[doc = "0x70 - Events 96-127 event queue overflow"]
    #[inline(always)]
    pub const fn err3(&self) -> &Err3 {
        &self.err3
    }
    #[doc = "0x74 - Events 128-159 event queue overflow"]
    #[inline(always)]
    pub const fn err4(&self) -> &Err4 {
        &self.err4
    }
    #[doc = "0x78 - Events 160-191 event queue overflow"]
    #[inline(always)]
    pub const fn err5(&self) -> &Err5 {
        &self.err5
    }
    #[doc = "0x7c - Events 191-223 event queue overflow"]
    #[inline(always)]
    pub const fn err6(&self) -> &Err6 {
        &self.err6
    }
    #[doc = "0x80 - Events 224-255 event queue overflow"]
    #[inline(always)]
    pub const fn err7(&self) -> &Err7 {
        &self.err7
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn timer_lo(&self) -> &TimerLo {
        &self.timer_lo
    }
    #[doc = "0x88 - Trigger Timer HI of APB Timer with event"]
    #[inline(always)]
    pub const fn timer_hi(&self) -> &TimerHi {
        &self.timer_hi
    }
}
#[doc = "SW_EVENT (w) register accessor: SoC software events trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_event::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_event`]
module"]
#[doc(alias = "SW_EVENT")]
pub type SwEvent = crate::Reg<sw_event::SwEventSpec>;
#[doc = "SoC software events trigger register"]
pub mod sw_event;
#[doc = "FC_MASK0 (rw) register accessor: Events 0-31 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask0`]
module"]
#[doc(alias = "FC_MASK0")]
pub type FcMask0 = crate::Reg<fc_mask0::FcMask0Spec>;
#[doc = "Events 0-31 dispatch mask to FC"]
pub mod fc_mask0;
#[doc = "FC_MASK1 (rw) register accessor: Events 32-63 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask1`]
module"]
#[doc(alias = "FC_MASK1")]
pub type FcMask1 = crate::Reg<fc_mask1::FcMask1Spec>;
#[doc = "Events 32-63 dispatch mask to FC"]
pub mod fc_mask1;
#[doc = "FC_MASK2 (rw) register accessor: Events 64-95 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask2`]
module"]
#[doc(alias = "FC_MASK2")]
pub type FcMask2 = crate::Reg<fc_mask2::FcMask2Spec>;
#[doc = "Events 64-95 dispatch mask to FC"]
pub mod fc_mask2;
#[doc = "FC_MASK3 (rw) register accessor: Events 96-127 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask3`]
module"]
#[doc(alias = "FC_MASK3")]
pub type FcMask3 = crate::Reg<fc_mask3::FcMask3Spec>;
#[doc = "Events 96-127 dispatch mask to FC"]
pub mod fc_mask3;
#[doc = "FC_MASK4 (rw) register accessor: Events 128-159 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask4`]
module"]
#[doc(alias = "FC_MASK4")]
pub type FcMask4 = crate::Reg<fc_mask4::FcMask4Spec>;
#[doc = "Events 128-159 dispatch mask to FC"]
pub mod fc_mask4;
#[doc = "FC_MASK5 (rw) register accessor: Events 160-191 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask5`]
module"]
#[doc(alias = "FC_MASK5")]
pub type FcMask5 = crate::Reg<fc_mask5::FcMask5Spec>;
#[doc = "Events 160-191 dispatch mask to FC"]
pub mod fc_mask5;
#[doc = "FC_MASK6 (rw) register accessor: Events 191-223 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask6`]
module"]
#[doc(alias = "FC_MASK6")]
pub type FcMask6 = crate::Reg<fc_mask6::FcMask6Spec>;
#[doc = "Events 191-223 dispatch mask to FC"]
pub mod fc_mask6;
#[doc = "FC_MASK7 (rw) register accessor: F Events 224-255 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask7`]
module"]
#[doc(alias = "FC_MASK7")]
pub type FcMask7 = crate::Reg<fc_mask7::FcMask7Spec>;
#[doc = "F Events 224-255 dispatch mask to peripherals"]
pub mod fc_mask7;
#[doc = "PR_MASK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask1`]
module"]
#[doc(alias = "PR_MASK1")]
pub type PrMask1 = crate::Reg<pr_mask1::PrMask1Spec>;
#[doc = ""]
pub mod pr_mask1;
#[doc = "PR_MASK2 (rw) register accessor: Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask2`]
module"]
#[doc(alias = "PR_MASK2")]
pub type PrMask2 = crate::Reg<pr_mask2::PrMask2Spec>;
#[doc = "Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals"]
pub mod pr_mask2;
#[doc = "PR_MASK3 (rw) register accessor: Events 96-127 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask3`]
module"]
#[doc(alias = "PR_MASK3")]
pub type PrMask3 = crate::Reg<pr_mask3::PrMask3Spec>;
#[doc = "Events 96-127 dispatch mask to peripherals"]
pub mod pr_mask3;
#[doc = "PR_MASK4 (rw) register accessor: Events 128-159 dispatch mask to peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask4`]
module"]
#[doc(alias = "PR_MASK4")]
pub type PrMask4 = crate::Reg<pr_mask4::PrMask4Spec>;
#[doc = "Events 128-159 dispatch mask to peripheral"]
pub mod pr_mask4;
#[doc = "PR_MASK5 (rw) register accessor: Events 160-191 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask5`]
module"]
#[doc(alias = "PR_MASK5")]
pub type PrMask5 = crate::Reg<pr_mask5::PrMask5Spec>;
#[doc = "Events 160-191 dispatch mask to peripherals"]
pub mod pr_mask5;
#[doc = "PR_MASK6 (rw) register accessor: Events 191-223 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask6`]
module"]
#[doc(alias = "PR_MASK6")]
pub type PrMask6 = crate::Reg<pr_mask6::PrMask6Spec>;
#[doc = "Events 191-223 dispatch mask to peripherals"]
pub mod pr_mask6;
#[doc = "PR_MASK7 (rw) register accessor: Events 224-255 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask7`]
module"]
#[doc(alias = "PR_MASK7")]
pub type PrMask7 = crate::Reg<pr_mask7::PrMask7Spec>;
#[doc = "Events 224-255 dispatch mask to peripherals"]
pub mod pr_mask7;
#[doc = "ERR0 (r) register accessor: Events 0-31 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err0`]
module"]
#[doc(alias = "ERR0")]
pub type Err0 = crate::Reg<err0::Err0Spec>;
#[doc = "Events 0-31 event queue overflow"]
pub mod err0;
#[doc = "PR_MASK0 (rw) register accessor: Events 0-31 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask0`]
module"]
#[doc(alias = "PR_MASK0")]
pub type PrMask0 = crate::Reg<pr_mask0::PrMask0Spec>;
#[doc = "Events 0-31 dispatch mask to peripherals"]
pub mod pr_mask0;
#[doc = "ERR1 (r) register accessor: Events 32-63 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err1`]
module"]
#[doc(alias = "ERR1")]
pub type Err1 = crate::Reg<err1::Err1Spec>;
#[doc = "Events 32-63 event queue overflow"]
pub mod err1;
#[doc = "ERR2 (r) register accessor: Events 64-95 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err2`]
module"]
#[doc(alias = "ERR2")]
pub type Err2 = crate::Reg<err2::Err2Spec>;
#[doc = "Events 64-95 event queue overflow"]
pub mod err2;
#[doc = "ERR3 (r) register accessor: Events 96-127 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err3`]
module"]
#[doc(alias = "ERR3")]
pub type Err3 = crate::Reg<err3::Err3Spec>;
#[doc = "Events 96-127 event queue overflow"]
pub mod err3;
#[doc = "ERR4 (r) register accessor: Events 128-159 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err4`]
module"]
#[doc(alias = "ERR4")]
pub type Err4 = crate::Reg<err4::Err4Spec>;
#[doc = "Events 128-159 event queue overflow"]
pub mod err4;
#[doc = "ERR5 (r) register accessor: Events 160-191 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err5`]
module"]
#[doc(alias = "ERR5")]
pub type Err5 = crate::Reg<err5::Err5Spec>;
#[doc = "Events 160-191 event queue overflow"]
pub mod err5;
#[doc = "ERR6 (r) register accessor: Events 191-223 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err6`]
module"]
#[doc(alias = "ERR6")]
pub type Err6 = crate::Reg<err6::Err6Spec>;
#[doc = "Events 191-223 event queue overflow"]
pub mod err6;
#[doc = "TIMER_HI (rw) register accessor: Trigger Timer HI of APB Timer with event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_hi`]
module"]
#[doc(alias = "TIMER_HI")]
pub type TimerHi = crate::Reg<timer_hi::TimerHiSpec>;
#[doc = "Trigger Timer HI of APB Timer with event"]
pub mod timer_hi;
#[doc = "ERR7 (r) register accessor: Events 224-255 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err7`]
module"]
#[doc(alias = "ERR7")]
pub type Err7 = crate::Reg<err7::Err7Spec>;
#[doc = "Events 224-255 event queue overflow"]
pub mod err7;
#[doc = "TIMER_LO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_lo`]
module"]
#[doc(alias = "TIMER_LO")]
pub type TimerLo = crate::Reg<timer_lo::TimerLoSpec>;
#[doc = ""]
pub mod timer_lo;
