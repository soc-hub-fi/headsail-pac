#[doc = "Register `HPC_PLL_ENABLE` reader"]
pub type R = crate::R<HPC_PLL_ENABLE_SPEC>;
#[doc = "Register `HPC_PLL_ENABLE` writer"]
pub type W = crate::W<HPC_PLL_ENABLE_SPEC>;
#[doc = "Field `spare_ctrl` reader - "]
pub type SPARE_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `spare_ctrl` writer - "]
pub type SPARE_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spare_ctrl(&self) -> SPARE_CTRL_R {
        SPARE_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPC_PLL_ENABLE")
            .field("spare_ctrl", &self.spare_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn spare_ctrl(&mut self) -> SPARE_CTRL_W<HPC_PLL_ENABLE_SPEC> {
        SPARE_CTRL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPC_PLL_ENABLE_SPEC;
impl crate::RegisterSpec for HPC_PLL_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpc_pll_enable::R`](R) reader structure"]
impl crate::Readable for HPC_PLL_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpc_pll_enable::W`](W) writer structure"]
impl crate::Writable for HPC_PLL_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPC_PLL_ENABLE to value 0"]
impl crate::Resettable for HPC_PLL_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
