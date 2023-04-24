#[doc = "Register `PADCFG4` reader"]
pub struct R(crate::R<PADCFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG4` writer"]
pub struct W(crate::W<PADCFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG4_SPEC>;
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
impl From<crate::W<PADCFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADCFG4` reader - "]
pub type PADCFG4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PADCFG4` writer - "]
pub type PADCFG4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PADCFG4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg4(&self) -> PADCFG4_R {
        PADCFG4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg4(&mut self) -> PADCFG4_W<0> {
        PADCFG4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg4](index.html) module"]
pub struct PADCFG4_SPEC;
impl crate::RegisterSpec for PADCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg4::R](R) reader structure"]
impl crate::Readable for PADCFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg4::W](W) writer structure"]
impl crate::Writable for PADCFG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCFG4 to value 0"]
impl crate::Resettable for PADCFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
