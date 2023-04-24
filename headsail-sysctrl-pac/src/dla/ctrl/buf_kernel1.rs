#[doc = "Register `buf_kernel1` reader"]
pub struct R(crate::R<BUF_KERNEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_KERNEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_KERNEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_KERNEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `buf_kernel1` writer"]
pub struct W(crate::W<BUF_KERNEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_KERNEL1_SPEC>;
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
impl From<crate::W<BUF_KERNEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_KERNEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `num` reader - "]
pub type NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `num` writer - "]
pub type NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_KERNEL1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NUM_W<0> {
        NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_kernel1](index.html) module"]
pub struct BUF_KERNEL1_SPEC;
impl crate::RegisterSpec for BUF_KERNEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_kernel1::R](R) reader structure"]
impl crate::Readable for BUF_KERNEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_kernel1::W](W) writer structure"]
impl crate::Writable for BUF_KERNEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets buf_kernel1 to value 0"]
impl crate::Resettable for BUF_KERNEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
