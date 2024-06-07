#[doc = "Register `UART_DATA` reader"]
pub type R = crate::R<UartDataSpec>;
#[doc = "Field `BYTE` reader - "]
pub type ByteR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn byte(&self) -> ByteR {
        ByteR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_DATA")
            .field("byte", &self.byte())
            .finish()
    }
}
#[doc = "RX read data for polling or interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartDataSpec;
impl crate::RegisterSpec for UartDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_data::R`](R) reader structure"]
impl crate::Readable for UartDataSpec {}
#[doc = "`reset()` method sets UART_DATA to value 0"]
impl crate::Resettable for UartDataSpec {
    const RESET_VALUE: u32 = 0;
}
