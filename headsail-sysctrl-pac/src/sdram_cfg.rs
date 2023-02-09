#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x74 - axi_lite_regs"]
    pub axi_lite_regs: AXI_LITE_REGS,
}
#[doc = "axi_lite_regs"]
pub use self::axi_lite_regs::AXI_LITE_REGS;
#[doc = r"Cluster"]
#[doc = "axi_lite_regs"]
pub mod axi_lite_regs;
