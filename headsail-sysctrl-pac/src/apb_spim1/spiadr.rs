#[doc = "Register `SPIADR` reader"]
pub type R = crate::R<SpiadrSpec>;
#[doc = "Register `SPIADR` writer"]
pub type W = crate::W<SpiadrSpec>;
#[doc = "Field `SPIADR` reader - Insert the address to be transmitted"]
pub type SpiadrR = crate::FieldReader<u32>;
#[doc = "Field `SPIADR` writer - Insert the address to be transmitted"]
pub type SpiadrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Insert the address to be transmitted"]
    #[inline(always)]
    pub fn spiadr(&self) -> SpiadrR {
        SpiadrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIADR")
            .field("spiadr", &self.spiadr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Insert the address to be transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn spiadr(&mut self) -> SpiadrW<SpiadrSpec> {
        SpiadrW::new(self, 0)
    }
}
#[doc = "SPI Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spiadr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spiadr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiadrSpec;
impl crate::RegisterSpec for SpiadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spiadr::R`](R) reader structure"]
impl crate::Readable for SpiadrSpec {}
#[doc = "`write(|w| ..)` method takes [`spiadr::W`](W) writer structure"]
impl crate::Writable for SpiadrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIADR to value 0"]
impl crate::Resettable for SpiadrSpec {
    const RESET_VALUE: u32 = 0;
}
