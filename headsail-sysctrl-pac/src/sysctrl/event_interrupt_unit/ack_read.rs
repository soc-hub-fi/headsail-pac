#[doc = "Register `ACK_read` reader"]
pub type R = crate::R<AckReadSpec>;
#[doc = "Register `ACK_read` writer"]
pub type W = crate::W<AckReadSpec>;
#[doc = "Field `ACK_read` reader - "]
pub type AckReadR = crate::FieldReader<u32>;
#[doc = "Field `ACK_read` writer - "]
pub type AckReadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ack_read(&self) -> AckReadR {
        AckReadR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACK_read")
            .field("ack_read", &self.ack_read())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ack_read(&mut self) -> AckReadW<AckReadSpec> {
        AckReadW::new(self, 0)
    }
}
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_read::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_read::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AckReadSpec;
impl crate::RegisterSpec for AckReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ack_read::R`](R) reader structure"]
impl crate::Readable for AckReadSpec {}
#[doc = "`write(|w| ..)` method takes [`ack_read::W`](W) writer structure"]
impl crate::Writable for AckReadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACK_read to value 0"]
impl crate::Resettable for AckReadSpec {
    const RESET_VALUE: u32 = 0;
}
