#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "EMA_SS"]
#[doc(alias = "EMA_SS")]
pub struct EmaSs {
    ema_sysctrl: EmaSysctrl,
    ema_tlp: EmaTlp,
    ema_dma: EmaDma,
    ema_sram: EmaSram,
}
impl EmaSs {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn ema_sysctrl(&self) -> &EmaSysctrl {
        &self.ema_sysctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn ema_tlp(&self) -> &EmaTlp {
        &self.ema_tlp
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ema_dma(&self) -> &EmaDma {
        &self.ema_dma
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ema_sram(&self) -> &EmaSram {
        &self.ema_sram
    }
}
#[doc = "EMA_SYSCTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_sysctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_sysctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema_sysctrl`]
module"]
#[doc(alias = "EMA_SYSCTRL")]
pub type EmaSysctrl = crate::Reg<ema_sysctrl::EmaSysctrlSpec>;
#[doc = ""]
pub mod ema_sysctrl;
#[doc = "EMA_TLP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_tlp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_tlp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema_tlp`]
module"]
#[doc(alias = "EMA_TLP")]
pub type EmaTlp = crate::Reg<ema_tlp::EmaTlpSpec>;
#[doc = ""]
pub mod ema_tlp;
#[doc = "EMA_DMA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema_dma`]
module"]
#[doc(alias = "EMA_DMA")]
pub type EmaDma = crate::Reg<ema_dma::EmaDmaSpec>;
#[doc = ""]
pub mod ema_dma;
#[doc = "EMA_SRAM (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_sram::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_sram::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema_sram`]
module"]
#[doc(alias = "EMA_SRAM")]
pub type EmaSram = crate::Reg<ema_sram::EmaSramSpec>;
#[doc = ""]
pub mod ema_sram;
