#[doc = "Register `UART_RX_SADDR` reader"]
pub type R = crate::R<UART_RX_SADDR_SPEC>;
#[doc = "Register `UART_RX_SADDR` writer"]
pub type W = crate::W<UART_RX_SADDR_SPEC>;
#[doc = "Field `RX_SADDR` reader - RX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets RX buffer base address"]
pub type RX_SADDR_R = crate::FieldReader<u32>;
#[doc = "Field `RX_SADDR` writer - RX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets RX buffer base address"]
pub type RX_SADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets RX buffer base address"]
    #[inline(always)]
    pub fn rx_saddr(&self) -> RX_SADDR_R {
        RX_SADDR_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_RX_SADDR")
            .field("rx_saddr", &self.rx_saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - RX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets RX buffer base address"]
    #[inline(always)]
    #[must_use]
    pub fn rx_saddr(&mut self) -> RX_SADDR_W<UART_RX_SADDR_SPEC> {
        RX_SADDR_W::new(self, 0)
    }
}
#[doc = "uDMA RX UART buffer base address configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_saddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_saddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_RX_SADDR_SPEC;
impl crate::RegisterSpec for UART_RX_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rx_saddr::R`](R) reader structure"]
impl crate::Readable for UART_RX_SADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_rx_saddr::W`](W) writer structure"]
impl crate::Writable for UART_RX_SADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_RX_SADDR to value 0"]
impl crate::Resettable for UART_RX_SADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
