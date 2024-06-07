#[doc = "Register `SOURCE_MODE` reader"]
pub type R = crate::R<SourceModeSpec>;
#[doc = "Register `SOURCE_MODE` writer"]
pub type W = crate::W<SourceModeSpec>;
#[doc = "Field `Source_Mode` reader - "]
pub type SourceModeR = crate::FieldReader<u32>;
#[doc = "Field `Source_Mode` writer - "]
pub type SourceModeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn source_mode(&self) -> SourceModeR {
        SourceModeR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOURCE_MODE")
            .field("source_mode", &self.source_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn source_mode(&mut self) -> SourceModeW<SourceModeSpec> {
        SourceModeW::new(self, 0)
    }
}
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SourceModeSpec;
impl crate::RegisterSpec for SourceModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`source_mode::R`](R) reader structure"]
impl crate::Readable for SourceModeSpec {}
#[doc = "`write(|w| ..)` method takes [`source_mode::W`](W) writer structure"]
impl crate::Writable for SourceModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOURCE_MODE to value 0"]
impl crate::Resettable for SourceModeSpec {
    const RESET_VALUE: u32 = 0;
}
