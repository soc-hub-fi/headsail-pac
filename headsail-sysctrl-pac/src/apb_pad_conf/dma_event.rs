#[doc = "Register `dma_event` reader"]
pub type R = crate::R<DMA_EVENT_SPEC>;
#[doc = "Register `dma_event` writer"]
pub type W = crate::W<DMA_EVENT_SPEC>;
#[doc = "Field `dma0_read` reader - "]
pub type DMA0_READ_R = crate::BitReader;
#[doc = "Field `dma0_read` writer - "]
pub type DMA0_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma0_write` reader - "]
pub type DMA0_WRITE_R = crate::BitReader;
#[doc = "Field `dma0_write` writer - "]
pub type DMA0_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma1_read` reader - "]
pub type DMA1_READ_R = crate::BitReader;
#[doc = "Field `dma1_read` writer - "]
pub type DMA1_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma1_write` reader - "]
pub type DMA1_WRITE_R = crate::BitReader;
#[doc = "Field `dma1_write` writer - "]
pub type DMA1_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma0_read(&self) -> DMA0_READ_R {
        DMA0_READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma0_write(&self) -> DMA0_WRITE_R {
        DMA0_WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma1_read(&self) -> DMA1_READ_R {
        DMA1_READ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma1_write(&self) -> DMA1_WRITE_R {
        DMA1_WRITE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("dma_event")
            .field("dma0_read", &self.dma0_read())
            .field("dma0_write", &self.dma0_write())
            .field("dma1_read", &self.dma1_read())
            .field("dma1_write", &self.dma1_write())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_read(&mut self) -> DMA0_READ_W<DMA_EVENT_SPEC> {
        DMA0_READ_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_write(&mut self) -> DMA0_WRITE_W<DMA_EVENT_SPEC> {
        DMA0_WRITE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_read(&mut self) -> DMA1_READ_W<DMA_EVENT_SPEC> {
        DMA1_READ_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_write(&mut self) -> DMA1_WRITE_W<DMA_EVENT_SPEC> {
        DMA1_WRITE_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_event::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_event::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_EVENT_SPEC;
impl crate::RegisterSpec for DMA_EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_event::R`](R) reader structure"]
impl crate::Readable for DMA_EVENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_event::W`](W) writer structure"]
impl crate::Writable for DMA_EVENT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_event to value 0"]
impl crate::Resettable for DMA_EVENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
