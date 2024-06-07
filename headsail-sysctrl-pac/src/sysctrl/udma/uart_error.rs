#[doc = "Register `UART_ERROR` reader"]
pub type R = crate::R<UartErrorSpec>;
#[doc = "Field `RX_ERR_OVERFLOW` reader - RX overflow error status flag: - 1'b0: no error - 1'b1: RX overflow error occurred"]
pub type RxErrOverflowR = crate::BitReader;
#[doc = "Field `RX_ERR_PARITY` reader - RX parity error status flag: - 1'b0: no error - 1'b1: RX parity error occurred"]
pub type RxErrParityR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RX overflow error status flag: - 1'b0: no error - 1'b1: RX overflow error occurred"]
    #[inline(always)]
    pub fn rx_err_overflow(&self) -> RxErrOverflowR {
        RxErrOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX parity error status flag: - 1'b0: no error - 1'b1: RX parity error occurred"]
    #[inline(always)]
    pub fn rx_err_parity(&self) -> RxErrParityR {
        RxErrParityR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_ERROR")
            .field("rx_err_overflow", &self.rx_err_overflow())
            .field("rx_err_parity", &self.rx_err_parity())
            .finish()
    }
}
#[doc = "uDMA UART Error status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_error::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartErrorSpec;
impl crate::RegisterSpec for UartErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_error::R`](R) reader structure"]
impl crate::Readable for UartErrorSpec {}
#[doc = "`reset()` method sets UART_ERROR to value 0"]
impl crate::Resettable for UartErrorSpec {
    const RESET_VALUE: u32 = 0;
}
