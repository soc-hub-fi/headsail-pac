#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `MCR` reader - "]
pub type McrR = crate::FieldReader;
#[doc = "Field `MCR` writer - "]
pub type McrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mcr(&self) -> McrR {
        McrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR").field("mcr", &self.mcr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn mcr(&mut self) -> McrW<McrSpec> {
        McrW::new(self, 0)
    }
}
#[doc = "modem control (not used)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u8 = 0;
}
