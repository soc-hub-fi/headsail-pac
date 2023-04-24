#[doc = "Register `buf_input` reader"]
pub struct R(crate::R<BUF_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `buf_input` writer"]
pub struct W(crate::W<BUF_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_INPUT_SPEC>;
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
impl From<crate::W<BUF_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `width` reader - "]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `width` writer - "]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_INPUT_SPEC, u16, u16, 9, O>;
#[doc = "Field `height` reader - "]
pub type HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `height` writer - "]
pub type HEIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_INPUT_SPEC, u16, u16, 9, O>;
#[doc = "Field `channels` reader - "]
pub type CHANNELS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `channels` writer - "]
pub type CHANNELS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_INPUT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:29"]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new(((self.bits >> 18) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<9> {
        HEIGHT_W::new(self)
    }
    #[doc = "Bits 18:29"]
    #[inline(always)]
    #[must_use]
    pub fn channels(&mut self) -> CHANNELS_W<18> {
        CHANNELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_input](index.html) module"]
pub struct BUF_INPUT_SPEC;
impl crate::RegisterSpec for BUF_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_input::R](R) reader structure"]
impl crate::Readable for BUF_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_input::W](W) writer structure"]
impl crate::Writable for BUF_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets buf_input to value 0"]
impl crate::Resettable for BUF_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
