#[doc = "Register `external_memory_len` reader"]
pub type R = crate::R<ExternalMemoryLenSpec>;
#[doc = "Register `external_memory_len` writer"]
pub type W = crate::W<ExternalMemoryLenSpec>;
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
        f.debug_struct("external_memory_len")
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<ExternalMemoryLenSpec> {
        LenW::new(self, 0)
    }
}
#[doc = "Original memory map calls this dram_addr_length but the register has been repurposed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`external_memory_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`external_memory_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExternalMemoryLenSpec;
impl crate::RegisterSpec for ExternalMemoryLenSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`external_memory_len::R`](R) reader structure"]
impl crate::Readable for ExternalMemoryLenSpec {}
#[doc = "`write(|w| ..)` method takes [`external_memory_len::W`](W) writer structure"]
impl crate::Writable for ExternalMemoryLenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets external_memory_len to value 0x0010_0000_0000"]
impl crate::Resettable for ExternalMemoryLenSpec {
    const RESET_VALUE: u64 = 0x0010_0000_0000;
}
