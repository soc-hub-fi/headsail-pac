#[doc = "Register `RXFIFO` reader"]
pub type R = crate::R<RxfifoSpec>;
#[doc = "Field `RX` reader - Read data from the FIFO."]
pub type RxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read data from the FIFO."]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXFIFO").field("rx", &self.rx()).finish()
    }
}
#[doc = "SPI Receive FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifoSpec;
impl crate::RegisterSpec for RxfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifo::R`](R) reader structure"]
impl crate::Readable for RxfifoSpec {}
#[doc = "`reset()` method sets RXFIFO to value 0"]
impl crate::Resettable for RxfifoSpec {
    const RESET_VALUE: u32 = 0;
}
