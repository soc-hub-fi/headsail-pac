#[doc = "Register `UART_IRQ_EN` reader"]
pub type R = crate::R<UartIrqEnSpec>;
#[doc = "Register `UART_IRQ_EN` writer"]
pub type W = crate::W<UartIrqEnSpec>;
#[doc = "Field `RX` reader - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_IRQ_EN")
            .field("rx", &self.rx())
            .field("error", &self.error())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rx interrupt in enable flag: - 1'b0: RX IRQ disable - 1'b1: RX IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RxW<UartIrqEnSpec> {
        RxW::new(self, 0)
    }
    #[doc = "Bit 1 - Error interrupt in enable flag: - 1'b0: Error IRQ disable - 1'b1: Error IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<UartIrqEnSpec> {
        ErrorW::new(self, 1)
    }
}
#[doc = "uDMA UART Read or Error interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIrqEnSpec;
impl crate::RegisterSpec for UartIrqEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_irq_en::R`](R) reader structure"]
impl crate::Readable for UartIrqEnSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_irq_en::W`](W) writer structure"]
impl crate::Writable for UartIrqEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_IRQ_EN to value 0"]
impl crate::Resettable for UartIrqEnSpec {
    const RESET_VALUE: u32 = 0;
}
