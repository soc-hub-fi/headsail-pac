#[doc = "Register `bus_trace1` reader"]
pub type R = crate::R<BUS_TRACE1_SPEC>;
#[doc = "Field `bus_trace` reader - "]
pub type BUS_TRACE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bus_trace(&self) -> BUS_TRACE_R {
        BUS_TRACE_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("bus_trace1")
            .field("bus_trace", &self.bus_trace())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_trace1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_TRACE1_SPEC;
impl crate::RegisterSpec for BUS_TRACE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_trace1::R`](R) reader structure"]
impl crate::Readable for BUS_TRACE1_SPEC {}
#[doc = "`reset()` method sets bus_trace1 to value 0"]
impl crate::Resettable for BUS_TRACE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
