#[doc = "Register `debug_feature_support` reader"]
pub struct R(crate::R<DEBUG_FEATURE_SUPPORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_FEATURE_SUPPORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_FEATURE_SUPPORT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_FEATURE_SUPPORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `debug_feature_support` reader - "]
pub type DEBUG_FEATURE_SUPPORT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn debug_feature_support(&self) -> DEBUG_FEATURE_SUPPORT_R {
        DEBUG_FEATURE_SUPPORT_R::new(self.bits)
    }
}
#[doc = "Debug feature support (0x1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_feature_support](index.html) module"]
pub struct DEBUG_FEATURE_SUPPORT_SPEC;
impl crate::RegisterSpec for DEBUG_FEATURE_SUPPORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_feature_support::R](R) reader structure"]
impl crate::Readable for DEBUG_FEATURE_SUPPORT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets debug_feature_support to value 0x01"]
impl crate::Resettable for DEBUG_FEATURE_SUPPORT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
