#[doc = "Register `PAD_CFG_8_11` reader"]
pub type R = crate::R<PAD_CFG_8_11_SPEC>;
#[doc = "Register `PAD_CFG_8_11` writer"]
pub type W = crate::W<PAD_CFG_8_11_SPEC>;
#[doc = "Field `PAD_CFG_8` reader - "]
pub type PAD_CFG_8_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_8` writer - "]
pub type PAD_CFG_8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_9` reader - "]
pub type PAD_CFG_9_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_9` writer - "]
pub type PAD_CFG_9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_10` reader - "]
pub type PAD_CFG_10_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_10` writer - "]
pub type PAD_CFG_10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_11` reader - "]
pub type PAD_CFG_11_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_11` writer - "]
pub type PAD_CFG_11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_8(&self) -> PAD_CFG_8_R {
        PAD_CFG_8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_9(&self) -> PAD_CFG_9_R {
        PAD_CFG_9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_10(&self) -> PAD_CFG_10_R {
        PAD_CFG_10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_11(&self) -> PAD_CFG_11_R {
        PAD_CFG_11_R::new(((self.bits >> 24) & 0xff) as u8)
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
    pub fn pad_cfg_8(&mut self) -> PAD_CFG_8_W<PAD_CFG_8_11_SPEC> {
        PAD_CFG_8_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_9(&mut self) -> PAD_CFG_9_W<PAD_CFG_8_11_SPEC> {
        PAD_CFG_9_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_10(&mut self) -> PAD_CFG_10_W<PAD_CFG_8_11_SPEC> {
        PAD_CFG_10_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_11(&mut self) -> PAD_CFG_11_W<PAD_CFG_8_11_SPEC> {
        PAD_CFG_11_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_8_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_8_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_CFG_8_11_SPEC;
impl crate::RegisterSpec for PAD_CFG_8_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg_8_11::R`](R) reader structure"]
impl crate::Readable for PAD_CFG_8_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg_8_11::W`](W) writer structure"]
impl crate::Writable for PAD_CFG_8_11_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_CFG_8_11 to value 0"]
impl crate::Resettable for PAD_CFG_8_11_SPEC {
    const RESET_VALUE: u32 = 0;
}
