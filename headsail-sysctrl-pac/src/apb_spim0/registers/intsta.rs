#[doc = "Register `INTSTA` reader"]
pub struct R(crate::R<INTSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTSTA` reader - Not used"]
pub type INTSTA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Not used"]
    #[inline(always)]
    pub fn intsta(&self) -> INTSTA_R {
        INTSTA_R::new(self.bits)
    }
}
#[doc = "This register isn't properly specified so we need to look at this\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsta](index.html) module"]
pub struct INTSTA_SPEC;
impl crate::RegisterSpec for INTSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsta::R](R) reader structure"]
impl crate::Readable for INTSTA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTA to value 0"]
impl crate::Resettable for INTSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
