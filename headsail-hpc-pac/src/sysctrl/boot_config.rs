#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "BootConfig"]
#[doc(alias = "BootConfig")]
pub struct BOOT_CONFIG {
    boot_cfg: BOOT_CFG,
    boot_status: BOOT_STATUS,
}
impl BOOT_CONFIG {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn boot_cfg(&self) -> &BOOT_CFG {
        &self.boot_cfg
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn boot_status(&self) -> &BOOT_STATUS {
        &self.boot_status
    }
}
#[doc = "BOOT_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_cfg`]
module"]
pub type BOOT_CFG = crate::Reg<boot_cfg::BOOT_CFG_SPEC>;
#[doc = ""]
pub mod boot_cfg;
#[doc = "BOOT_STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_status`]
module"]
pub type BOOT_STATUS = crate::Reg<boot_status::BOOT_STATUS_SPEC>;
#[doc = ""]
pub mod boot_status;
