#[doc = "Register `INTSTATUS` reader"]
pub type R = crate::R<INTSTATUS_SPEC>;
#[doc = "Field `INTSTATUS` reader - "]
pub type INTSTATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTATUS")
            .field("intstatus", &self.intstatus())
            .finish()
    }
}
#[doc = "Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTATUS_SPEC;
impl crate::RegisterSpec for INTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for INTSTATUS_SPEC {}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for INTSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
