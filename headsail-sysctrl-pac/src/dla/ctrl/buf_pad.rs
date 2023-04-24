#[doc = "Register `buf_pad` reader"]
pub struct R(crate::R<BUF_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `buf_pad` writer"]
pub struct W(crate::W<BUF_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_PAD_SPEC>;
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
impl From<crate::W<BUF_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `top` reader - "]
pub type TOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `top` writer - "]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_PAD_SPEC, u8, u8, 4, O>;
#[doc = "Field `right` reader - "]
pub type RIGHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `right` writer - "]
pub type RIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_PAD_SPEC, u8, u8, 4, O>;
#[doc = "Field `bottom` reader - "]
pub type BOTTOM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bottom` writer - "]
pub type BOTTOM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_PAD_SPEC, u8, u8, 4, O>;
#[doc = "Field `left` reader - "]
pub type LEFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `left` writer - "]
pub type LEFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_PAD_SPEC, u8, u8, 4, O>;
#[doc = "Field `value` reader - "]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `value` writer - "]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_PAD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn right(&self) -> RIGHT_R {
        RIGHT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn bottom(&self) -> BOTTOM_R {
        BOTTOM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn left(&self) -> LEFT_R {
        LEFT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<0> {
        TOP_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn right(&mut self) -> RIGHT_W<4> {
        RIGHT_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn bottom(&mut self) -> BOTTOM_W<8> {
        BOTTOM_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn left(&mut self) -> LEFT_W<12> {
        LEFT_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<16> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_pad](index.html) module"]
pub struct BUF_PAD_SPEC;
impl crate::RegisterSpec for BUF_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_pad::R](R) reader structure"]
impl crate::Readable for BUF_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_pad::W](W) writer structure"]
impl crate::Writable for BUF_PAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets buf_pad to value 0"]
impl crate::Resettable for BUF_PAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
