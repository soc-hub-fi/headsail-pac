#[doc = "Register `TXFIFO` writer"]
pub type W = crate::W<TXFIFO_SPEC>;
#[doc = "Field `TX` writer - Write data into the FIFO."]
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<TXFIFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write data into the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<TXFIFO_SPEC> {
        TX_W::new(self, 0)
    }
}
#[doc = "SPI Transmit FIFO\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFIFO_SPEC;
impl crate::RegisterSpec for TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txfifo::W`](W) writer structure"]
impl crate::Writable for TXFIFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TXFIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
