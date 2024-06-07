#[doc = "Register `INTSTA` reader"]
pub type R = crate::R<INTSTA_SPEC>;
#[doc = "Field `INTSTA` reader - Not used"]
pub type INTSTA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Not used"]
    #[inline(always)]
    pub fn intsta(&self) -> INTSTA_R {
        INTSTA_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTA")
            .field("intsta", &self.intsta())
            .finish()
    }
}
#[doc = "This register isn't properly specified so we need to look at this\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsta::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTA_SPEC;
impl crate::RegisterSpec for INTSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsta::R`](R) reader structure"]
impl crate::Readable for INTSTA_SPEC {}
#[doc = "`reset()` method sets INTSTA to value 0"]
impl crate::Resettable for INTSTA_SPEC {
    const RESET_VALUE: u32 = 0;
}
