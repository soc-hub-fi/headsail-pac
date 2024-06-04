#[doc = "Register `RESET_LO` reader"]
pub type R = crate::R<RESET_LO_SPEC>;
#[doc = "Register `RESET_LO` writer"]
pub type W = crate::W<RESET_LO_SPEC>;
#[doc = "Field `RESET_LO` reader - "]
pub type RESET_LO_R = crate::BitReader;
#[doc = "Field `RESET_LO` writer - "]
pub type RESET_LO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_lo(&self) -> RESET_LO_R {
        RESET_LO_R::new((self.bits & 1) != 0)
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
    pub fn reset_lo(&mut self) -> RESET_LO_W<RESET_LO_SPEC> {
        RESET_LO_W::new(self, 0)
    }
}
#[doc = "Reset Timer Low counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_LO_SPEC;
impl crate::RegisterSpec for RESET_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_lo::R`](R) reader structure"]
impl crate::Readable for RESET_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_lo::W`](W) writer structure"]
impl crate::Writable for RESET_LO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_LO to value 0"]
impl crate::Resettable for RESET_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
