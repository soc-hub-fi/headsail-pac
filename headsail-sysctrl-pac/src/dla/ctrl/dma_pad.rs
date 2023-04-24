#[doc = "Register `dma_pad` reader"]
pub struct R(crate::R<DMA_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `config` reader - "]
pub type CONFIG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_pad](index.html) module"]
pub struct DMA_PAD_SPEC;
impl crate::RegisterSpec for DMA_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_pad::R](R) reader structure"]
impl crate::Readable for DMA_PAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dma_pad to value 0"]
impl crate::Resettable for DMA_PAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
