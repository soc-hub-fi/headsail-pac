#[doc = "Register `claim_complete_5` reader"]
pub struct R(crate::R<CLAIM_COMPLETE_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIM_COMPLETE_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIM_COMPLETE_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIM_COMPLETE_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `claim_complete_5` writer"]
pub struct W(crate::W<CLAIM_COMPLETE_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIM_COMPLETE_5_SPEC>;
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
impl From<crate::W<CLAIM_COMPLETE_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIM_COMPLETE_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `claim_complete` reader - "]
pub type CLAIM_COMPLETE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `claim_complete` writer - "]
pub type CLAIM_COMPLETE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLAIM_COMPLETE_5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn claim_complete(&self) -> CLAIM_COMPLETE_R {
        CLAIM_COMPLETE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn claim_complete(&mut self) -> CLAIM_COMPLETE_W<0> {
        CLAIM_COMPLETE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Claim/complete for Hart 2 S-Mode (context #5) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claim_complete_5](index.html) module"]
pub struct CLAIM_COMPLETE_5_SPEC;
impl crate::RegisterSpec for CLAIM_COMPLETE_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claim_complete_5::R](R) reader structure"]
impl crate::Readable for CLAIM_COMPLETE_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claim_complete_5::W](W) writer structure"]
impl crate::Writable for CLAIM_COMPLETE_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets claim_complete_5 to value 0"]
impl crate::Resettable for CLAIM_COMPLETE_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
