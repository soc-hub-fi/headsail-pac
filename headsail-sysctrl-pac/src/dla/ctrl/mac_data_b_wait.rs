#[doc = "Register `mac_data_b_wait` reader"]
pub type R = crate::R<MAC_DATA_B_WAIT_SPEC>;
#[doc = "Field `cycles` reader - "]
pub type CYCLES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new(self.bits)
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
pub struct MAC_DATA_B_WAIT_SPEC;
impl crate::RegisterSpec for MAC_DATA_B_WAIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_data_b_wait::R`](R) reader structure"]
impl crate::Readable for MAC_DATA_B_WAIT_SPEC {}
#[doc = "`reset()` method sets mac_data_b_wait to value 0"]
impl crate::Resettable for MAC_DATA_B_WAIT_SPEC {
    const RESET_VALUE: u32 = 0;
}
