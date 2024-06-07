#[doc = "Register `SOURCE_LENGTH` reader"]
pub type R = crate::R<SourceLengthSpec>;
#[doc = "Register `SOURCE_LENGTH` writer"]
pub type W = crate::W<SourceLengthSpec>;
#[doc = "Field `Source_Length` reader - "]
pub type SourceLengthR = crate::FieldReader<u32>;
#[doc = "Field `Source_Length` writer - "]
pub type SourceLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn source_length(&self) -> SourceLengthR {
        SourceLengthR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOURCE_LENGTH")
            .field("source_length", &self.source_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn source_length(&mut self) -> SourceLengthW<SourceLengthSpec> {
        SourceLengthW::new(self, 0)
    }
}
#[doc = "Length of read transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SourceLengthSpec;
impl crate::RegisterSpec for SourceLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`source_length::R`](R) reader structure"]
impl crate::Readable for SourceLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`source_length::W`](W) writer structure"]
impl crate::Writable for SourceLengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOURCE_LENGTH to value 0"]
impl crate::Resettable for SourceLengthSpec {
    const RESET_VALUE: u32 = 0;
}
