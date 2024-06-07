#[doc = "Register `PADIN` writer"]
pub type W = crate::W<PadinSpec>;
#[doc = "Field `PADIN_0` writer - "]
pub type Padin0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<PadinSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padin_0(&mut self) -> Padin0W<PadinSpec> {
        Padin0W::new(self, 0)
    }
}
#[doc = "Input Values.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadinSpec;
impl crate::RegisterSpec for PadinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`padin::W`](W) writer structure"]
impl crate::Writable for PadinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADIN to value 0"]
impl crate::Resettable for PadinSpec {
    const RESET_VALUE: u32 = 0;
}
