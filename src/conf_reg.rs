#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x44 - padCfg"]
    pub pad_cfg: PAD_CFG,
}
#[doc = "padCfg"]
pub use self::pad_cfg::PAD_CFG;
#[doc = r"Cluster"]
#[doc = "padCfg"]
pub mod pad_cfg;
