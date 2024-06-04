#[doc = "Register `PADCFG4` reader"]
pub type R = crate::R<PADCFG4_SPEC>;
#[doc = "Register `PADCFG4` writer"]
pub type W = crate::W<PADCFG4_SPEC>;
#[doc = "Field `PADCFG4` reader - "]
pub type PADCFG4_R = crate::FieldReader<u32>;
#[doc = "Field `PADCFG4` writer - "]
pub type PADCFG4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg4(&self) -> PADCFG4_R {
        PADCFG4_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADCFG4")
            .field("padcfg4", &self.padcfg4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg4(&mut self) -> PADCFG4_W<PADCFG4_SPEC> {
        PADCFG4_W::new(self, 0)
    }
}
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCFG4_SPEC;
impl crate::RegisterSpec for PADCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padcfg4::R`](R) reader structure"]
impl crate::Readable for PADCFG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padcfg4::W`](W) writer structure"]
impl crate::Writable for PADCFG4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCFG4 to value 0"]
impl crate::Resettable for PADCFG4_SPEC {
    const RESET_VALUE: u32 = 0;
}
