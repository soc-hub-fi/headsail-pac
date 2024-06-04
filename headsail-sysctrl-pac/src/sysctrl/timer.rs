#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Timer"]
#[doc(alias = "Timer")]
pub struct TIMER {
    cfg_lo: CFG_LO,
    cfg_hi: CFG_HI,
    cnt_lo: CNT_LO,
    cnt_hi: CNT_HI,
    cmp_lo: CMP_LO,
    cmp_hi: CMP_HI,
    start_lo: START_LO,
    start_hi: START_HI,
    reset_lo: RESET_LO,
    reset_hi: RESET_HI,
}
impl TIMER {
    #[doc = "0x00 - Timer Low Configuration register"]
    #[inline(always)]
    pub const fn cfg_lo(&self) -> &CFG_LO {
        &self.cfg_lo
    }
    #[doc = "0x04 - Timer High Configuration register"]
    #[inline(always)]
    pub const fn cfg_hi(&self) -> &CFG_HI {
        &self.cfg_hi
    }
    #[doc = "0x08 - Timer Low counter value register"]
    #[inline(always)]
    pub const fn cnt_lo(&self) -> &CNT_LO {
        &self.cnt_lo
    }
    #[doc = "0x0c - Timer High counter value register"]
    #[inline(always)]
    pub const fn cnt_hi(&self) -> &CNT_HI {
        &self.cnt_hi
    }
    #[doc = "0x10 - Timer Low comparator value register"]
    #[inline(always)]
    pub const fn cmp_lo(&self) -> &CMP_LO {
        &self.cmp_lo
    }
    #[doc = "0x14 - Timer High comparator value register"]
    #[inline(always)]
    pub const fn cmp_hi(&self) -> &CMP_HI {
        &self.cmp_hi
    }
    #[doc = "0x18 - Start Timer Low counting register"]
    #[inline(always)]
    pub const fn start_lo(&self) -> &START_LO {
        &self.start_lo
    }
    #[doc = "0x1c - Start Timer High counting register"]
    #[inline(always)]
    pub const fn start_hi(&self) -> &START_HI {
        &self.start_hi
    }
    #[doc = "0x20 - Reset Timer Low counter register"]
    #[inline(always)]
    pub const fn reset_lo(&self) -> &RESET_LO {
        &self.reset_lo
    }
    #[doc = "0x24 - Reset Timer High counter register"]
    #[inline(always)]
    pub const fn reset_hi(&self) -> &RESET_HI {
        &self.reset_hi
    }
}
#[doc = "CFG_LO (rw) register accessor: Timer Low Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lo`]
module"]
pub type CFG_LO = crate::Reg<cfg_lo::CFG_LO_SPEC>;
#[doc = "Timer Low Configuration register"]
pub mod cfg_lo;
#[doc = "CFG_HI (rw) register accessor: Timer High Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_hi`]
module"]
pub type CFG_HI = crate::Reg<cfg_hi::CFG_HI_SPEC>;
#[doc = "Timer High Configuration register"]
pub mod cfg_hi;
#[doc = "CNT_LO (rw) register accessor: Timer Low counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt_lo`]
module"]
pub type CNT_LO = crate::Reg<cnt_lo::CNT_LO_SPEC>;
#[doc = "Timer Low counter value register"]
pub mod cnt_lo;
#[doc = "CNT_HI (rw) register accessor: Timer High counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt_hi`]
module"]
pub type CNT_HI = crate::Reg<cnt_hi::CNT_HI_SPEC>;
#[doc = "Timer High counter value register"]
pub mod cnt_hi;
#[doc = "CMP_LO (rw) register accessor: Timer Low comparator value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_lo`]
module"]
pub type CMP_LO = crate::Reg<cmp_lo::CMP_LO_SPEC>;
#[doc = "Timer Low comparator value register"]
pub mod cmp_lo;
#[doc = "CMP_HI (rw) register accessor: Timer High comparator value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_hi`]
module"]
pub type CMP_HI = crate::Reg<cmp_hi::CMP_HI_SPEC>;
#[doc = "Timer High comparator value register"]
pub mod cmp_hi;
#[doc = "START_LO (rw) register accessor: Start Timer Low counting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_lo`]
module"]
pub type START_LO = crate::Reg<start_lo::START_LO_SPEC>;
#[doc = "Start Timer Low counting register"]
pub mod start_lo;
#[doc = "START_HI (rw) register accessor: Start Timer High counting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_hi`]
module"]
pub type START_HI = crate::Reg<start_hi::START_HI_SPEC>;
#[doc = "Start Timer High counting register"]
pub mod start_hi;
#[doc = "RESET_LO (rw) register accessor: Reset Timer Low counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_lo`]
module"]
pub type RESET_LO = crate::Reg<reset_lo::RESET_LO_SPEC>;
#[doc = "Reset Timer Low counter register"]
pub mod reset_lo;
#[doc = "RESET_HI (rw) register accessor: Reset Timer High counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_hi`]
module"]
pub type RESET_HI = crate::Reg<reset_hi::RESET_HI_SPEC>;
#[doc = "Reset Timer High counter register"]
pub mod reset_hi;
