#[doc = "Register `CLK_PRESCALER` reader"]
pub type R = crate::R<CLK_PRESCALER_SPEC>;
#[doc = "Register `CLK_PRESCALER` writer"]
pub type W = crate::W<CLK_PRESCALER_SPEC>;
#[doc = "Field `PRE` reader - Sets the clock prescaler by the value in PRE to achieve the desired I2C clock by dividing"]
pub type PRE_R = crate::FieldReader<u16>;
#[doc = "Field `PRE` writer - Sets the clock prescaler by the value in PRE to achieve the desired I2C clock by dividing"]
pub type PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sets the clock prescaler by the value in PRE to achieve the desired I2C clock by dividing"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0xffff) as u16)
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
    pub fn pre(&mut self) -> PRE_W<CLK_PRESCALER_SPEC> {
        PRE_W::new(self, 0)
    }
}
#[doc = "Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_prescaler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_prescaler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PRESCALER_SPEC;
impl crate::RegisterSpec for CLK_PRESCALER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_prescaler::R`](R) reader structure"]
impl crate::Readable for CLK_PRESCALER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_prescaler::W`](W) writer structure"]
impl crate::Writable for CLK_PRESCALER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_PRESCALER to value 0"]
impl crate::Resettable for CLK_PRESCALER_SPEC {
    const RESET_VALUE: u32 = 0;
}
