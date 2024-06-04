#[doc = "Register `pp_ptrl` reader"]
pub type R = crate::R<PP_PTRL_SPEC>;
#[doc = "Register `pp_ptrl` writer"]
pub type W = crate::W<PP_PTRL_SPEC>;
#[doc = "Field `active_mode` reader - "]
pub type ACTIVE_MODE_R = crate::FieldReader;
#[doc = "Field `active_mode` writer - "]
pub type ACTIVE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reLu` reader - "]
pub type RE_LU_R = crate::FieldReader;
#[doc = "Field `reLu` writer - "]
pub type RE_LU_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `max` reader - "]
pub type MAX_R = crate::FieldReader;
#[doc = "Field `max` writer - "]
pub type MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pp_select` reader - "]
pub type PP_SELECT_R = crate::BitReader;
#[doc = "Field `pp_select` writer - "]
pub type PP_SELECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pool_mode` reader - "]
pub type POOL_MODE_R = crate::FieldReader;
#[doc = "Field `pool_mode` writer - "]
pub type POOL_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clip` reader - "]
pub type CLIP_R = crate::FieldReader;
#[doc = "Field `clip` writer - "]
pub type CLIP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn active_mode(&self) -> ACTIVE_MODE_R {
        ACTIVE_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn re_lu(&self) -> RE_LU_R {
        RE_LU_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pp_select(&self) -> PP_SELECT_R {
        PP_SELECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn pool_mode(&self) -> POOL_MODE_R {
        POOL_MODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn clip(&self) -> CLIP_R {
        CLIP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pp_ptrl")
            .field("active_mode", &self.active_mode())
            .field("re_lu", &self.re_lu())
            .field("max", &self.max())
            .field("pp_select", &self.pp_select())
            .field("pool_mode", &self.pool_mode())
            .field("clip", &self.clip())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn active_mode(&mut self) -> ACTIVE_MODE_W<PP_PTRL_SPEC> {
        ACTIVE_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn re_lu(&mut self) -> RE_LU_W<PP_PTRL_SPEC> {
        RE_LU_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn max(&mut self) -> MAX_W<PP_PTRL_SPEC> {
        MAX_W::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pp_select(&mut self) -> PP_SELECT_W<PP_PTRL_SPEC> {
        PP_SELECT_W::new(self, 6)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn pool_mode(&mut self) -> POOL_MODE_W<PP_PTRL_SPEC> {
        POOL_MODE_W::new(self, 7)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn clip(&mut self) -> CLIP_W<PP_PTRL_SPEC> {
        CLIP_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_ptrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_ptrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_PTRL_SPEC;
impl crate::RegisterSpec for PP_PTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp_ptrl::R`](R) reader structure"]
impl crate::Readable for PP_PTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp_ptrl::W`](W) writer structure"]
impl crate::Writable for PP_PTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pp_ptrl to value 0"]
impl crate::Resettable for PP_PTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
