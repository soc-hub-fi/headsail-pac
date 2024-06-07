#[doc = "Register `T0_COUNTER` reader"]
pub type R = crate::R<T0CounterSpec>;
#[doc = "Field `T0_COUNTER` reader - "]
pub type T0CounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t0_counter(&self) -> T0CounterR {
        T0CounterR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0_COUNTER")
            .field("t0_counter", &self.t0_counter())
            .finish()
    }
}
#[doc = "ADV_TIMER0 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_counter::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0CounterSpec;
impl crate::RegisterSpec for T0CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_counter::R`](R) reader structure"]
impl crate::Readable for T0CounterSpec {}
#[doc = "`reset()` method sets T0_COUNTER to value 0"]
impl crate::Resettable for T0CounterSpec {
    const RESET_VALUE: u32 = 0;
}
