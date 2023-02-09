#[doc = "Register `IER_DLM` reader"]
pub struct R(crate::R<IER_DLM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_DLM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_DLM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_DLM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER_DLM` writer"]
pub struct W(crate::W<IER_DLM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_DLM_SPEC>;
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
impl From<crate::W<IER_DLM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_DLM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IER_DLM` reader - "]
pub type IER_DLM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IER_DLM` writer - "]
pub type IER_DLM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, IER_DLM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ier_dlm(&self) -> IER_DLM_R {
        IER_DLM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ier_dlm(&mut self) -> IER_DLM_W<0> {
        IER_DLM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IER interrupt enable, DLM divisor latch MSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier_dlm](index.html) module"]
pub struct IER_DLM_SPEC;
impl crate::RegisterSpec for IER_DLM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ier_dlm::R](R) reader structure"]
impl crate::Readable for IER_DLM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier_dlm::W](W) writer structure"]
impl crate::Writable for IER_DLM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER_DLM to value 0"]
impl crate::Resettable for IER_DLM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
