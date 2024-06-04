#[doc = "Register `ERR6` reader"]
pub type R = crate::R<ERR6_SPEC>;
#[doc = "Field `ERR6` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type ERR6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err6(&self) -> ERR6_R {
        ERR6_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR6").field("err6", &self.err6()).finish()
    }
}
#[doc = "Events 191-223 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR6_SPEC;
impl crate::RegisterSpec for ERR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err6::R`](R) reader structure"]
impl crate::Readable for ERR6_SPEC {}
#[doc = "`reset()` method sets ERR6 to value 0"]
impl crate::Resettable for ERR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
