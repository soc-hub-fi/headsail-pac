#[doc = "Register `HPC_PLL_TMUX_SEL` reader"]
pub type R = crate::R<HpcPllTmuxSelSpec>;
#[doc = "Register `HPC_PLL_TMUX_SEL` writer"]
pub type W = crate::W<HpcPllTmuxSelSpec>;
#[doc = "Field `Tmux_1` reader - "]
pub type Tmux1R = crate::FieldReader;
#[doc = "Field `Tmux_1` writer - "]
pub type Tmux1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Tmux_2` reader - "]
pub type Tmux2R = crate::FieldReader;
#[doc = "Field `Tmux_2` writer - "]
pub type Tmux2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tmux_1(&self) -> Tmux1R {
        Tmux1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tmux_2(&self) -> Tmux2R {
        Tmux2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPC_PLL_TMUX_SEL")
            .field("tmux_1", &self.tmux_1())
            .field("tmux_2", &self.tmux_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_1(&mut self) -> Tmux1W<HpcPllTmuxSelSpec> {
        Tmux1W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_2(&mut self) -> Tmux2W<HpcPllTmuxSelSpec> {
        Tmux2W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_tmux_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_tmux_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpcPllTmuxSelSpec;
impl crate::RegisterSpec for HpcPllTmuxSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpc_pll_tmux_sel::R`](R) reader structure"]
impl crate::Readable for HpcPllTmuxSelSpec {}
#[doc = "`write(|w| ..)` method takes [`hpc_pll_tmux_sel::W`](W) writer structure"]
impl crate::Writable for HpcPllTmuxSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPC_PLL_TMUX_SEL to value 0"]
impl crate::Resettable for HpcPllTmuxSelSpec {
    const RESET_VALUE: u32 = 0;
}
