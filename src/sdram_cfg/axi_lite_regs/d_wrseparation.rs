#[doc = "Register `d_wrseparation` reader"]
pub struct R(crate::R<D_WRSEPARATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WRSEPARATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WRSEPARATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WRSEPARATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `d_wrseparation` writer"]
pub struct W(crate::W<D_WRSEPARATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_WRSEPARATION_SPEC>;
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
impl From<crate::W<D_WRSEPARATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_WRSEPARATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `d_wrseparation` reader - "]
pub type D_WRSEPARATION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `d_wrseparation` writer - "]
pub type D_WRSEPARATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_WRSEPARATION_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_wrseparation(&self) -> D_WRSEPARATION_R {
        D_WRSEPARATION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn d_wrseparation(&mut self) -> D_WRSEPARATION_W<0> {
        D_WRSEPARATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_wrseparation](index.html) module"]
pub struct D_WRSEPARATION_SPEC;
impl crate::RegisterSpec for D_WRSEPARATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_wrseparation::R](R) reader structure"]
impl crate::Readable for D_WRSEPARATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_wrseparation::W](W) writer structure"]
impl crate::Writable for D_WRSEPARATION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets d_wrseparation to value 0"]
impl crate::Resettable for D_WRSEPARATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
