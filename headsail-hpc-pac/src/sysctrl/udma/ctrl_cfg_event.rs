#[doc = "Register `CTRL_CFG_EVENT` reader"]
pub type R = crate::R<CtrlCfgEventSpec>;
#[doc = "Register `CTRL_CFG_EVENT` writer"]
pub type W = crate::W<CtrlCfgEventSpec>;
#[doc = "Field `CMP_EVT0` reader - Forward event with ID matching CMP_EVT0 to peripherals as event0"]
pub type CmpEvt0R = crate::FieldReader;
#[doc = "Field `CMP_EVT0` writer - Forward event with ID matching CMP_EVT0 to peripherals as event0"]
pub type CmpEvt0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_EVT1` reader - Forward event with ID matching CMP_EVT1 to peripherals as event1"]
pub type CmpEvt1R = crate::FieldReader;
#[doc = "Field `CMP_EVT1` writer - Forward event with ID matching CMP_EVT1 to peripherals as event1"]
pub type CmpEvt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_EVT2` reader - Forward event with ID matching CMP_EVT2 to peripherals as event2"]
pub type CmpEvt2R = crate::FieldReader;
#[doc = "Field `CMP_EVT2` writer - Forward event with ID matching CMP_EVT2 to peripherals as event2"]
pub type CmpEvt2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMP_EVT3` reader - Forward event with ID matching CMP_EVT3 to peripherals as event3"]
pub type CmpEvt3R = crate::FieldReader;
#[doc = "Field `CMP_EVT3` writer - Forward event with ID matching CMP_EVT3 to peripherals as event3"]
pub type CmpEvt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Forward event with ID matching CMP_EVT0 to peripherals as event0"]
    #[inline(always)]
    pub fn cmp_evt0(&self) -> CmpEvt0R {
        CmpEvt0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Forward event with ID matching CMP_EVT1 to peripherals as event1"]
    #[inline(always)]
    pub fn cmp_evt1(&self) -> CmpEvt1R {
        CmpEvt1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Forward event with ID matching CMP_EVT2 to peripherals as event2"]
    #[inline(always)]
    pub fn cmp_evt2(&self) -> CmpEvt2R {
        CmpEvt2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Forward event with ID matching CMP_EVT3 to peripherals as event3"]
    #[inline(always)]
    pub fn cmp_evt3(&self) -> CmpEvt3R {
        CmpEvt3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_CFG_EVENT")
            .field("cmp_evt0", &self.cmp_evt0())
            .field("cmp_evt1", &self.cmp_evt1())
            .field("cmp_evt2", &self.cmp_evt2())
            .field("cmp_evt3", &self.cmp_evt3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Forward event with ID matching CMP_EVT0 to peripherals as event0"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_evt0(&mut self) -> CmpEvt0W<CtrlCfgEventSpec> {
        CmpEvt0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Forward event with ID matching CMP_EVT1 to peripherals as event1"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_evt1(&mut self) -> CmpEvt1W<CtrlCfgEventSpec> {
        CmpEvt1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Forward event with ID matching CMP_EVT2 to peripherals as event2"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_evt2(&mut self) -> CmpEvt2W<CtrlCfgEventSpec> {
        CmpEvt2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Forward event with ID matching CMP_EVT3 to peripherals as event3"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_evt3(&mut self) -> CmpEvt3W<CtrlCfgEventSpec> {
        CmpEvt3W::new(self, 24)
    }
}
#[doc = "uDMA peripherals external event configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_event::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_event::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlCfgEventSpec;
impl crate::RegisterSpec for CtrlCfgEventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_cfg_event::R`](R) reader structure"]
impl crate::Readable for CtrlCfgEventSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_cfg_event::W`](W) writer structure"]
impl crate::Writable for CtrlCfgEventSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_CFG_EVENT to value 0"]
impl crate::Resettable for CtrlCfgEventSpec {
    const RESET_VALUE: u32 = 0;
}
