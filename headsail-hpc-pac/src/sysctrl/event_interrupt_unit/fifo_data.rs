#[doc = "Register `FIFO_DATA` reader"]
pub type R = crate::R<FifoDataSpec>;
#[doc = "Field `FIFO_DATA` reader - This is a read-only register that contain the first valid value of the FIFO"]
pub type FifoDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This is a read-only register that contain the first valid value of the FIFO"]
    #[inline(always)]
    pub fn fifo_data(&self) -> FifoDataR {
        FifoDataR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_DATA")
            .field("fifo_data", &self.fifo_data())
            .finish()
    }
}
#[doc = "Fifo Content. This is a read-only register that contain the first valid value of the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoDataSpec;
impl crate::RegisterSpec for FifoDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_data::R`](R) reader structure"]
impl crate::Readable for FifoDataSpec {}
#[doc = "`reset()` method sets FIFO_DATA to value 0"]
impl crate::Resettable for FifoDataSpec {
    const RESET_VALUE: u32 = 0;
}
