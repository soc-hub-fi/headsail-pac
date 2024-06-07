#[doc = "Register `RESET_LO` reader"]
pub type R = crate::R<ResetLoSpec>;
#[doc = "Register `RESET_LO` writer"]
pub type W = crate::W<ResetLoSpec>;
#[doc = "Field `RESET_LO` reader - "]
pub type ResetLoR = crate::BitReader;
#[doc = "Field `RESET_LO` writer - "]
pub type ResetLoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_lo(&self) -> ResetLoR {
        ResetLoR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_LO")
            .field("reset_lo", &self.reset_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reset_lo(&mut self) -> ResetLoW<ResetLoSpec> {
        ResetLoW::new(self, 0)
    }
}
#[doc = "Reset Timer Low counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetLoSpec;
impl crate::RegisterSpec for ResetLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_lo::R`](R) reader structure"]
impl crate::Readable for ResetLoSpec {}
#[doc = "`write(|w| ..)` method takes [`reset_lo::W`](W) writer structure"]
impl crate::Writable for ResetLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_LO to value 0"]
impl crate::Resettable for ResetLoSpec {
    const RESET_VALUE: u32 = 0;
}
