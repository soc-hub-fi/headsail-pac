#[doc = "Register `prio_thr_2` reader"]
pub struct R(crate::R<PRIO_THR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIO_THR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIO_THR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIO_THR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prio_thr_2` writer"]
pub struct W(crate::W<PRIO_THR_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIO_THR_2_SPEC>;
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
impl From<crate::W<PRIO_THR_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIO_THR_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `thr` reader - "]
pub type THR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `thr` writer - "]
pub type THR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRIO_THR_2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn thr(&self) -> THR_R {
        THR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn thr(&mut self) -> THR_W<0> {
        THR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority threshold for Hart 1 M-Mode (context #2) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prio_thr_2](index.html) module"]
pub struct PRIO_THR_2_SPEC;
impl crate::RegisterSpec for PRIO_THR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prio_thr_2::R](R) reader structure"]
impl crate::Readable for PRIO_THR_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prio_thr_2::W](W) writer structure"]
impl crate::Writable for PRIO_THR_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prio_thr_2 to value 0"]
impl crate::Resettable for PRIO_THR_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
