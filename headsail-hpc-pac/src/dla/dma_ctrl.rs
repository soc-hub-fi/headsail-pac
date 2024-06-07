#[doc = "Register `dma_ctrl` reader"]
pub type R = crate::R<DMA_CTRL_SPEC>;
#[doc = "Register `dma_ctrl` writer"]
pub type W = crate::W<DMA_CTRL_SPEC>;
#[doc = "Field `read_event` reader - "]
pub type READ_EVENT_R = crate::BitReader;
#[doc = "Field `read_event` writer - "]
pub type READ_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `write_event` reader - "]
pub type WRITE_EVENT_R = crate::BitReader;
#[doc = "Field `write_event` writer - "]
pub type WRITE_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_event(&self) -> READ_EVENT_R {
        READ_EVENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_event(&self) -> WRITE_EVENT_R {
        WRITE_EVENT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("dma_ctrl")
            .field("read_event", &self.read_event())
            .field("write_event", &self.write_event())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn read_event(&mut self) -> READ_EVENT_W<DMA_CTRL_SPEC> {
        READ_EVENT_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn write_event(&mut self) -> WRITE_EVENT_W<DMA_CTRL_SPEC> {
        WRITE_EVENT_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ctrl::R`](R) reader structure"]
impl crate::Readable for DMA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_ctrl::W`](W) writer structure"]
impl crate::Writable for DMA_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_ctrl to value 0"]
impl crate::Resettable for DMA_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
