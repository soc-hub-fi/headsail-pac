#[doc = "Register `ERR0` reader"]
pub type R = crate::R<ERR0_SPEC>;
#[doc = "Field `ERR0` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type ERR0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR0").field("err0", &self.err0()).finish()
    }
}
#[doc = "Events 0-31 event queue overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR0_SPEC;
impl crate::RegisterSpec for ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err0::R`](R) reader structure"]
impl crate::Readable for ERR0_SPEC {}
#[doc = "`reset()` method sets ERR0 to value 0"]
impl crate::Resettable for ERR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
