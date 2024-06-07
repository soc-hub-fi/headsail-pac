#[doc = "Register `instr_mem_start_hi` reader"]
pub type R = crate::R<InstrMemStartHiSpec>;
#[doc = "Field `start_hi` reader - "]
pub type StartHiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn start_hi(&self) -> StartHiR {
        StartHiR::new(self.bits)
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
pub struct InstrMemStartHiSpec;
impl crate::RegisterSpec for InstrMemStartHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instr_mem_start_hi::R`](R) reader structure"]
impl crate::Readable for InstrMemStartHiSpec {}
#[doc = "`reset()` method sets instr_mem_start_hi to value 0"]
impl crate::Resettable for InstrMemStartHiSpec {
    const RESET_VALUE: u32 = 0;
}
