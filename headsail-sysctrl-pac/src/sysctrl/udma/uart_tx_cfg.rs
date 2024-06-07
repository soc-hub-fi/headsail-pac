#[doc = "Register `UART_TX_CFG` reader"]
pub type R = crate::R<UartTxCfgSpec>;
#[doc = "Register `UART_TX_CFG` writer"]
pub type W = crate::W<UartTxCfgSpec>;
#[doc = "Field `CONTINOUS` reader - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
pub type ContinousR = crate::BitReader;
#[doc = "Field `CONTINOUS` writer - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
pub type ContinousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - TX channel enable and start transfer bitfield: -1'b0: disabled -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - TX channel enable and start transfer bitfield: -1'b0: disabled -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDING` reader - TX transfer pending in queue status flag: -1'b0: no pending transfer in the queue -1'b1: pending transfer in the queue"]
pub type PendingR = crate::BitReader;
#[doc = "Field `CLR` writer - RX channel clear and stop transfer: -1'b0: disable -1'b1: stop and clear the on-going transfer"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&self) -> ContinousR {
        ContinousR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TX channel enable and start transfer bitfield: -1'b0: disabled -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX transfer pending in queue status flag: -1'b0: no pending transfer in the queue -1'b1: pending transfer in the queue"]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_TX_CFG")
            .field("continous", &self.continous())
            .field("en", &self.en())
            .field("pending", &self.pending())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
    #[inline(always)]
    #[must_use]
    pub fn continous(&mut self) -> ContinousW<UartTxCfgSpec> {
        ContinousW::new(self, 0)
    }
    #[doc = "Bit 4 - TX channel enable and start transfer bitfield: -1'b0: disabled -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<UartTxCfgSpec> {
        EnW::new(self, 4)
    }
    #[doc = "Bit 6 - RX channel clear and stop transfer: -1'b0: disable -1'b1: stop and clear the on-going transfer"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<UartTxCfgSpec> {
        ClrW::new(self, 6)
    }
}
#[doc = "uDMA TX UART stream configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartTxCfgSpec;
impl crate::RegisterSpec for UartTxCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_tx_cfg::R`](R) reader structure"]
impl crate::Readable for UartTxCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_tx_cfg::W`](W) writer structure"]
impl crate::Writable for UartTxCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_TX_CFG to value 0"]
impl crate::Resettable for UartTxCfgSpec {
    const RESET_VALUE: u32 = 0;
}
