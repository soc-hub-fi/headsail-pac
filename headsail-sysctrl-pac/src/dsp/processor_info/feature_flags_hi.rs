#[doc = "Register `feature_flags_hi` reader"]
pub type R = crate::R<FeatureFlagsHiSpec>;
#[doc = "Field `feature_flags_hi` reader - "]
pub type FeatureFlagsHiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn feature_flags_hi(&self) -> FeatureFlagsHiR {
        FeatureFlagsHiR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("feature_flags_hi")
            .field("feature_flags_hi", &self.feature_flags_hi())
            .finish()
    }
}
#[doc = "Feature flags, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature_flags_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeatureFlagsHiSpec;
impl crate::RegisterSpec for FeatureFlagsHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feature_flags_hi::R`](R) reader structure"]
impl crate::Readable for FeatureFlagsHiSpec {}
#[doc = "`reset()` method sets feature_flags_hi to value 0"]
impl crate::Resettable for FeatureFlagsHiSpec {
    const RESET_VALUE: u32 = 0;
}
