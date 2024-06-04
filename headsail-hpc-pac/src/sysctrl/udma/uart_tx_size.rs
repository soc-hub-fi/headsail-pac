#[doc = "Register `UART_TX_SIZE` reader"]
pub type R = crate::R<UART_TX_SIZE_SPEC>;
#[doc = "Register `UART_TX_SIZE` writer"]
pub type W = crate::W<UART_TX_SIZE_SPEC>;
#[doc = "Field `TX_SIZE` reader - TX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub type TX_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `TX_SIZE` writer - TX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub type TX_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - TX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_TX_SIZE")
            .field("tx_size", &self.tx_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16 - TX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    #[must_use]
    pub fn tx_size(&mut self) -> TX_SIZE_W<UART_TX_SIZE_SPEC> {
        TX_SIZE_W::new(self, 0)
    }
}
#[doc = "uDMA TX UART buffer size configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_TX_SIZE_SPEC;
impl crate::RegisterSpec for UART_TX_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_tx_size::R`](R) reader structure"]
impl crate::Readable for UART_TX_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_tx_size::W`](W) writer structure"]
impl crate::Writable for UART_TX_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_TX_SIZE to value 0"]
impl crate::Resettable for UART_TX_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
