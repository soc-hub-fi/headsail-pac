#[doc = "Register `ERR1` reader"]
pub struct R(crate::R<ERR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERR1` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type ERR1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(self.bits)
    }
}
#[doc = "Events 32-63 event queue overflow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err1](index.html) module"]
pub struct ERR1_SPEC;
impl crate::RegisterSpec for ERR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err1::R](R) reader structure"]
impl crate::Readable for ERR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR1 to value 0"]
impl crate::Resettable for ERR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
