#[doc = "Register `c_bank_mask` reader"]
pub type R = crate::R<CBankMaskSpec>;
#[doc = "Register `c_bank_mask` writer"]
pub type W = crate::W<CBankMaskSpec>;
#[doc = "Field `c_bank_mask` reader - "]
pub type CBankMaskR = crate::FieldReader<u32>;
#[doc = "Field `c_bank_mask` writer - "]
pub type CBankMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_bank_mask(&self) -> CBankMaskR {
        CBankMaskR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_bank_mask")
            .field("c_bank_mask", &self.c_bank_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_bank_mask(&mut self) -> CBankMaskW<CBankMaskSpec> {
        CBankMaskW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_bank_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_bank_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBankMaskSpec;
impl crate::RegisterSpec for CBankMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_bank_mask::R`](R) reader structure"]
impl crate::Readable for CBankMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`c_bank_mask::W`](W) writer structure"]
impl crate::Writable for CBankMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_bank_mask to value 0x03"]
impl crate::Resettable for CBankMaskSpec {
    const RESET_VALUE: u32 = 0x03;
}
