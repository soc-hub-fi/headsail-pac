#[doc = "Register `axi_enabled` reader"]
pub type R = crate::R<AxiEnabledSpec>;
#[doc = "Register `axi_enabled` writer"]
pub type W = crate::W<AxiEnabledSpec>;
#[doc = "Field `axi_enabled` reader - "]
pub type AxiEnabledR = crate::FieldReader<u32>;
#[doc = "Field `axi_enabled` writer - "]
pub type AxiEnabledW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn axi_enabled(&self) -> AxiEnabledR {
        AxiEnabledR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("axi_enabled")
            .field("axi_enabled", &self.axi_enabled())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn axi_enabled(&mut self) -> AxiEnabledW<AxiEnabledSpec> {
        AxiEnabledW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_enabled::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_enabled::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiEnabledSpec;
impl crate::RegisterSpec for AxiEnabledSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_enabled::R`](R) reader structure"]
impl crate::Readable for AxiEnabledSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_enabled::W`](W) writer structure"]
impl crate::Writable for AxiEnabledSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets axi_enabled to value 0x01"]
impl crate::Resettable for AxiEnabledSpec {
    const RESET_VALUE: u32 = 0x01;
}
