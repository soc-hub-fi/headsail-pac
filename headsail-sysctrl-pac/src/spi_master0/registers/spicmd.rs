#[doc = "Register `SPICMD` reader"]
pub struct R(crate::R<SPICMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPICMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPICMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPICMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPICMD` writer"]
pub struct W(crate::W<SPICMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPICMD_SPEC>;
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
impl From<crate::W<SPICMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPICMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPICMD` reader - "]
pub type SPICMD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPICMD` writer - "]
pub type SPICMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPICMD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spicmd(&self) -> SPICMD_R {
        SPICMD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn spicmd(&mut self) -> SPICMD_W<0> {
        SPICMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spicmd](index.html) module"]
pub struct SPICMD_SPEC;
impl crate::RegisterSpec for SPICMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spicmd::R](R) reader structure"]
impl crate::Readable for SPICMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spicmd::W](W) writer structure"]
impl crate::Writable for SPICMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPICMD to value 0"]
impl crate::Resettable for SPICMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
