#[doc = "Register `ERR5` reader"]
pub type R = crate::R<ERR5_SPEC>;
#[doc = "Field `ERR5` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type ERR5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR5").field("err5", &self.err5()).finish()
    }
}
#[doc = "Events 160-191 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR5_SPEC;
impl crate::RegisterSpec for ERR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err5::R`](R) reader structure"]
impl crate::Readable for ERR5_SPEC {}
#[doc = "`reset()` method sets ERR5 to value 0"]
impl crate::Resettable for ERR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
