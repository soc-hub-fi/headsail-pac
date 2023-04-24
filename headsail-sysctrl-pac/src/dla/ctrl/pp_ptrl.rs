#[doc = "Register `pp_ptrl` reader"]
pub struct R(crate::R<PP_PTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_PTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_PTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_PTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pp_ptrl` writer"]
pub struct W(crate::W<PP_PTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_PTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PP_PTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_PTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `active_mode` reader - "]
pub type ACTIVE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `active_mode` writer - "]
pub type ACTIVE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PP_PTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `reLu` reader - "]
pub type RE_LU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reLu` writer - "]
pub type RE_LU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PP_PTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `max` reader - "]
pub type MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `max` writer - "]
pub type MAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PP_PTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `pp_select` reader - "]
pub type PP_SELECT_R = crate::BitReader<bool>;
#[doc = "Field `pp_select` writer - "]
pub type PP_SELECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PP_PTRL_SPEC, bool, O>;
#[doc = "Field `pool_mode` reader - "]
pub type POOL_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pool_mode` writer - "]
pub type POOL_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PP_PTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `clip` reader - "]
pub type CLIP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clip` writer - "]
pub type CLIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PP_PTRL_SPEC, u8, u8, 5, O>;
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
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn active_mode(&mut self) -> ACTIVE_MODE_W<0> {
        ACTIVE_MODE_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn re_lu(&mut self) -> RE_LU_W<2> {
        RE_LU_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn max(&mut self) -> MAX_W<4> {
        MAX_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pp_select(&mut self) -> PP_SELECT_W<6> {
        PP_SELECT_W::new(self)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn pool_mode(&mut self) -> POOL_MODE_W<7> {
        POOL_MODE_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn clip(&mut self) -> CLIP_W<16> {
        CLIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp_ptrl](index.html) module"]
pub struct PP_PTRL_SPEC;
impl crate::RegisterSpec for PP_PTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp_ptrl::R](R) reader structure"]
impl crate::Readable for PP_PTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp_ptrl::W](W) writer structure"]
impl crate::Writable for PP_PTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pp_ptrl to value 0"]
impl crate::Resettable for PP_PTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
