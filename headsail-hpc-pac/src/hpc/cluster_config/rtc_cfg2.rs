#[doc = "Register `rtc_cfg2` reader"]
pub type R = crate::R<RTC_CFG2_SPEC>;
#[doc = "Register `rtc_cfg2` writer"]
pub type W = crate::W<RTC_CFG2_SPEC>;
#[doc = "Field `cfg` reader - "]
pub type CFG_R = crate::FieldReader<u64>;
#[doc = "Field `cfg` writer - "]
pub type CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(self.bits)
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
    pub fn cfg(&mut self) -> CFG_W<RTC_CFG2_SPEC> {
        CFG_W::new(self, 0)
    }
}
#[doc = "Real-time clock generator clock fine tune\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_CFG2_SPEC;
impl crate::RegisterSpec for RTC_CFG2_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`rtc_cfg2::R`](R) reader structure"]
impl crate::Readable for RTC_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_cfg2::W`](W) writer structure"]
impl crate::Writable for RTC_CFG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets rtc_cfg2 to value 0x01"]
impl crate::Resettable for RTC_CFG2_SPEC {
    const RESET_VALUE: u64 = 0x01;
}
