#[doc = "Register `INTCFG` reader"]
pub type R = crate::R<INTCFG_SPEC>;
#[doc = "Register `INTCFG` writer"]
pub type W = crate::W<INTCFG_SPEC>;
#[doc = "Field `THTX` reader - Insert the number of elements in the txfifo before which the events_o\\[0\\]
is high"]
pub type THTX_R = crate::FieldReader;
#[doc = "Field `THTX` writer - Insert the number of elements in the txfifo before which the events_o\\[0\\]
is high"]
pub type THTX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THRX` reader - Insert the number of elements in the rxfifo afterwhich the events_o\\[0\\]
is high"]
pub type THRX_R = crate::FieldReader;
#[doc = "Field `THRX` writer - Insert the number of elements in the rxfifo afterwhich the events_o\\[0\\]
is high"]
pub type THRX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTTX` reader - Specify the number of data words sent before the assertion of events_o\\[0\\]"]
pub type CNTTX_R = crate::FieldReader;
#[doc = "Field `CNTTX` writer - Specify the number of data words sent before the assertion of events_o\\[0\\]"]
pub type CNTTX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTRX` reader - Specify the number of data words received before the assertion of events_o\\[0\\]"]
pub type CNTRX_R = crate::FieldReader;
#[doc = "Field `CNTRX` writer - Specify the number of data words received before the assertion of events_o\\[0\\]"]
pub type CNTRX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTEN` reader - Enable counter to enable the previous 2 functionalities"]
pub type CNTEN_R = crate::BitReader;
#[doc = "Field `CNTEN` writer - Enable counter to enable the previous 2 functionalities"]
pub type CNTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Enable interrupt"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable interrupt"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Insert the number of elements in the txfifo before which the events_o\\[0\\]
is high"]
    #[inline(always)]
    pub fn thtx(&self) -> THTX_R {
        THTX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Insert the number of elements in the rxfifo afterwhich the events_o\\[0\\]
is high"]
    #[inline(always)]
    pub fn thrx(&self) -> THRX_R {
        THRX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Specify the number of data words sent before the assertion of events_o\\[0\\]"]
    #[inline(always)]
    pub fn cnttx(&self) -> CNTTX_R {
        CNTTX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Specify the number of data words received before the assertion of events_o\\[0\\]"]
    #[inline(always)]
    pub fn cntrx(&self) -> CNTRX_R {
        CNTRX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Enable counter to enable the previous 2 functionalities"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable interrupt"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTCFG")
            .field("thtx", &self.thtx())
            .field("thrx", &self.thrx())
            .field("cnttx", &self.cnttx())
            .field("cntrx", &self.cntrx())
            .field("cnten", &self.cnten())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Insert the number of elements in the txfifo before which the events_o\\[0\\]
is high"]
    #[inline(always)]
    #[must_use]
    pub fn thtx(&mut self) -> THTX_W<INTCFG_SPEC> {
        THTX_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Insert the number of elements in the rxfifo afterwhich the events_o\\[0\\]
is high"]
    #[inline(always)]
    #[must_use]
    pub fn thrx(&mut self) -> THRX_W<INTCFG_SPEC> {
        THRX_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Specify the number of data words sent before the assertion of events_o\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cnttx(&mut self) -> CNTTX_W<INTCFG_SPEC> {
        CNTTX_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Specify the number of data words received before the assertion of events_o\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cntrx(&mut self) -> CNTRX_W<INTCFG_SPEC> {
        CNTRX_W::new(self, 24)
    }
    #[doc = "Bit 30 - Enable counter to enable the previous 2 functionalities"]
    #[inline(always)]
    #[must_use]
    pub fn cnten(&mut self) -> CNTEN_W<INTCFG_SPEC> {
        CNTEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<INTCFG_SPEC> {
        EN_W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCFG_SPEC;
impl crate::RegisterSpec for INTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intcfg::R`](R) reader structure"]
impl crate::Readable for INTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intcfg::W`](W) writer structure"]
impl crate::Writable for INTCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCFG to value 0"]
impl crate::Resettable for INTCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
