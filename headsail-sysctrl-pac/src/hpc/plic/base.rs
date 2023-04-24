#[doc = "Register `base` reader"]
pub struct R(crate::R<BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `base` reader - "]
pub type BASE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base](index.html) module"]
pub struct BASE_SPEC;
impl crate::RegisterSpec for BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base::R](R) reader structure"]
impl crate::Readable for BASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets base to value 0"]
impl crate::Resettable for BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
