#[doc = "Register `stall_count_hi` reader"]
pub struct R(crate::R<STALL_COUNT_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALL_COUNT_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALL_COUNT_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALL_COUNT_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `hi` reader - "]
pub type HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits)
    }
}
#[doc = "High part of Stall count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stall_count_hi](index.html) module"]
pub struct STALL_COUNT_HI_SPEC;
impl crate::RegisterSpec for STALL_COUNT_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stall_count_hi::R](R) reader structure"]
impl crate::Readable for STALL_COUNT_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets stall_count_hi to value 0"]
impl crate::Resettable for STALL_COUNT_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
