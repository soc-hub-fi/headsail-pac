#[doc = "Register `PADCFG1` reader"]
pub type R = crate::R<PADCFG1_SPEC>;
#[doc = "Register `PADCFG1` writer"]
pub type W = crate::W<PADCFG1_SPEC>;
#[doc = "Field `PADCFG1` reader - "]
pub type PADCFG1_R = crate::FieldReader<u32>;
#[doc = "Field `PADCFG1` writer - "]
pub type PADCFG1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg1(&self) -> PADCFG1_R {
        PADCFG1_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADCFG1")
            .field("padcfg1", &self.padcfg1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg1(&mut self) -> PADCFG1_W<PADCFG1_SPEC> {
        PADCFG1_W::new(self, 0)
    }
}
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCFG1_SPEC;
impl crate::RegisterSpec for PADCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padcfg1::R`](R) reader structure"]
impl crate::Readable for PADCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padcfg1::W`](W) writer structure"]
impl crate::Writable for PADCFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCFG1 to value 0"]
impl crate::Resettable for PADCFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
