#[doc = "Register `instr_mem_start_hi` reader"]
pub type R = crate::R<INSTR_MEM_START_HI_SPEC>;
#[doc = "Field `start_hi` reader - "]
pub type START_HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn start_hi(&self) -> START_HI_R {
        START_HI_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("instr_mem_start_hi")
            .field("start_hi", &self.start_hi())
            .finish()
    }
}
#[doc = "Start of instruction memory, high 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_start_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTR_MEM_START_HI_SPEC;
impl crate::RegisterSpec for INSTR_MEM_START_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instr_mem_start_hi::R`](R) reader structure"]
impl crate::Readable for INSTR_MEM_START_HI_SPEC {}
#[doc = "`reset()` method sets instr_mem_start_hi to value 0"]
impl crate::Resettable for INSTR_MEM_START_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
