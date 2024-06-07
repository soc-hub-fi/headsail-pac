#[doc = "Register `claim_complete_6` reader"]
pub type R = crate::R<ClaimComplete6Spec>;
#[doc = "Register `claim_complete_6` writer"]
pub type W = crate::W<ClaimComplete6Spec>;
#[doc = "Field `claim_complete` reader - "]
pub type ClaimCompleteR = crate::FieldReader<u32>;
#[doc = "Field `claim_complete` writer - "]
pub type ClaimCompleteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn claim_complete(&self) -> ClaimCompleteR {
        ClaimCompleteR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("claim_complete_6")
            .field("claim_complete", &self.claim_complete())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn claim_complete(&mut self) -> ClaimCompleteW<ClaimComplete6Spec> {
        ClaimCompleteW::new(self, 0)
    }
}
#[doc = "Claim/complete for Hart 3 M-mode (context #6) Read performs 'claim', which returns the ID of the highest-priority pending interrupt or zero if there is no pending interrupts. A successful claim also atomically clears the corresponding pending bit on the interrupt source. A 'complete' is performed by writing the received interrupt ID back into claim/complete, once an interrupt handler has been completed. If the completion ID does not match an interrupt source that is currently enabled for the target, the completion is silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimComplete6Spec;
impl crate::RegisterSpec for ClaimComplete6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claim_complete_6::R`](R) reader structure"]
impl crate::Readable for ClaimComplete6Spec {}
#[doc = "`write(|w| ..)` method takes [`claim_complete_6::W`](W) writer structure"]
impl crate::Writable for ClaimComplete6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets claim_complete_6 to value 0"]
impl crate::Resettable for ClaimComplete6Spec {
    const RESET_VALUE: u32 = 0;
}
