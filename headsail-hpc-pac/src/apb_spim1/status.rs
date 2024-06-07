#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `RD` reader - Assert this for read operation"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Assert this for read operation"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` reader - Assert this for write operation"]
pub type WrR = crate::BitReader;
#[doc = "Field `WR` writer - Assert this for write operation"]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QRD` reader - Assert this for quad read mode"]
pub type QrdR = crate::BitReader;
#[doc = "Field `QRD` writer - Assert this for quad read mode"]
pub type QrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QWR` reader - Assert this for quad write mode"]
pub type QwrR = crate::BitReader;
#[doc = "Field `QWR` writer - Assert this for quad write mode"]
pub type QwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST` reader - Assert this to clear the rx and tx FIFOs"]
pub type SrstR = crate::BitReader;
#[doc = "Field `SRST` writer - Assert this to clear the rx and tx FIFOs"]
pub type SrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
pub type CsR = crate::FieldReader;
#[doc = "Field `CS` writer - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
pub type CsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Assert this for read operation"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Assert this for write operation"]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Assert this for quad read mode"]
    #[inline(always)]
    pub fn qrd(&self) -> QrdR {
        QrdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Assert this for quad write mode"]
    #[inline(always)]
    pub fn qwr(&self) -> QwrR {
        QwrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Assert this to clear the rx and tx FIFOs"]
    #[inline(always)]
    pub fn srst(&self) -> SrstR {
        SrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 8) & 0x0f) as u8)
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
    pub fn rd(&mut self) -> RdW<StatusSpec> {
        RdW::new(self, 0)
    }
    #[doc = "Bit 1 - Assert this for write operation"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WrW<StatusSpec> {
        WrW::new(self, 1)
    }
    #[doc = "Bit 2 - Assert this for quad read mode"]
    #[inline(always)]
    #[must_use]
    pub fn qrd(&mut self) -> QrdW<StatusSpec> {
        QrdW::new(self, 2)
    }
    #[doc = "Bit 3 - Assert this for quad write mode"]
    #[inline(always)]
    #[must_use]
    pub fn qwr(&mut self) -> QwrW<StatusSpec> {
        QwrW::new(self, 3)
    }
    #[doc = "Bit 4 - Assert this to clear the rx and tx FIFOs"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SrstW<StatusSpec> {
        SrstW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Each one of these bits corresponds to one of the spi lines. Enable any of them to enable the cs on the corresponding spi line."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<StatusSpec> {
        CsW::new(self, 8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
