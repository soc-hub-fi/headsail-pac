#[doc = "Register `timer2_cmp` reader"]
pub struct R(crate::R<TIMER2_CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer2_cmp` writer"]
pub struct W(crate::W<TIMER2_CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_CMP_SPEC>;
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
impl From<crate::W<TIMER2_CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmp` reader - "]
pub type CMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `cmp` writer - "]
pub type CMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER2_CMP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<0> {
        CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer compare register for timer 2. Writing this will zero timer register for timer 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_cmp](index.html) module"]
pub struct TIMER2_CMP_SPEC;
impl crate::RegisterSpec for TIMER2_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_cmp::R](R) reader structure"]
impl crate::Readable for TIMER2_CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2_cmp::W](W) writer structure"]
impl crate::Writable for TIMER2_CMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets timer2_cmp to value 0"]
impl crate::Resettable for TIMER2_CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
