#[doc = "Register `CLK_PRESCALER` reader"]
pub type R = crate::R<ClkPrescalerSpec>;
#[doc = "Register `CLK_PRESCALER` writer"]
pub type W = crate::W<ClkPrescalerSpec>;
#[doc = "Field `PRE` reader - Sets the clock prescaler by the value in PRE to achieve the desired I2C clock by dividing"]
pub type PreR = crate::FieldReader<u16>;
#[doc = "Field `PRE` writer - Sets the clock prescaler by the value in PRE to achieve the desired I2C clock by dividing"]
pub type PreW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sets the clock prescaler by the value in PRE to achieve the desired I2C clock by dividing"]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_PRESCALER")
            .field("pre", &self.pre())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets the clock prescaler by the value in PRE to achieve the desired I2C clock by dividing"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PreW<ClkPrescalerSpec> {
        PreW::new(self, 0)
    }
}
#[doc = "Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_prescaler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_prescaler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPrescalerSpec;
impl crate::RegisterSpec for ClkPrescalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_prescaler::R`](R) reader structure"]
impl crate::Readable for ClkPrescalerSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_prescaler::W`](W) writer structure"]
impl crate::Writable for ClkPrescalerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_PRESCALER to value 0"]
impl crate::Resettable for ClkPrescalerSpec {
    const RESET_VALUE: u32 = 0;
}
