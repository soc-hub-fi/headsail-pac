#[doc = "Register `SPIDUM` reader"]
pub struct R(crate::R<SPIDUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIDUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIDUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIDUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIDUM` writer"]
pub struct W(crate::W<SPIDUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIDUM_SPEC>;
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
impl From<crate::W<SPIDUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIDUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUMMYRD` reader - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and reading the data."]
pub type DUMMYRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUMMYRD` writer - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and reading the data."]
pub type DUMMYRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIDUM_SPEC, u16, u16, 16, O>;
#[doc = "Field `DUMMYWR` reader - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and writing the data."]
pub type DUMMYWR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUMMYWR` writer - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and writing the data."]
pub type DUMMYWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIDUM_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and reading the data."]
    #[inline(always)]
    pub fn dummyrd(&self) -> DUMMYRD_R {
        DUMMYRD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and writing the data."]
    #[inline(always)]
    pub fn dummywr(&self) -> DUMMYWR_R {
        DUMMYWR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and reading the data."]
    #[inline(always)]
    #[must_use]
    pub fn dummyrd(&mut self) -> DUMMYRD_W<0> {
        DUMMYRD_W::new(self)
    }
    #[doc = "Bits 16:31 - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and writing the data."]
    #[inline(always)]
    #[must_use]
    pub fn dummywr(&mut self) -> DUMMYWR_W<16> {
        DUMMYWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Dummy Cycles\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spidum](index.html) module"]
pub struct SPIDUM_SPEC;
impl crate::RegisterSpec for SPIDUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spidum::R](R) reader structure"]
impl crate::Readable for SPIDUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spidum::W](W) writer structure"]
impl crate::Writable for SPIDUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPIDUM to value 0"]
impl crate::Resettable for SPIDUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
