#[doc = "Register `read` reader"]
pub type R = crate::R<READ_SPEC>;
#[doc = "Field `read` reader - "]
pub type READ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("read").field("read", &self.read()).finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READ_SPEC;
impl crate::RegisterSpec for READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read::R`](R) reader structure"]
impl crate::Readable for READ_SPEC {}
#[doc = "`reset()` method sets read to value 0"]
impl crate::Resettable for READ_SPEC {
    const RESET_VALUE: u32 = 0;
}
