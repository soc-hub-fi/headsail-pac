#[doc = "Register `sdio_sddata3` reader"]
pub type R = crate::R<SdioSddata3Spec>;
#[doc = "Register `sdio_sddata3` writer"]
pub type W = crate::W<SdioSddata3Spec>;
#[doc = "Field `DS0` reader - "]
pub type Ds0R = crate::BitReader;
#[doc = "Field `DS0` writer - "]
pub type Ds0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DS1` reader - "]
pub type Ds1R = crate::BitReader;
#[doc = "Field `DS1` writer - "]
pub type Ds1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST0` reader - "]
pub type St0R = crate::BitReader;
#[doc = "Field `ST0` writer - "]
pub type St0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST1` reader - "]
pub type St1R = crate::BitReader;
#[doc = "Field `ST1` writer - "]
pub type St1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATE_CONTROL` reader - "]
pub type RateControlR = crate::BitReader;
#[doc = "Field `RATE_CONTROL` writer - "]
pub type RateControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_EN` reader - "]
pub type OutputEnR = crate::BitReader;
#[doc = "Field `OUTPUT_EN` writer - "]
pub type OutputEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOLD` reader - "]
pub type HoldR = crate::BitReader;
#[doc = "Field `HOLD` writer - "]
pub type HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULL_ENABLE` reader - "]
pub type PullEnableR = crate::BitReader;
#[doc = "Field `PULL_ENABLE` writer - "]
pub type PullEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULL_DIR` reader - "]
pub type PullDirR = crate::BitReader;
#[doc = "Field `PULL_DIR` writer - "]
pub type PullDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_EN` reader - "]
pub type InputEnR = crate::BitReader;
#[doc = "Field `INPUT_EN` writer - "]
pub type InputEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ds0(&self) -> Ds0R {
        Ds0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ds1(&self) -> Ds1R {
        Ds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn st0(&self) -> St0R {
        St0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn st1(&self) -> St1R {
        St1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rate_control(&self) -> RateControlR {
        RateControlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_en(&self) -> OutputEnR {
        OutputEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PullEnableR {
        PullEnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pull_dir(&self) -> PullDirR {
        PullDirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn input_en(&self) -> InputEnR {
        InputEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sdio_sddata3")
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("st0", &self.st0())
            .field("st1", &self.st1())
            .field("rate_control", &self.rate_control())
            .field("output_en", &self.output_en())
            .field("hold", &self.hold())
            .field("pull_enable", &self.pull_enable())
            .field("pull_dir", &self.pull_dir())
            .field("input_en", &self.input_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ds0(&mut self) -> Ds0W<SdioSddata3Spec> {
        Ds0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ds1(&mut self) -> Ds1W<SdioSddata3Spec> {
        Ds1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> St0W<SdioSddata3Spec> {
        St0W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> St1W<SdioSddata3Spec> {
        St1W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rate_control(&mut self) -> RateControlW<SdioSddata3Spec> {
        RateControlW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn output_en(&mut self) -> OutputEnW<SdioSddata3Spec> {
        OutputEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hold(&mut self) -> HoldW<SdioSddata3Spec> {
        HoldW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pull_enable(&mut self) -> PullEnableW<SdioSddata3Spec> {
        PullEnableW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pull_dir(&mut self) -> PullDirW<SdioSddata3Spec> {
        PullDirW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn input_en(&mut self) -> InputEnW<SdioSddata3Spec> {
        InputEnW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_sddata3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_sddata3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioSddata3Spec;
impl crate::RegisterSpec for SdioSddata3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_sddata3::R`](R) reader structure"]
impl crate::Readable for SdioSddata3Spec {}
#[doc = "`write(|w| ..)` method takes [`sdio_sddata3::W`](W) writer structure"]
impl crate::Writable for SdioSddata3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sdio_sddata3 to value 0"]
impl crate::Resettable for SdioSddata3Spec {
    const RESET_VALUE: u32 = 0;
}
