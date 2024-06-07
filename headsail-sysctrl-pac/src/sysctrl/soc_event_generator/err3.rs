#[doc = "Register `ERR3` reader"]
pub type R = crate::R<Err3Spec>;
#[doc = "Field `ERR3` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type Err3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err3(&self) -> Err3R {
        Err3R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR3").field("err3", &self.err3()).finish()
    }
}
#[doc = "Events 96-127 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Err3Spec;
impl crate::RegisterSpec for Err3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err3::R`](R) reader structure"]
impl crate::Readable for Err3Spec {}
#[doc = "`reset()` method sets ERR3 to value 0"]
impl crate::Resettable for Err3Spec {
    const RESET_VALUE: u32 = 0;
}
