#[doc = "Register `PAD_CFG_12_15` reader"]
pub type R = crate::R<PAD_CFG_12_15_SPEC>;
#[doc = "Register `PAD_CFG_12_15` writer"]
pub type W = crate::W<PAD_CFG_12_15_SPEC>;
#[doc = "Field `PAD_CFG_12` reader - "]
pub type PAD_CFG_12_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_12` writer - "]
pub type PAD_CFG_12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_13` reader - "]
pub type PAD_CFG_13_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_13` writer - "]
pub type PAD_CFG_13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_14` reader - "]
pub type PAD_CFG_14_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_14` writer - "]
pub type PAD_CFG_14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_15` reader - "]
pub type PAD_CFG_15_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_15` writer - "]
pub type PAD_CFG_15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_12(&self) -> PAD_CFG_12_R {
        PAD_CFG_12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_13(&self) -> PAD_CFG_13_R {
        PAD_CFG_13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_14(&self) -> PAD_CFG_14_R {
        PAD_CFG_14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_15(&self) -> PAD_CFG_15_R {
        PAD_CFG_15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_CFG_12_15")
            .field("pad_cfg_12", &self.pad_cfg_12())
            .field("pad_cfg_13", &self.pad_cfg_13())
            .field("pad_cfg_14", &self.pad_cfg_14())
            .field("pad_cfg_15", &self.pad_cfg_15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_12(&mut self) -> PAD_CFG_12_W<PAD_CFG_12_15_SPEC> {
        PAD_CFG_12_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_13(&mut self) -> PAD_CFG_13_W<PAD_CFG_12_15_SPEC> {
        PAD_CFG_13_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_14(&mut self) -> PAD_CFG_14_W<PAD_CFG_12_15_SPEC> {
        PAD_CFG_14_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_15(&mut self) -> PAD_CFG_15_W<PAD_CFG_12_15_SPEC> {
        PAD_CFG_15_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_12_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_12_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_CFG_12_15_SPEC;
impl crate::RegisterSpec for PAD_CFG_12_15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg_12_15::R`](R) reader structure"]
impl crate::Readable for PAD_CFG_12_15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg_12_15::W`](W) writer structure"]
impl crate::Writable for PAD_CFG_12_15_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_CFG_12_15 to value 0"]
impl crate::Resettable for PAD_CFG_12_15_SPEC {
    const RESET_VALUE: u32 = 0;
}
