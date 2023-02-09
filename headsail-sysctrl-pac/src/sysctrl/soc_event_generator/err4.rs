#[doc = "Register `ERR4` reader"]
pub struct R(crate::R<ERR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERR4` reader - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
pub type ERR4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event queue overflow. Clear after read. Reading 0b1 at ERR\\[i\\]
means the event queue of event with id i overflowed."]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(self.bits)
    }
}
#[doc = "Events 128-159 event queue overflow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err4](index.html) module"]
pub struct ERR4_SPEC;
impl crate::RegisterSpec for ERR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err4::R](R) reader structure"]
impl crate::Readable for ERR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR4 to value 0"]
impl crate::Resettable for ERR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
