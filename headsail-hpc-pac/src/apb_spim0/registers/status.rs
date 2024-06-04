#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `RD` reader - Assert this for read operation"]
pub type RD_R = crate::BitReader;
#[doc = "Field `RD` writer - Assert this for read operation"]
pub type RD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` reader - Assert this for write operation"]
pub type WR_R = crate::BitReader;
#[doc = "Field `WR` writer - Assert this for write operation"]
pub type WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QRD` reader - Assert this for quad read mode"]
pub type QRD_R = crate::BitReader;
#[doc = "Field `QRD` writer - Assert this for quad read mode"]
pub type QRD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QWR` reader - Assert this for quad write mode"]
pub type QWR_R = crate::BitReader;
#[doc = "Field `QWR` writer - Assert this for quad write mode"]
pub type QWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST` reader - Assert this to clear the rx and tx FIFOs"]
pub type SRST_R = crate::BitReader;
#[doc = "Field `SRST` writer - Assert this to clear the rx and tx FIFOs"]
pub type SRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
pub type CS_R = crate::FieldReader;
#[doc = "Field `CS` writer - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("rd", &self.rd())
            .field("wr", &self.wr())
            .field("qrd", &self.qrd())
            .field("qwr", &self.qwr())
            .field("srst", &self.srst())
            .field("cs", &self.cs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Assert this for read operation"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<STATUS_SPEC> {
        RD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Assert this for write operation"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<STATUS_SPEC> {
        WR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Assert this for quad read mode"]
    #[inline(always)]
    #[must_use]
    pub fn qrd(&mut self) -> QRD_W<STATUS_SPEC> {
        QRD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Assert this for quad write mode"]
    #[inline(always)]
    #[must_use]
    pub fn qwr(&mut self) -> QWR_W<STATUS_SPEC> {
        QWR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Assert this to clear the rx and tx FIFOs"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SRST_W<STATUS_SPEC> {
        SRST_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<STATUS_SPEC> {
        CS_W::new(self, 8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
