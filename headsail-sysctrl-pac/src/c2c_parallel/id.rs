#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `ID` reader - ID register. Constant ID value"]
pub type IdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID register. Constant ID value"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID").field("id", &self.id()).finish()
    }
}
#[doc = "ID register. Constant ID value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets ID to value 0x1234_abcd"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0x1234_abcd;
}
