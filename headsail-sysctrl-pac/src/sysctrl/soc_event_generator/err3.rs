#[doc = "Register `ERR3` reader"]
pub type R = crate::R<ERR3_SPEC>;
#[doc = "Field `ERR3` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type ERR3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR3").field("err3", &self.err3()).finish()
    }
}
#[doc = "Events 96-127 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR3_SPEC;
impl crate::RegisterSpec for ERR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err3::R`](R) reader structure"]
impl crate::Readable for ERR3_SPEC {}
#[doc = "`reset()` method sets ERR3 to value 0"]
impl crate::Resettable for ERR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
