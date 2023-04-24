#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD` reader - Assert this for read operation"]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - Assert this for read operation"]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `WR` reader - Assert this for write operation"]
pub type WR_R = crate::BitReader<bool>;
#[doc = "Field `WR` writer - Assert this for write operation"]
pub type WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `QRD` reader - Assert this for quad read mode"]
pub type QRD_R = crate::BitReader<bool>;
#[doc = "Field `QRD` writer - Assert this for quad read mode"]
pub type QRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `QWR` reader - Assert this for quad write mode"]
pub type QWR_R = crate::BitReader<bool>;
#[doc = "Field `QWR` writer - Assert this for quad write mode"]
pub type QWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `SRST` reader - Assert this to clear the rx and tx FIFOs"]
pub type SRST_R = crate::BitReader<bool>;
#[doc = "Field `SRST` writer - Assert this to clear the rx and tx FIFOs"]
pub type SRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CS` reader - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
pub type CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS` writer - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
pub type CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Assert this for read operation"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Assert this for write operation"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Assert this for quad read mode"]
    #[inline(always)]
    pub fn qrd(&self) -> QRD_R {
        QRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Assert this for quad write mode"]
    #[inline(always)]
    pub fn qwr(&self) -> QWR_R {
        QWR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Assert this to clear the rx and tx FIFOs"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Assert this for read operation"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<0> {
        RD_W::new(self)
    }
    #[doc = "Bit 1 - Assert this for write operation"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<1> {
        WR_W::new(self)
    }
    #[doc = "Bit 2 - Assert this for quad read mode"]
    #[inline(always)]
    #[must_use]
    pub fn qrd(&mut self) -> QRD_W<2> {
        QRD_W::new(self)
    }
    #[doc = "Bit 3 - Assert this for quad write mode"]
    #[inline(always)]
    #[must_use]
    pub fn qwr(&mut self) -> QWR_W<3> {
        QWR_W::new(self)
    }
    #[doc = "Bit 4 - Assert this to clear the rx and tx FIFOs"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SRST_W<4> {
        SRST_W::new(self)
    }
    #[doc = "Bits 8:11 - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<8> {
        CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
