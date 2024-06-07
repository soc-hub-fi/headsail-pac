#[doc = "Register `TXFIFO` writer"]
pub type W = crate::W<TxfifoSpec>;
#[doc = "Field `TX` writer - Write data into the FIFO."]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<TxfifoSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write data into the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<TxfifoSpec> {
        TxW::new(self, 0)
    }
}
#[doc = "SPI Transmit FIFO\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifoSpec;
impl crate::RegisterSpec for TxfifoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txfifo::W`](W) writer structure"]
impl crate::Writable for TxfifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TxfifoSpec {
    const RESET_VALUE: u32 = 0;
}
