#[doc = "Register `LANE_STATUS[%s]` reader"]
pub type R = crate::R<LANE_STATUS_SPEC>;
#[doc = "Field `STATUS` reader - "]
pub type STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LANE_STATUS")
            .field("status", &self.status())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LANE_STATUS_SPEC;
impl crate::RegisterSpec for LANE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lane_status::R`](R) reader structure"]
impl crate::Readable for LANE_STATUS_SPEC {}
#[doc = "`reset()` method sets LANE_STATUS[%s]
to value 0"]
impl crate::Resettable for LANE_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
