#[doc = "Register `POWEREVENT` reader"]
pub struct R(crate::R<POWEREVENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWEREVENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWEREVENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWEREVENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWEREVENT` writer"]
pub struct W(crate::W<POWEREVENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWEREVENT_SPEC>;
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
impl From<crate::W<POWEREVENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWEREVENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWEREVENT` reader - "]
pub type POWEREVENT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `POWEREVENT` writer - "]
pub type POWEREVENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, POWEREVENT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn powerevent(&self) -> POWEREVENT_R {
        POWEREVENT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn powerevent(&mut self) -> POWEREVENT_W<0> {
        POWEREVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerevent](index.html) module"]
pub struct POWEREVENT_SPEC;
impl crate::RegisterSpec for POWEREVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [powerevent::R](R) reader structure"]
impl crate::Readable for POWEREVENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [powerevent::W](W) writer structure"]
impl crate::Writable for POWEREVENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWEREVENT to value 0"]
impl crate::Resettable for POWEREVENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
