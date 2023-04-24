#[doc = "Register `cycle_count_hi` reader"]
pub struct R(crate::R<CYCLE_COUNT_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCLE_COUNT_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCLE_COUNT_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCLE_COUNT_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `hi` reader - Cycle count (high 32 bits)"]
pub type HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cycle count (high 32 bits)"]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits)
    }
}
#[doc = "High part of Cycle count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle_count_hi](index.html) module"]
pub struct CYCLE_COUNT_HI_SPEC;
impl crate::RegisterSpec for CYCLE_COUNT_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cycle_count_hi::R](R) reader structure"]
impl crate::Readable for CYCLE_COUNT_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cycle_count_hi to value 0"]
impl crate::Resettable for CYCLE_COUNT_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
