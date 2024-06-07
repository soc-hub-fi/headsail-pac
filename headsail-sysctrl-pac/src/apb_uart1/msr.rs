#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Register `MSR` writer"]
pub type W = crate::W<MsrSpec>;
#[doc = "Field `MSR` reader - "]
pub type MsrR = crate::FieldReader;
#[doc = "Field `MSR` writer - "]
pub type MsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn msr(&self) -> MsrR {
        MsrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSR").field("msr", &self.msr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn msr(&mut self) -> MsrW<MsrSpec> {
        MsrW::new(self, 0)
    }
}
#[doc = "modem status (not used)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u8 = 0;
}
