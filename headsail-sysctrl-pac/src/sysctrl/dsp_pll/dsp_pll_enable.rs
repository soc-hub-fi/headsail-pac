#[doc = "Register `DSP_PLL_ENABLE` reader"]
pub type R = crate::R<DspPllEnableSpec>;
#[doc = "Register `DSP_PLL_ENABLE` writer"]
pub type W = crate::W<DspPllEnableSpec>;
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
        f.debug_struct("DSP_PLL_ENABLE")
            .field("spare_ctrl", &self.spare_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn spare_ctrl(&mut self) -> SpareCtrlW<DspPllEnableSpec> {
        SpareCtrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_pll_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_pll_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspPllEnableSpec;
impl crate::RegisterSpec for DspPllEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_pll_enable::R`](R) reader structure"]
impl crate::Readable for DspPllEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_pll_enable::W`](W) writer structure"]
impl crate::Writable for DspPllEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_PLL_ENABLE to value 0"]
impl crate::Resettable for DspPllEnableSpec {
    const RESET_VALUE: u32 = 0;
}
