#[doc = "Register `IRQ_CONFIG` reader"]
pub type R = crate::R<IrqConfigSpec>;
#[doc = "Register `IRQ_CONFIG` writer"]
pub type W = crate::W<IrqConfigSpec>;
#[doc = "Field `Clear` reader - Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register"]
pub type ClearR = crate::BitReader;
#[doc = "Field `Clear` writer - Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Polarity` reader - Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub type PolarityR = crate::BitReader;
#[doc = "Field `Polarity` writer - Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity: '0' = IRQ active low '1' = IRQ active high"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_CONFIG")
            .field("clear", &self.clear())
            .field("polarity", &self.polarity())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<IrqConfigSpec> {
        ClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Polarity: '0' = IRQ active low '1' = IRQ active high"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> PolarityW<IrqConfigSpec> {
        PolarityW::new(self, 1)
    }
}
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqConfigSpec;
impl crate::RegisterSpec for IrqConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_config::R`](R) reader structure"]
impl crate::Readable for IrqConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_config::W`](W) writer structure"]
impl crate::Writable for IrqConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_CONFIG to value 0"]
impl crate::Resettable for IrqConfigSpec {
    const RESET_VALUE: u32 = 0;
}
