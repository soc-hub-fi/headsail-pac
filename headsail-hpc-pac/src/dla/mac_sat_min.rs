#[doc = "Register `mac_sat_min` reader"]
pub type R = crate::R<MacSatMinSpec>;
#[doc = "Field `minimum` reader - "]
pub type MinimumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn minimum(&self) -> MinimumR {
        MinimumR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("mac_sat_min")
            .field("minimum", &self.minimum())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_sat_min::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacSatMinSpec;
impl crate::RegisterSpec for MacSatMinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_sat_min::R`](R) reader structure"]
impl crate::Readable for MacSatMinSpec {}
#[doc = "`reset()` method sets mac_sat_min to value 0"]
impl crate::Resettable for MacSatMinSpec {
    const RESET_VALUE: u32 = 0;
}
