#[doc = "Register `base` reader"]
pub type R = crate::R<BASE_SPEC>;
#[doc = "Field `base` reader - "]
pub type BASE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("base").field("base", &self.base()).finish()
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BASE_SPEC;
impl crate::RegisterSpec for BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base::R`](R) reader structure"]
impl crate::Readable for BASE_SPEC {}
#[doc = "`reset()` method sets base to value 0"]
impl crate::Resettable for BASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
