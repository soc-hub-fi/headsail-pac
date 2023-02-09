#[doc = "Register `UART_TX_CFG` reader"]
pub struct R(crate::R<UART_TX_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TX_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TX_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TX_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TX_CFG` writer"]
pub struct W(crate::W<UART_TX_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TX_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UART_TX_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TX_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINOUS` reader - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
pub type CONTINOUS_R = crate::BitReader<bool>;
#[doc = "Field `CONTINOUS` writer - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
pub type CONTINOUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_TX_CFG_SPEC, bool, O>;
#[doc = "Field `EN` reader - TX channel enable and start transfer bitfield: -1'b0: disabled -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - TX channel enable and start transfer bitfield: -1'b0: disabled -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_TX_CFG_SPEC, bool, O>;
#[doc = "Field `PENDING` reader - TX transfer pending in queue status flag: -1'b0: no pending transfer in the queue -1'b1: pending transfer in the queue"]
pub type PENDING_R = crate::BitReader<bool>;
#[doc = "Field `CLR` writer - RX channel clear and stop transfer: -1'b0: disable -1'b1: stop and clear the on-going transfer"]
pub type CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_TX_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&self) -> CONTINOUS_R {
        CONTINOUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TX channel enable and start transfer bitfield: -1'b0: disabled -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX transfer pending in queue status flag: -1'b0: no pending transfer in the queue -1'b1: pending transfer in the queue"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
    #[inline(always)]
    #[must_use]
    pub fn continous(&mut self) -> CONTINOUS_W<0> {
        CONTINOUS_W::new(self)
    }
    #[doc = "Bit 4 - TX channel enable and start transfer bitfield: -1'b0: disabled -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<4> {
        EN_W::new(self)
    }
    #[doc = "Bit 6 - RX channel clear and stop transfer: -1'b0: disable -1'b1: stop and clear the on-going transfer"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<6> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA TX UART stream configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tx_cfg](index.html) module"]
pub struct UART_TX_CFG_SPEC;
impl crate::RegisterSpec for UART_TX_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tx_cfg::R](R) reader structure"]
impl crate::Readable for UART_TX_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tx_cfg::W](W) writer structure"]
impl crate::Writable for UART_TX_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_TX_CFG to value 0"]
impl crate::Resettable for UART_TX_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
