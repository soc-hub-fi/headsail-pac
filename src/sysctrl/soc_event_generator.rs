#[doc = r"Register block"]
#[repr(C)]
pub struct SOC_EVENT_GENERATOR {
    #[doc = "0x00 - SoC software events trigger register"]
    pub sw_event: SW_EVENT,
    #[doc = "0x04 - Events 0-31 dispatch mask to FC"]
    pub fc_mask0: FC_MASK0,
    #[doc = "0x08 - Events 32-63 dispatch mask to FC"]
    pub fc_mask1: FC_MASK1,
    #[doc = "0x0c - Events 64-95 dispatch mask to FC"]
    pub fc_mask2: FC_MASK2,
    #[doc = "0x10 - Events 96-127 dispatch mask to FC"]
    pub fc_mask3: FC_MASK3,
    #[doc = "0x14 - Events 128-159 dispatch mask to FC"]
    pub fc_mask4: FC_MASK4,
    #[doc = "0x18 - Events 160-191 dispatch mask to FC"]
    pub fc_mask5: FC_MASK5,
    #[doc = "0x1c - Events 191-223 dispatch mask to FC"]
    pub fc_mask6: FC_MASK6,
    #[doc = "0x20 - F Events 224-255 dispatch mask to peripherals"]
    pub fc_mask7: FC_MASK7,
    _reserved9: [u8; 0x20],
    #[doc = "0x44 - Events 0-31 dispatch mask to peripherals"]
    pub pr_mask0: PR_MASK0,
    #[doc = "0x48 - "]
    pub pr_mask1: PR_MASK1,
    #[doc = "0x4c - Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals"]
    pub pr_mask2: PR_MASK2,
    #[doc = "0x50 - Events 96-127 dispatch mask to peripherals"]
    pub pr_mask3: PR_MASK3,
    #[doc = "0x54 - Events 128-159 dispatch mask to peripheral"]
    pub pr_mask4: PR_MASK4,
    #[doc = "0x58 - Events 160-191 dispatch mask to peripherals"]
    pub pr_mask5: PR_MASK5,
    #[doc = "0x5c - Events 191-223 dispatch mask to peripherals"]
    pub pr_mask6: PR_MASK6,
    #[doc = "0x60 - Events 224-255 dispatch mask to peripherals"]
    pub pr_mask7: PR_MASK7,
    #[doc = "0x64 - Events 0-31 event queue overflow"]
    pub err0: ERR0,
    #[doc = "0x68 - Events 32-63 event queue overflow"]
    pub err1: ERR1,
    #[doc = "0x6c - Events 64-95 event queue overflow"]
    pub err2: ERR2,
    #[doc = "0x70 - Events 96-127 event queue overflow"]
    pub err3: ERR3,
    #[doc = "0x74 - Events 128-159 event queue overflow"]
    pub err4: ERR4,
    #[doc = "0x78 - Events 160-191 event queue overflow"]
    pub err5: ERR5,
    #[doc = "0x7c - Events 191-223 event queue overflow"]
    pub err6: ERR6,
    #[doc = "0x80 - Events 224-255 event queue overflow"]
    pub err7: ERR7,
    #[doc = "0x84 - "]
    pub timer_lo: TIMER_LO,
    #[doc = "0x88 - Trigger Timer HI of APB Timer with event"]
    pub timer_hi: TIMER_HI,
}
#[doc = "SW_EVENT (w) register accessor: an alias for `Reg<SW_EVENT_SPEC>`"]
pub type SW_EVENT = crate::Reg<sw_event::SW_EVENT_SPEC>;
#[doc = "SoC software events trigger register"]
pub mod sw_event;
#[doc = "FC_MASK0 (rw) register accessor: an alias for `Reg<FC_MASK0_SPEC>`"]
pub type FC_MASK0 = crate::Reg<fc_mask0::FC_MASK0_SPEC>;
#[doc = "Events 0-31 dispatch mask to FC"]
pub mod fc_mask0;
#[doc = "FC_MASK1 (rw) register accessor: an alias for `Reg<FC_MASK1_SPEC>`"]
pub type FC_MASK1 = crate::Reg<fc_mask1::FC_MASK1_SPEC>;
#[doc = "Events 32-63 dispatch mask to FC"]
pub mod fc_mask1;
#[doc = "FC_MASK2 (rw) register accessor: an alias for `Reg<FC_MASK2_SPEC>`"]
pub type FC_MASK2 = crate::Reg<fc_mask2::FC_MASK2_SPEC>;
#[doc = "Events 64-95 dispatch mask to FC"]
pub mod fc_mask2;
#[doc = "FC_MASK3 (rw) register accessor: an alias for `Reg<FC_MASK3_SPEC>`"]
pub type FC_MASK3 = crate::Reg<fc_mask3::FC_MASK3_SPEC>;
#[doc = "Events 96-127 dispatch mask to FC"]
pub mod fc_mask3;
#[doc = "FC_MASK4 (rw) register accessor: an alias for `Reg<FC_MASK4_SPEC>`"]
pub type FC_MASK4 = crate::Reg<fc_mask4::FC_MASK4_SPEC>;
#[doc = "Events 128-159 dispatch mask to FC"]
pub mod fc_mask4;
#[doc = "FC_MASK5 (rw) register accessor: an alias for `Reg<FC_MASK5_SPEC>`"]
pub type FC_MASK5 = crate::Reg<fc_mask5::FC_MASK5_SPEC>;
#[doc = "Events 160-191 dispatch mask to FC"]
pub mod fc_mask5;
#[doc = "FC_MASK6 (rw) register accessor: an alias for `Reg<FC_MASK6_SPEC>`"]
pub type FC_MASK6 = crate::Reg<fc_mask6::FC_MASK6_SPEC>;
#[doc = "Events 191-223 dispatch mask to FC"]
pub mod fc_mask6;
#[doc = "FC_MASK7 (rw) register accessor: an alias for `Reg<FC_MASK7_SPEC>`"]
pub type FC_MASK7 = crate::Reg<fc_mask7::FC_MASK7_SPEC>;
#[doc = "F Events 224-255 dispatch mask to peripherals"]
pub mod fc_mask7;
#[doc = "PR_MASK1 (rw) register accessor: an alias for `Reg<PR_MASK1_SPEC>`"]
pub type PR_MASK1 = crate::Reg<pr_mask1::PR_MASK1_SPEC>;
#[doc = ""]
pub mod pr_mask1;
#[doc = "PR_MASK2 (rw) register accessor: an alias for `Reg<PR_MASK2_SPEC>`"]
pub type PR_MASK2 = crate::Reg<pr_mask2::PR_MASK2_SPEC>;
#[doc = "Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals"]
pub mod pr_mask2;
#[doc = "PR_MASK3 (rw) register accessor: an alias for `Reg<PR_MASK3_SPEC>`"]
pub type PR_MASK3 = crate::Reg<pr_mask3::PR_MASK3_SPEC>;
#[doc = "Events 96-127 dispatch mask to peripherals"]
pub mod pr_mask3;
#[doc = "PR_MASK4 (rw) register accessor: an alias for `Reg<PR_MASK4_SPEC>`"]
pub type PR_MASK4 = crate::Reg<pr_mask4::PR_MASK4_SPEC>;
#[doc = "Events 128-159 dispatch mask to peripheral"]
pub mod pr_mask4;
#[doc = "PR_MASK5 (rw) register accessor: an alias for `Reg<PR_MASK5_SPEC>`"]
pub type PR_MASK5 = crate::Reg<pr_mask5::PR_MASK5_SPEC>;
#[doc = "Events 160-191 dispatch mask to peripherals"]
pub mod pr_mask5;
#[doc = "PR_MASK6 (rw) register accessor: an alias for `Reg<PR_MASK6_SPEC>`"]
pub type PR_MASK6 = crate::Reg<pr_mask6::PR_MASK6_SPEC>;
#[doc = "Events 191-223 dispatch mask to peripherals"]
pub mod pr_mask6;
#[doc = "PR_MASK7 (rw) register accessor: an alias for `Reg<PR_MASK7_SPEC>`"]
pub type PR_MASK7 = crate::Reg<pr_mask7::PR_MASK7_SPEC>;
#[doc = "Events 224-255 dispatch mask to peripherals"]
pub mod pr_mask7;
#[doc = "ERR0 (r) register accessor: an alias for `Reg<ERR0_SPEC>`"]
pub type ERR0 = crate::Reg<err0::ERR0_SPEC>;
#[doc = "Events 0-31 event queue overflow"]
pub mod err0;
#[doc = "PR_MASK0 (rw) register accessor: an alias for `Reg<PR_MASK0_SPEC>`"]
pub type PR_MASK0 = crate::Reg<pr_mask0::PR_MASK0_SPEC>;
#[doc = "Events 0-31 dispatch mask to peripherals"]
pub mod pr_mask0;
#[doc = "ERR1 (r) register accessor: an alias for `Reg<ERR1_SPEC>`"]
pub type ERR1 = crate::Reg<err1::ERR1_SPEC>;
#[doc = "Events 32-63 event queue overflow"]
pub mod err1;
#[doc = "ERR2 (r) register accessor: an alias for `Reg<ERR2_SPEC>`"]
pub type ERR2 = crate::Reg<err2::ERR2_SPEC>;
#[doc = "Events 64-95 event queue overflow"]
pub mod err2;
#[doc = "ERR3 (r) register accessor: an alias for `Reg<ERR3_SPEC>`"]
pub type ERR3 = crate::Reg<err3::ERR3_SPEC>;
#[doc = "Events 96-127 event queue overflow"]
pub mod err3;
#[doc = "ERR4 (r) register accessor: an alias for `Reg<ERR4_SPEC>`"]
pub type ERR4 = crate::Reg<err4::ERR4_SPEC>;
#[doc = "Events 128-159 event queue overflow"]
pub mod err4;
#[doc = "ERR5 (r) register accessor: an alias for `Reg<ERR5_SPEC>`"]
pub type ERR5 = crate::Reg<err5::ERR5_SPEC>;
#[doc = "Events 160-191 event queue overflow"]
pub mod err5;
#[doc = "ERR6 (r) register accessor: an alias for `Reg<ERR6_SPEC>`"]
pub type ERR6 = crate::Reg<err6::ERR6_SPEC>;
#[doc = "Events 191-223 event queue overflow"]
pub mod err6;
#[doc = "TIMER_HI (rw) register accessor: an alias for `Reg<TIMER_HI_SPEC>`"]
pub type TIMER_HI = crate::Reg<timer_hi::TIMER_HI_SPEC>;
#[doc = "Trigger Timer HI of APB Timer with event"]
pub mod timer_hi;
#[doc = "ERR7 (r) register accessor: an alias for `Reg<ERR7_SPEC>`"]
pub type ERR7 = crate::Reg<err7::ERR7_SPEC>;
#[doc = "Events 224-255 event queue overflow"]
pub mod err7;
#[doc = "TIMER_LO (rw) register accessor: an alias for `Reg<TIMER_LO_SPEC>`"]
pub type TIMER_LO = crate::Reg<timer_lo::TIMER_LO_SPEC>;
#[doc = ""]
pub mod timer_lo;
