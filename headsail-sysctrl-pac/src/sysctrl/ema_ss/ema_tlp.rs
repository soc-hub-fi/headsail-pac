#[doc = "Register `EMA_TLP` reader"]
pub type R = crate::R<EmaTlpSpec>;
#[doc = "Register `EMA_TLP` writer"]
pub type W = crate::W<EmaTlpSpec>;
#[doc = "Field `DPRAM_EMA` reader - "]
pub type DpramEmaR = crate::FieldReader;
#[doc = "Field `DPRAM_EMA` writer - "]
pub type DpramEmaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPRAM_EMA` reader - "]
pub type SpramEmaR = crate::FieldReader;
#[doc = "Field `SPRAM_EMA` writer - "]
pub type SpramEmaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dpram_ema(&self) -> DpramEmaR {
        DpramEmaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn spram_ema(&self) -> SpramEmaR {
        SpramEmaR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMA_TLP")
            .field("dpram_ema", &self.dpram_ema())
            .field("spram_ema", &self.spram_ema())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dpram_ema(&mut self) -> DpramEmaW<EmaTlpSpec> {
        DpramEmaW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn spram_ema(&mut self) -> SpramEmaW<EmaTlpSpec> {
        SpramEmaW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_tlp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_tlp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmaTlpSpec;
impl crate::RegisterSpec for EmaTlpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ema_tlp::R`](R) reader structure"]
impl crate::Readable for EmaTlpSpec {}
#[doc = "`write(|w| ..)` method takes [`ema_tlp::W`](W) writer structure"]
impl crate::Writable for EmaTlpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMA_TLP to value 0x00ff_00ff"]
impl crate::Resettable for EmaTlpSpec {
    const RESET_VALUE: u32 = 0x00ff_00ff;
}
