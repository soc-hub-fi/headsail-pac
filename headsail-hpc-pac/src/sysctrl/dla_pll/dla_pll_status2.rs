#[doc = "Register `DLA_PLL_STATUS2` reader"]
pub type R = crate::R<DLA_PLL_STATUS2_SPEC>;
#[doc = "Field `status2` reader - "]
pub type STATUS2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status2(&self) -> STATUS2_R {
        STATUS2_R::new(self.bits)
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
pub struct DLA_PLL_STATUS2_SPEC;
impl crate::RegisterSpec for DLA_PLL_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dla_pll_status2::R`](R) reader structure"]
impl crate::Readable for DLA_PLL_STATUS2_SPEC {}
#[doc = "`reset()` method sets DLA_PLL_STATUS2 to value 0"]
impl crate::Resettable for DLA_PLL_STATUS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
