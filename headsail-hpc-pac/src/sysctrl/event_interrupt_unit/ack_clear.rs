#[doc = "Register `ACK_clear` reader"]
pub type R = crate::R<ACK_CLEAR_SPEC>;
#[doc = "Register `ACK_clear` writer"]
pub type W = crate::W<ACK_CLEAR_SPEC>;
#[doc = "Field `ACK_clear` reader - "]
pub type ACK_CLEAR_R = crate::FieldReader<u32>;
#[doc = "Field `ACK_clear` writer - "]
pub type ACK_CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ack_clear(&self) -> ACK_CLEAR_R {
        ACK_CLEAR_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACK_clear")
            .field("ack_clear", &self.ack_clear())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ack_clear(&mut self) -> ACK_CLEAR_W<ACK_CLEAR_SPEC> {
        ACK_CLEAR_W::new(self, 0)
    }
}
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACK_CLEAR_SPEC;
impl crate::RegisterSpec for ACK_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ack_clear::R`](R) reader structure"]
impl crate::Readable for ACK_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ack_clear::W`](W) writer structure"]
impl crate::Writable for ACK_CLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACK_clear to value 0"]
impl crate::Resettable for ACK_CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
