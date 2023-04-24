#[doc = "Register `pp_input` reader"]
pub struct R(crate::R<PP_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pp_input` writer"]
pub struct W(crate::W<PP_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_INPUT_SPEC>;
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
impl From<crate::W<PP_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `width` reader - "]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `width` writer - "]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PP_INPUT_SPEC, u16, u16, 9, O>;
#[doc = "Field `height` reader - "]
pub type HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `height` writer - "]
pub type HEIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PP_INPUT_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<16> {
        HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp_input](index.html) module"]
pub struct PP_INPUT_SPEC;
impl crate::RegisterSpec for PP_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp_input::R](R) reader structure"]
impl crate::Readable for PP_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp_input::W](W) writer structure"]
impl crate::Writable for PP_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pp_input to value 0"]
impl crate::Resettable for PP_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
