#[doc = "Register `dma_event` reader"]
pub type R = crate::R<DmaEventSpec>;
#[doc = "Register `dma_event` writer"]
pub type W = crate::W<DmaEventSpec>;
#[doc = "Field `dma0_read` reader - "]
pub type Dma0ReadR = crate::BitReader;
#[doc = "Field `dma0_read` writer - "]
pub type Dma0ReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma0_write` reader - "]
pub type Dma0WriteR = crate::BitReader;
#[doc = "Field `dma0_write` writer - "]
pub type Dma0WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma1_read` reader - "]
pub type Dma1ReadR = crate::BitReader;
#[doc = "Field `dma1_read` writer - "]
pub type Dma1ReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma1_write` reader - "]
pub type Dma1WriteR = crate::BitReader;
#[doc = "Field `dma1_write` writer - "]
pub type Dma1WriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma0_read(&self) -> Dma0ReadR {
        Dma0ReadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma0_write(&self) -> Dma0WriteR {
        Dma0WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma1_read(&self) -> Dma1ReadR {
        Dma1ReadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma1_write(&self) -> Dma1WriteR {
        Dma1WriteR::new(((self.bits >> 3) & 1) != 0)
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
    pub fn dma0_read(&mut self) -> Dma0ReadW<DmaEventSpec> {
        Dma0ReadW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_write(&mut self) -> Dma0WriteW<DmaEventSpec> {
        Dma0WriteW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_read(&mut self) -> Dma1ReadW<DmaEventSpec> {
        Dma1ReadW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_write(&mut self) -> Dma1WriteW<DmaEventSpec> {
        Dma1WriteW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_event::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_event::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEventSpec;
impl crate::RegisterSpec for DmaEventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_event::R`](R) reader structure"]
impl crate::Readable for DmaEventSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_event::W`](W) writer structure"]
impl crate::Writable for DmaEventSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_event to value 0"]
impl crate::Resettable for DmaEventSpec {
    const RESET_VALUE: u32 = 0;
}
