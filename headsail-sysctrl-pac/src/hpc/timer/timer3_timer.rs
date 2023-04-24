#[doc = "Register `timer3_timer` reader"]
pub struct R(crate::R<TIMER3_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER3_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER3_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER3_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `timer` reader - "]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "Monotonically increasing timer register for timer 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer3_timer](index.html) module"]
pub struct TIMER3_TIMER_SPEC;
impl crate::RegisterSpec for TIMER3_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer3_timer::R](R) reader structure"]
impl crate::Readable for TIMER3_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets timer3_timer to value 0"]
impl crate::Resettable for TIMER3_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
