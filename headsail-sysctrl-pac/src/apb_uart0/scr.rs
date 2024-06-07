#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `SCR` reader - "]
pub type ScrR = crate::FieldReader;
#[doc = "Field `SCR` writer - "]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR").field("scr", &self.scr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> ScrW<ScrSpec> {
        ScrW::new(self, 0)
    }
}
#[doc = "scratch (not used)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u8 = 0;
}
