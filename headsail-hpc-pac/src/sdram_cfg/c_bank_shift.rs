#[doc = "Register `c_bank_shift` reader"]
pub type R = crate::R<CBankShiftSpec>;
#[doc = "Register `c_bank_shift` writer"]
pub type W = crate::W<CBankShiftSpec>;
#[doc = "Field `c_bank_shift` reader - "]
pub type CBankShiftR = crate::FieldReader<u32>;
#[doc = "Field `c_bank_shift` writer - "]
pub type CBankShiftW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_bank_shift(&self) -> CBankShiftR {
        CBankShiftR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_bank_shift")
            .field("c_bank_shift", &self.c_bank_shift())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_bank_shift(&mut self) -> CBankShiftW<CBankShiftSpec> {
        CBankShiftW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_bank_shift::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_bank_shift::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBankShiftSpec;
impl crate::RegisterSpec for CBankShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_bank_shift::R`](R) reader structure"]
impl crate::Readable for CBankShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`c_bank_shift::W`](W) writer structure"]
impl crate::Writable for CBankShiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_bank_shift to value 0x0b"]
impl crate::Resettable for CBankShiftSpec {
    const RESET_VALUE: u32 = 0x0b;
}
