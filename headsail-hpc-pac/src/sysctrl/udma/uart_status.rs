#[doc = "Register `UART_STATUS` reader"]
pub type R = crate::R<UART_STATUS_SPEC>;
#[doc = "Field `TX_BUSY` reader - TX busy status flag: - 1'b0: no TX transfer on-going - 1'b1: TX transfer on-going"]
pub type TX_BUSY_R = crate::BitReader;
#[doc = "Field `RX_BUSY` reader - RX busy status flag: - 1'b0: no RX transfer on-going - 1'b1: RX transfer on-going"]
pub type RX_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TX busy status flag: - 1'b0: no TX transfer on-going - 1'b1: TX transfer on-going"]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX busy status flag: - 1'b0: no RX transfer on-going - 1'b1: RX transfer on-going"]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_STATUS")
            .field("tx_busy", &self.tx_busy())
            .field("rx_busy", &self.rx_busy())
            .finish()
    }
}
#[doc = "uDMA UART status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_STATUS_SPEC;
impl crate::RegisterSpec for UART_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_status::R`](R) reader structure"]
impl crate::Readable for UART_STATUS_SPEC {}
#[doc = "`reset()` method sets UART_STATUS to value 0"]
impl crate::Resettable for UART_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
