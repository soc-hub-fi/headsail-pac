#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `buf_done` reader - "]
pub type BUF_DONE_R = crate::BitReader;
#[doc = "Field `mac_done` reader - "]
pub type MAC_DONE_R = crate::BitReader;
#[doc = "Field `pp_done` reader - "]
pub type PP_DONE_R = crate::BitReader;
#[doc = "Field `dma_irq` reader - "]
pub type DMA_IRQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn buf_done(&self) -> BUF_DONE_R {
        BUF_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mac_done(&self) -> MAC_DONE_R {
        MAC_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pp_done(&self) -> PP_DONE_R {
        PP_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_irq(&self) -> DMA_IRQ_R {
        DMA_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("status")
            .field("buf_done", &self.buf_done())
            .field("mac_done", &self.mac_done())
            .field("pp_done", &self.pp_done())
            .field("dma_irq", &self.dma_irq())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
