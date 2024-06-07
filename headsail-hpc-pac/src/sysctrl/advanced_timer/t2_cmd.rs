#[doc = "Register `T2_CMD` reader"]
pub type R = crate::R<T2CmdSpec>;
#[doc = "Register `T2_CMD` writer"]
pub type W = crate::W<T2CmdSpec>;
#[doc = "Field `START` reader - ADV_TIMER2 start command bitfield"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - ADV_TIMER2 start command bitfield"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - ADV_TIMER2 stop command bitfield."]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - ADV_TIMER2 stop command bitfield."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE` reader - ADV_TIMER2 update command bitfield."]
pub type UpdateR = crate::BitReader;
#[doc = "Field `UPDATE` writer - ADV_TIMER2 update command bitfield."]
pub type UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - ADV_TIMER2 reset command bitfield"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - ADV_TIMER2 reset command bitfield"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARM` reader - ADV_TIMER2 arm command bitfield."]
pub type ArmR = crate::BitReader;
#[doc = "Field `ARM` writer - ADV_TIMER2 arm command bitfield."]
pub type ArmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADV_TIMER2 start command bitfield"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADV_TIMER2 stop command bitfield."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADV_TIMER2 update command bitfield."]
    #[inline(always)]
    pub fn update(&self) -> UpdateR {
        UpdateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADV_TIMER2 reset command bitfield"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADV_TIMER2 arm command bitfield."]
    #[inline(always)]
    pub fn arm(&self) -> ArmR {
        ArmR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T2_CMD")
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("update", &self.update())
            .field("reset", &self.reset())
            .field("arm", &self.arm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADV_TIMER2 start command bitfield"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<T2CmdSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - ADV_TIMER2 stop command bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<T2CmdSpec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - ADV_TIMER2 update command bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UpdateW<T2CmdSpec> {
        UpdateW::new(self, 2)
    }
    #[doc = "Bit 3 - ADV_TIMER2 reset command bitfield"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<T2CmdSpec> {
        ResetW::new(self, 3)
    }
    #[doc = "Bit 4 - ADV_TIMER2 arm command bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ArmW<T2CmdSpec> {
        ArmW::new(self, 4)
    }
}
#[doc = "ADV_TIMER2 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T2CmdSpec;
impl crate::RegisterSpec for T2CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t2_cmd::R`](R) reader structure"]
impl crate::Readable for T2CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`t2_cmd::W`](W) writer structure"]
impl crate::Writable for T2CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T2_CMD to value 0"]
impl crate::Resettable for T2CmdSpec {
    const RESET_VALUE: u32 = 0;
}
