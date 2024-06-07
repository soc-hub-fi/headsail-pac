#[doc = "Register `int_ack` reader"]
pub type R = crate::R<IntAckSpec>;
#[doc = "Register `int_ack` writer"]
pub type W = crate::W<IntAckSpec>;
#[doc = "Field `spim0_int0_ack` reader - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
pub type Spim0Int0AckR = crate::BitReader;
#[doc = "Field `spim0_int0_ack` writer - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
pub type Spim0Int0AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim0_int1_ack` reader - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
pub type Spim0Int1AckR = crate::BitReader;
#[doc = "Field `spim0_int1_ack` writer - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
pub type Spim0Int1AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_int0_ack` reader - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
pub type Spim1Int0AckR = crate::BitReader;
#[doc = "Field `spim1_int0_ack` writer - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
pub type Spim1Int0AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_int1_ack` reader - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
pub type Spim1Int1AckR = crate::BitReader;
#[doc = "Field `spim1_int1_ack` writer - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
pub type Spim1Int1AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_int_ack` reader - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
pub type GpioIntAckR = crate::BitReader;
#[doc = "Field `gpio_int_ack` writer - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
pub type GpioIntAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim0_int0_ack(&self) -> Spim0Int0AckR {
        Spim0Int0AckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim0_int1_ack(&self) -> Spim0Int1AckR {
        Spim0Int1AckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim1_int0_ack(&self) -> Spim1Int0AckR {
        Spim1Int0AckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim1_int1_ack(&self) -> Spim1Int1AckR {
        Spim1Int1AckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn gpio_int_ack(&self) -> GpioIntAckR {
        GpioIntAckR::new(((self.bits >> 4) & 1) != 0)
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
    pub fn spim0_int0_ack(&mut self) -> Spim0Int0AckW<IntAckSpec> {
        Spim0Int0AckW::new(self, 0)
    }
    #[doc = "Bit 1 - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim0_int1_ack(&mut self) -> Spim0Int1AckW<IntAckSpec> {
        Spim0Int1AckW::new(self, 1)
    }
    #[doc = "Bit 2 - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim1_int0_ack(&mut self) -> Spim1Int0AckW<IntAckSpec> {
        Spim1Int0AckW::new(self, 2)
    }
    #[doc = "Bit 3 - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim1_int1_ack(&mut self) -> Spim1Int1AckW<IntAckSpec> {
        Spim1Int1AckW::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int_ack(&mut self) -> GpioIntAckW<IntAckSpec> {
        GpioIntAckW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntAckSpec;
impl crate::RegisterSpec for IntAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ack::R`](R) reader structure"]
impl crate::Readable for IntAckSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ack::W`](W) writer structure"]
impl crate::Writable for IntAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int_ack to value 0"]
impl crate::Resettable for IntAckSpec {
    const RESET_VALUE: u32 = 0;
}
