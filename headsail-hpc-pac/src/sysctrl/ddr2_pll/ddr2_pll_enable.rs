#[doc = "Register `DDR2_PLL_ENABLE` reader"]
pub type R = crate::R<Ddr2PllEnableSpec>;
#[doc = "Register `DDR2_PLL_ENABLE` writer"]
pub type W = crate::W<Ddr2PllEnableSpec>;
#[doc = "Field `spare_ctrl` reader - "]
pub type SpareCtrlR = crate::FieldReader<u32>;
#[doc = "Field `spare_ctrl` writer - "]
pub type SpareCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spare_ctrl(&self) -> SpareCtrlR {
        SpareCtrlR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR2_PLL_ENABLE")
            .field("spare_ctrl", &self.spare_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn spare_ctrl(&mut self) -> SpareCtrlW<Ddr2PllEnableSpec> {
        SpareCtrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ddr2PllEnableSpec;
impl crate::RegisterSpec for Ddr2PllEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr2_pll_enable::R`](R) reader structure"]
impl crate::Readable for Ddr2PllEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ddr2_pll_enable::W`](W) writer structure"]
impl crate::Writable for Ddr2PllEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR2_PLL_ENABLE to value 0"]
impl crate::Resettable for Ddr2PllEnableSpec {
    const RESET_VALUE: u32 = 0;
}
