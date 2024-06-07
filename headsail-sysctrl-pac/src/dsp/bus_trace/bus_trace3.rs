#[doc = "Register `bus_trace3` reader"]
pub type R = crate::R<BusTrace3Spec>;
#[doc = "Field `bus_trace` reader - "]
pub type BusTraceR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bus_trace(&self) -> BusTraceR {
        BusTraceR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("bus_trace3")
            .field("bus_trace", &self.bus_trace())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_trace3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusTrace3Spec;
impl crate::RegisterSpec for BusTrace3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_trace3::R`](R) reader structure"]
impl crate::Readable for BusTrace3Spec {}
#[doc = "`reset()` method sets bus_trace3 to value 0"]
impl crate::Resettable for BusTrace3Spec {
    const RESET_VALUE: u32 = 0;
}
