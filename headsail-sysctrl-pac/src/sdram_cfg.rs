#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    axi_lite_regs: AXI_LITE_REGS,
}
impl RegisterBlock {
    #[doc = "0x00..0x74 - axi_lite_regs"]
    #[inline(always)]
    pub const fn axi_lite_regs(&self) -> &AXI_LITE_REGS {
        &self.axi_lite_regs
    }
}
#[doc = "axi_lite_regs"]
pub use self::axi_lite_regs::AXI_LITE_REGS;
#[doc = r"Cluster"]
#[doc = "axi_lite_regs"]
pub mod axi_lite_regs;
