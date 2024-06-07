#[doc = "Register `SPIDUM` reader"]
pub type R = crate::R<SPIDUM_SPEC>;
#[doc = "Register `SPIDUM` writer"]
pub type W = crate::W<SPIDUM_SPEC>;
#[doc = "Field `DUMMYRD` reader - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and reading the data."]
pub type DUMMYRD_R = crate::FieldReader<u16>;
#[doc = "Field `DUMMYRD` writer - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and reading the data."]
pub type DUMMYRD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DUMMYWR` reader - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and writing the data."]
pub type DUMMYWR_R = crate::FieldReader<u16>;
#[doc = "Field `DUMMYWR` writer - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and writing the data."]
pub type DUMMYWR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and reading the data."]
    #[inline(always)]
    pub fn dummyrd(&self) -> DUMMYRD_R {
        DUMMYRD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and writing the data."]
    #[inline(always)]
    pub fn dummywr(&self) -> DUMMYWR_R {
        DUMMYWR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIDUM")
            .field("dummyrd", &self.dummyrd())
            .field("dummywr", &self.dummywr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and reading the data."]
    #[inline(always)]
    #[must_use]
    pub fn dummyrd(&mut self) -> DUMMYRD_W<SPIDUM_SPEC> {
        DUMMYRD_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Dummy cycles (nothing being written or read) between sending the SPI command + SPI address and writing the data."]
    #[inline(always)]
    #[must_use]
    pub fn dummywr(&mut self) -> DUMMYWR_W<SPIDUM_SPEC> {
        DUMMYWR_W::new(self, 16)
    }
}
#[doc = "SPI Dummy Cycles\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spidum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spidum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPIDUM_SPEC;
impl crate::RegisterSpec for SPIDUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spidum::R`](R) reader structure"]
impl crate::Readable for SPIDUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spidum::W`](W) writer structure"]
impl crate::Writable for SPIDUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIDUM to value 0"]
impl crate::Resettable for SPIDUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
