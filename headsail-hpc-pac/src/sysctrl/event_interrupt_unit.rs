#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "EventInterruptUnit"]
pub struct EventInterruptUnit {
    mask_read: MaskRead,
    mask_set: MaskSet,
    mask_clear: MaskClear,
    int_read: IntRead,
    int_set: IntSet,
    int_clear: IntClear,
    ack_read: AckRead,
    ack_set: AckSet,
    ack_clear: AckClear,
    fifo_data: FifoData,
}
impl EventInterruptUnit {
    #[doc = "0x00 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    #[inline(always)]
    pub const fn mask_read(&self) -> &MaskRead {
        &self.mask_read
    }
    #[doc = "0x04 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    #[inline(always)]
    pub const fn mask_set(&self) -> &MaskSet {
        &self.mask_set
    }
    #[doc = "0x08 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    #[inline(always)]
    pub const fn mask_clear(&self) -> &MaskClear {
        &self.mask_clear
    }
    #[doc = "0x0c - This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected."]
    #[inline(always)]
    pub const fn int_read(&self) -> &IntRead {
        &self.int_read
    }
    #[doc = "0x10 - INT_read"]
    #[inline(always)]
    pub const fn int_set(&self) -> &IntSet {
        &self.int_set
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn int_clear(&self) -> &IntClear {
        &self.int_clear
    }
    #[doc = "0x18 - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    #[inline(always)]
    pub const fn ack_read(&self) -> &AckRead {
        &self.ack_read
    }
    #[doc = "0x1c - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    #[inline(always)]
    pub const fn ack_set(&self) -> &AckSet {
        &self.ack_set
    }
    #[doc = "0x20 - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    #[inline(always)]
    pub const fn ack_clear(&self) -> &AckClear {
        &self.ack_clear
    }
    #[doc = "0x24 - Fifo Content. This is a read-only register that contain the first valid value of the FIFO."]
    #[inline(always)]
    pub const fn fifo_data(&self) -> &FifoData {
        &self.fifo_data
    }
}
#[doc = "MASK_read (rw) register accessor: This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_read`]
module"]
#[doc(alias = "MASK_read")]
pub type MaskRead = crate::Reg<mask_read::MaskReadSpec>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_read;
#[doc = "MASK_set (rw) register accessor: This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_set`]
module"]
#[doc(alias = "MASK_set")]
pub type MaskSet = crate::Reg<mask_set::MaskSetSpec>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_set;
#[doc = "MASK_clear (rw) register accessor: This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_clear`]
module"]
#[doc(alias = "MASK_clear")]
pub type MaskClear = crate::Reg<mask_clear::MaskClearSpec>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_clear;
#[doc = "INT_read (rw) register accessor: This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_read`]
module"]
#[doc(alias = "INT_read")]
pub type IntRead = crate::Reg<int_read::IntReadSpec>;
#[doc = "This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected."]
pub mod int_read;
#[doc = "INT_set (rw) register accessor: INT_read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_set`]
module"]
#[doc(alias = "INT_set")]
pub type IntSet = crate::Reg<int_set::IntSetSpec>;
#[doc = "INT_read"]
pub mod int_set;
#[doc = "INT_clear (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`]
module"]
#[doc(alias = "INT_clear")]
pub type IntClear = crate::Reg<int_clear::IntClearSpec>;
#[doc = ""]
pub mod int_clear;
#[doc = "ACK_read (rw) register accessor: This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_read`]
module"]
#[doc(alias = "ACK_read")]
pub type AckRead = crate::Reg<ack_read::AckReadSpec>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_read;
#[doc = "ACK_set (rw) register accessor: This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_set`]
module"]
#[doc(alias = "ACK_set")]
pub type AckSet = crate::Reg<ack_set::AckSetSpec>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_set;
#[doc = "ACK_clear (rw) register accessor: This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_clear`]
module"]
#[doc(alias = "ACK_clear")]
pub type AckClear = crate::Reg<ack_clear::AckClearSpec>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_clear;
#[doc = "FIFO_DATA (r) register accessor: Fifo Content. This is a read-only register that contain the first valid value of the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data`]
module"]
#[doc(alias = "FIFO_DATA")]
pub type FifoData = crate::Reg<fifo_data::FifoDataSpec>;
#[doc = "Fifo Content. This is a read-only register that contain the first valid value of the FIFO."]
pub mod fifo_data;
