#[doc = "Register `SOURCE_LENGTH` reader"]
pub struct R(crate::R<SOURCE_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCE_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCE_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCE_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCE_LENGTH` writer"]
pub struct W(crate::W<SOURCE_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCE_LENGTH_SPEC>;
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
impl From<crate::W<SOURCE_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCE_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Source_Length` reader - "]
pub type SOURCE_LENGTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Source_Length` writer - "]
pub type SOURCE_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCE_LENGTH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn source_length(&self) -> SOURCE_LENGTH_R {
        SOURCE_LENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn source_length(&mut self) -> SOURCE_LENGTH_W<0> {
        SOURCE_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Length of read transfer in bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [source_length](index.html) module"]
pub struct SOURCE_LENGTH_SPEC;
impl crate::RegisterSpec for SOURCE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [source_length::R](R) reader structure"]
impl crate::Readable for SOURCE_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [source_length::W](W) writer structure"]
impl crate::Writable for SOURCE_LENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCE_LENGTH to value 0"]
impl crate::Resettable for SOURCE_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
