#[doc = "Register `instr_mem_start_lo` reader"]
pub type R = crate::R<INSTR_MEM_START_LO_SPEC>;
#[doc = "Field `start_lo` reader - "]
pub type START_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn start_lo(&self) -> START_LO_R {
        START_LO_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("instr_mem_start_lo")
            .field("start_lo", &self.start_lo())
            .finish()
    }
}
#[doc = "Start of instruction memory, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_start_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTR_MEM_START_LO_SPEC;
impl crate::RegisterSpec for INSTR_MEM_START_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instr_mem_start_lo::R`](R) reader structure"]
impl crate::Readable for INSTR_MEM_START_LO_SPEC {}
#[doc = "`reset()` method sets instr_mem_start_lo to value 0xff81_0000"]
impl crate::Resettable for INSTR_MEM_START_LO_SPEC {
    const RESET_VALUE: u32 = 0xff81_0000;
}
