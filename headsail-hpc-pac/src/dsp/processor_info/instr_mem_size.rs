#[doc = "Register `instr_mem_size` reader"]
pub type R = crate::R<InstrMemSizeSpec>;
#[doc = "Field `size` reader - "]
pub type SizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
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
pub struct InstrMemSizeSpec;
impl crate::RegisterSpec for InstrMemSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instr_mem_size::R`](R) reader structure"]
impl crate::Readable for InstrMemSizeSpec {}
#[doc = "`reset()` method sets instr_mem_size to value 0x0001_0000"]
impl crate::Resettable for InstrMemSizeSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
