#[doc = "Register `SPIM_RX_CFG` reader"]
pub type R = crate::R<SPIM_RX_CFG_SPEC>;
#[doc = "Register `SPIM_RX_CFG` writer"]
pub type W = crate::W<SPIM_RX_CFG_SPEC>;
#[doc = "Field `CONTINOUS` reader - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
pub type CONTINOUS_R = crate::BitReader;
#[doc = "Field `CONTINOUS` writer - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
pub type CONTINOUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 (\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATASIZE_A {
    #[doc = "0: `0`"]
    _8BITS = 0,
    #[doc = "1: `1`"]
    _16BITS = 1,
    #[doc = "2: `10`"]
    _32BITS = 2,
}
impl From<DATASIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATASIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATASIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for DATASIZE_A {}
#[doc = "Field `DATASIZE` reader - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
pub type DATASIZE_R = crate::FieldReader<DATASIZE_A>;
impl DATASIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATASIZE_A> {
        match self.bits {
            0 => Some(DATASIZE_A::_8BITS),
            1 => Some(DATASIZE_A::_16BITS),
            2 => Some(DATASIZE_A::_32BITS),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == DATASIZE_A::_8BITS
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        *self == DATASIZE_A::_16BITS
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == DATASIZE_A::_32BITS
    }
}
#[doc = "Field `DATASIZE` writer - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
pub type DATASIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATASIZE_A>;
impl<'a, REG> DATASIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZE_A::_8BITS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZE_A::_16BITS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut crate::W<REG> {
        self.variant(DATASIZE_A::_32BITS)
    }
}
#[doc = "Field `EN` reader - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDING` reader - Transfer pending in queue status flag: -1'b0: free -1'b1: pending"]
pub type PENDING_R = crate::BitReader;
#[doc = "Field `CLR` writer - Channel clear and stop transfer: -1'b0: disable -1'b1: enable"]
pub type CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&self) -> CONTINOUS_R {
        CONTINOUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
    #[inline(always)]
    pub fn datasize(&self) -> DATASIZE_R {
        DATASIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transfer pending in queue status flag: -1'b0: free -1'b1: pending"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIM_RX_CFG")
            .field("continous", &self.continous())
            .field("datasize", &self.datasize())
            .field("en", &self.en())
            .field("pending", &self.pending())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
    #[inline(always)]
    #[must_use]
    pub fn continous(&mut self) -> CONTINOUS_W<SPIM_RX_CFG_SPEC> {
        CONTINOUS_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
    #[inline(always)]
    #[must_use]
    pub fn datasize(&mut self) -> DATASIZE_W<SPIM_RX_CFG_SPEC> {
        DATASIZE_W::new(self, 1)
    }
    #[doc = "Bit 4 - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<SPIM_RX_CFG_SPEC> {
        EN_W::new(self, 4)
    }
    #[doc = "Bit 6 - Channel clear and stop transfer: -1'b0: disable -1'b1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<SPIM_RX_CFG_SPEC> {
        CLR_W::new(self, 6)
    }
}
#[doc = "RX SPI uDMA transfer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_rx_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_rx_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPIM_RX_CFG_SPEC;
impl crate::RegisterSpec for SPIM_RX_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spim_rx_cfg::R`](R) reader structure"]
impl crate::Readable for SPIM_RX_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spim_rx_cfg::W`](W) writer structure"]
impl crate::Writable for SPIM_RX_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIM_RX_CFG to value 0x04"]
impl crate::Resettable for SPIM_RX_CFG_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
