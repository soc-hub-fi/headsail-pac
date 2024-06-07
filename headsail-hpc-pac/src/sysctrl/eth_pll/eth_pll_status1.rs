#[doc = "Register `ETH_PLL_STATUS1` reader"]
pub type R = crate::R<EthPllStatus1Spec>;
#[doc = "Field `status1` reader - "]
pub type Status1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status1(&self) -> Status1R {
        Status1R::new(self.bits)
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
pub struct EthPllStatus1Spec;
impl crate::RegisterSpec for EthPllStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_pll_status1::R`](R) reader structure"]
impl crate::Readable for EthPllStatus1Spec {}
#[doc = "`reset()` method sets ETH_PLL_STATUS1 to value 0"]
impl crate::Resettable for EthPllStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
