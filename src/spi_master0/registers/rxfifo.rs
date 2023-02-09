#[doc = "Register `RXFIFO` reader"]
pub struct R(crate::R<RXFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX` reader - Read data from the FIFO."]
pub type RX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read data from the FIFO."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(self.bits)
    }
}
#[doc = "SPI Receive FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo](index.html) module"]
pub struct RXFIFO_SPEC;
impl crate::RegisterSpec for RXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifo::R](R) reader structure"]
impl crate::Readable for RXFIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIFO to value 0"]
impl crate::Resettable for RXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
