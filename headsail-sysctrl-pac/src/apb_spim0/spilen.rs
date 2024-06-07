#[doc = "Register `SPILEN` reader"]
pub type R = crate::R<SPILEN_SPEC>;
#[doc = "Register `SPILEN` writer"]
pub type W = crate::W<SPILEN_SPEC>;
#[doc = "Field `CMDLEN` reader - The number of bits of the SPI command that should be sent."]
pub type CMDLEN_R = crate::FieldReader;
#[doc = "Field `CMDLEN` writer - The number of bits of the SPI command that should be sent."]
pub type CMDLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADDRLEN` reader - The number of bits of the SPI address that should be sent."]
pub type ADDRLEN_R = crate::FieldReader;
#[doc = "Field `ADDRLEN` writer - The number of bits of the SPI address that should be sent."]
pub type ADDRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DATALEN` reader - The number of data bits to be read or written."]
pub type DATALEN_R = crate::FieldReader<u16>;
#[doc = "Field `DATALEN` writer - The number of data bits to be read or written."]
pub type DATALEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - The number of bits of the SPI command that should be sent."]
    #[inline(always)]
    pub fn cmdlen(&self) -> CMDLEN_R {
        CMDLEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The number of bits of the SPI address that should be sent."]
    #[inline(always)]
    pub fn addrlen(&self) -> ADDRLEN_R {
        ADDRLEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - The number of data bits to be read or written."]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 16) & 0xffff) as u16)
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
    pub fn cmdlen(&mut self) -> CMDLEN_W<SPILEN_SPEC> {
        CMDLEN_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The number of bits of the SPI address that should be sent."]
    #[inline(always)]
    #[must_use]
    pub fn addrlen(&mut self) -> ADDRLEN_W<SPILEN_SPEC> {
        ADDRLEN_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - The number of data bits to be read or written."]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DATALEN_W<SPILEN_SPEC> {
        DATALEN_W::new(self, 16)
    }
}
#[doc = "SPI Transfer Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spilen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spilen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPILEN_SPEC;
impl crate::RegisterSpec for SPILEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spilen::R`](R) reader structure"]
impl crate::Readable for SPILEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spilen::W`](W) writer structure"]
impl crate::Writable for SPILEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPILEN to value 0"]
impl crate::Resettable for SPILEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
