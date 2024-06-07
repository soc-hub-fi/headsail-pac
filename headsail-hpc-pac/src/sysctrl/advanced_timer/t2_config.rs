#[doc = "Register `T2_CONFIG` reader"]
pub type R = crate::R<T2ConfigSpec>;
#[doc = "Register `T2_CONFIG` writer"]
pub type W = crate::W<T2ConfigSpec>;
#[doc = "Field `INSEL` reader - ADV_TIMER2 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
pub type InselR = crate::FieldReader;
#[doc = "Field `INSEL` writer - ADV_TIMER2 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
pub type InselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MODE` reader - ADV_TIMER2 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - ADV_TIMER2 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKSEL` reader - ADV_TIMER2 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
pub type ClkselR = crate::BitReader;
#[doc = "Field `CLKSEL` writer - ADV_TIMER2 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDOWNSEL` reader - ADV_TIMER2 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
pub type UpdownselR = crate::BitReader;
#[doc = "Field `UPDOWNSEL` writer - ADV_TIMER2 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
pub type UpdownselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESC` reader - ADV_TIMER2 prescaler value configuration bitfield."]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - ADV_TIMER2 prescaler value configuration bitfield."]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ADV_TIMER2 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
    #[inline(always)]
    pub fn insel(&self) -> InselR {
        InselR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - ADV_TIMER2 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ADV_TIMER2 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADV_TIMER2 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
    #[inline(always)]
    pub fn updownsel(&self) -> UpdownselR {
        UpdownselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - ADV_TIMER2 prescaler value configuration bitfield."]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T2_CONFIG")
            .field("insel", &self.insel())
            .field("mode", &self.mode())
            .field("clksel", &self.clksel())
            .field("updownsel", &self.updownsel())
            .field("presc", &self.presc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - ADV_TIMER2 input source configuration bitfield: - 0-31: GPIO\\[0\\]
to GPIO\\[31\\]
- 32-35: Channel 0 to 3 of ADV_TIMER0 - 36-39: Channel 0 to 3 of ADV_TIMER1 - 40-43: Channel 0 to 3 of ADV_TIMER2 - 44-47: Channel 0 to 3 of ADV_TIMER3"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> InselW<T2ConfigSpec> {
        InselW::new(self, 0)
    }
    #[doc = "Bits 8:10 - ADV_TIMER2 trigger mode configuration bitfield: - 3'h0: trigger event at each clock cycle. - 3'h1: trigger event if input source is 0 - 3'h2: trigger event if input source is 1 - 3'h3: trigger event on input source rising edge - 3'h4: trigger event on input source falling edge - 3'h5: trigger event on input source falling or rising edge - 3'h6: trigger event on input source rising edge when armed - 3'h7: trigger event on input source falling edge when armed"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<T2ConfigSpec> {
        ModeW::new(self, 8)
    }
    #[doc = "Bit 11 - ADV_TIMER2 clock source configuration bitfield: - 1'b0: FLL - 1'b1: reference clock at 32kHz"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<T2ConfigSpec> {
        ClkselW::new(self, 11)
    }
    #[doc = "Bit 12 - ADV_TIMER2 center-aligned mode configuration bitfield: - 1'b0: The counter counts up and down alternatively. - 1'b1: The counter counts up and resets to 0 when reach threshold."]
    #[inline(always)]
    #[must_use]
    pub fn updownsel(&mut self) -> UpdownselW<T2ConfigSpec> {
        UpdownselW::new(self, 12)
    }
    #[doc = "Bits 16:23 - ADV_TIMER2 prescaler value configuration bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<T2ConfigSpec> {
        PrescW::new(self, 16)
    }
}
#[doc = "ADV_TIMER2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T2ConfigSpec;
impl crate::RegisterSpec for T2ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t2_config::R`](R) reader structure"]
impl crate::Readable for T2ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`t2_config::W`](W) writer structure"]
impl crate::Writable for T2ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T2_CONFIG to value 0"]
impl crate::Resettable for T2ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
