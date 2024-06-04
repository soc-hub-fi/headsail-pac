#[doc = "Register `mac_sat_max` reader"]
pub type R = crate::R<MAC_SAT_MAX_SPEC>;
#[doc = "Field `maximum` reader - "]
pub type MAXIMUM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn maximum(&self) -> MAXIMUM_R {
        MAXIMUM_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("mac_sat_max")
            .field("maximum", &self.maximum())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_sat_max::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_SAT_MAX_SPEC;
impl crate::RegisterSpec for MAC_SAT_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_sat_max::R`](R) reader structure"]
impl crate::Readable for MAC_SAT_MAX_SPEC {}
#[doc = "`reset()` method sets mac_sat_max to value 0"]
impl crate::Resettable for MAC_SAT_MAX_SPEC {
    const RESET_VALUE: u32 = 0;
}
