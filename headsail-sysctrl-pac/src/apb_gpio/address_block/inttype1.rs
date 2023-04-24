#[doc = "Register `INTTYPE1` reader"]
pub struct R(crate::R<INTTYPE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTYPE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTYPE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTYPE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTYPE1` writer"]
pub struct W(crate::W<INTTYPE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTYPE1_SPEC>;
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
impl From<crate::W<INTTYPE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTYPE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTTYPE1` reader - "]
pub type INTTYPE1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTTYPE1` writer - "]
pub type INTTYPE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTTYPE1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inttype1(&self) -> INTTYPE1_R {
        INTTYPE1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn inttype1(&mut self) -> INTTYPE1_W<0> {
        INTTYPE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttype1](index.html) module"]
pub struct INTTYPE1_SPEC;
impl crate::RegisterSpec for INTTYPE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttype1::R](R) reader structure"]
impl crate::Readable for INTTYPE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttype1::W](W) writer structure"]
impl crate::Writable for INTTYPE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTTYPE1 to value 0"]
impl crate::Resettable for INTTYPE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
