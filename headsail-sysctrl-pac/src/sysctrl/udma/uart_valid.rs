#[doc = "Register `UART_VALID` reader"]
pub type R = crate::R<UART_VALID_SPEC>;
#[doc = "Field `READY` reader - Used only in RX polling method to indicate data is ready for read: - 1'b0: Data is not ready to read - 1'b1: Data is ready to read"]
pub type READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Used only in RX polling method to indicate data is ready for read: - 1'b0: Data is not ready to read - 1'b1: Data is ready to read"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_VALID")
            .field("ready", &self.ready())
            .finish()
    }
}
#[doc = "uDMA UART Read polling data valid flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_valid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_VALID_SPEC;
impl crate::RegisterSpec for UART_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_valid::R`](R) reader structure"]
impl crate::Readable for UART_VALID_SPEC {}
#[doc = "`reset()` method sets UART_VALID to value 0"]
impl crate::Resettable for UART_VALID_SPEC {
    const RESET_VALUE: u32 = 0;
}
