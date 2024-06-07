#[doc = "Register `ctrl_size` reader"]
pub type R = crate::R<CtrlSizeSpec>;
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
        f.debug_struct("ctrl_size")
            .field("size", &self.size())
            .finish()
    }
}
#[doc = "CTRL size, per core, in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSizeSpec;
impl crate::RegisterSpec for CtrlSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_size::R`](R) reader structure"]
impl crate::Readable for CtrlSizeSpec {}
#[doc = "`reset()` method sets ctrl_size to value 0x0400"]
impl crate::Resettable for CtrlSizeSpec {
    const RESET_VALUE: u32 = 0x0400;
}
