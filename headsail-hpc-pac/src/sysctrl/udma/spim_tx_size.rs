#[doc = "Register `SPIM_TX_SIZE` reader"]
pub type R = crate::R<SpimTxSizeSpec>;
#[doc = "Register `SPIM_TX_SIZE` writer"]
pub type W = crate::W<SpimTxSizeSpec>;
#[doc = "Field `TX_SIZE` reader - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
pub type TxSizeR = crate::FieldReader<u32>;
#[doc = "Field `TX_SIZE` writer - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
pub type TxSizeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
    #[inline(always)]
    pub fn tx_size(&self) -> TxSizeR {
        TxSizeR::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIM_TX_SIZE")
            .field("tx_size", &self.tx_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
    #[inline(always)]
    #[must_use]
    pub fn tx_size(&mut self) -> TxSizeW<SpimTxSizeSpec> {
        TxSizeW::new(self, 0)
    }
}
#[doc = "TX SPI uDMA transfer size of buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_tx_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_tx_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpimTxSizeSpec;
impl crate::RegisterSpec for SpimTxSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spim_tx_size::R`](R) reader structure"]
impl crate::Readable for SpimTxSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`spim_tx_size::W`](W) writer structure"]
impl crate::Writable for SpimTxSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIM_TX_SIZE to value 0"]
impl crate::Resettable for SpimTxSizeSpec {
    const RESET_VALUE: u32 = 0;
}
