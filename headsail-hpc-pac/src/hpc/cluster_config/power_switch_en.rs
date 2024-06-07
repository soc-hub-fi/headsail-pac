#[doc = "Register `power_switch_en` reader"]
pub type R = crate::R<PowerSwitchEnSpec>;
#[doc = "Register `power_switch_en` writer"]
pub type W = crate::W<PowerSwitchEnSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: `1`"]
    Enabled = 1,
    #[doc = "0: `0`"]
    Disabled = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `en` reader - "]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::Enabled,
            false => En::Disabled,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En::Enabled
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
}
#[doc = "Field `en` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("power_switch_en")
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<PowerSwitchEnSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_switch_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_switch_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSwitchEnSpec;
impl crate::RegisterSpec for PowerSwitchEnSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`power_switch_en::R`](R) reader structure"]
impl crate::Readable for PowerSwitchEnSpec {}
#[doc = "`write(|w| ..)` method takes [`power_switch_en::W`](W) writer structure"]
impl crate::Writable for PowerSwitchEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets power_switch_en to value 0"]
impl crate::Resettable for PowerSwitchEnSpec {
    const RESET_VALUE: u64 = 0;
}
