#[doc = "Register `LANE_CONFIG[%s]` reader"]
pub type R = crate::R<LANE_CONFIG_SPEC>;
#[doc = "Register `LANE_CONFIG[%s]` writer"]
pub type W = crate::W<LANE_CONFIG_SPEC>;
#[doc = "Field `CONFIG` reader - "]
pub type CONFIG_R = crate::FieldReader<u32>;
#[doc = "Field `CONFIG` writer - "]
pub type CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LANE_CONFIG")
            .field("config", &self.config())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn config(&mut self) -> CONFIG_W<LANE_CONFIG_SPEC> {
        CONFIG_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lane_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LANE_CONFIG_SPEC;
impl crate::RegisterSpec for LANE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lane_config::R`](R) reader structure"]
impl crate::Readable for LANE_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lane_config::W`](W) writer structure"]
impl crate::Writable for LANE_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LANE_CONFIG[%s]
to value 0"]
impl crate::Resettable for LANE_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
