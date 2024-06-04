#[doc = "Register `instr_mem_size` reader"]
pub type R = crate::R<INSTR_MEM_SIZE_SPEC>;
#[doc = "Field `size` reader - "]
pub type SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("instr_mem_size")
            .field("size", &self.size())
            .finish()
    }
}
#[doc = "Instruction memory size, in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSTR_MEM_SIZE_SPEC;
impl crate::RegisterSpec for INSTR_MEM_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instr_mem_size::R`](R) reader structure"]
impl crate::Readable for INSTR_MEM_SIZE_SPEC {}
#[doc = "`reset()` method sets instr_mem_size to value 0x0001_0000"]
impl crate::Resettable for INSTR_MEM_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
