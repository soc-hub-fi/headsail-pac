#[doc = "Register `timer2_ctrl` reader"]
pub type R = crate::R<TIMER2_CTRL_SPEC>;
#[doc = "Register `timer2_ctrl` writer"]
pub type W = crate::W<TIMER2_CTRL_SPEC>;
#[doc = "Field `en` reader - Enable timer"]
pub type EN_R = crate::BitReader;
#[doc = "Field `en` writer - Enable timer"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prescaler` reader - Prescaler bits"]
pub type PRESCALER_R = crate::FieldReader;
#[doc = "Field `prescaler` writer - Prescaler bits"]
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable timer"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:5 - Prescaler bits"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer2_ctrl")
            .field("en", &self.en())
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable timer"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<TIMER2_CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Prescaler bits"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<TIMER2_CTRL_SPEC> {
        PRESCALER_W::new(self, 3)
    }
}
#[doc = "Control register for timer 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER2_CTRL_SPEC;
impl crate::RegisterSpec for TIMER2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2_ctrl::R`](R) reader structure"]
impl crate::Readable for TIMER2_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer2_ctrl::W`](W) writer structure"]
impl crate::Writable for TIMER2_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timer2_ctrl to value 0"]
impl crate::Resettable for TIMER2_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
