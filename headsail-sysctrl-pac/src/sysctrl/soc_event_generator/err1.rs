#[doc = "Register `ERR1` reader"]
pub type R = crate::R<Err1Spec>;
#[doc = "Field `ERR1` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type Err1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err1(&self) -> Err1R {
        Err1R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR1").field("err1", &self.err1()).finish()
    }
}
#[doc = "Events 32-63 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Err1Spec;
impl crate::RegisterSpec for Err1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err1::R`](R) reader structure"]
impl crate::Readable for Err1Spec {}
#[doc = "`reset()` method sets ERR1 to value 0"]
impl crate::Resettable for Err1Spec {
    const RESET_VALUE: u32 = 0;
}
