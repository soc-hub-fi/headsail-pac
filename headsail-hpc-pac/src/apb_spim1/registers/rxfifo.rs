#[doc = "Register `RXFIFO` reader"]
pub type R = crate::R<RXFIFO_SPEC>;
#[doc = "Field `RX` reader - Read data from the FIFO."]
pub type RX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read data from the FIFO."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXFIFO").field("rx", &self.rx()).finish()
    }
}
#[doc = "SPI Receive FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFIFO_SPEC;
impl crate::RegisterSpec for RXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifo::R`](R) reader structure"]
impl crate::Readable for RXFIFO_SPEC {}
#[doc = "`reset()` method sets RXFIFO to value 0"]
impl crate::Resettable for RXFIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
