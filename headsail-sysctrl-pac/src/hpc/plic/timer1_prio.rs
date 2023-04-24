#[doc = "Register `timer1_prio[%s]` reader"]
pub struct R(crate::R<TIMER1_PRIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_PRIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_PRIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_PRIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer1_prio[%s]` writer"]
pub struct W(crate::W<TIMER1_PRIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_PRIO_SPEC>;
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
impl From<crate::W<TIMER1_PRIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_PRIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `prio` reader - "]
pub type PRIO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `prio` writer - "]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER1_PRIO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<0> {
        PRIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt priority for timer 1 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_prio](index.html) module"]
pub struct TIMER1_PRIO_SPEC;
impl crate::RegisterSpec for TIMER1_PRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_prio::R](R) reader structure"]
impl crate::Readable for TIMER1_PRIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_prio::W](W) writer structure"]
impl crate::Writable for TIMER1_PRIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets timer1_prio[%s]
to value 0"]
impl crate::Resettable for TIMER1_PRIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
