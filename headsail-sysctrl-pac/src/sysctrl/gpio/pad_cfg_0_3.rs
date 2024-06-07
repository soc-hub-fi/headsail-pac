#[doc = "Register `PAD_CFG_0_3` reader"]
pub type R = crate::R<PadCfg0_3Spec>;
#[doc = "Register `PAD_CFG_0_3` writer"]
pub type W = crate::W<PadCfg0_3Spec>;
#[doc = "Field `PAD_CFG_0` reader - "]
pub type PadCfg0R = crate::FieldReader;
#[doc = "Field `PAD_CFG_0` writer - "]
pub type PadCfg0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_1` reader - "]
pub type PadCfg1R = crate::FieldReader;
#[doc = "Field `PAD_CFG_1` writer - "]
pub type PadCfg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_2` reader - "]
pub type PadCfg2R = crate::FieldReader;
#[doc = "Field `PAD_CFG_2` writer - "]
pub type PadCfg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_3` reader - "]
pub type PadCfg3R = crate::FieldReader;
#[doc = "Field `PAD_CFG_3` writer - "]
pub type PadCfg3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_0(&self) -> PadCfg0R {
        PadCfg0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_1(&self) -> PadCfg1R {
        PadCfg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_2(&self) -> PadCfg2R {
        PadCfg2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_3(&self) -> PadCfg3R {
        PadCfg3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_CFG_0_3")
            .field("pad_cfg_0", &self.pad_cfg_0())
            .field("pad_cfg_1", &self.pad_cfg_1())
            .field("pad_cfg_2", &self.pad_cfg_2())
            .field("pad_cfg_3", &self.pad_cfg_3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_0(&mut self) -> PadCfg0W<PadCfg0_3Spec> {
        PadCfg0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_1(&mut self) -> PadCfg1W<PadCfg0_3Spec> {
        PadCfg1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_2(&mut self) -> PadCfg2W<PadCfg0_3Spec> {
        PadCfg2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_3(&mut self) -> PadCfg3W<PadCfg0_3Spec> {
        PadCfg3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_0_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_0_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadCfg0_3Spec;
impl crate::RegisterSpec for PadCfg0_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg_0_3::R`](R) reader structure"]
impl crate::Readable for PadCfg0_3Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg_0_3::W`](W) writer structure"]
impl crate::Writable for PadCfg0_3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_CFG_0_3 to value 0"]
impl crate::Resettable for PadCfg0_3Spec {
    const RESET_VALUE: u32 = 0;
}
