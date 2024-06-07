#[doc = "Register `read` reader"]
pub type R = crate::R<ReadSpec>;
#[doc = "Field `read` reader - "]
pub type ReadR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("read").field("read", &self.read()).finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadSpec;
impl crate::RegisterSpec for ReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read::R`](R) reader structure"]
impl crate::Readable for ReadSpec {}
#[doc = "`reset()` method sets read to value 0"]
impl crate::Resettable for ReadSpec {
    const RESET_VALUE: u32 = 0;
}
