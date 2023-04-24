#[doc = "Register `cycle_count_bp` reader"]
pub struct R(crate::R<CYCLE_COUNT_BP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCLE_COUNT_BP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCLE_COUNT_BP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCLE_COUNT_BP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cycle_count_bp` writer"]
pub struct W(crate::W<CYCLE_COUNT_BP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CYCLE_COUNT_BP_SPEC>;
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
impl From<crate::W<CYCLE_COUNT_BP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CYCLE_COUNT_BP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bp` reader - "]
pub type BP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `bp` writer - "]
pub type BP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CYCLE_COUNT_BP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BP_W<0> {
        BP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cycle count breakpoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle_count_bp](index.html) module"]
pub struct CYCLE_COUNT_BP_SPEC;
impl crate::RegisterSpec for CYCLE_COUNT_BP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cycle_count_bp::R](R) reader structure"]
impl crate::Readable for CYCLE_COUNT_BP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cycle_count_bp::W](W) writer structure"]
impl crate::Writable for CYCLE_COUNT_BP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cycle_count_bp to value 0"]
impl crate::Resettable for CYCLE_COUNT_BP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
