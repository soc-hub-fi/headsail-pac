#[doc = "Register `ERR2` reader"]
pub type R = crate::R<ERR2_SPEC>;
#[doc = "Field `ERR02` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type ERR02_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err02(&self) -> ERR02_R {
        ERR02_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR2")
            .field("err02", &self.err02())
            .finish()
    }
}
#[doc = "Events 64-95 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR2_SPEC;
impl crate::RegisterSpec for ERR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err2::R`](R) reader structure"]
impl crate::Readable for ERR2_SPEC {}
#[doc = "`reset()` method sets ERR2 to value 0"]
impl crate::Resettable for ERR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
