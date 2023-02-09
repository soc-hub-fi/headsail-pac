#[doc = "Register `LANE0_CONFIG` reader"]
pub struct R(crate::R<LANE0_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LANE0_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LANE0_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LANE0_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LANE0_CONFIG` writer"]
pub struct W(crate::W<LANE0_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LANE0_CONFIG_SPEC>;
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
impl From<crate::W<LANE0_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LANE0_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LANE0_CONFIG` reader - "]
pub type LANE0_CONFIG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LANE0_CONFIG` writer - "]
pub type LANE0_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LANE0_CONFIG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lane0_config(&self) -> LANE0_CONFIG_R {
        LANE0_CONFIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lane0_config(&mut self) -> LANE0_CONFIG_W<0> {
        LANE0_CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lane0_config](index.html) module"]
pub struct LANE0_CONFIG_SPEC;
impl crate::RegisterSpec for LANE0_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lane0_config::R](R) reader structure"]
impl crate::Readable for LANE0_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lane0_config::W](W) writer structure"]
impl crate::Writable for LANE0_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LANE0_CONFIG to value 0"]
impl crate::Resettable for LANE0_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
