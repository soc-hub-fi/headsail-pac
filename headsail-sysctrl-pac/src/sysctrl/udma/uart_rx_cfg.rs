#[doc = "Register `UART_RX_CFG` reader"]
pub type R = crate::R<UART_RX_CFG_SPEC>;
#[doc = "Register `UART_RX_CFG` writer"]
pub type W = crate::W<UART_RX_CFG_SPEC>;
#[doc = "Field `CONTINOUS` reader - RX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
pub type CONTINOUS_R = crate::BitReader;
#[doc = "Field `CONTINOUS` writer - RX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
pub type CONTINOUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - RX channel enable and start transfer bitfield: -1'b0: disable -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - RX channel enable and start transfer bitfield: -1'b0: disable -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDING` reader - "]
pub type PENDING_R = crate::BitReader;
#[doc = "Field `CLR` writer - RX channel clear and stop transfer: -1'b0: disable -1'b1: stop and clear the on-going transfer"]
pub type CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&self) -> CONTINOUS_R {
        CONTINOUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - RX channel enable and start transfer bitfield: -1'b0: disable -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_RX_CFG")
            .field("continous", &self.continous())
            .field("en", &self.en())
            .field("pending", &self.pending())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
    #[inline(always)]
    #[must_use]
    pub fn continous(&mut self) -> CONTINOUS_W<UART_RX_CFG_SPEC> {
        CONTINOUS_W::new(self, 0)
    }
    #[doc = "Bit 4 - RX channel enable and start transfer bitfield: -1'b0: disable -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<UART_RX_CFG_SPEC> {
        EN_W::new(self, 4)
    }
    #[doc = "Bit 6 - RX channel clear and stop transfer: -1'b0: disable -1'b1: stop and clear the on-going transfer"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<UART_RX_CFG_SPEC> {
        CLR_W::new(self, 6)
    }
}
#[doc = "uDMA RX UART stream configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_RX_CFG_SPEC;
impl crate::RegisterSpec for UART_RX_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rx_cfg::R`](R) reader structure"]
impl crate::Readable for UART_RX_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_rx_cfg::W`](W) writer structure"]
impl crate::Writable for UART_RX_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_RX_CFG to value 0"]
impl crate::Resettable for UART_RX_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
