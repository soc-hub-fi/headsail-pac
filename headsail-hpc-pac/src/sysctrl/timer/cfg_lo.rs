#[doc = "Register `CFG_LO` reader"]
pub type R = crate::R<CFG_LO_SPEC>;
#[doc = "Register `CFG_LO` writer"]
pub type W = crate::W<CFG_LO_SPEC>;
#[doc = "Field `EN` reader - Timer enable (starts couting) bit"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Timer enable (starts couting) bit"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Timer low reset, cleared after the reset happened"]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - Timer low reset, cleared after the reset happened"]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQEN` reader - Timer low interrupt generation on compare match enable"]
pub type IRQEN_R = crate::BitReader;
#[doc = "Field `IRQEN` writer - Timer low interrupt generation on compare match enable"]
pub type IRQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Timer low continuous mode configuration 0b0: Continue incrementing timer low counter after a compare match with CMP_LO 0b1: Reset timer to after a compare match with CMP_LO"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Timer low continuous mode configuration 0b0: Continue incrementing timer low counter after a compare match with CMP_LO 0b1: Reset timer to after a compare match with CMP_LO"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONES` reader - Timer low one shot configuration"]
pub type ONES_R = crate::BitReader;
#[doc = "Field `ONES` writer - Timer low one shot configuration"]
pub type ONES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Timer low prescaler enable bit"]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - Timer low prescaler enable bit"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFG` reader - Timer low clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
pub type CCFG_R = crate::BitReader;
#[doc = "Field `CCFG` writer - Timer low clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
pub type CCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVAL` reader - Timer low prescaler value. ftimer = fclk/(1 + P V AL)"]
pub type PVAL_R = crate::FieldReader;
#[doc = "Field `PVAL` writer - Timer low prescaler value. ftimer = fclk/(1 + P V AL)"]
pub type PVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CASC` reader - Timer low and Timer high 64-bit cascaded mode enable bit"]
pub type CASC_R = crate::BitReader;
#[doc = "Field `CASC` writer - Timer low and Timer high 64-bit cascaded mode enable bit"]
pub type CASC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer enable (starts couting) bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer low reset, cleared after the reset happened"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer low interrupt generation on compare match enable"]
    #[inline(always)]
    pub fn irqen(&self) -> IRQEN_R {
        IRQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer low continuous mode configuration 0b0: Continue incrementing timer low counter after a compare match with CMP_LO 0b1: Reset timer to after a compare match with CMP_LO"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer low one shot configuration"]
    #[inline(always)]
    pub fn ones(&self) -> ONES_R {
        ONES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer low prescaler enable bit"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer low clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
    #[inline(always)]
    pub fn ccfg(&self) -> CCFG_R {
        CCFG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Timer low prescaler value. ftimer = fclk/(1 + P V AL)"]
    #[inline(always)]
    pub fn pval(&self) -> PVAL_R {
        PVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Timer low and Timer high 64-bit cascaded mode enable bit"]
    #[inline(always)]
    pub fn casc(&self) -> CASC_R {
        CASC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_LO")
            .field("en", &self.en())
            .field("rst", &self.rst())
            .field("irqen", &self.irqen())
            .field("mode", &self.mode())
            .field("ones", &self.ones())
            .field("pen", &self.pen())
            .field("ccfg", &self.ccfg())
            .field("pval", &self.pval())
            .field("casc", &self.casc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable (starts couting) bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CFG_LO_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer low reset, cleared after the reset happened"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<CFG_LO_SPEC> {
        RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer low interrupt generation on compare match enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqen(&mut self) -> IRQEN_W<CFG_LO_SPEC> {
        IRQEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Timer low continuous mode configuration 0b0: Continue incrementing timer low counter after a compare match with CMP_LO 0b1: Reset timer to after a compare match with CMP_LO"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CFG_LO_SPEC> {
        MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer low one shot configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ones(&mut self) -> ONES_W<CFG_LO_SPEC> {
        ONES_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer low prescaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<CFG_LO_SPEC> {
        PEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer low clock source configuration 0b0: FLL or FLL plus Prescaler 0b1: 32 kHz reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn ccfg(&mut self) -> CCFG_W<CFG_LO_SPEC> {
        CCFG_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Timer low prescaler value. ftimer = fclk/(1 + P V AL)"]
    #[inline(always)]
    #[must_use]
    pub fn pval(&mut self) -> PVAL_W<CFG_LO_SPEC> {
        PVAL_W::new(self, 8)
    }
    #[doc = "Bit 31 - Timer low and Timer high 64-bit cascaded mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn casc(&mut self) -> CASC_W<CFG_LO_SPEC> {
        CASC_W::new(self, 31)
    }
}
#[doc = "Timer Low Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_LO_SPEC;
impl crate::RegisterSpec for CFG_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_lo::R`](R) reader structure"]
impl crate::Readable for CFG_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_lo::W`](W) writer structure"]
impl crate::Writable for CFG_LO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LO to value 0"]
impl crate::Resettable for CFG_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
