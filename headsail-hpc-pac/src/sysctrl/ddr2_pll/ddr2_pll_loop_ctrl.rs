#[doc = "Register `DDR2_PLL_LOOP_CTRL` reader"]
pub type R = crate::R<DDR2_PLL_LOOP_CTRL_SPEC>;
#[doc = "Register `DDR2_PLL_LOOP_CTRL` writer"]
pub type W = crate::W<DDR2_PLL_LOOP_CTRL_SPEC>;
#[doc = "Field `LOOP_CTRL` reader - "]
pub type LOOP_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `LOOP_CTRL` writer - "]
pub type LOOP_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn loop_ctrl(&self) -> LOOP_CTRL_R {
        LOOP_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR2_PLL_LOOP_CTRL")
            .field("loop_ctrl", &self.loop_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn loop_ctrl(&mut self) -> LOOP_CTRL_W<DDR2_PLL_LOOP_CTRL_SPEC> {
        LOOP_CTRL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr2_pll_loop_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr2_pll_loop_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDR2_PLL_LOOP_CTRL_SPEC;
impl crate::RegisterSpec for DDR2_PLL_LOOP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr2_pll_loop_ctrl::R`](R) reader structure"]
impl crate::Readable for DDR2_PLL_LOOP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddr2_pll_loop_ctrl::W`](W) writer structure"]
impl crate::Writable for DDR2_PLL_LOOP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR2_PLL_LOOP_CTRL to value 0"]
impl crate::Resettable for DDR2_PLL_LOOP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
