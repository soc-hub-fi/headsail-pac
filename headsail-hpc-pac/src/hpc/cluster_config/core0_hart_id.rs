#[doc = "Register `core0_hart_id` reader"]
pub type R = crate::R<CORE0_HART_ID_SPEC>;
#[doc = "Field `hart_id` reader - "]
pub type HART_ID_R = crate::FieldReader<u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn hart_id(&self) -> HART_ID_R {
        HART_ID_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("core0_hart_id")
            .field("hart_id", &self.hart_id())
            .finish()
    }
}
#[doc = "Core #0 Hart ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_hart_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_HART_ID_SPEC;
impl crate::RegisterSpec for CORE0_HART_ID_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`core0_hart_id::R`](R) reader structure"]
impl crate::Readable for CORE0_HART_ID_SPEC {}
#[doc = "`reset()` method sets core0_hart_id to value 0"]
impl crate::Resettable for CORE0_HART_ID_SPEC {
    const RESET_VALUE: u64 = 0;
}
