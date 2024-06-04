#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Apb_software_irq"]
#[doc(alias = "Apb_software_irq")]
pub struct APB_SOFTWARE_IRQ {
    read: READ,
    set: SET,
    clear: CLEAR,
}
impl APB_SOFTWARE_IRQ {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn read(&self) -> &READ {
        &self.read
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn set(&self) -> &SET {
        &self.set
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn clear(&self) -> &CLEAR {
        &self.clear
    }
}
#[doc = "read (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read`]
module"]
#[doc(alias = "read")]
pub type READ = crate::Reg<read::READ_SPEC>;
#[doc = ""]
pub mod read;
#[doc = "set (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
#[doc(alias = "set")]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = ""]
pub mod set;
#[doc = "clear (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`]
module"]
#[doc(alias = "clear")]
pub type CLEAR = crate::Reg<clear::CLEAR_SPEC>;
#[doc = ""]
pub mod clear;
