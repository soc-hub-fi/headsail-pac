#[doc = "Register `EMA_SYSCTRL` reader"]
pub type R = crate::R<EMA_SYSCTRL_SPEC>;
#[doc = "Register `EMA_SYSCTRL` writer"]
pub type W = crate::W<EMA_SYSCTRL_SPEC>;
#[doc = "Field `DPRAM_EMA` reader - "]
pub type DPRAM_EMA_R = crate::FieldReader;
#[doc = "Field `DPRAM_EMA` writer - "]
pub type DPRAM_EMA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPRAM_EMA` reader - "]
pub type SPRAM_EMA_R = crate::FieldReader;
#[doc = "Field `SPRAM_EMA` writer - "]
pub type SPRAM_EMA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dpram_ema(&self) -> DPRAM_EMA_R {
        DPRAM_EMA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn spram_ema(&self) -> SPRAM_EMA_R {
        SPRAM_EMA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMA_SYSCTRL")
            .field("dpram_ema", &self.dpram_ema())
            .field("spram_ema", &self.spram_ema())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dpram_ema(&mut self) -> DPRAM_EMA_W<EMA_SYSCTRL_SPEC> {
        DPRAM_EMA_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn spram_ema(&mut self) -> SPRAM_EMA_W<EMA_SYSCTRL_SPEC> {
        SPRAM_EMA_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema_sysctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema_sysctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMA_SYSCTRL_SPEC;
impl crate::RegisterSpec for EMA_SYSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ema_sysctrl::R`](R) reader structure"]
impl crate::Readable for EMA_SYSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ema_sysctrl::W`](W) writer structure"]
impl crate::Writable for EMA_SYSCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMA_SYSCTRL to value 0x00ff_00ff"]
impl crate::Resettable for EMA_SYSCTRL_SPEC {
    const RESET_VALUE: u32 = 0x00ff_00ff;
}
