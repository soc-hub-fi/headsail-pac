#[doc = "Register `core3_hart_id` reader"]
pub type R = crate::R<Core3HartIdSpec>;
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
        f.debug_struct("core3_hart_id")
            .field("hart_id", &self.hart_id())
            .finish()
    }
}
#[doc = "Core #3 Hart ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core3_hart_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core3HartIdSpec;
impl crate::RegisterSpec for Core3HartIdSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`core3_hart_id::R`](R) reader structure"]
impl crate::Readable for Core3HartIdSpec {}
#[doc = "`reset()` method sets core3_hart_id to value 0x03"]
impl crate::Resettable for Core3HartIdSpec {
    const RESET_VALUE: u64 = 0x03;
}
