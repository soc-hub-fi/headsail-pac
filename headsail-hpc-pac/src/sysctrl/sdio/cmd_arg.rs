#[doc = "Register `CMD_ARG` writer"]
pub type W = crate::W<CmdArgSpec>;
#[doc = "Field `CMD_ARGUMENT` writer - "]
pub type CmdArgumentW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<CmdArgSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_argument(&mut self) -> CmdArgumentW<CmdArgSpec> {
        CmdArgumentW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_arg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdArgSpec;
impl crate::RegisterSpec for CmdArgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd_arg::W`](W) writer structure"]
impl crate::Writable for CmdArgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_ARG to value 0"]
impl crate::Resettable for CmdArgSpec {
    const RESET_VALUE: u32 = 0;
}
