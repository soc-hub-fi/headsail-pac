#[doc = "Register `dma_ctrl` reader"]
pub struct R(crate::R<DMA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dma_ctrl` writer"]
pub struct W(crate::W<DMA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CTRL_SPEC>;
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
impl From<crate::W<DMA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `read_event` reader - "]
pub type READ_EVENT_R = crate::BitReader<bool>;
#[doc = "Field `read_event` writer - "]
pub type READ_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CTRL_SPEC, bool, O>;
#[doc = "Field `write_event` reader - "]
pub type WRITE_EVENT_R = crate::BitReader<bool>;
#[doc = "Field `write_event` writer - "]
pub type WRITE_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CTRL_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn read_event(&mut self) -> READ_EVENT_W<0> {
        READ_EVENT_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn write_event(&mut self) -> WRITE_EVENT_W<1> {
        WRITE_EVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctrl](index.html) module"]
pub struct DMA_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_ctrl::R](R) reader structure"]
impl crate::Readable for DMA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_ctrl::W](W) writer structure"]
impl crate::Writable for DMA_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dma_ctrl to value 0"]
impl crate::Resettable for DMA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
