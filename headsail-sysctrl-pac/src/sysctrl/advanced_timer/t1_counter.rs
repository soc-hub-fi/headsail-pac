#[doc = "Register `T1_COUNTER` reader"]
pub type R = crate::R<T1CounterSpec>;
#[doc = "Field `T1_COUNTER` reader - "]
pub type T1CounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t1_counter(&self) -> T1CounterR {
        T1CounterR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1_COUNTER")
            .field("t1_counter", &self.t1_counter())
            .finish()
    }
}
#[doc = "ADV_TIMER1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_counter::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1CounterSpec;
impl crate::RegisterSpec for T1CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1_counter::R`](R) reader structure"]
impl crate::Readable for T1CounterSpec {}
#[doc = "`reset()` method sets T1_COUNTER to value 0"]
impl crate::Resettable for T1CounterSpec {
    const RESET_VALUE: u32 = 0;
}
