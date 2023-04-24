#[doc = r"Register block"]
#[repr(C)]
pub struct L2_CONFIG {
    #[doc = "0x00..0x08 - Enable L2 cache"]
    pub en: EN,
}
#[doc = "en (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable L2 cache"]
pub mod en;
