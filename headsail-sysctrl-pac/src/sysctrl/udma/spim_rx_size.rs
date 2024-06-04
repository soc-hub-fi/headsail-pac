#[doc = "Register `SPIM_RX_SIZE` reader"]
pub type R = crate::R<SPIM_RX_SIZE_SPEC>;
#[doc = "Register `SPIM_RX_SIZE` writer"]
pub type W = crate::W<SPIM_RX_SIZE_SPEC>;
#[doc = "Field `RX_SIZE` reader - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
pub type RX_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `RX_SIZE` writer - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
pub type RX_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIM_RX_SIZE")
            .field("rx_size", &self.rx_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
    #[inline(always)]
    #[must_use]
    pub fn rx_size(&mut self) -> RX_SIZE_W<SPIM_RX_SIZE_SPEC> {
        RX_SIZE_W::new(self, 0)
    }
}
#[doc = "RX SPI uDMA transfer size of buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_rx_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_rx_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPIM_RX_SIZE_SPEC;
impl crate::RegisterSpec for SPIM_RX_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spim_rx_size::R`](R) reader structure"]
impl crate::Readable for SPIM_RX_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spim_rx_size::W`](W) writer structure"]
impl crate::Writable for SPIM_RX_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIM_RX_SIZE to value 0"]
impl crate::Resettable for SPIM_RX_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
