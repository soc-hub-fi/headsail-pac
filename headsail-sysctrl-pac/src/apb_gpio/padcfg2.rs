#[doc = "Register `PADCFG2` reader"]
pub type R = crate::R<PADCFG2_SPEC>;
#[doc = "Register `PADCFG2` writer"]
pub type W = crate::W<PADCFG2_SPEC>;
#[doc = "Field `PADCFG2` reader - "]
pub type PADCFG2_R = crate::FieldReader<u32>;
#[doc = "Field `PADCFG2` writer - "]
pub type PADCFG2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg2(&self) -> PADCFG2_R {
        PADCFG2_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADCFG2")
            .field("padcfg2", &self.padcfg2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg2(&mut self) -> PADCFG2_W<PADCFG2_SPEC> {
        PADCFG2_W::new(self, 0)
    }
}
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCFG2_SPEC;
impl crate::RegisterSpec for PADCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padcfg2::R`](R) reader structure"]
impl crate::Readable for PADCFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padcfg2::W`](W) writer structure"]
impl crate::Writable for PADCFG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCFG2 to value 0"]
impl crate::Resettable for PADCFG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
