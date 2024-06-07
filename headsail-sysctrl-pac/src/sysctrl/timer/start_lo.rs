#[doc = "Register `START_LO` reader"]
pub type R = crate::R<StartLoSpec>;
#[doc = "Register `START_LO` writer"]
pub type W = crate::W<StartLoSpec>;
#[doc = "Field `START_LO` reader - Timer high start command (sets EN in CFG_LO)"]
pub type StartLoR = crate::BitReader;
#[doc = "Field `START_LO` writer - Timer high start command (sets EN in CFG_LO)"]
pub type StartLoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer high start command (sets EN in CFG_LO)"]
    #[inline(always)]
    pub fn start_lo(&self) -> StartLoR {
        StartLoR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START_LO")
            .field("start_lo", &self.start_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer high start command (sets EN in CFG_LO)"]
    #[inline(always)]
    #[must_use]
    pub fn start_lo(&mut self) -> StartLoW<StartLoSpec> {
        StartLoW::new(self, 0)
    }
}
#[doc = "Start Timer Low counting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartLoSpec;
impl crate::RegisterSpec for StartLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start_lo::R`](R) reader structure"]
impl crate::Readable for StartLoSpec {}
#[doc = "`write(|w| ..)` method takes [`start_lo::W`](W) writer structure"]
impl crate::Writable for StartLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START_LO to value 0"]
impl crate::Resettable for StartLoSpec {
    const RESET_VALUE: u32 = 0;
}
