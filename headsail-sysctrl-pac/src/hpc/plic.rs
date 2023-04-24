#[doc = r"Register block"]
#[repr(C)]
pub struct PLIC {
    #[doc = "0x00 - Reserved"]
    pub base: BASE,
    #[doc = "0x04..0x0c - Interrupt priority for timer 0 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    pub timer0_prio: [TIMER0_PRIO; 2],
    #[doc = "0x0c..0x14 - Interrupt priority for timer 1 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    pub timer1_prio: [TIMER1_PRIO; 2],
    #[doc = "0x14..0x1c - Interrupt priority for timer 2 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    pub timer2_prio: [TIMER2_PRIO; 2],
    #[doc = "0x1c..0x24 - Interrupt priority for timer 3 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    pub timer3_prio: [TIMER3_PRIO; 2],
    #[doc = "0x24..0x80 - Interrupt priority for external interrupt \\[%s\\]"]
    pub ext_prio: [EXT_PRIO; 23],
    _reserved6: [u8; 0x1f80],
    #[doc = "0x2000..0x2020 - IRQ enable bits for Hart context #\\[%s\\]. `idx / 2` is core-#. Even indices are M-mode, odd indices are S-mode."]
    pub irq_enable: [IRQ_ENABLE; 8],
    _reserved7: [u8; 0x1fe0],
    #[doc = "0x4000 - Priority threshold for Hart 0 M-Mode (context #0) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    pub prio_thr_0: PRIO_THR_0,
    #[doc = "0x4004 - Claim/complete for Hart 0 M-mode (context #0) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    pub claim_complete_0: CLAIM_COMPLETE_0,
    _reserved9: [u8; 0x0ff8],
    #[doc = "0x5000 - Priority threshold for Hart 0 S-Mode (context #1) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    pub prio_thr_1: PRIO_THR_1,
    #[doc = "0x5004 - Claim/complete for Hart 0 S-mode (context #1) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    pub claim_complete_1: CLAIM_COMPLETE_1,
    _reserved11: [u8; 0x0ff8],
    #[doc = "0x6000 - Priority threshold for Hart 1 M-Mode (context #2) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    pub prio_thr_2: PRIO_THR_2,
    #[doc = "0x6004 - Claim/complete for Hart 1 M-mode (context #2) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    pub claim_complete_2: CLAIM_COMPLETE_2,
    _reserved13: [u8; 0x0ff8],
    #[doc = "0x7000 - Priority threshold for Hart 1 S-Mode (context #3) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    pub prio_thr_3: PRIO_THR_3,
    #[doc = "0x7004 - Claim/complete for Hart 1 S-mode (context #3) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    pub claim_complete_3: CLAIM_COMPLETE_3,
    _reserved15: [u8; 0x0ff8],
    #[doc = "0x8000 - Priority threshold for Hart 2 M-Mode (context #4) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    pub prio_thr_4: PRIO_THR_4,
    #[doc = "0x8004 - Claim/complete for Hart 2 M-mode (context #4) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    pub claim_complete_4: CLAIM_COMPLETE_4,
    _reserved17: [u8; 0x0ff8],
    #[doc = "0x9000 - Priority threshold for Hart 2 S-Mode (context #5) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    pub prio_thr_5: PRIO_THR_5,
    #[doc = "0x9004 - Claim/complete for Hart 2 S-Mode (context #5) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    pub claim_complete_5: CLAIM_COMPLETE_5,
    _reserved19: [u8; 0x0ff8],
    #[doc = "0xa000 - Priority threshold for Hart 3 M-Mode (context #6) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    pub prio_thr_6: PRIO_THR_6,
    #[doc = "0xa004 - Claim/complete for Hart 3 M-mode (context #6) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    pub claim_complete_6: CLAIM_COMPLETE_6,
    _reserved21: [u8; 0x0ff8],
    #[doc = "0xb000 - Priority threshold for Hart 3 S-Mode (context #7) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    pub prio_thr_7: PRIO_THR_7,
    #[doc = "0xb004 - Claim/complete for Hart 3 S-mode (context #7) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    pub claim_complete_7: CLAIM_COMPLETE_7,
}
#[doc = "base (r) register accessor: an alias for `Reg<BASE_SPEC>`"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "Reserved"]
pub mod base;
#[doc = "claim_complete_7 (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_7_SPEC>`"]
pub type CLAIM_COMPLETE_7 = crate::Reg<claim_complete_7::CLAIM_COMPLETE_7_SPEC>;
#[doc = "Claim/complete for Hart 3 S-mode (context #7) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_7;
#[doc = "prio_thr_7 (rw) register accessor: an alias for `Reg<PRIO_THR_7_SPEC>`"]
pub type PRIO_THR_7 = crate::Reg<prio_thr_7::PRIO_THR_7_SPEC>;
#[doc = "Priority threshold for Hart 3 S-Mode (context #7) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_7;
#[doc = "claim_complete_6 (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_6_SPEC>`"]
pub type CLAIM_COMPLETE_6 = crate::Reg<claim_complete_6::CLAIM_COMPLETE_6_SPEC>;
#[doc = "Claim/complete for Hart 3 M-mode (context #6) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_6;
#[doc = "prio_thr_6 (rw) register accessor: an alias for `Reg<PRIO_THR_6_SPEC>`"]
pub type PRIO_THR_6 = crate::Reg<prio_thr_6::PRIO_THR_6_SPEC>;
#[doc = "Priority threshold for Hart 3 M-Mode (context #6) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_6;
#[doc = "claim_complete_5 (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_5_SPEC>`"]
pub type CLAIM_COMPLETE_5 = crate::Reg<claim_complete_5::CLAIM_COMPLETE_5_SPEC>;
#[doc = "Claim/complete for Hart 2 S-Mode (context #5) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_5;
#[doc = "prio_thr_5 (rw) register accessor: an alias for `Reg<PRIO_THR_5_SPEC>`"]
pub type PRIO_THR_5 = crate::Reg<prio_thr_5::PRIO_THR_5_SPEC>;
#[doc = "Priority threshold for Hart 2 S-Mode (context #5) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_5;
#[doc = "claim_complete_4 (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_4_SPEC>`"]
pub type CLAIM_COMPLETE_4 = crate::Reg<claim_complete_4::CLAIM_COMPLETE_4_SPEC>;
#[doc = "Claim/complete for Hart 2 M-mode (context #4) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_4;
#[doc = "prio_thr_4 (rw) register accessor: an alias for `Reg<PRIO_THR_4_SPEC>`"]
pub type PRIO_THR_4 = crate::Reg<prio_thr_4::PRIO_THR_4_SPEC>;
#[doc = "Priority threshold for Hart 2 M-Mode (context #4) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_4;
#[doc = "claim_complete_3 (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_3_SPEC>`"]
pub type CLAIM_COMPLETE_3 = crate::Reg<claim_complete_3::CLAIM_COMPLETE_3_SPEC>;
#[doc = "Claim/complete for Hart 1 S-mode (context #3) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_3;
#[doc = "prio_thr_3 (rw) register accessor: an alias for `Reg<PRIO_THR_3_SPEC>`"]
pub type PRIO_THR_3 = crate::Reg<prio_thr_3::PRIO_THR_3_SPEC>;
#[doc = "Priority threshold for Hart 1 S-Mode (context #3) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_3;
#[doc = "claim_complete_2 (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_2_SPEC>`"]
pub type CLAIM_COMPLETE_2 = crate::Reg<claim_complete_2::CLAIM_COMPLETE_2_SPEC>;
#[doc = "Claim/complete for Hart 1 M-mode (context #2) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_2;
#[doc = "prio_thr_2 (rw) register accessor: an alias for `Reg<PRIO_THR_2_SPEC>`"]
pub type PRIO_THR_2 = crate::Reg<prio_thr_2::PRIO_THR_2_SPEC>;
#[doc = "Priority threshold for Hart 1 M-Mode (context #2) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_2;
#[doc = "claim_complete_1 (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_1_SPEC>`"]
pub type CLAIM_COMPLETE_1 = crate::Reg<claim_complete_1::CLAIM_COMPLETE_1_SPEC>;
#[doc = "Claim/complete for Hart 0 S-mode (context #1) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_1;
#[doc = "prio_thr_1 (rw) register accessor: an alias for `Reg<PRIO_THR_1_SPEC>`"]
pub type PRIO_THR_1 = crate::Reg<prio_thr_1::PRIO_THR_1_SPEC>;
#[doc = "Priority threshold for Hart 0 S-Mode (context #1) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_1;
#[doc = "claim_complete_0 (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_0_SPEC>`"]
pub type CLAIM_COMPLETE_0 = crate::Reg<claim_complete_0::CLAIM_COMPLETE_0_SPEC>;
#[doc = "Claim/complete for Hart 0 M-mode (context #0) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_0;
#[doc = "prio_thr_0 (rw) register accessor: an alias for `Reg<PRIO_THR_0_SPEC>`"]
pub type PRIO_THR_0 = crate::Reg<prio_thr_0::PRIO_THR_0_SPEC>;
#[doc = "Priority threshold for Hart 0 M-Mode (context #0) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_0;
#[doc = "irq_enable (rw) register accessor: an alias for `Reg<IRQ_ENABLE_SPEC>`"]
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLE_SPEC>;
#[doc = "IRQ enable bits for Hart context #\\[%s\\]. `idx / 2` is core-#. Even indices are M-mode, odd indices are S-mode."]
pub mod irq_enable;
#[doc = "ext_prio (rw) register accessor: an alias for `Reg<EXT_PRIO_SPEC>`"]
pub type EXT_PRIO = crate::Reg<ext_prio::EXT_PRIO_SPEC>;
#[doc = "Interrupt priority for external interrupt \\[%s\\]"]
pub mod ext_prio;
#[doc = "timer3_prio (rw) register accessor: an alias for `Reg<TIMER3_PRIO_SPEC>`"]
pub type TIMER3_PRIO = crate::Reg<timer3_prio::TIMER3_PRIO_SPEC>;
#[doc = "Interrupt priority for timer 3 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
pub mod timer3_prio;
#[doc = "timer2_prio (rw) register accessor: an alias for `Reg<TIMER2_PRIO_SPEC>`"]
pub type TIMER2_PRIO = crate::Reg<timer2_prio::TIMER2_PRIO_SPEC>;
#[doc = "Interrupt priority for timer 2 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
pub mod timer2_prio;
#[doc = "timer1_prio (rw) register accessor: an alias for `Reg<TIMER1_PRIO_SPEC>`"]
pub type TIMER1_PRIO = crate::Reg<timer1_prio::TIMER1_PRIO_SPEC>;
#[doc = "Interrupt priority for timer 1 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
pub mod timer1_prio;
#[doc = "timer0_prio (rw) register accessor: an alias for `Reg<TIMER0_PRIO_SPEC>`"]
pub type TIMER0_PRIO = crate::Reg<timer0_prio::TIMER0_PRIO_SPEC>;
#[doc = "Interrupt priority for timer 0 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
pub mod timer0_prio;
