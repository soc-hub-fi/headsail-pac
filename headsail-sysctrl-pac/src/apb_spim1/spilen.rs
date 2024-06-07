#[doc = "Register `SPILEN` reader"]
pub type R = crate::R<SpilenSpec>;
#[doc = "Register `SPILEN` writer"]
pub type W = crate::W<SpilenSpec>;
#[doc = "Field `CMDLEN` reader - The number of bits of the SPI command that should be sent."]
pub type CmdlenR = crate::FieldReader;
#[doc = "Field `CMDLEN` writer - The number of bits of the SPI command that should be sent."]
pub type CmdlenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADDRLEN` reader - The number of bits of the SPI address that should be sent."]
pub type AddrlenR = crate::FieldReader;
#[doc = "Field `ADDRLEN` writer - The number of bits of the SPI address that should be sent."]
pub type AddrlenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DATALEN` reader - The number of data bits to be read or written."]
pub type DatalenR = crate::FieldReader<u16>;
#[doc = "Field `DATALEN` writer - The number of data bits to be read or written."]
pub type DatalenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - The number of bits of the SPI command that should be sent."]
    #[inline(always)]
    pub fn cmdlen(&self) -> CmdlenR {
        CmdlenR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The number of bits of the SPI address that should be sent."]
    #[inline(always)]
    pub fn addrlen(&self) -> AddrlenR {
        AddrlenR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - The number of data bits to be read or written."]
    #[inline(always)]
    pub fn datalen(&self) -> DatalenR {
        DatalenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPILEN")
            .field("cmdlen", &self.cmdlen())
            .field("addrlen", &self.addrlen())
            .field("datalen", &self.datalen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - The number of bits of the SPI command that should be sent."]
    #[inline(always)]
    #[must_use]
    pub fn cmdlen(&mut self) -> CmdlenW<SpilenSpec> {
        CmdlenW::new(self, 0)
    }
    #[doc = "Bits 8:13 - The number of bits of the SPI address that should be sent."]
    #[inline(always)]
    #[must_use]
    pub fn addrlen(&mut self) -> AddrlenW<SpilenSpec> {
        AddrlenW::new(self, 8)
    }
    #[doc = "Bits 16:31 - The number of data bits to be read or written."]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DatalenW<SpilenSpec> {
        DatalenW::new(self, 16)
    }
}
#[doc = "SPI Transfer Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spilen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spilen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpilenSpec;
impl crate::RegisterSpec for SpilenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spilen::R`](R) reader structure"]
impl crate::Readable for SpilenSpec {}
#[doc = "`write(|w| ..)` method takes [`spilen::W`](W) writer structure"]
impl crate::Writable for SpilenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPILEN to value 0"]
impl crate::Resettable for SpilenSpec {
    const RESET_VALUE: u32 = 0;
}
