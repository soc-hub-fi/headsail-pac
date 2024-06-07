#[doc = "Register `ERR6` reader"]
pub type R = crate::R<Err6Spec>;
#[doc = "Field `ERR6` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type Err6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err6(&self) -> Err6R {
        Err6R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR6").field("err6", &self.err6()).finish()
    }
}
#[doc = "Events 191-223 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Err6Spec;
impl crate::RegisterSpec for Err6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err6::R`](R) reader structure"]
impl crate::Readable for Err6Spec {}
#[doc = "`reset()` method sets ERR6 to value 0"]
impl crate::Resettable for Err6Spec {
    const RESET_VALUE: u32 = 0;
}
