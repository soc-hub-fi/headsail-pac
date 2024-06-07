#[doc = "Register `bp_count` reader"]
pub type R = crate::R<BpCountSpec>;
#[doc = "Field `count` reader - "]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("bp_count")
            .field("count", &self.count())
            .finish()
    }
}
#[doc = "Breakpoint count (0x2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpCountSpec;
impl crate::RegisterSpec for BpCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bp_count::R`](R) reader structure"]
impl crate::Readable for BpCountSpec {}
#[doc = "`reset()` method sets bp_count to value 0x02"]
impl crate::Resettable for BpCountSpec {
    const RESET_VALUE: u32 = 0x02;
}
