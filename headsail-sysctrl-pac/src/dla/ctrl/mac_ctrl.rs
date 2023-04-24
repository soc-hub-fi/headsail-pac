#[doc = "Register `mac_ctrl` reader"]
pub struct R(crate::R<MAC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mac_ctrl` writer"]
pub struct W(crate::W<MAC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_CTRL_SPEC>;
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
impl From<crate::W<MAC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `simdSelect` reader - "]
pub type SIMD_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `simdSelect` writer - "]
pub type SIMD_SELECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `clip` reader - "]
pub type CLIP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clip` writer - "]
pub type CLIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_CTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn simd_select(&self) -> SIMD_SELECT_R {
        SIMD_SELECT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn clip(&self) -> CLIP_R {
        CLIP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn simd_select(&mut self) -> SIMD_SELECT_W<1> {
        SIMD_SELECT_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn clip(&mut self) -> CLIP_W<8> {
        CLIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_ctrl](index.html) module"]
pub struct MAC_CTRL_SPEC;
impl crate::RegisterSpec for MAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_ctrl::R](R) reader structure"]
impl crate::Readable for MAC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_ctrl::W](W) writer structure"]
impl crate::Writable for MAC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mac_ctrl to value 0"]
impl crate::Resettable for MAC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
