#[doc = "Register `DLA_PLL_TMUX_SEL` reader"]
pub type R = crate::R<DLA_PLL_TMUX_SEL_SPEC>;
#[doc = "Register `DLA_PLL_TMUX_SEL` writer"]
pub type W = crate::W<DLA_PLL_TMUX_SEL_SPEC>;
#[doc = "Field `Tmux_1` reader - "]
pub type TMUX_1_R = crate::FieldReader;
#[doc = "Field `Tmux_1` writer - "]
pub type TMUX_1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Tmux_2` reader - "]
pub type TMUX_2_R = crate::FieldReader;
#[doc = "Field `Tmux_2` writer - "]
pub type TMUX_2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tmux_1(&self) -> TMUX_1_R {
        TMUX_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tmux_2(&self) -> TMUX_2_R {
        TMUX_2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLA_PLL_TMUX_SEL")
            .field("tmux_1", &self.tmux_1())
            .field("tmux_2", &self.tmux_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_1(&mut self) -> TMUX_1_W<DLA_PLL_TMUX_SEL_SPEC> {
        TMUX_1_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_2(&mut self) -> TMUX_2_W<DLA_PLL_TMUX_SEL_SPEC> {
        TMUX_2_W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_pll_tmux_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_pll_tmux_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLA_PLL_TMUX_SEL_SPEC;
impl crate::RegisterSpec for DLA_PLL_TMUX_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dla_pll_tmux_sel::R`](R) reader structure"]
impl crate::Readable for DLA_PLL_TMUX_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dla_pll_tmux_sel::W`](W) writer structure"]
impl crate::Writable for DLA_PLL_TMUX_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLA_PLL_TMUX_SEL to value 0"]
impl crate::Resettable for DLA_PLL_TMUX_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
