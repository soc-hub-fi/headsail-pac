#[doc = "Register `ETH_PLL_STATUS1` reader"]
pub type R = crate::R<ETH_PLL_STATUS1_SPEC>;
#[doc = "Field `status1` reader - "]
pub type STATUS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status1(&self) -> STATUS1_R {
        STATUS1_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_PLL_STATUS1")
            .field("status1", &self.status1())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_PLL_STATUS1_SPEC;
impl crate::RegisterSpec for ETH_PLL_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_pll_status1::R`](R) reader structure"]
impl crate::Readable for ETH_PLL_STATUS1_SPEC {}
#[doc = "`reset()` method sets ETH_PLL_STATUS1 to value 0"]
impl crate::Resettable for ETH_PLL_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
