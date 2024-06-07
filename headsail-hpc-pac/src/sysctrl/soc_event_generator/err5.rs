#[doc = "Register `ERR5` reader"]
pub type R = crate::R<Err5Spec>;
#[doc = "Field `ERR5` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type Err5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err5(&self) -> Err5R {
        Err5R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR5").field("err5", &self.err5()).finish()
    }
}
#[doc = "Events 160-191 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Err5Spec;
impl crate::RegisterSpec for Err5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err5::R`](R) reader structure"]
impl crate::Readable for Err5Spec {}
#[doc = "`reset()` method sets ERR5 to value 0"]
impl crate::Resettable for Err5Spec {
    const RESET_VALUE: u32 = 0;
}
