#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Timer"]
pub struct Timer {
    cfg_lo: CfgLo,
    cfg_hi: CfgHi,
    cnt_lo: CntLo,
    cnt_hi: CntHi,
    cmp_lo: CmpLo,
    cmp_hi: CmpHi,
    start_lo: StartLo,
    start_hi: StartHi,
    reset_lo: ResetLo,
    reset_hi: ResetHi,
}
impl Timer {
    #[doc = "0x00 - Timer Low Configuration register"]
    #[inline(always)]
    pub const fn cfg_lo(&self) -> &CfgLo {
        &self.cfg_lo
    }
    #[doc = "0x04 - Timer High Configuration register"]
    #[inline(always)]
    pub const fn cfg_hi(&self) -> &CfgHi {
        &self.cfg_hi
    }
    #[doc = "0x08 - Timer Low counter value register"]
    #[inline(always)]
    pub const fn cnt_lo(&self) -> &CntLo {
        &self.cnt_lo
    }
    #[doc = "0x0c - Timer High counter value register"]
    #[inline(always)]
    pub const fn cnt_hi(&self) -> &CntHi {
        &self.cnt_hi
    }
    #[doc = "0x10 - Timer Low comparator value register"]
    #[inline(always)]
    pub const fn cmp_lo(&self) -> &CmpLo {
        &self.cmp_lo
    }
    #[doc = "0x14 - Timer High comparator value register"]
    #[inline(always)]
    pub const fn cmp_hi(&self) -> &CmpHi {
        &self.cmp_hi
    }
    #[doc = "0x18 - Start Timer Low counting register"]
    #[inline(always)]
    pub const fn start_lo(&self) -> &StartLo {
        &self.start_lo
    }
    #[doc = "0x1c - Start Timer High counting register"]
    #[inline(always)]
    pub const fn start_hi(&self) -> &StartHi {
        &self.start_hi
    }
    #[doc = "0x20 - Reset Timer Low counter register"]
    #[inline(always)]
    pub const fn reset_lo(&self) -> &ResetLo {
        &self.reset_lo
    }
    #[doc = "0x24 - Reset Timer High counter register"]
    #[inline(always)]
    pub const fn reset_hi(&self) -> &ResetHi {
        &self.reset_hi
    }
}
#[doc = "CFG_LO (rw) register accessor: Timer Low Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_lo`]
module"]
#[doc(alias = "CFG_LO")]
pub type CfgLo = crate::Reg<cfg_lo::CfgLoSpec>;
#[doc = "Timer Low Configuration register"]
pub mod cfg_lo;
#[doc = "CFG_HI (rw) register accessor: Timer High Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_hi`]
module"]
#[doc(alias = "CFG_HI")]
pub type CfgHi = crate::Reg<cfg_hi::CfgHiSpec>;
#[doc = "Timer High Configuration register"]
pub mod cfg_hi;
#[doc = "CNT_LO (rw) register accessor: Timer Low counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt_lo`]
module"]
#[doc(alias = "CNT_LO")]
pub type CntLo = crate::Reg<cnt_lo::CntLoSpec>;
#[doc = "Timer Low counter value register"]
pub mod cnt_lo;
#[doc = "CNT_HI (rw) register accessor: Timer High counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt_hi`]
module"]
#[doc(alias = "CNT_HI")]
pub type CntHi = crate::Reg<cnt_hi::CntHiSpec>;
#[doc = "Timer High counter value register"]
pub mod cnt_hi;
#[doc = "CMP_LO (rw) register accessor: Timer Low comparator value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_lo`]
module"]
#[doc(alias = "CMP_LO")]
pub type CmpLo = crate::Reg<cmp_lo::CmpLoSpec>;
#[doc = "Timer Low comparator value register"]
pub mod cmp_lo;
#[doc = "CMP_HI (rw) register accessor: Timer High comparator value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_hi`]
module"]
#[doc(alias = "CMP_HI")]
pub type CmpHi = crate::Reg<cmp_hi::CmpHiSpec>;
#[doc = "Timer High comparator value register"]
pub mod cmp_hi;
#[doc = "START_LO (rw) register accessor: Start Timer Low counting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_lo`]
module"]
#[doc(alias = "START_LO")]
pub type StartLo = crate::Reg<start_lo::StartLoSpec>;
#[doc = "Start Timer Low counting register"]
pub mod start_lo;
#[doc = "START_HI (rw) register accessor: Start Timer High counting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_hi`]
module"]
#[doc(alias = "START_HI")]
pub type StartHi = crate::Reg<start_hi::StartHiSpec>;
#[doc = "Start Timer High counting register"]
pub mod start_hi;
#[doc = "RESET_LO (rw) register accessor: Reset Timer Low counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_lo`]
module"]
#[doc(alias = "RESET_LO")]
pub type ResetLo = crate::Reg<reset_lo::ResetLoSpec>;
#[doc = "Reset Timer Low counter register"]
pub mod reset_lo;
#[doc = "RESET_HI (rw) register accessor: Reset Timer High counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_hi`]
module"]
#[doc(alias = "RESET_HI")]
pub type ResetHi = crate::Reg<reset_hi::ResetHiSpec>;
#[doc = "Reset Timer High counter register"]
pub mod reset_hi;
