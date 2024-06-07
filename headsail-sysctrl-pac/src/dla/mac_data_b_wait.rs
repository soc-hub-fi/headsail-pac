#[doc = "Register `mac_data_b_wait` reader"]
pub type R = crate::R<MacDataBWaitSpec>;
#[doc = "Field `cycles` reader - "]
pub type CyclesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cycles(&self) -> CyclesR {
        CyclesR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("mac_data_b_wait")
            .field("cycles", &self.cycles())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_data_b_wait::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacDataBWaitSpec;
impl crate::RegisterSpec for MacDataBWaitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_data_b_wait::R`](R) reader structure"]
impl crate::Readable for MacDataBWaitSpec {}
#[doc = "`reset()` method sets mac_data_b_wait to value 0"]
impl crate::Resettable for MacDataBWaitSpec {
    const RESET_VALUE: u32 = 0;
}
