#[doc = "Register `bp_count` reader"]
pub struct R(crate::R<BP_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BP_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BP_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BP_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `count` reader - "]
pub type COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "Breakpoint count (0x2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bp_count](index.html) module"]
pub struct BP_COUNT_SPEC;
impl crate::RegisterSpec for BP_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bp_count::R](R) reader structure"]
impl crate::Readable for BP_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets bp_count to value 0x02"]
impl crate::Resettable for BP_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
