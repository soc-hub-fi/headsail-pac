#[doc = "Register `cmd` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `cmd` writer - "]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<CmdSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CmdSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "Command (1 = reset, 2 = run, 4 = break)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
