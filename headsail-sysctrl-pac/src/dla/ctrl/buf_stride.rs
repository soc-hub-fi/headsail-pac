#[doc = "Register `buf_stride` reader"]
pub struct R(crate::R<BUF_STRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_STRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_STRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_STRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `buf_stride` writer"]
pub struct W(crate::W<BUF_STRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_STRIDE_SPEC>;
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
impl From<crate::W<BUF_STRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_STRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `x` reader - "]
pub type X_R = crate::FieldReader<u8, u8>;
#[doc = "Field `x` writer - "]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_STRIDE_SPEC, u8, u8, 4, O>;
#[doc = "Field `y` reader - "]
pub type Y_R = crate::FieldReader<u8, u8>;
#[doc = "Field `y` writer - "]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_STRIDE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<0> {
        X_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<16> {
        Y_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_stride](index.html) module"]
pub struct BUF_STRIDE_SPEC;
impl crate::RegisterSpec for BUF_STRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_stride::R](R) reader structure"]
impl crate::Readable for BUF_STRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_stride::W](W) writer structure"]
impl crate::Writable for BUF_STRIDE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets buf_stride to value 0"]
impl crate::Resettable for BUF_STRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
