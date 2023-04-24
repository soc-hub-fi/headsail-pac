#[doc = "Register `cycle_count_lo` reader"]
pub struct R(crate::R<CYCLE_COUNT_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCLE_COUNT_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCLE_COUNT_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCLE_COUNT_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `lo` reader - Cycle count (low 32 bits)"]
pub type LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cycle count (low 32 bits)"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[doc = "Low part of Cycle count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle_count_lo](index.html) module"]
pub struct CYCLE_COUNT_LO_SPEC;
impl crate::RegisterSpec for CYCLE_COUNT_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cycle_count_lo::R](R) reader structure"]
impl crate::Readable for CYCLE_COUNT_LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cycle_count_lo to value 0"]
impl crate::Resettable for CYCLE_COUNT_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
