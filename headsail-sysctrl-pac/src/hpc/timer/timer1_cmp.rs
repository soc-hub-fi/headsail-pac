#[doc = "Register `timer1_cmp` reader"]
pub type R = crate::R<Timer1CmpSpec>;
#[doc = "Register `timer1_cmp` writer"]
pub type W = crate::W<Timer1CmpSpec>;
#[doc = "Field `cmp` reader - "]
pub type CmpR = crate::FieldReader<u32>;
#[doc = "Field `cmp` writer - "]
pub type CmpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer1_cmp")
            .field("cmp", &self.cmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CmpW<Timer1CmpSpec> {
        CmpW::new(self, 0)
    }
}
#[doc = "Timer compare register for timer 1. Writing this will zero timer register for timer 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1CmpSpec;
impl crate::RegisterSpec for Timer1CmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1_cmp::R`](R) reader structure"]
impl crate::Readable for Timer1CmpSpec {}
#[doc = "`write(|w| ..)` method takes [`timer1_cmp::W`](W) writer structure"]
impl crate::Writable for Timer1CmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timer1_cmp to value 0"]
impl crate::Resettable for Timer1CmpSpec {
    const RESET_VALUE: u32 = 0;
}
