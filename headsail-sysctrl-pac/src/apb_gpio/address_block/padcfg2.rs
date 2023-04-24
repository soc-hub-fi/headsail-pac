#[doc = "Register `PADCFG2` reader"]
pub struct R(crate::R<PADCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG2` writer"]
pub struct W(crate::W<PADCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG2_SPEC>;
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
impl From<crate::W<PADCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADCFG2` reader - "]
pub type PADCFG2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PADCFG2` writer - "]
pub type PADCFG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PADCFG2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg2(&self) -> PADCFG2_R {
        PADCFG2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg2(&mut self) -> PADCFG2_W<0> {
        PADCFG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg2](index.html) module"]
pub struct PADCFG2_SPEC;
impl crate::RegisterSpec for PADCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg2::R](R) reader structure"]
impl crate::Readable for PADCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg2::W](W) writer structure"]
impl crate::Writable for PADCFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCFG2 to value 0"]
impl crate::Resettable for PADCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
