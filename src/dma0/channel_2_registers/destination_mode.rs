#[doc = "Register `DESTINATION_MODE` reader"]
pub struct R(crate::R<DESTINATION_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATION_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATION_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATION_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESTINATION_MODE` writer"]
pub struct W(crate::W<DESTINATION_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATION_MODE_SPEC>;
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
impl From<crate::W<DESTINATION_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATION_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Destination_Mode` reader - "]
pub type DESTINATION_MODE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Destination_Mode` writer - "]
pub type DESTINATION_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATION_MODE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn destination_mode(&self) -> DESTINATION_MODE_R {
        DESTINATION_MODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn destination_mode(&mut self) -> DESTINATION_MODE_W<0> {
        DESTINATION_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destination_mode](index.html) module"]
pub struct DESTINATION_MODE_SPEC;
impl crate::RegisterSpec for DESTINATION_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destination_mode::R](R) reader structure"]
impl crate::Readable for DESTINATION_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destination_mode::W](W) writer structure"]
impl crate::Writable for DESTINATION_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATION_MODE to value 0"]
impl crate::Resettable for DESTINATION_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
