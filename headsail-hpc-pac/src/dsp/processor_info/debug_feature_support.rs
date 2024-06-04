#[doc = "Register `debug_feature_support` reader"]
pub type R = crate::R<DEBUG_FEATURE_SUPPORT_SPEC>;
#[doc = "Field `debug_feature_support` reader - "]
pub type DEBUG_FEATURE_SUPPORT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn debug_feature_support(&self) -> DEBUG_FEATURE_SUPPORT_R {
        DEBUG_FEATURE_SUPPORT_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("debug_feature_support")
            .field("debug_feature_support", &self.debug_feature_support())
            .finish()
    }
}
#[doc = "Debug feature support (0x1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_feature_support::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_FEATURE_SUPPORT_SPEC;
impl crate::RegisterSpec for DEBUG_FEATURE_SUPPORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_feature_support::R`](R) reader structure"]
impl crate::Readable for DEBUG_FEATURE_SUPPORT_SPEC {}
#[doc = "`reset()` method sets debug_feature_support to value 0x01"]
impl crate::Resettable for DEBUG_FEATURE_SUPPORT_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
