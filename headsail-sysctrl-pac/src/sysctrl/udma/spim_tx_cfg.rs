#[doc = "Register `SPIM_TX_CFG` reader"]
pub type R = crate::R<SpimTxCfgSpec>;
#[doc = "Register `SPIM_TX_CFG` writer"]
pub type W = crate::W<SpimTxCfgSpec>;
#[doc = "Field `CONTINOUS` reader - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
pub type ContinousR = crate::BitReader;
#[doc = "Field `CONTINOUS` writer - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
pub type ContinousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 (\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datasize {
    #[doc = "0: `0`"]
    _8bits = 0,
    #[doc = "1: `1`"]
    _16bits = 1,
    #[doc = "2: `10`"]
    _32bits = 2,
}
impl From<Datasize> for u8 {
    #[inline(always)]
    fn from(variant: Datasize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datasize {
    type Ux = u8;
}
impl crate::IsEnum for Datasize {}
#[doc = "Field `DATASIZE` reader - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
pub type DatasizeR = crate::FieldReader<Datasize>;
impl DatasizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datasize> {
        match self.bits {
            0 => Some(Datasize::_8bits),
            1 => Some(Datasize::_16bits),
            2 => Some(Datasize::_32bits),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == Datasize::_8bits
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        *self == Datasize::_16bits
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == Datasize::_32bits
    }
}
#[doc = "Field `DATASIZE` writer - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
pub type DatasizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datasize>;
impl<'a, REG> DatasizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datasize::_8bits)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datasize::_16bits)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut crate::W<REG> {
        self.variant(Datasize::_32bits)
    }
}
#[doc = "Field `EN` reader - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDING` reader - Transfer pending in queue status flag: -1'b0: free -1'b1: pending"]
pub type PendingR = crate::BitReader;
#[doc = "Field `CLR` writer - Channel clear and stop transfer: -1'b0: disable -1'b1: enable"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&self) -> ContinousR {
        ContinousR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
    #[inline(always)]
    pub fn datasize(&self) -> DatasizeR {
        DatasizeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transfer pending in queue status flag: -1'b0: free -1'b1: pending"]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIM_TX_CFG")
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
    pub fn continous(&mut self) -> ContinousW<SpimTxCfgSpec> {
        ContinousW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
    #[inline(always)]
    #[must_use]
    pub fn datasize(&mut self) -> DatasizeW<SpimTxCfgSpec> {
        DatasizeW::new(self, 1)
    }
    #[doc = "Bit 4 - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<SpimTxCfgSpec> {
        EnW::new(self, 4)
    }
    #[doc = "Bit 6 - Channel clear and stop transfer: -1'b0: disable -1'b1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<SpimTxCfgSpec> {
        ClrW::new(self, 6)
    }
}
#[doc = "TX SPI uDMA transfer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_tx_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_tx_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpimTxCfgSpec;
impl crate::RegisterSpec for SpimTxCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spim_tx_cfg::R`](R) reader structure"]
impl crate::Readable for SpimTxCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`spim_tx_cfg::W`](W) writer structure"]
impl crate::Writable for SpimTxCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIM_TX_CFG to value 0x04"]
impl crate::Resettable for SpimTxCfgSpec {
    const RESET_VALUE: u32 = 0x04;
}
