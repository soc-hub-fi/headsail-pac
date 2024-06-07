#[doc = "Register `HPC_PLL_STATUS1` reader"]
pub type R = crate::R<HpcPllStatus1Spec>;
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
        f.debug_struct("HPC_PLL_STATUS1")
            .field("status1", &self.status1())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpcPllStatus1Spec;
impl crate::RegisterSpec for HpcPllStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpc_pll_status1::R`](R) reader structure"]
impl crate::Readable for HpcPllStatus1Spec {}
#[doc = "`reset()` method sets HPC_PLL_STATUS1 to value 0"]
impl crate::Resettable for HpcPllStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
