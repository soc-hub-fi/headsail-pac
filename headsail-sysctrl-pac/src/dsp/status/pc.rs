#[doc = "Register `pc` reader"]
pub type R = crate::R<PC_SPEC>;
#[doc = "Field `pc` reader - "]
pub type PC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pc").field("pc", &self.pc()).finish()
    }
}
#[doc = "Program counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc::R`](R) reader structure"]
impl crate::Readable for PC_SPEC {}
#[doc = "`reset()` method sets pc to value 0"]
impl crate::Resettable for PC_SPEC {
    const RESET_VALUE: u32 = 0;
}
