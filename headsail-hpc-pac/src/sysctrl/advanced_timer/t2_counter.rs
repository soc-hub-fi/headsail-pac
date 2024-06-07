#[doc = "Register `T2_COUNTER` reader"]
pub type R = crate::R<T2CounterSpec>;
#[doc = "Register `T2_COUNTER` writer"]
pub type W = crate::W<T2CounterSpec>;
#[doc = "Field `T2_COUNTER` reader - "]
pub type T2CounterR = crate::FieldReader<u32>;
#[doc = "Field `T2_COUNTER` writer - "]
pub type T2CounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t2_counter(&self) -> T2CounterR {
        T2CounterR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T2_COUNTER")
            .field("t2_counter", &self.t2_counter())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t2_counter(&mut self) -> T2CounterW<T2CounterSpec> {
        T2CounterW::new(self, 0)
    }
}
#[doc = "ADV_TIMER2 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_counter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_counter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T2CounterSpec;
impl crate::RegisterSpec for T2CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t2_counter::R`](R) reader structure"]
impl crate::Readable for T2CounterSpec {}
#[doc = "`write(|w| ..)` method takes [`t2_counter::W`](W) writer structure"]
impl crate::Writable for T2CounterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T2_COUNTER to value 0"]
impl crate::Resettable for T2CounterSpec {
    const RESET_VALUE: u32 = 0;
}
