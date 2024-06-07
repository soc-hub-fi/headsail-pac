#[doc = "Register `rom_len` reader"]
pub type R = crate::R<RomLenSpec>;
#[doc = "Register `rom_len` writer"]
pub type W = crate::W<RomLenSpec>;
#[doc = "Field `len` reader - "]
pub type LenR = crate::FieldReader<u64>;
#[doc = "Field `len` writer - "]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("rom_len").field("len", &self.len()).finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<RomLenSpec> {
        LenW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomLenSpec;
impl crate::RegisterSpec for RomLenSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`rom_len::R`](R) reader structure"]
impl crate::Readable for RomLenSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_len::W`](W) writer structure"]
impl crate::Writable for RomLenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets rom_len to value 0x8000"]
impl crate::Resettable for RomLenSpec {
    const RESET_VALUE: u64 = 0x8000;
}
