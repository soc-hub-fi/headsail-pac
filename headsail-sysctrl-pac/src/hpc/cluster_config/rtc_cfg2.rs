#[doc = "Register `rtc_cfg2` reader"]
pub type R = crate::R<RtcCfg2Spec>;
#[doc = "Register `rtc_cfg2` writer"]
pub type W = crate::W<RtcCfg2Spec>;
#[doc = "Field `cfg` reader - "]
pub type CfgR = crate::FieldReader<u64>;
#[doc = "Field `cfg` writer - "]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("rtc_cfg2")
            .field("cfg", &self.cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<RtcCfg2Spec> {
        CfgW::new(self, 0)
    }
}
#[doc = "Real-time clock generator clock fine tune\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCfg2Spec;
impl crate::RegisterSpec for RtcCfg2Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`rtc_cfg2::R`](R) reader structure"]
impl crate::Readable for RtcCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cfg2::W`](W) writer structure"]
impl crate::Writable for RtcCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets rtc_cfg2 to value 0x01"]
impl crate::Resettable for RtcCfg2Spec {
    const RESET_VALUE: u64 = 0x01;
}
