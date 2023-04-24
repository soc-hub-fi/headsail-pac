#[doc = "Register `INTTYPE0` reader"]
pub struct R(crate::R<INTTYPE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTYPE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTYPE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTYPE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTYPE0` writer"]
pub struct W(crate::W<INTTYPE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTYPE0_SPEC>;
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
impl From<crate::W<INTTYPE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTYPE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTTYPE0` reader - "]
pub type INTTYPE0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTTYPE0` writer - "]
pub type INTTYPE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTTYPE0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inttype0(&self) -> INTTYPE0_R {
        INTTYPE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn inttype0(&mut self) -> INTTYPE0_W<0> {
        INTTYPE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttype0](index.html) module"]
pub struct INTTYPE0_SPEC;
impl crate::RegisterSpec for INTTYPE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttype0::R](R) reader structure"]
impl crate::Readable for INTTYPE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttype0::W](W) writer structure"]
impl crate::Writable for INTTYPE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTTYPE0 to value 0"]
impl crate::Resettable for INTTYPE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
