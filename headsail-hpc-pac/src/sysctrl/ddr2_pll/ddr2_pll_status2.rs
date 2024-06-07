#[doc = "Register `DDR2_PLL_STATUS2` reader"]
pub type R = crate::R<Ddr2PllStatus2Spec>;
#[doc = "Field `status2` reader - "]
pub type Status2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status2(&self) -> Status2R {
        Status2R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR2_PLL_STATUS2")
            .field("status2", &self.status2())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ddr2PllStatus2Spec;
impl crate::RegisterSpec for Ddr2PllStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr2_pll_status2::R`](R) reader structure"]
impl crate::Readable for Ddr2PllStatus2Spec {}
#[doc = "`reset()` method sets DDR2_PLL_STATUS2 to value 0"]
impl crate::Resettable for Ddr2PllStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
