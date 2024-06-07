#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "BootConfig"]
pub struct BootConfig {
    boot_cfg: BootCfg,
    boot_status: BootStatus,
}
impl BootConfig {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn boot_cfg(&self) -> &BootCfg {
        &self.boot_cfg
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn boot_status(&self) -> &BootStatus {
        &self.boot_status
    }
}
#[doc = "BOOT_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_cfg`]
module"]
#[doc(alias = "BOOT_CFG")]
pub type BootCfg = crate::Reg<boot_cfg::BootCfgSpec>;
#[doc = ""]
pub mod boot_cfg;
#[doc = "BOOT_STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_status`]
module"]
#[doc(alias = "BOOT_STATUS")]
pub type BootStatus = crate::Reg<boot_status::BootStatusSpec>;
#[doc = ""]
pub mod boot_status;
