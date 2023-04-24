#[doc = "Register `TXFIFO` writer"]
pub struct W(crate::W<TXFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFIFO_SPEC>;
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
impl From<crate::W<TXFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX` writer - Write data into the FIFO."]
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXFIFO_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Write data into the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<0> {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Transmit FIFO\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifo](index.html) module"]
pub struct TXFIFO_SPEC;
impl crate::RegisterSpec for TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txfifo::W](W) writer structure"]
impl crate::Writable for TXFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
