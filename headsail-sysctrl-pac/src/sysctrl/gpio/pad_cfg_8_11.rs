#[doc = "Register `PAD_CFG_8_11` reader"]
pub type R = crate::R<PadCfg8_11Spec>;
#[doc = "Register `PAD_CFG_8_11` writer"]
pub type W = crate::W<PadCfg8_11Spec>;
#[doc = "Field `PAD_CFG_8` reader - "]
pub type PadCfg8R = crate::FieldReader;
#[doc = "Field `PAD_CFG_8` writer - "]
pub type PadCfg8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_9` reader - "]
pub type PadCfg9R = crate::FieldReader;
#[doc = "Field `PAD_CFG_9` writer - "]
pub type PadCfg9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_10` reader - "]
pub type PadCfg10R = crate::FieldReader;
#[doc = "Field `PAD_CFG_10` writer - "]
pub type PadCfg10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_11` reader - "]
pub type PadCfg11R = crate::FieldReader;
#[doc = "Field `PAD_CFG_11` writer - "]
pub type PadCfg11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_8(&self) -> PadCfg8R {
        PadCfg8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_9(&self) -> PadCfg9R {
        PadCfg9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_10(&self) -> PadCfg10R {
        PadCfg10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_11(&self) -> PadCfg11R {
        PadCfg11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_CFG_8_11")
            .field("pad_cfg_8", &self.pad_cfg_8())
            .field("pad_cfg_9", &self.pad_cfg_9())
            .field("pad_cfg_10", &self.pad_cfg_10())
            .field("pad_cfg_11", &self.pad_cfg_11())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_8(&mut self) -> PadCfg8W<PadCfg8_11Spec> {
        PadCfg8W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_9(&mut self) -> PadCfg9W<PadCfg8_11Spec> {
        PadCfg9W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_10(&mut self) -> PadCfg10W<PadCfg8_11Spec> {
        PadCfg10W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_11(&mut self) -> PadCfg11W<PadCfg8_11Spec> {
        PadCfg11W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_8_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_8_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadCfg8_11Spec;
impl crate::RegisterSpec for PadCfg8_11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg_8_11::R`](R) reader structure"]
impl crate::Readable for PadCfg8_11Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg_8_11::W`](W) writer structure"]
impl crate::Writable for PadCfg8_11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_CFG_8_11 to value 0"]
impl crate::Resettable for PadCfg8_11Spec {
    const RESET_VALUE: u32 = 0;
}
