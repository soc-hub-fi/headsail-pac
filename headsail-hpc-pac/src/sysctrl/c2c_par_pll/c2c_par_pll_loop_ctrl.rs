#[doc = "Register `C2C_PAR_PLL_LOOP_CTRL` reader"]
pub type R = crate::R<C2cParPllLoopCtrlSpec>;
#[doc = "Register `C2C_PAR_PLL_LOOP_CTRL` writer"]
pub type W = crate::W<C2cParPllLoopCtrlSpec>;
#[doc = "Field `LOOP_CTRL` reader - "]
pub type LoopCtrlR = crate::FieldReader<u32>;
#[doc = "Field `LOOP_CTRL` writer - "]
pub type LoopCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn loop_ctrl(&self) -> LoopCtrlR {
        LoopCtrlR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2C_PAR_PLL_LOOP_CTRL")
            .field("loop_ctrl", &self.loop_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn loop_ctrl(&mut self) -> LoopCtrlW<C2cParPllLoopCtrlSpec> {
        LoopCtrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2c_par_pll_loop_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2c_par_pll_loop_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2cParPllLoopCtrlSpec;
impl crate::RegisterSpec for C2cParPllLoopCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2c_par_pll_loop_ctrl::R`](R) reader structure"]
impl crate::Readable for C2cParPllLoopCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`c2c_par_pll_loop_ctrl::W`](W) writer structure"]
impl crate::Writable for C2cParPllLoopCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2C_PAR_PLL_LOOP_CTRL to value 0"]
impl crate::Resettable for C2cParPllLoopCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
