#[doc = "Register `EVENT_CFG` reader"]
pub type R = crate::R<EventCfgSpec>;
#[doc = "Register `EVENT_CFG` writer"]
pub type W = crate::W<EventCfgSpec>;
#[doc = "Field `EVENT_CFG` reader - "]
pub type EventCfgR = crate::FieldReader<u32>;
#[doc = "Field `EVENT_CFG` writer - "]
pub type EventCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn event_cfg(&self) -> EventCfgR {
        EventCfgR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVENT_CFG")
            .field("event_cfg", &self.event_cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn event_cfg(&mut self) -> EventCfgW<EventCfgSpec> {
        EventCfgW::new(self, 0)
    }
}
#[doc = "ADV_TIMERS events configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventCfgSpec;
impl crate::RegisterSpec for EventCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_cfg::R`](R) reader structure"]
impl crate::Readable for EventCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`event_cfg::W`](W) writer structure"]
impl crate::Writable for EventCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENT_CFG to value 0"]
impl crate::Resettable for EventCfgSpec {
    const RESET_VALUE: u32 = 0;
}
