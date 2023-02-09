#[doc = "Register `ERR2` reader"]
pub struct R(crate::R<ERR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERR02` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type ERR02_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err02(&self) -> ERR02_R {
        ERR02_R::new(self.bits)
    }
}
#[doc = "Events 64-95 event queue overflow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err2](index.html) module"]
pub struct ERR2_SPEC;
impl crate::RegisterSpec for ERR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err2::R](R) reader structure"]
impl crate::Readable for ERR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR2 to value 0"]
impl crate::Resettable for ERR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
