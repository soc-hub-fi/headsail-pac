#[doc = "Register `feature_flags_lo` reader"]
pub type R = crate::R<FeatureFlagsLoSpec>;
#[doc = "Field `feature_flags_lo` reader - "]
pub type FeatureFlagsLoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn feature_flags_lo(&self) -> FeatureFlagsLoR {
        FeatureFlagsLoR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("feature_flags_lo")
            .field("feature_flags_lo", &self.feature_flags_lo())
            .finish()
    }
}
#[doc = "Feature flags, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature_flags_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeatureFlagsLoSpec;
impl crate::RegisterSpec for FeatureFlagsLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feature_flags_lo::R`](R) reader structure"]
impl crate::Readable for FeatureFlagsLoSpec {}
#[doc = "`reset()` method sets feature_flags_lo to value 0x01"]
impl crate::Resettable for FeatureFlagsLoSpec {
    const RESET_VALUE: u32 = 0x01;
}
