#[doc = "Register `CFG_HI` reader"]
pub type R = crate::R<CfgHiSpec>;
#[doc = "Register `CFG_HI` writer"]
pub type W = crate::W<CfgHiSpec>;
#[doc = "Field `EN` reader - Timer enable (starts couting) bit"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Timer enable (starts couting) bit"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Timer high reset, cleared after the reset happened"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Timer high reset, cleared after the reset happened"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQEN` reader - Timer hi interrupt generation on compare match enable"]
pub type IrqenR = crate::BitReader;
#[doc = "Field `IRQEN` writer - Timer hi interrupt generation on compare match enable"]
pub type IrqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONES` reader - Timer hi one shot configuration"]
pub type OnesR = crate::BitReader;
#[doc = "Field `ONES` writer - Timer hi one shot configuration"]
pub type OnesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Timer hi prescaler enable bit"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Timer hi prescaler enable bit"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFG` reader - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
pub type CcfgR = crate::BitReader;
#[doc = "Field `CCFG` writer - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
pub type CcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVAL` reader - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
pub type PvalR = crate::FieldReader;
#[doc = "Field `PVAL` writer - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
pub type PvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Timer enable (starts couting) bit"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer high reset, cleared after the reset happened"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer hi interrupt generation on compare match enable"]
    #[inline(always)]
    pub fn irqen(&self) -> IrqenR {
        IrqenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer hi one shot configuration"]
    #[inline(always)]
    pub fn ones(&self) -> OnesR {
        OnesR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer hi prescaler enable bit"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
    #[inline(always)]
    pub fn ccfg(&self) -> CcfgR {
        CcfgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
    #[inline(always)]
    pub fn pval(&self) -> PvalR {
        PvalR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_HI")
            .field("en", &self.en())
            .field("rst", &self.rst())
            .field("irqen", &self.irqen())
            .field("mode", &self.mode())
            .field("ones", &self.ones())
            .field("pen", &self.pen())
            .field("ccfg", &self.ccfg())
            .field("pval", &self.pval())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable (starts couting) bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CfgHiSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer high reset, cleared after the reset happened"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<CfgHiSpec> {
        RstW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer hi interrupt generation on compare match enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqen(&mut self) -> IrqenW<CfgHiSpec> {
        IrqenW::new(self, 2)
    }
    #[doc = "Bit 4 - Timer hi continuous mode configuration 0b0: Continue incrementing timer hi counter after a compare match with CMP_HI 0b1: Reset timer to after a compare match with CMP_HI"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CfgHiSpec> {
        ModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer hi one shot configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ones(&mut self) -> OnesW<CfgHiSpec> {
        OnesW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer hi prescaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<CfgHiSpec> {
        PenW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer hi clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn ccfg(&mut self) -> CcfgW<CfgHiSpec> {
        CcfgW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Timer hi prescaler value. ftimer = fclk/(1 + PVAL)"]
    #[inline(always)]
    #[must_use]
    pub fn pval(&mut self) -> PvalW<CfgHiSpec> {
        PvalW::new(self, 8)
    }
}
#[doc = "Timer High Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgHiSpec;
impl crate::RegisterSpec for CfgHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_hi::R`](R) reader structure"]
impl crate::Readable for CfgHiSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_hi::W`](W) writer structure"]
impl crate::Writable for CfgHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_HI to value 0"]
impl crate::Resettable for CfgHiSpec {
    const RESET_VALUE: u32 = 0;
}
