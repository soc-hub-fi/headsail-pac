#[doc = "Register `plic_base` reader"]
pub struct R(crate::R<PLIC_BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLIC_BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLIC_BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLIC_BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `plic_base` writer"]
pub struct W(crate::W<PLIC_BASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLIC_BASE_SPEC>;
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
impl From<crate::W<PLIC_BASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLIC_BASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `base` reader - "]
pub type BASE_R = crate::FieldReader<u64, u64>;
#[doc = "Field `base` writer - "]
pub type BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u64, PLIC_BASE_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<0> {
        BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plic_base](index.html) module"]
pub struct PLIC_BASE_SPEC;
impl crate::RegisterSpec for PLIC_BASE_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [plic_base::R](R) reader structure"]
impl crate::Readable for PLIC_BASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plic_base::W](W) writer structure"]
impl crate::Writable for PLIC_BASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets plic_base to value 0x0008_0000"]
impl crate::Resettable for PLIC_BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0000;
}
