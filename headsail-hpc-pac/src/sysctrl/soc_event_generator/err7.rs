#[doc = "Register `ERR7` reader"]
pub type R = crate::R<Err7Spec>;
#[doc = "Field `ERR7` reader - "]
pub type Err7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn err7(&self) -> Err7R {
        Err7R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR7").field("err7", &self.err7()).finish()
    }
}
#[doc = "Events 224-255 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Err7Spec;
impl crate::RegisterSpec for Err7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err7::R`](R) reader structure"]
impl crate::Readable for Err7Spec {}
#[doc = "`reset()` method sets ERR7 to value 0"]
impl crate::Resettable for Err7Spec {
    const RESET_VALUE: u32 = 0;
}
