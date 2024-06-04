#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "EventInterruptUnit"]
#[doc(alias = "EventInterruptUnit")]
pub struct EVENT_INTERRUPT_UNIT {
    mask_read: MASK_READ,
    mask_set: MASK_SET,
    mask_clear: MASK_CLEAR,
    int_read: INT_READ,
    int_set: INT_SET,
    int_clear: INT_CLEAR,
    ack_read: ACK_READ,
    ack_set: ACK_SET,
    ack_clear: ACK_CLEAR,
    fifo_data: FIFO_DATA,
}
impl EVENT_INTERRUPT_UNIT {
    #[doc = "0x00 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    #[inline(always)]
    pub const fn mask_read(&self) -> &MASK_READ {
        &self.mask_read
    }
    #[doc = "0x04 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    #[inline(always)]
    pub const fn mask_set(&self) -> &MASK_SET {
        &self.mask_set
    }
    #[doc = "0x08 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    #[inline(always)]
    pub const fn mask_clear(&self) -> &MASK_CLEAR {
        &self.mask_clear
    }
    #[doc = "0x0c - This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected."]
    #[inline(always)]
    pub const fn int_read(&self) -> &INT_READ {
        &self.int_read
    }
    #[doc = "0x10 - INT_read"]
    #[inline(always)]
    pub const fn int_set(&self) -> &INT_SET {
        &self.int_set
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn int_clear(&self) -> &INT_CLEAR {
        &self.int_clear
    }
    #[doc = "0x18 - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    #[inline(always)]
    pub const fn ack_read(&self) -> &ACK_READ {
        &self.ack_read
    }
    #[doc = "0x1c - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    #[inline(always)]
    pub const fn ack_set(&self) -> &ACK_SET {
        &self.ack_set
    }
    #[doc = "0x20 - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    #[inline(always)]
    pub const fn ack_clear(&self) -> &ACK_CLEAR {
        &self.ack_clear
    }
    #[doc = "0x24 - Fifo Content. This is a read-only register that contain the first valid value of the FIFO."]
    #[inline(always)]
    pub const fn fifo_data(&self) -> &FIFO_DATA {
        &self.fifo_data
    }
}
#[doc = "MASK_read (rw) register accessor: This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_read`]
module"]
#[doc(alias = "MASK_read")]
pub type MASK_READ = crate::Reg<mask_read::MASK_READ_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_read;
#[doc = "MASK_set (rw) register accessor: This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_set`]
module"]
#[doc(alias = "MASK_set")]
pub type MASK_SET = crate::Reg<mask_set::MASK_SET_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_set;
#[doc = "MASK_clear (rw) register accessor: This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_clear`]
module"]
#[doc(alias = "MASK_clear")]
pub type MASK_CLEAR = crate::Reg<mask_clear::MASK_CLEAR_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_clear;
#[doc = "INT_read (rw) register accessor: This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_read`]
module"]
#[doc(alias = "INT_read")]
pub type INT_READ = crate::Reg<int_read::INT_READ_SPEC>;
#[doc = "This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected."]
pub mod int_read;
#[doc = "INT_set (rw) register accessor: INT_read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_set`]
module"]
#[doc(alias = "INT_set")]
pub type INT_SET = crate::Reg<int_set::INT_SET_SPEC>;
#[doc = "INT_read"]
pub mod int_set;
#[doc = "INT_clear (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`]
module"]
#[doc(alias = "INT_clear")]
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEAR_SPEC>;
#[doc = ""]
pub mod int_clear;
#[doc = "ACK_read (rw) register accessor: This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_read`]
module"]
#[doc(alias = "ACK_read")]
pub type ACK_READ = crate::Reg<ack_read::ACK_READ_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_read;
#[doc = "ACK_set (rw) register accessor: This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_set`]
module"]
#[doc(alias = "ACK_set")]
pub type ACK_SET = crate::Reg<ack_set::ACK_SET_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_set;
#[doc = "ACK_clear (rw) register accessor: This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_clear`]
module"]
#[doc(alias = "ACK_clear")]
pub type ACK_CLEAR = crate::Reg<ack_clear::ACK_CLEAR_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_clear;
#[doc = "FIFO_DATA (r) register accessor: Fifo Content. This is a read-only register that contain the first valid value of the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data`]
module"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "Fifo Content. This is a read-only register that contain the first valid value of the FIFO."]
pub mod fifo_data;
