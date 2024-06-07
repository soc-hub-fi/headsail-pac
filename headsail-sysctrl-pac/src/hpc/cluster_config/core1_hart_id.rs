#[doc = "Register `core1_hart_id` reader"]
pub type R = crate::R<Core1HartIdSpec>;
#[doc = "Field `hart_id` reader - "]
pub type HartIdR = crate::FieldReader<u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn hart_id(&self) -> HartIdR {
        HartIdR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("core1_hart_id")
            .field("hart_id", &self.hart_id())
            .finish()
    }
}
#[doc = "Core #1 Hart ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_hart_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1HartIdSpec;
impl crate::RegisterSpec for Core1HartIdSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`core1_hart_id::R`](R) reader structure"]
impl crate::Readable for Core1HartIdSpec {}
#[doc = "`reset()` method sets core1_hart_id to value 0x01"]
impl crate::Resettable for Core1HartIdSpec {
    const RESET_VALUE: u64 = 0x01;
}
