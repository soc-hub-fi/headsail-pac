#[doc = "Register `SPILEN` reader"]
pub struct R(crate::R<SPILEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPILEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPILEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPILEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPILEN` writer"]
pub struct W(crate::W<SPILEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPILEN_SPEC>;
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
impl From<crate::W<SPILEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPILEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDLEN` reader - The number of bits of the SPI command that should be sent."]
pub type CMDLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDLEN` writer - The number of bits of the SPI command that should be sent."]
pub type CMDLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPILEN_SPEC, u8, u8, 6, O>;
#[doc = "Field `ADDRLEN` reader - The number of bits of the SPI address that should be sent."]
pub type ADDRLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRLEN` writer - The number of bits of the SPI address that should be sent."]
pub type ADDRLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPILEN_SPEC, u8, u8, 6, O>;
#[doc = "Field `DATALEN` reader - The number of bits read or written. Note that first the SPI command and address are"]
pub type DATALEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATALEN` writer - The number of bits read or written. Note that first the SPI command and address are"]
pub type DATALEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPILEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:5 - The number of bits of the SPI command that should be sent."]
    #[inline(always)]
    pub fn cmdlen(&self) -> CMDLEN_R {
        CMDLEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The number of bits of the SPI address that should be sent."]
    #[inline(always)]
    pub fn addrlen(&self) -> ADDRLEN_R {
        ADDRLEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - The number of bits read or written. Note that first the SPI command and address are"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - The number of bits of the SPI command that should be sent."]
    #[inline(always)]
    #[must_use]
    pub fn cmdlen(&mut self) -> CMDLEN_W<0> {
        CMDLEN_W::new(self)
    }
    #[doc = "Bits 8:13 - The number of bits of the SPI address that should be sent."]
    #[inline(always)]
    #[must_use]
    pub fn addrlen(&mut self) -> ADDRLEN_W<8> {
        ADDRLEN_W::new(self)
    }
    #[doc = "Bits 16:31 - The number of bits read or written. Note that first the SPI command and address are"]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DATALEN_W<16> {
        DATALEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Transfer Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spilen](index.html) module"]
pub struct SPILEN_SPEC;
impl crate::RegisterSpec for SPILEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spilen::R](R) reader structure"]
impl crate::Readable for SPILEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spilen::W](W) writer structure"]
impl crate::Writable for SPILEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPILEN to value 0"]
impl crate::Resettable for SPILEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
