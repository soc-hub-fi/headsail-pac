#[doc = "Register `DLA_PLL_STATUS2` reader"]
pub type R = crate::R<DlaPllStatus2Spec>;
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
        f.debug_struct("DLA_PLL_STATUS2")
            .field("status2", &self.status2())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlaPllStatus2Spec;
impl crate::RegisterSpec for DlaPllStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dla_pll_status2::R`](R) reader structure"]
impl crate::Readable for DlaPllStatus2Spec {}
#[doc = "`reset()` method sets DLA_PLL_STATUS2 to value 0"]
impl crate::Resettable for DlaPllStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
