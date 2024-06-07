#[doc = "Register `START` writer"]
pub type W = crate::W<StartSpec>;
#[doc = "Field `START` writer - "]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<StartSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<StartSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {
    const RESET_VALUE: u32 = 0;
}
