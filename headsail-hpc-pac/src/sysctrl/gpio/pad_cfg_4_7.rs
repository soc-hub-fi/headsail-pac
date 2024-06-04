#[doc = "Register `PAD_CFG_4_7` reader"]
pub type R = crate::R<PAD_CFG_4_7_SPEC>;
#[doc = "Register `PAD_CFG_4_7` writer"]
pub type W = crate::W<PAD_CFG_4_7_SPEC>;
#[doc = "Field `PAD_CFG_4` reader - "]
pub type PAD_CFG_4_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_4` writer - "]
pub type PAD_CFG_4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_5` reader - "]
pub type PAD_CFG_5_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_5` writer - "]
pub type PAD_CFG_5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_6` reader - "]
pub type PAD_CFG_6_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_6` writer - "]
pub type PAD_CFG_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_7` reader - "]
pub type PAD_CFG_7_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_7` writer - "]
pub type PAD_CFG_7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_4(&self) -> PAD_CFG_4_R {
        PAD_CFG_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_5(&self) -> PAD_CFG_5_R {
        PAD_CFG_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_6(&self) -> PAD_CFG_6_R {
        PAD_CFG_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_7(&self) -> PAD_CFG_7_R {
        PAD_CFG_7_R::new(((self.bits >> 24) & 0xff) as u8)
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
    pub fn pad_cfg_4(&mut self) -> PAD_CFG_4_W<PAD_CFG_4_7_SPEC> {
        PAD_CFG_4_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_5(&mut self) -> PAD_CFG_5_W<PAD_CFG_4_7_SPEC> {
        PAD_CFG_5_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_6(&mut self) -> PAD_CFG_6_W<PAD_CFG_4_7_SPEC> {
        PAD_CFG_6_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_7(&mut self) -> PAD_CFG_7_W<PAD_CFG_4_7_SPEC> {
        PAD_CFG_7_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_4_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_4_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_CFG_4_7_SPEC;
impl crate::RegisterSpec for PAD_CFG_4_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg_4_7::R`](R) reader structure"]
impl crate::Readable for PAD_CFG_4_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg_4_7::W`](W) writer structure"]
impl crate::Writable for PAD_CFG_4_7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_CFG_4_7 to value 0"]
impl crate::Resettable for PAD_CFG_4_7_SPEC {
    const RESET_VALUE: u32 = 0;
}
