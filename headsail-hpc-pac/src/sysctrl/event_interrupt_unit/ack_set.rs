#[doc = "Register `ACK_set` reader"]
pub type R = crate::R<AckSetSpec>;
#[doc = "Register `ACK_set` writer"]
pub type W = crate::W<AckSetSpec>;
#[doc = "Field `ACK_set` reader - "]
pub type AckSetR = crate::FieldReader<u32>;
#[doc = "Field `ACK_set` writer - "]
pub type AckSetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ack_set(&self) -> AckSetR {
        AckSetR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACK_set")
            .field("ack_set", &self.ack_set())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ack_set(&mut self) -> AckSetW<AckSetSpec> {
        AckSetW::new(self, 0)
    }
}
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AckSetSpec;
impl crate::RegisterSpec for AckSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ack_set::R`](R) reader structure"]
impl crate::Readable for AckSetSpec {}
#[doc = "`write(|w| ..)` method takes [`ack_set::W`](W) writer structure"]
impl crate::Writable for AckSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACK_set to value 0"]
impl crate::Resettable for AckSetSpec {
    const RESET_VALUE: u32 = 0;
}
