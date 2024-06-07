#[doc = "Register `PAD_CFG_4_7` reader"]
pub type R = crate::R<PadCfg4_7Spec>;
#[doc = "Register `PAD_CFG_4_7` writer"]
pub type W = crate::W<PadCfg4_7Spec>;
#[doc = "Field `PAD_CFG_4` reader - "]
pub type PadCfg4R = crate::FieldReader;
#[doc = "Field `PAD_CFG_4` writer - "]
pub type PadCfg4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_5` reader - "]
pub type PadCfg5R = crate::FieldReader;
#[doc = "Field `PAD_CFG_5` writer - "]
pub type PadCfg5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_6` reader - "]
pub type PadCfg6R = crate::FieldReader;
#[doc = "Field `PAD_CFG_6` writer - "]
pub type PadCfg6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_7` reader - "]
pub type PadCfg7R = crate::FieldReader;
#[doc = "Field `PAD_CFG_7` writer - "]
pub type PadCfg7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_4(&self) -> PadCfg4R {
        PadCfg4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_5(&self) -> PadCfg5R {
        PadCfg5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_6(&self) -> PadCfg6R {
        PadCfg6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_7(&self) -> PadCfg7R {
        PadCfg7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_CFG_4_7")
            .field("pad_cfg_4", &self.pad_cfg_4())
            .field("pad_cfg_5", &self.pad_cfg_5())
            .field("pad_cfg_6", &self.pad_cfg_6())
            .field("pad_cfg_7", &self.pad_cfg_7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_4(&mut self) -> PadCfg4W<PadCfg4_7Spec> {
        PadCfg4W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_5(&mut self) -> PadCfg5W<PadCfg4_7Spec> {
        PadCfg5W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_6(&mut self) -> PadCfg6W<PadCfg4_7Spec> {
        PadCfg6W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_7(&mut self) -> PadCfg7W<PadCfg4_7Spec> {
        PadCfg7W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_4_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_4_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadCfg4_7Spec;
impl crate::RegisterSpec for PadCfg4_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg_4_7::R`](R) reader structure"]
impl crate::Readable for PadCfg4_7Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg_4_7::W`](W) writer structure"]
impl crate::Writable for PadCfg4_7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_CFG_4_7 to value 0"]
impl crate::Resettable for PadCfg4_7Spec {
    const RESET_VALUE: u32 = 0;
}
