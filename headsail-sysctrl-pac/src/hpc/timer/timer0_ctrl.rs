#[doc = "Register `timer0_ctrl` reader"]
pub type R = crate::R<Timer0CtrlSpec>;
#[doc = "Register `timer0_ctrl` writer"]
pub type W = crate::W<Timer0CtrlSpec>;
#[doc = "Field `en` reader - Enable timer"]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - Enable timer"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prescaler` reader - Prescaler bits"]
pub type PrescalerR = crate::FieldReader;
#[doc = "Field `prescaler` writer - Prescaler bits"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable timer"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:5 - Prescaler bits"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer0_ctrl")
            .field("en", &self.en())
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable timer"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Timer0CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Prescaler bits"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<Timer0CtrlSpec> {
        PrescalerW::new(self, 3)
    }
}
#[doc = "Control register for timer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer0CtrlSpec;
impl crate::RegisterSpec for Timer0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0_ctrl::R`](R) reader structure"]
impl crate::Readable for Timer0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer0_ctrl::W`](W) writer structure"]
impl crate::Writable for Timer0CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timer0_ctrl to value 0"]
impl crate::Resettable for Timer0CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
