#[doc = "Register `buf_kernel0` reader"]
pub struct R(crate::R<BUF_KERNEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_KERNEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_KERNEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_KERNEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `buf_kernel0` writer"]
pub struct W(crate::W<BUF_KERNEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_KERNEL0_SPEC>;
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
impl From<crate::W<BUF_KERNEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_KERNEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `width` reader - "]
pub type WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `width` writer - "]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_KERNEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `height` reader - "]
pub type HEIGHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `height` writer - "]
pub type HEIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_KERNEL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `s_channels` reader - "]
pub type S_CHANNELS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `s_channels` writer - "]
pub type S_CHANNELS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUF_KERNEL0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:19"]
    #[inline(always)]
    pub fn s_channels(&self) -> S_CHANNELS_R {
        S_CHANNELS_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<4> {
        HEIGHT_W::new(self)
    }
    #[doc = "Bits 8:19"]
    #[inline(always)]
    #[must_use]
    pub fn s_channels(&mut self) -> S_CHANNELS_W<8> {
        S_CHANNELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_kernel0](index.html) module"]
pub struct BUF_KERNEL0_SPEC;
impl crate::RegisterSpec for BUF_KERNEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_kernel0::R](R) reader structure"]
impl crate::Readable for BUF_KERNEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_kernel0::W](W) writer structure"]
impl crate::Writable for BUF_KERNEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets buf_kernel0 to value 0"]
impl crate::Resettable for BUF_KERNEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
