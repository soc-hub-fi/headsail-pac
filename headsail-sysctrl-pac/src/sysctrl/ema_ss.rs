#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "EMA_SS"]
pub struct EMA_SS {
    ema_sysctrl: EMA_SYSCTRL,
    ema_tlp: EMA_TLP,
    ema_dma: EMA_DMA,
    ema_sram: EMA_SRAM,
}
impl EMA_SS {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn ema_sysctrl(&self) -> &EMA_SYSCTRL {
        &self.ema_sysctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn ema_tlp(&self) -> &EMA_TLP {
        &self.ema_tlp
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ema_dma(&self) -> &EMA_DMA {
        &self.ema_dma
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ema_sram(&self) -> &EMA_SRAM {
        &self.ema_sram
    }
}
#[doc = "EMA_SYSCTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_sysctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_sysctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema_sysctrl`]
module"]
pub type EMA_SYSCTRL = crate::Reg<ema_sysctrl::EMA_SYSCTRL_SPEC>;
#[doc = ""]
pub mod ema_sysctrl;
#[doc = "EMA_TLP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_tlp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_tlp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema_tlp`]
module"]
pub type EMA_TLP = crate::Reg<ema_tlp::EMA_TLP_SPEC>;
#[doc = ""]
pub mod ema_tlp;
#[doc = "EMA_DMA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema_dma`]
module"]
pub type EMA_DMA = crate::Reg<ema_dma::EMA_DMA_SPEC>;
#[doc = ""]
pub mod ema_dma;
#[doc = "EMA_SRAM (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_sram::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_sram::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema_sram`]
module"]
pub type EMA_SRAM = crate::Reg<ema_sram::EMA_SRAM_SPEC>;
#[doc = ""]
pub mod ema_sram;
