#[doc = "Register `STATUS1` reader"]
pub struct R(crate::R<STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS1` reader - "]
pub type STATUS1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status1(&self) -> STATUS1_R {
        STATUS1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status1](index.html) module"]
pub struct STATUS1_SPEC;
impl crate::RegisterSpec for STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status1::R](R) reader structure"]
impl crate::Readable for STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS1 to value 0"]
impl crate::Resettable for STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
