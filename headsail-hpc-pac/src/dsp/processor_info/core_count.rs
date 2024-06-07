#[doc = "Register `core_count` reader"]
pub type R = crate::R<CoreCountSpec>;
#[doc = "Field `core_count` reader - "]
pub type CoreCountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_count(&self) -> CoreCountR {
        CoreCountR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("core_count")
            .field("core_count", &self.core_count())
            .finish()
    }
}
#[doc = "Core count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCountSpec;
impl crate::RegisterSpec for CoreCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_count::R`](R) reader structure"]
impl crate::Readable for CoreCountSpec {}
#[doc = "`reset()` method sets core_count to value 0x01"]
impl crate::Resettable for CoreCountSpec {
    const RESET_VALUE: u32 = 0x01;
}
