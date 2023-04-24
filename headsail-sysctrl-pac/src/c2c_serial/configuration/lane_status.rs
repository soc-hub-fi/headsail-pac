#[doc = "Register `LANE_STATUS[%s]` reader"]
pub struct R(crate::R<LANE_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LANE_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LANE_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LANE_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - "]
pub type STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lane_status](index.html) module"]
pub struct LANE_STATUS_SPEC;
impl crate::RegisterSpec for LANE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lane_status::R](R) reader structure"]
impl crate::Readable for LANE_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LANE_STATUS[%s]
to value 0"]
impl crate::Resettable for LANE_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
