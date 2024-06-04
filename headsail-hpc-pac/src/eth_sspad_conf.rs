#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pad_cfg: PAD_CFG,
}
impl RegisterBlock {
    #[doc = "0x00..0x44 - padCfg"]
    #[inline(always)]
    pub const fn pad_cfg(&self) -> &PAD_CFG {
        &self.pad_cfg
    }
}
#[doc = "padCfg"]
pub use self::pad_cfg::PAD_CFG;
#[doc = r"Cluster"]
#[doc = "padCfg"]
pub mod pad_cfg;
