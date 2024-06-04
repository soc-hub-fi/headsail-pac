#[doc = "Register `UART_ERROR` reader"]
pub type R = crate::R<UART_ERROR_SPEC>;
#[doc = "Field `RX_ERR_OVERFLOW` reader - RX overflow error status flag: - 1'b0: no error - 1'b1: RX overflow error occurred"]
pub type RX_ERR_OVERFLOW_R = crate::BitReader;
#[doc = "Field `RX_ERR_PARITY` reader - RX parity error status flag: - 1'b0: no error - 1'b1: RX parity error occurred"]
pub type RX_ERR_PARITY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RX overflow error status flag: - 1'b0: no error - 1'b1: RX overflow error occurred"]
    #[inline(always)]
    pub fn rx_err_overflow(&self) -> RX_ERR_OVERFLOW_R {
        RX_ERR_OVERFLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX parity error status flag: - 1'b0: no error - 1'b1: RX parity error occurred"]
    #[inline(always)]
    pub fn rx_err_parity(&self) -> RX_ERR_PARITY_R {
        RX_ERR_PARITY_R::new(((self.bits >> 1) & 1) != 0)
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
pub struct UART_ERROR_SPEC;
impl crate::RegisterSpec for UART_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_error::R`](R) reader structure"]
impl crate::Readable for UART_ERROR_SPEC {}
#[doc = "`reset()` method sets UART_ERROR to value 0"]
impl crate::Resettable for UART_ERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
