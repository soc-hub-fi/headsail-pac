#[doc = "Register `dma_pad` reader"]
pub type R = crate::R<DmaPadSpec>;
#[doc = "Field `config` reader - "]
pub type ConfigR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn config(&self) -> ConfigR {
        ConfigR::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("dma_pad")
            .field("config", &self.config())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_pad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaPadSpec;
impl crate::RegisterSpec for DmaPadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_pad::R`](R) reader structure"]
impl crate::Readable for DmaPadSpec {}
#[doc = "`reset()` method sets dma_pad to value 0"]
impl crate::Resettable for DmaPadSpec {
    const RESET_VALUE: u32 = 0;
}
