#[doc = "Register `rtc_cfg1` reader"]
pub type R = crate::R<RtcCfg1Spec>;
#[doc = "Register `rtc_cfg1` writer"]
pub type W = crate::W<RtcCfg1Spec>;
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
        f.debug_struct("rtc_cfg1")
            .field("cfg", &self.cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<RtcCfg1Spec> {
        CfgW::new(self, 0)
    }
}
#[doc = "Real-time clock generator clock low count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCfg1Spec;
impl crate::RegisterSpec for RtcCfg1Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`rtc_cfg1::R`](R) reader structure"]
impl crate::Readable for RtcCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cfg1::W`](W) writer structure"]
impl crate::Writable for RtcCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets rtc_cfg1 to value 0x05f5_e100_0324"]
impl crate::Resettable for RtcCfg1Spec {
    const RESET_VALUE: u64 = 0x05f5_e100_0324;
}
