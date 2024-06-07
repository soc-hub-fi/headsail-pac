#[doc = "Register `ERR4` reader"]
pub type R = crate::R<Err4Spec>;
#[doc = "Field `ERR4` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type Err4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err4(&self) -> Err4R {
        Err4R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR4").field("err4", &self.err4()).finish()
    }
}
#[doc = "Events 128-159 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Err4Spec;
impl crate::RegisterSpec for Err4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err4::R`](R) reader structure"]
impl crate::Readable for Err4Spec {}
#[doc = "`reset()` method sets ERR4 to value 0"]
impl crate::Resettable for Err4Spec {
    const RESET_VALUE: u32 = 0;
}
