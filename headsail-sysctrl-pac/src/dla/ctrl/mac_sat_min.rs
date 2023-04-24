#[doc = "Register `mac_sat_min` reader"]
pub struct R(crate::R<MAC_SAT_MIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SAT_MIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_SAT_MIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_SAT_MIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `minimum` reader - "]
pub type MINIMUM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn minimum(&self) -> MINIMUM_R {
        MINIMUM_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sat_min](index.html) module"]
pub struct MAC_SAT_MIN_SPEC;
impl crate::RegisterSpec for MAC_SAT_MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_sat_min::R](R) reader structure"]
impl crate::Readable for MAC_SAT_MIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets mac_sat_min to value 0"]
impl crate::Resettable for MAC_SAT_MIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
