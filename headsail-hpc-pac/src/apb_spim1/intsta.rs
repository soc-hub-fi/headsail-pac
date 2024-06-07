#[doc = "Register `INTSTA` reader"]
pub type R = crate::R<IntstaSpec>;
#[doc = "Field `INTSTA` reader - Not used"]
pub type IntstaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Not used"]
    #[inline(always)]
    pub fn intsta(&self) -> IntstaR {
        IntstaR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTA")
            .field("intsta", &self.intsta())
            .finish()
    }
}
#[doc = "This register isn't properly specified so we need to look at this\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsta::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstaSpec;
impl crate::RegisterSpec for IntstaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsta::R`](R) reader structure"]
impl crate::Readable for IntstaSpec {}
#[doc = "`reset()` method sets INTSTA to value 0"]
impl crate::Resettable for IntstaSpec {
    const RESET_VALUE: u32 = 0;
}
