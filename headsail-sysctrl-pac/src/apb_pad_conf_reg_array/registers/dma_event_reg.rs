#[doc = "Register `dma_event_reg` reader"]
pub struct R(crate::R<DMA_EVENT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_EVENT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_EVENT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_EVENT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dma_event_reg` writer"]
pub struct W(crate::W<DMA_EVENT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_EVENT_REG_SPEC>;
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
impl From<crate::W<DMA_EVENT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_EVENT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma0_read` reader - "]
pub type DMA0_READ_R = crate::BitReader<bool>;
#[doc = "Field `dma0_read` writer - "]
pub type DMA0_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_EVENT_REG_SPEC, bool, O>;
#[doc = "Field `dma0_write` reader - "]
pub type DMA0_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `dma0_write` writer - "]
pub type DMA0_WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_EVENT_REG_SPEC, bool, O>;
#[doc = "Field `dma1_read` reader - "]
pub type DMA1_READ_R = crate::BitReader<bool>;
#[doc = "Field `dma1_read` writer - "]
pub type DMA1_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_EVENT_REG_SPEC, bool, O>;
#[doc = "Field `dma1_write` reader - "]
pub type DMA1_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `dma1_write` writer - "]
pub type DMA1_WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_EVENT_REG_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_read(&mut self) -> DMA0_READ_W<0> {
        DMA0_READ_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_write(&mut self) -> DMA0_WRITE_W<1> {
        DMA0_WRITE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_read(&mut self) -> DMA1_READ_W<2> {
        DMA1_READ_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_write(&mut self) -> DMA1_WRITE_W<3> {
        DMA1_WRITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_event_reg](index.html) module"]
pub struct DMA_EVENT_REG_SPEC;
impl crate::RegisterSpec for DMA_EVENT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_event_reg::R](R) reader structure"]
impl crate::Readable for DMA_EVENT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_event_reg::W](W) writer structure"]
impl crate::Writable for DMA_EVENT_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dma_event_reg to value 0"]
impl crate::Resettable for DMA_EVENT_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
