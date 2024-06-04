#[doc = "Register `RCA` reader"]
pub type R = crate::R<RCA_SPEC>;
#[doc = "Field `RCA` reader - "]
pub type RCA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rca(&self) -> RCA_R {
        RCA_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCA").field("rca", &self.rca()).finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rca::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCA_SPEC;
impl crate::RegisterSpec for RCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rca::R`](R) reader structure"]
impl crate::Readable for RCA_SPEC {}
#[doc = "`reset()` method sets RCA to value 0"]
impl crate::Resettable for RCA_SPEC {
    const RESET_VALUE: u32 = 0;
}
