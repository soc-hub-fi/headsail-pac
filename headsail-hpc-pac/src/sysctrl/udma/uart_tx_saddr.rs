#[doc = "Register `UART_TX_SADDR` reader"]
pub type R = crate::R<UartTxSaddrSpec>;
#[doc = "Register `UART_TX_SADDR` writer"]
pub type W = crate::W<UartTxSaddrSpec>;
#[doc = "Field `TX_SADDR` reader - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
pub type TxSaddrR = crate::FieldReader<u32>;
#[doc = "Field `TX_SADDR` writer - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
pub type TxSaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
    #[inline(always)]
    pub fn tx_saddr(&self) -> TxSaddrR {
        TxSaddrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_TX_SADDR")
            .field("tx_saddr", &self.tx_saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
    #[inline(always)]
    #[must_use]
    pub fn tx_saddr(&mut self) -> TxSaddrW<UartTxSaddrSpec> {
        TxSaddrW::new(self, 0)
    }
}
#[doc = "uDMA TX UART buffer base address configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_saddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_saddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartTxSaddrSpec;
impl crate::RegisterSpec for UartTxSaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_tx_saddr::R`](R) reader structure"]
impl crate::Readable for UartTxSaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_tx_saddr::W`](W) writer structure"]
impl crate::Writable for UartTxSaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_TX_SADDR to value 0"]
impl crate::Resettable for UartTxSaddrSpec {
    const RESET_VALUE: u32 = 0;
}
