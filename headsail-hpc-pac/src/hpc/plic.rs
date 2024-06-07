#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "plic"]
#[doc(alias = "plic")]
pub struct Plic {
    base: Base,
    timer0_prio: [Timer0Prio; 2],
    timer1_prio: [Timer1Prio; 2],
    timer2_prio: [Timer2Prio; 2],
    timer3_prio: [Timer3Prio; 2],
    ext_prio: [ExtPrio; 23],
    _reserved6: [u8; 0x1f80],
    irq_enable: [IrqEnable; 8],
    _reserved7: [u8; 0x1fe0],
    prio_thr_0: PrioThr0,
    claim_complete_0: ClaimComplete0,
    _reserved9: [u8; 0x0ff8],
    prio_thr_1: PrioThr1,
    claim_complete_1: ClaimComplete1,
    _reserved11: [u8; 0x0ff8],
    prio_thr_2: PrioThr2,
    claim_complete_2: ClaimComplete2,
    _reserved13: [u8; 0x0ff8],
    prio_thr_3: PrioThr3,
    claim_complete_3: ClaimComplete3,
    _reserved15: [u8; 0x0ff8],
    prio_thr_4: PrioThr4,
    claim_complete_4: ClaimComplete4,
    _reserved17: [u8; 0x0ff8],
    prio_thr_5: PrioThr5,
    claim_complete_5: ClaimComplete5,
    _reserved19: [u8; 0x0ff8],
    prio_thr_6: PrioThr6,
    claim_complete_6: ClaimComplete6,
    _reserved21: [u8; 0x0ff8],
    prio_thr_7: PrioThr7,
    claim_complete_7: ClaimComplete7,
}
impl Plic {
    #[doc = "0x00 - Reserved"]
    #[inline(always)]
    pub const fn base(&self) -> &Base {
        &self.base
    }
    #[doc = "0x04..0x0c - Interrupt priority for timer 0 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    #[inline(always)]
    pub const fn timer0_prio(&self, n: usize) -> &Timer0Prio {
        &self.timer0_prio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Interrupt priority for timer 0 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    #[inline(always)]
    pub fn timer0_prio_iter(&self) -> impl Iterator<Item = &Timer0Prio> {
        self.timer0_prio.iter()
    }
    #[doc = "0x0c..0x14 - Interrupt priority for timer 1 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    #[inline(always)]
    pub const fn timer1_prio(&self, n: usize) -> &Timer1Prio {
        &self.timer1_prio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x14 - Interrupt priority for timer 1 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    #[inline(always)]
    pub fn timer1_prio_iter(&self) -> impl Iterator<Item = &Timer1Prio> {
        self.timer1_prio.iter()
    }
    #[doc = "0x14..0x1c - Interrupt priority for timer 2 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    #[inline(always)]
    pub const fn timer2_prio(&self, n: usize) -> &Timer2Prio {
        &self.timer2_prio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x1c - Interrupt priority for timer 2 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    #[inline(always)]
    pub fn timer2_prio_iter(&self) -> impl Iterator<Item = &Timer2Prio> {
        self.timer2_prio.iter()
    }
    #[doc = "0x1c..0x24 - Interrupt priority for timer 3 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    #[inline(always)]
    pub const fn timer3_prio(&self, n: usize) -> &Timer3Prio {
        &self.timer3_prio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - Interrupt priority for timer 3 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
    #[inline(always)]
    pub fn timer3_prio_iter(&self) -> impl Iterator<Item = &Timer3Prio> {
        self.timer3_prio.iter()
    }
    #[doc = "0x24..0x80 - Interrupt priority for external interrupt \\[%s\\]"]
    #[inline(always)]
    pub const fn ext_prio(&self, n: usize) -> &ExtPrio {
        &self.ext_prio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x80 - Interrupt priority for external interrupt \\[%s\\]"]
    #[inline(always)]
    pub fn ext_prio_iter(&self) -> impl Iterator<Item = &ExtPrio> {
        self.ext_prio.iter()
    }
    #[doc = "0x2000..0x2020 - IRQ enable bits for Hart context #\\[%s\\]. `idx / 2` is core-#. Even indices are M-mode, odd indices are S-mode."]
    #[inline(always)]
    pub const fn irq_enable(&self, n: usize) -> &IrqEnable {
        &self.irq_enable[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2020 - IRQ enable bits for Hart context #\\[%s\\]. `idx / 2` is core-#. Even indices are M-mode, odd indices are S-mode."]
    #[inline(always)]
    pub fn irq_enable_iter(&self) -> impl Iterator<Item = &IrqEnable> {
        self.irq_enable.iter()
    }
    #[doc = "0x4000 - Priority threshold for Hart 0 M-Mode (context #0) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    #[inline(always)]
    pub const fn prio_thr_0(&self) -> &PrioThr0 {
        &self.prio_thr_0
    }
    #[doc = "0x4004 - Claim/complete for Hart 0 M-mode (context #0) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    #[inline(always)]
    pub const fn claim_complete_0(&self) -> &ClaimComplete0 {
        &self.claim_complete_0
    }
    #[doc = "0x5000 - Priority threshold for Hart 0 S-Mode (context #1) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    #[inline(always)]
    pub const fn prio_thr_1(&self) -> &PrioThr1 {
        &self.prio_thr_1
    }
    #[doc = "0x5004 - Claim/complete for Hart 0 S-mode (context #1) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    #[inline(always)]
    pub const fn claim_complete_1(&self) -> &ClaimComplete1 {
        &self.claim_complete_1
    }
    #[doc = "0x6000 - Priority threshold for Hart 1 M-Mode (context #2) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    #[inline(always)]
    pub const fn prio_thr_2(&self) -> &PrioThr2 {
        &self.prio_thr_2
    }
    #[doc = "0x6004 - Claim/complete for Hart 1 M-mode (context #2) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    #[inline(always)]
    pub const fn claim_complete_2(&self) -> &ClaimComplete2 {
        &self.claim_complete_2
    }
    #[doc = "0x7000 - Priority threshold for Hart 1 S-Mode (context #3) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    #[inline(always)]
    pub const fn prio_thr_3(&self) -> &PrioThr3 {
        &self.prio_thr_3
    }
    #[doc = "0x7004 - Claim/complete for Hart 1 S-mode (context #3) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    #[inline(always)]
    pub const fn claim_complete_3(&self) -> &ClaimComplete3 {
        &self.claim_complete_3
    }
    #[doc = "0x8000 - Priority threshold for Hart 2 M-Mode (context #4) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    #[inline(always)]
    pub const fn prio_thr_4(&self) -> &PrioThr4 {
        &self.prio_thr_4
    }
    #[doc = "0x8004 - Claim/complete for Hart 2 M-mode (context #4) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    #[inline(always)]
    pub const fn claim_complete_4(&self) -> &ClaimComplete4 {
        &self.claim_complete_4
    }
    #[doc = "0x9000 - Priority threshold for Hart 2 S-Mode (context #5) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    #[inline(always)]
    pub const fn prio_thr_5(&self) -> &PrioThr5 {
        &self.prio_thr_5
    }
    #[doc = "0x9004 - Claim/complete for Hart 2 S-Mode (context #5) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    #[inline(always)]
    pub const fn claim_complete_5(&self) -> &ClaimComplete5 {
        &self.claim_complete_5
    }
    #[doc = "0xa000 - Priority threshold for Hart 3 M-Mode (context #6) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    #[inline(always)]
    pub const fn prio_thr_6(&self) -> &PrioThr6 {
        &self.prio_thr_6
    }
    #[doc = "0xa004 - Claim/complete for Hart 3 M-mode (context #6) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    #[inline(always)]
    pub const fn claim_complete_6(&self) -> &ClaimComplete6 {
        &self.claim_complete_6
    }
    #[doc = "0xb000 - Priority threshold for Hart 3 S-Mode (context #7) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
    #[inline(always)]
    pub const fn prio_thr_7(&self) -> &PrioThr7 {
        &self.prio_thr_7
    }
    #[doc = "0xb004 - Claim/complete for Hart 3 S-mode (context #7) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
    #[inline(always)]
    pub const fn claim_complete_7(&self) -> &ClaimComplete7 {
        &self.claim_complete_7
    }
}
#[doc = "base (r) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base`]
module"]
#[doc(alias = "base")]
pub type Base = crate::Reg<base::BaseSpec>;
#[doc = "Reserved"]
pub mod base;
#[doc = "claim_complete_7 (rw) register accessor: Claim/complete for Hart 3 S-mode (context #7) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete_7`]
module"]
#[doc(alias = "claim_complete_7")]
pub type ClaimComplete7 = crate::Reg<claim_complete_7::ClaimComplete7Spec>;
#[doc = "Claim/complete for Hart 3 S-mode (context #7) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_7;
#[doc = "prio_thr_7 (rw) register accessor: Priority threshold for Hart 3 S-Mode (context #7) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio_thr_7`]
module"]
#[doc(alias = "prio_thr_7")]
pub type PrioThr7 = crate::Reg<prio_thr_7::PrioThr7Spec>;
#[doc = "Priority threshold for Hart 3 S-Mode (context #7) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_7;
#[doc = "claim_complete_6 (rw) register accessor: Claim/complete for Hart 3 M-mode (context #6) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete_6`]
module"]
#[doc(alias = "claim_complete_6")]
pub type ClaimComplete6 = crate::Reg<claim_complete_6::ClaimComplete6Spec>;
#[doc = "Claim/complete for Hart 3 M-mode (context #6) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_6;
#[doc = "prio_thr_6 (rw) register accessor: Priority threshold for Hart 3 M-Mode (context #6) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio_thr_6`]
module"]
#[doc(alias = "prio_thr_6")]
pub type PrioThr6 = crate::Reg<prio_thr_6::PrioThr6Spec>;
#[doc = "Priority threshold for Hart 3 M-Mode (context #6) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_6;
#[doc = "claim_complete_5 (rw) register accessor: Claim/complete for Hart 2 S-Mode (context #5) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete_5`]
module"]
#[doc(alias = "claim_complete_5")]
pub type ClaimComplete5 = crate::Reg<claim_complete_5::ClaimComplete5Spec>;
#[doc = "Claim/complete for Hart 2 S-Mode (context #5) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_5;
#[doc = "prio_thr_5 (rw) register accessor: Priority threshold for Hart 2 S-Mode (context #5) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio_thr_5`]
module"]
#[doc(alias = "prio_thr_5")]
pub type PrioThr5 = crate::Reg<prio_thr_5::PrioThr5Spec>;
#[doc = "Priority threshold for Hart 2 S-Mode (context #5) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_5;
#[doc = "claim_complete_4 (rw) register accessor: Claim/complete for Hart 2 M-mode (context #4) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete_4`]
module"]
#[doc(alias = "claim_complete_4")]
pub type ClaimComplete4 = crate::Reg<claim_complete_4::ClaimComplete4Spec>;
#[doc = "Claim/complete for Hart 2 M-mode (context #4) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_4;
#[doc = "prio_thr_4 (rw) register accessor: Priority threshold for Hart 2 M-Mode (context #4) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio_thr_4`]
module"]
#[doc(alias = "prio_thr_4")]
pub type PrioThr4 = crate::Reg<prio_thr_4::PrioThr4Spec>;
#[doc = "Priority threshold for Hart 2 M-Mode (context #4) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_4;
#[doc = "claim_complete_3 (rw) register accessor: Claim/complete for Hart 1 S-mode (context #3) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete_3`]
module"]
#[doc(alias = "claim_complete_3")]
pub type ClaimComplete3 = crate::Reg<claim_complete_3::ClaimComplete3Spec>;
#[doc = "Claim/complete for Hart 1 S-mode (context #3) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_3;
#[doc = "prio_thr_3 (rw) register accessor: Priority threshold for Hart 1 S-Mode (context #3) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio_thr_3`]
module"]
#[doc(alias = "prio_thr_3")]
pub type PrioThr3 = crate::Reg<prio_thr_3::PrioThr3Spec>;
#[doc = "Priority threshold for Hart 1 S-Mode (context #3) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_3;
#[doc = "claim_complete_2 (rw) register accessor: Claim/complete for Hart 1 M-mode (context #2) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete_2`]
module"]
#[doc(alias = "claim_complete_2")]
pub type ClaimComplete2 = crate::Reg<claim_complete_2::ClaimComplete2Spec>;
#[doc = "Claim/complete for Hart 1 M-mode (context #2) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_2;
#[doc = "prio_thr_2 (rw) register accessor: Priority threshold for Hart 1 M-Mode (context #2) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio_thr_2`]
module"]
#[doc(alias = "prio_thr_2")]
pub type PrioThr2 = crate::Reg<prio_thr_2::PrioThr2Spec>;
#[doc = "Priority threshold for Hart 1 M-Mode (context #2) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_2;
#[doc = "claim_complete_1 (rw) register accessor: Claim/complete for Hart 0 S-mode (context #1) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete_1`]
module"]
#[doc(alias = "claim_complete_1")]
pub type ClaimComplete1 = crate::Reg<claim_complete_1::ClaimComplete1Spec>;
#[doc = "Claim/complete for Hart 0 S-mode (context #1) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_1;
#[doc = "prio_thr_1 (rw) register accessor: Priority threshold for Hart 0 S-Mode (context #1) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio_thr_1`]
module"]
#[doc(alias = "prio_thr_1")]
pub type PrioThr1 = crate::Reg<prio_thr_1::PrioThr1Spec>;
#[doc = "Priority threshold for Hart 0 S-Mode (context #1) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_1;
#[doc = "claim_complete_0 (rw) register accessor: Claim/complete for Hart 0 M-mode (context #0) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim_complete_0`]
module"]
#[doc(alias = "claim_complete_0")]
pub type ClaimComplete0 = crate::Reg<claim_complete_0::ClaimComplete0Spec>;
#[doc = "Claim/complete for Hart 0 M-mode (context #0) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored."]
pub mod claim_complete_0;
#[doc = "prio_thr_0 (rw) register accessor: Priority threshold for Hart 0 M-Mode (context #0) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prio_thr_0`]
module"]
#[doc(alias = "prio_thr_0")]
pub type PrioThr0 = crate::Reg<prio_thr_0::PrioThr0Spec>;
#[doc = "Priority threshold for Hart 0 M-Mode (context #0) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts."]
pub mod prio_thr_0;
#[doc = "irq_enable (rw) register accessor: IRQ enable bits for Hart context #\\[%s\\]. `idx / 2` is core-#. Even indices are M-mode, odd indices are S-mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enable`]
module"]
#[doc(alias = "irq_enable")]
pub type IrqEnable = crate::Reg<irq_enable::IrqEnableSpec>;
#[doc = "IRQ enable bits for Hart context #\\[%s\\]. `idx / 2` is core-#. Even indices are M-mode, odd indices are S-mode."]
pub mod irq_enable;
#[doc = "ext_prio (rw) register accessor: Interrupt priority for external interrupt \\[%s\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_prio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_prio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_prio`]
module"]
#[doc(alias = "ext_prio")]
pub type ExtPrio = crate::Reg<ext_prio::ExtPrioSpec>;
#[doc = "Interrupt priority for external interrupt \\[%s\\]"]
pub mod ext_prio;
#[doc = "timer3_prio (rw) register accessor: Interrupt priority for timer 3 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_prio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3_prio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_prio`]
module"]
#[doc(alias = "timer3_prio")]
pub type Timer3Prio = crate::Reg<timer3_prio::Timer3PrioSpec>;
#[doc = "Interrupt priority for timer 3 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
pub mod timer3_prio;
#[doc = "timer2_prio (rw) register accessor: Interrupt priority for timer 2 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_prio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_prio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_prio`]
module"]
#[doc(alias = "timer2_prio")]
pub type Timer2Prio = crate::Reg<timer2_prio::Timer2PrioSpec>;
#[doc = "Interrupt priority for timer 2 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
pub mod timer2_prio;
#[doc = "timer1_prio (rw) register accessor: Interrupt priority for timer 1 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_prio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_prio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_prio`]
module"]
#[doc(alias = "timer1_prio")]
pub type Timer1Prio = crate::Reg<timer1_prio::Timer1PrioSpec>;
#[doc = "Interrupt priority for timer 1 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
pub mod timer1_prio;
#[doc = "timer0_prio (rw) register accessor: Interrupt priority for timer 0 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_prio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_prio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_prio`]
module"]
#[doc(alias = "timer0_prio")]
pub type Timer0Prio = crate::Reg<timer0_prio::Timer0PrioSpec>;
#[doc = "Interrupt priority for timer 0 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7."]
pub mod timer0_prio;
