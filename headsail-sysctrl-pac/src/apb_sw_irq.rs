#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    read: Read,
    set: Set,
    clear: Clear,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn read(&self) -> &Read {
        &self.read
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn set(&self) -> &Set {
        &self.set
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
}
#[doc = "read (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read`]
module"]
#[doc(alias = "read")]
pub type Read = crate::Reg<read::ReadSpec>;
#[doc = ""]
pub mod read;
#[doc = "set (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
#[doc(alias = "set")]
pub type Set = crate::Reg<set::SetSpec>;
#[doc = ""]
pub mod set;
#[doc = "clear (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`]
module"]
#[doc(alias = "clear")]
pub type Clear = crate::Reg<clear::ClearSpec>;
#[doc = ""]
pub mod clear;
