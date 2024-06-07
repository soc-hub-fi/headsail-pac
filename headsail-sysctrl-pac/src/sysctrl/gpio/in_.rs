#[doc = "Register `IN` reader"]
pub type R = crate::R<InSpec>;
#[doc = "Field `DATA_IN` reader - "]
pub type DataInR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_in(&self) -> DataInR {
        DataInR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN")
            .field("data_in", &self.data_in())
            .finish()
    }
}
#[doc = "GPIO Data IN register. Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSpec;
impl crate::RegisterSpec for InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for InSpec {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for InSpec {
    const RESET_VALUE: u32 = 0;
}
