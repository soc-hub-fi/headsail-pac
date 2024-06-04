#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "SocEventGenerator"]
#[doc(alias = "SocEventGenerator")]
pub struct SOC_EVENT_GENERATOR {
    sw_event: SW_EVENT,
    fc_mask0: FC_MASK0,
    fc_mask1: FC_MASK1,
    fc_mask2: FC_MASK2,
    fc_mask3: FC_MASK3,
    fc_mask4: FC_MASK4,
    fc_mask5: FC_MASK5,
    fc_mask6: FC_MASK6,
    fc_mask7: FC_MASK7,
    _reserved9: [u8; 0x20],
    pr_mask0: PR_MASK0,
    pr_mask1: PR_MASK1,
    pr_mask2: PR_MASK2,
    pr_mask3: PR_MASK3,
    pr_mask4: PR_MASK4,
    pr_mask5: PR_MASK5,
    pr_mask6: PR_MASK6,
    pr_mask7: PR_MASK7,
    err0: ERR0,
    err1: ERR1,
    err2: ERR2,
    err3: ERR3,
    err4: ERR4,
    err5: ERR5,
    err6: ERR6,
    err7: ERR7,
    timer_lo: TIMER_LO,
    timer_hi: TIMER_HI,
}
impl SOC_EVENT_GENERATOR {
    #[doc = "0x00 - SoC software events trigger register"]
    #[inline(always)]
    pub const fn sw_event(&self) -> &SW_EVENT {
        &self.sw_event
    }
    #[doc = "0x04 - Events 0-31 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask0(&self) -> &FC_MASK0 {
        &self.fc_mask0
    }
    #[doc = "0x08 - Events 32-63 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask1(&self) -> &FC_MASK1 {
        &self.fc_mask1
    }
    #[doc = "0x0c - Events 64-95 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask2(&self) -> &FC_MASK2 {
        &self.fc_mask2
    }
    #[doc = "0x10 - Events 96-127 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask3(&self) -> &FC_MASK3 {
        &self.fc_mask3
    }
    #[doc = "0x14 - Events 128-159 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask4(&self) -> &FC_MASK4 {
        &self.fc_mask4
    }
    #[doc = "0x18 - Events 160-191 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask5(&self) -> &FC_MASK5 {
        &self.fc_mask5
    }
    #[doc = "0x1c - Events 191-223 dispatch mask to FC"]
    #[inline(always)]
    pub const fn fc_mask6(&self) -> &FC_MASK6 {
        &self.fc_mask6
    }
    #[doc = "0x20 - F Events 224-255 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn fc_mask7(&self) -> &FC_MASK7 {
        &self.fc_mask7
    }
    #[doc = "0x44 - Events 0-31 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask0(&self) -> &PR_MASK0 {
        &self.pr_mask0
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn pr_mask1(&self) -> &PR_MASK1 {
        &self.pr_mask1
    }
    #[doc = "0x4c - Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask2(&self) -> &PR_MASK2 {
        &self.pr_mask2
    }
    #[doc = "0x50 - Events 96-127 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask3(&self) -> &PR_MASK3 {
        &self.pr_mask3
    }
    #[doc = "0x54 - Events 128-159 dispatch mask to peripheral"]
    #[inline(always)]
    pub const fn pr_mask4(&self) -> &PR_MASK4 {
        &self.pr_mask4
    }
    #[doc = "0x58 - Events 160-191 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask5(&self) -> &PR_MASK5 {
        &self.pr_mask5
    }
    #[doc = "0x5c - Events 191-223 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask6(&self) -> &PR_MASK6 {
        &self.pr_mask6
    }
    #[doc = "0x60 - Events 224-255 dispatch mask to peripherals"]
    #[inline(always)]
    pub const fn pr_mask7(&self) -> &PR_MASK7 {
        &self.pr_mask7
    }
    #[doc = "0x64 - Events 0-31 event queue overflow"]
    #[inline(always)]
    pub const fn err0(&self) -> &ERR0 {
        &self.err0
    }
    #[doc = "0x68 - Events 32-63 event queue overflow"]
    #[inline(always)]
    pub const fn err1(&self) -> &ERR1 {
        &self.err1
    }
    #[doc = "0x6c - Events 64-95 event queue overflow"]
    #[inline(always)]
    pub const fn err2(&self) -> &ERR2 {
        &self.err2
    }
    #[doc = "0x70 - Events 96-127 event queue overflow"]
    #[inline(always)]
    pub const fn err3(&self) -> &ERR3 {
        &self.err3
    }
    #[doc = "0x74 - Events 128-159 event queue overflow"]
    #[inline(always)]
    pub const fn err4(&self) -> &ERR4 {
        &self.err4
    }
    #[doc = "0x78 - Events 160-191 event queue overflow"]
    #[inline(always)]
    pub const fn err5(&self) -> &ERR5 {
        &self.err5
    }
    #[doc = "0x7c - Events 191-223 event queue overflow"]
    #[inline(always)]
    pub const fn err6(&self) -> &ERR6 {
        &self.err6
    }
    #[doc = "0x80 - Events 224-255 event queue overflow"]
    #[inline(always)]
    pub const fn err7(&self) -> &ERR7 {
        &self.err7
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn timer_lo(&self) -> &TIMER_LO {
        &self.timer_lo
    }
    #[doc = "0x88 - Trigger Timer HI of APB Timer with event"]
    #[inline(always)]
    pub const fn timer_hi(&self) -> &TIMER_HI {
        &self.timer_hi
    }
}
#[doc = "SW_EVENT (w) register accessor: SoC software events trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_event::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_event`]
module"]
pub type SW_EVENT = crate::Reg<sw_event::SW_EVENT_SPEC>;
#[doc = "SoC software events trigger register"]
pub mod sw_event;
#[doc = "FC_MASK0 (rw) register accessor: Events 0-31 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask0`]
module"]
pub type FC_MASK0 = crate::Reg<fc_mask0::FC_MASK0_SPEC>;
#[doc = "Events 0-31 dispatch mask to FC"]
pub mod fc_mask0;
#[doc = "FC_MASK1 (rw) register accessor: Events 32-63 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask1`]
module"]
pub type FC_MASK1 = crate::Reg<fc_mask1::FC_MASK1_SPEC>;
#[doc = "Events 32-63 dispatch mask to FC"]
pub mod fc_mask1;
#[doc = "FC_MASK2 (rw) register accessor: Events 64-95 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask2`]
module"]
pub type FC_MASK2 = crate::Reg<fc_mask2::FC_MASK2_SPEC>;
#[doc = "Events 64-95 dispatch mask to FC"]
pub mod fc_mask2;
#[doc = "FC_MASK3 (rw) register accessor: Events 96-127 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask3`]
module"]
pub type FC_MASK3 = crate::Reg<fc_mask3::FC_MASK3_SPEC>;
#[doc = "Events 96-127 dispatch mask to FC"]
pub mod fc_mask3;
#[doc = "FC_MASK4 (rw) register accessor: Events 128-159 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask4`]
module"]
pub type FC_MASK4 = crate::Reg<fc_mask4::FC_MASK4_SPEC>;
#[doc = "Events 128-159 dispatch mask to FC"]
pub mod fc_mask4;
#[doc = "FC_MASK5 (rw) register accessor: Events 160-191 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask5`]
module"]
pub type FC_MASK5 = crate::Reg<fc_mask5::FC_MASK5_SPEC>;
#[doc = "Events 160-191 dispatch mask to FC"]
pub mod fc_mask5;
#[doc = "FC_MASK6 (rw) register accessor: Events 191-223 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask6`]
module"]
pub type FC_MASK6 = crate::Reg<fc_mask6::FC_MASK6_SPEC>;
#[doc = "Events 191-223 dispatch mask to FC"]
pub mod fc_mask6;
#[doc = "FC_MASK7 (rw) register accessor: F Events 224-255 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc_mask7`]
module"]
pub type FC_MASK7 = crate::Reg<fc_mask7::FC_MASK7_SPEC>;
#[doc = "F Events 224-255 dispatch mask to peripherals"]
pub mod fc_mask7;
#[doc = "PR_MASK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask1`]
module"]
pub type PR_MASK1 = crate::Reg<pr_mask1::PR_MASK1_SPEC>;
#[doc = ""]
pub mod pr_mask1;
#[doc = "PR_MASK2 (rw) register accessor: Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask2`]
module"]
pub type PR_MASK2 = crate::Reg<pr_mask2::PR_MASK2_SPEC>;
#[doc = "Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals"]
pub mod pr_mask2;
#[doc = "PR_MASK3 (rw) register accessor: Events 96-127 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask3`]
module"]
pub type PR_MASK3 = crate::Reg<pr_mask3::PR_MASK3_SPEC>;
#[doc = "Events 96-127 dispatch mask to peripherals"]
pub mod pr_mask3;
#[doc = "PR_MASK4 (rw) register accessor: Events 128-159 dispatch mask to peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask4`]
module"]
pub type PR_MASK4 = crate::Reg<pr_mask4::PR_MASK4_SPEC>;
#[doc = "Events 128-159 dispatch mask to peripheral"]
pub mod pr_mask4;
#[doc = "PR_MASK5 (rw) register accessor: Events 160-191 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask5`]
module"]
pub type PR_MASK5 = crate::Reg<pr_mask5::PR_MASK5_SPEC>;
#[doc = "Events 160-191 dispatch mask to peripherals"]
pub mod pr_mask5;
#[doc = "PR_MASK6 (rw) register accessor: Events 191-223 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask6`]
module"]
pub type PR_MASK6 = crate::Reg<pr_mask6::PR_MASK6_SPEC>;
#[doc = "Events 191-223 dispatch mask to peripherals"]
pub mod pr_mask6;
#[doc = "PR_MASK7 (rw) register accessor: Events 224-255 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask7`]
module"]
pub type PR_MASK7 = crate::Reg<pr_mask7::PR_MASK7_SPEC>;
#[doc = "Events 224-255 dispatch mask to peripherals"]
pub mod pr_mask7;
#[doc = "ERR0 (r) register accessor: Events 0-31 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err0`]
module"]
pub type ERR0 = crate::Reg<err0::ERR0_SPEC>;
#[doc = "Events 0-31 event queue overflow"]
pub mod err0;
#[doc = "PR_MASK0 (rw) register accessor: Events 0-31 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_mask0`]
module"]
pub type PR_MASK0 = crate::Reg<pr_mask0::PR_MASK0_SPEC>;
#[doc = "Events 0-31 dispatch mask to peripherals"]
pub mod pr_mask0;
#[doc = "ERR1 (r) register accessor: Events 32-63 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err1`]
module"]
pub type ERR1 = crate::Reg<err1::ERR1_SPEC>;
#[doc = "Events 32-63 event queue overflow"]
pub mod err1;
#[doc = "ERR2 (r) register accessor: Events 64-95 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err2`]
module"]
pub type ERR2 = crate::Reg<err2::ERR2_SPEC>;
#[doc = "Events 64-95 event queue overflow"]
pub mod err2;
#[doc = "ERR3 (r) register accessor: Events 96-127 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err3`]
module"]
pub type ERR3 = crate::Reg<err3::ERR3_SPEC>;
#[doc = "Events 96-127 event queue overflow"]
pub mod err3;
#[doc = "ERR4 (r) register accessor: Events 128-159 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err4`]
module"]
pub type ERR4 = crate::Reg<err4::ERR4_SPEC>;
#[doc = "Events 128-159 event queue overflow"]
pub mod err4;
#[doc = "ERR5 (r) register accessor: Events 160-191 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err5`]
module"]
pub type ERR5 = crate::Reg<err5::ERR5_SPEC>;
#[doc = "Events 160-191 event queue overflow"]
pub mod err5;
#[doc = "ERR6 (r) register accessor: Events 191-223 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err6`]
module"]
pub type ERR6 = crate::Reg<err6::ERR6_SPEC>;
#[doc = "Events 191-223 event queue overflow"]
pub mod err6;
#[doc = "TIMER_HI (rw) register accessor: Trigger Timer HI of APB Timer with event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_hi`]
module"]
pub type TIMER_HI = crate::Reg<timer_hi::TIMER_HI_SPEC>;
#[doc = "Trigger Timer HI of APB Timer with event"]
pub mod timer_hi;
#[doc = "ERR7 (r) register accessor: Events 224-255 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err7`]
module"]
pub type ERR7 = crate::Reg<err7::ERR7_SPEC>;
#[doc = "Events 224-255 event queue overflow"]
pub mod err7;
#[doc = "TIMER_LO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_lo`]
module"]
pub type TIMER_LO = crate::Reg<timer_lo::TIMER_LO_SPEC>;
#[doc = ""]
pub mod timer_lo;
