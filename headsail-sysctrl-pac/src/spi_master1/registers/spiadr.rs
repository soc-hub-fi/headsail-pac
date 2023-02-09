#[doc = "Register `SPIADR` reader"]
pub struct R(crate::R<SPIADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIADR` writer"]
pub struct W(crate::W<SPIADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIADR_SPEC>;
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
impl From<crate::W<SPIADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIADR` reader - "]
pub type SPIADR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPIADR` writer - "]
pub type SPIADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIADR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spiadr(&self) -> SPIADR_R {
        SPIADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn spiadr(&mut self) -> SPIADR_W<0> {
        SPIADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spiadr](index.html) module"]
pub struct SPIADR_SPEC;
impl crate::RegisterSpec for SPIADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spiadr::R](R) reader structure"]
impl crate::Readable for SPIADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spiadr::W](W) writer structure"]
impl crate::Writable for SPIADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPIADR to value 0"]
impl crate::Resettable for SPIADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
