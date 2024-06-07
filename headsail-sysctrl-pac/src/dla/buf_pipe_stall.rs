#[doc = "Register `buf_pipe_stall` reader"]
pub type R = crate::R<BufPipeStallSpec>;
#[doc = "Field `cycles` reader - "]
pub type CyclesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cycles(&self) -> CyclesR {
        CyclesR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_pipe_stall")
            .field("cycles", &self.cycles())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_pipe_stall::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufPipeStallSpec;
impl crate::RegisterSpec for BufPipeStallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_pipe_stall::R`](R) reader structure"]
impl crate::Readable for BufPipeStallSpec {}
#[doc = "`reset()` method sets buf_pipe_stall to value 0"]
impl crate::Resettable for BufPipeStallSpec {
    const RESET_VALUE: u32 = 0;
}
