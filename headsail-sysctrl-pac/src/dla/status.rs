#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `buf_done` reader - "]
pub type BufDoneR = crate::BitReader;
#[doc = "Field `mac_done` reader - "]
pub type MacDoneR = crate::BitReader;
#[doc = "Field `pp_done` reader - "]
pub type PpDoneR = crate::BitReader;
#[doc = "Field `dma_irq` reader - "]
pub type DmaIrqR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn buf_done(&self) -> BufDoneR {
        BufDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mac_done(&self) -> MacDoneR {
        MacDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pp_done(&self) -> PpDoneR {
        PpDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_irq(&self) -> DmaIrqR {
        DmaIrqR::new(((self.bits >> 3) & 1) != 0)
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
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
