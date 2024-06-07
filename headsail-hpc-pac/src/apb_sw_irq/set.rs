#[doc = "Register `set` writer"]
pub type W = crate::W<SetSpec>;
#[doc = "Field `set` writer - "]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<SetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<SetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetSpec;
impl crate::RegisterSpec for SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set::W`](W) writer structure"]
impl crate::Writable for SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets set to value 0"]
impl crate::Resettable for SetSpec {
    const RESET_VALUE: u32 = 0;
}
