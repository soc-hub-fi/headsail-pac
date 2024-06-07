#[doc = "Register `T3_COUNTER` reader"]
pub type R = crate::R<T3CounterSpec>;
#[doc = "Field `T3_COUNTER` reader - "]
pub type T3CounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t3_counter(&self) -> T3CounterR {
        T3CounterR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T3_COUNTER")
            .field("t3_counter", &self.t3_counter())
            .finish()
    }
}
#[doc = "ADV_TIMER3 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_counter::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T3CounterSpec;
impl crate::RegisterSpec for T3CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t3_counter::R`](R) reader structure"]
impl crate::Readable for T3CounterSpec {}
#[doc = "`reset()` method sets T3_COUNTER to value 0"]
impl crate::Resettable for T3CounterSpec {
    const RESET_VALUE: u32 = 0;
}
