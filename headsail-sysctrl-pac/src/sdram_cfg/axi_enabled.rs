#[doc = "Register `axi_enabled` reader"]
pub type R = crate::R<AXI_ENABLED_SPEC>;
#[doc = "Register `axi_enabled` writer"]
pub type W = crate::W<AXI_ENABLED_SPEC>;
#[doc = "Field `axi_enabled` reader - "]
pub type AXI_ENABLED_R = crate::FieldReader<u32>;
#[doc = "Field `axi_enabled` writer - "]
pub type AXI_ENABLED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn axi_enabled(&self) -> AXI_ENABLED_R {
        AXI_ENABLED_R::new(self.bits)
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
    pub fn axi_enabled(&mut self) -> AXI_ENABLED_W<AXI_ENABLED_SPEC> {
        AXI_ENABLED_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_enabled::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_enabled::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_ENABLED_SPEC;
impl crate::RegisterSpec for AXI_ENABLED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_enabled::R`](R) reader structure"]
impl crate::Readable for AXI_ENABLED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_enabled::W`](W) writer structure"]
impl crate::Writable for AXI_ENABLED_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets axi_enabled to value 0x01"]
impl crate::Resettable for AXI_ENABLED_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
