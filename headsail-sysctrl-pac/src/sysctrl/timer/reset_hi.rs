#[doc = "Register `RESET_HI` reader"]
pub type R = crate::R<ResetHiSpec>;
#[doc = "Register `RESET_HI` writer"]
pub type W = crate::W<ResetHiSpec>;
#[doc = "Field `RESET_HI` reader - "]
pub type ResetHiR = crate::BitReader;
#[doc = "Field `RESET_HI` writer - "]
pub type ResetHiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_hi(&self) -> ResetHiR {
        ResetHiR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_HI")
            .field("reset_hi", &self.reset_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reset_hi(&mut self) -> ResetHiW<ResetHiSpec> {
        ResetHiW::new(self, 0)
    }
}
#[doc = "Reset Timer High counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetHiSpec;
impl crate::RegisterSpec for ResetHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_hi::R`](R) reader structure"]
impl crate::Readable for ResetHiSpec {}
#[doc = "`write(|w| ..)` method takes [`reset_hi::W`](W) writer structure"]
impl crate::Writable for ResetHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_HI to value 0"]
impl crate::Resettable for ResetHiSpec {
    const RESET_VALUE: u32 = 0;
}
