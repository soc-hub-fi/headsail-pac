#[doc = "Register `UART_RX_SIZE` reader"]
pub type R = crate::R<UartRxSizeSpec>;
#[doc = "Register `UART_RX_SIZE` writer"]
pub type W = crate::W<UartRxSizeSpec>;
#[doc = "Field `RX_SIZE` reader - RX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub type RxSizeR = crate::FieldReader<u32>;
#[doc = "Field `RX_SIZE` writer - RX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub type RxSizeW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - RX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    pub fn rx_size(&self) -> RxSizeR {
        RxSizeR::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_RX_SIZE")
            .field("rx_size", &self.rx_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16 - RX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    #[must_use]
    pub fn rx_size(&mut self) -> RxSizeW<UartRxSizeSpec> {
        RxSizeW::new(self, 0)
    }
}
#[doc = "uDMA RX UART buffer size configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRxSizeSpec;
impl crate::RegisterSpec for UartRxSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rx_size::R`](R) reader structure"]
impl crate::Readable for UartRxSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_rx_size::W`](W) writer structure"]
impl crate::Writable for UartRxSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_RX_SIZE to value 0"]
impl crate::Resettable for UartRxSizeSpec {
    const RESET_VALUE: u32 = 0;
}
