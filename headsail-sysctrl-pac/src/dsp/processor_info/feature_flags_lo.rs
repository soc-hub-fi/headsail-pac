#[doc = "Register `feature_flags_lo` reader"]
pub struct R(crate::R<FEATURE_FLAGS_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATURE_FLAGS_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEATURE_FLAGS_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEATURE_FLAGS_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `feature_flags_lo` reader - "]
pub type FEATURE_FLAGS_LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn feature_flags_lo(&self) -> FEATURE_FLAGS_LO_R {
        FEATURE_FLAGS_LO_R::new(self.bits)
    }
}
#[doc = "Feature flags, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feature_flags_lo](index.html) module"]
pub struct FEATURE_FLAGS_LO_SPEC;
impl crate::RegisterSpec for FEATURE_FLAGS_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [feature_flags_lo::R](R) reader structure"]
impl crate::Readable for FEATURE_FLAGS_LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets feature_flags_lo to value 0x01"]
impl crate::Resettable for FEATURE_FLAGS_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
