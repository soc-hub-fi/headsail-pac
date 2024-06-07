#[doc = "Register `START_HI` reader"]
pub type R = crate::R<StartHiSpec>;
#[doc = "Register `START_HI` writer"]
pub type W = crate::W<StartHiSpec>;
#[doc = "Field `START_HI` reader - Timer high reset command (writes RST in CFG_LO)"]
pub type StartHiR = crate::BitReader;
#[doc = "Field `START_HI` writer - Timer high reset command (writes RST in CFG_LO)"]
pub type StartHiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer high reset command (writes RST in CFG_LO)"]
    #[inline(always)]
    pub fn start_hi(&self) -> StartHiR {
        StartHiR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START_HI")
            .field("start_hi", &self.start_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer high reset command (writes RST in CFG_LO)"]
    #[inline(always)]
    #[must_use]
    pub fn start_hi(&mut self) -> StartHiW<StartHiSpec> {
        StartHiW::new(self, 0)
    }
}
#[doc = "Start Timer High counting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartHiSpec;
impl crate::RegisterSpec for StartHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start_hi::R`](R) reader structure"]
impl crate::Readable for StartHiSpec {}
#[doc = "`write(|w| ..)` method takes [`start_hi::W`](W) writer structure"]
impl crate::Writable for StartHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START_HI to value 0"]
impl crate::Resettable for StartHiSpec {
    const RESET_VALUE: u32 = 0;
}
