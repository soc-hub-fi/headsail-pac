#[doc = "Register `int_ack` reader"]
pub type R = crate::R<INT_ACK_SPEC>;
#[doc = "Register `int_ack` writer"]
pub type W = crate::W<INT_ACK_SPEC>;
#[doc = "Field `spim0_int0_ack` reader - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
pub type SPIM0_INT0_ACK_R = crate::BitReader;
#[doc = "Field `spim0_int0_ack` writer - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
pub type SPIM0_INT0_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim0_int1_ack` reader - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
pub type SPIM0_INT1_ACK_R = crate::BitReader;
#[doc = "Field `spim0_int1_ack` writer - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
pub type SPIM0_INT1_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_int0_ack` reader - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
pub type SPIM1_INT0_ACK_R = crate::BitReader;
#[doc = "Field `spim1_int0_ack` writer - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
pub type SPIM1_INT0_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_int1_ack` reader - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
pub type SPIM1_INT1_ACK_R = crate::BitReader;
#[doc = "Field `spim1_int1_ack` writer - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
pub type SPIM1_INT1_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_int_ack` reader - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
pub type GPIO_INT_ACK_R = crate::BitReader;
#[doc = "Field `gpio_int_ack` writer - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
pub type GPIO_INT_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim0_int0_ack(&self) -> SPIM0_INT0_ACK_R {
        SPIM0_INT0_ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim0_int1_ack(&self) -> SPIM0_INT1_ACK_R {
        SPIM0_INT1_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim1_int0_ack(&self) -> SPIM1_INT0_ACK_R {
        SPIM1_INT0_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim1_int1_ack(&self) -> SPIM1_INT1_ACK_R {
        SPIM1_INT1_ACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn gpio_int_ack(&self) -> GPIO_INT_ACK_R {
        GPIO_INT_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("int_ack")
            .field("spim0_int0_ack", &self.spim0_int0_ack())
            .field("spim0_int1_ack", &self.spim0_int1_ack())
            .field("spim1_int0_ack", &self.spim1_int0_ack())
            .field("spim1_int1_ack", &self.spim1_int1_ack())
            .field("gpio_int_ack", &self.gpio_int_ack())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim0_int0_ack(&mut self) -> SPIM0_INT0_ACK_W<INT_ACK_SPEC> {
        SPIM0_INT0_ACK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim0_int1_ack(&mut self) -> SPIM0_INT1_ACK_W<INT_ACK_SPEC> {
        SPIM0_INT1_ACK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim1_int0_ack(&mut self) -> SPIM1_INT0_ACK_W<INT_ACK_SPEC> {
        SPIM1_INT0_ACK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim1_int1_ack(&mut self) -> SPIM1_INT1_ACK_W<INT_ACK_SPEC> {
        SPIM1_INT1_ACK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int_ack(&mut self) -> GPIO_INT_ACK_W<INT_ACK_SPEC> {
        GPIO_INT_ACK_W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ACK_SPEC;
impl crate::RegisterSpec for INT_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ack::R`](R) reader structure"]
impl crate::Readable for INT_ACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ack::W`](W) writer structure"]
impl crate::Writable for INT_ACK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int_ack to value 0"]
impl crate::Resettable for INT_ACK_SPEC {
    const RESET_VALUE: u32 = 0;
}
