#[doc = r"Register block"]
#[repr(C)]
pub struct EMA_SS {
    #[doc = "0x00 - "]
    pub ema_sysctrl: EMA_SYSCTRL,
    #[doc = "0x04 - "]
    pub ema_tlp: EMA_TLP,
    #[doc = "0x08 - "]
    pub ema_dma: EMA_DMA,
    #[doc = "0x0c - "]
    pub ema_sram: EMA_SRAM,
}
#[doc = "EMA_SYSCTRL (rw) register accessor: an alias for `Reg<EMA_SYSCTRL_SPEC>`"]
pub type EMA_SYSCTRL = crate::Reg<ema_sysctrl::EMA_SYSCTRL_SPEC>;
#[doc = ""]
pub mod ema_sysctrl;
#[doc = "EMA_TLP (rw) register accessor: an alias for `Reg<EMA_TLP_SPEC>`"]
pub type EMA_TLP = crate::Reg<ema_tlp::EMA_TLP_SPEC>;
#[doc = ""]
pub mod ema_tlp;
#[doc = "EMA_DMA (rw) register accessor: an alias for `Reg<EMA_DMA_SPEC>`"]
pub type EMA_DMA = crate::Reg<ema_dma::EMA_DMA_SPEC>;
#[doc = ""]
pub mod ema_dma;
#[doc = "EMA_SRAM (rw) register accessor: an alias for `Reg<EMA_SRAM_SPEC>`"]
pub type EMA_SRAM = crate::Reg<ema_sram::EMA_SRAM_SPEC>;
#[doc = ""]
pub mod ema_sram;
