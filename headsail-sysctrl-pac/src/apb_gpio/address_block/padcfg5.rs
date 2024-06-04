#[doc = "Register `PADCFG5` reader"]
pub type R = crate::R<PADCFG5_SPEC>;
#[doc = "Register `PADCFG5` writer"]
pub type W = crate::W<PADCFG5_SPEC>;
#[doc = "Field `PADCFG5` reader - "]
pub type PADCFG5_R = crate::FieldReader<u32>;
#[doc = "Field `PADCFG5` writer - "]
pub type PADCFG5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg5(&self) -> PADCFG5_R {
        PADCFG5_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADCFG5")
            .field("padcfg5", &self.padcfg5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg5(&mut self) -> PADCFG5_W<PADCFG5_SPEC> {
        PADCFG5_W::new(self, 0)
    }
}
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCFG5_SPEC;
impl crate::RegisterSpec for PADCFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padcfg5::R`](R) reader structure"]
impl crate::Readable for PADCFG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padcfg5::W`](W) writer structure"]
impl crate::Writable for PADCFG5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCFG5 to value 0"]
impl crate::Resettable for PADCFG5_SPEC {
    const RESET_VALUE: u32 = 0;
}
