#[doc = "Register `read` reader"]
pub struct R(crate::R<READ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `read` reader - "]
pub type READ_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [read](index.html) module"]
pub struct READ_SPEC;
impl crate::RegisterSpec for READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [read::R](R) reader structure"]
impl crate::Readable for READ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets read to value 0"]
impl crate::Resettable for READ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
