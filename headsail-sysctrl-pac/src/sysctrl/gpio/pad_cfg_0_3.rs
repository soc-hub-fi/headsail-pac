#[doc = "Register `PAD_CFG_0_3` reader"]
pub type R = crate::R<PAD_CFG_0_3_SPEC>;
#[doc = "Register `PAD_CFG_0_3` writer"]
pub type W = crate::W<PAD_CFG_0_3_SPEC>;
#[doc = "Field `PAD_CFG_0` reader - "]
pub type PAD_CFG_0_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_0` writer - "]
pub type PAD_CFG_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_1` reader - "]
pub type PAD_CFG_1_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_1` writer - "]
pub type PAD_CFG_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_2` reader - "]
pub type PAD_CFG_2_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_2` writer - "]
pub type PAD_CFG_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_CFG_3` reader - "]
pub type PAD_CFG_3_R = crate::FieldReader;
#[doc = "Field `PAD_CFG_3` writer - "]
pub type PAD_CFG_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_cfg_0(&self) -> PAD_CFG_0_R {
        PAD_CFG_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_cfg_1(&self) -> PAD_CFG_1_R {
        PAD_CFG_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pad_cfg_2(&self) -> PAD_CFG_2_R {
        PAD_CFG_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pad_cfg_3(&self) -> PAD_CFG_3_R {
        PAD_CFG_3_R::new(((self.bits >> 24) & 0xff) as u8)
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
    pub fn pad_cfg_0(&mut self) -> PAD_CFG_0_W<PAD_CFG_0_3_SPEC> {
        PAD_CFG_0_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_1(&mut self) -> PAD_CFG_1_W<PAD_CFG_0_3_SPEC> {
        PAD_CFG_1_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_2(&mut self) -> PAD_CFG_2_W<PAD_CFG_0_3_SPEC> {
        PAD_CFG_2_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_cfg_3(&mut self) -> PAD_CFG_3_W<PAD_CFG_0_3_SPEC> {
        PAD_CFG_3_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_0_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_0_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_CFG_0_3_SPEC;
impl crate::RegisterSpec for PAD_CFG_0_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg_0_3::R`](R) reader structure"]
impl crate::Readable for PAD_CFG_0_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg_0_3::W`](W) writer structure"]
impl crate::Writable for PAD_CFG_0_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_CFG_0_3 to value 0"]
impl crate::Resettable for PAD_CFG_0_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
