#[doc = "Register `PADCFG0` reader"]
pub type R = crate::R<Padcfg0Spec>;
#[doc = "Register `PADCFG0` writer"]
pub type W = crate::W<Padcfg0Spec>;
#[doc = "Field `PADCFG0` reader - "]
pub type Padcfg0R = crate::FieldReader<u32>;
#[doc = "Field `PADCFG0` writer - "]
pub type Padcfg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padcfg0(&self) -> Padcfg0R {
        Padcfg0R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADCFG0")
            .field("padcfg0", &self.padcfg0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg0(&mut self) -> Padcfg0W<Padcfg0Spec> {
        Padcfg0W::new(self, 0)
    }
}
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Padcfg0Spec;
impl crate::RegisterSpec for Padcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padcfg0::R`](R) reader structure"]
impl crate::Readable for Padcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`padcfg0::W`](W) writer structure"]
impl crate::Writable for Padcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCFG0 to value 0"]
impl crate::Resettable for Padcfg0Spec {
    const RESET_VALUE: u32 = 0;
}
