#[doc = "Register `CG` reader"]
pub type R = crate::R<CgSpec>;
#[doc = "Register `CG` writer"]
pub type W = crate::W<CgSpec>;
#[doc = "Field `ENA` reader - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
pub type EnaR = crate::FieldReader<u16>;
#[doc = "Field `ENA` writer - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
pub type EnaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
    #[inline(always)]
    pub fn ena(&self) -> EnaR {
        EnaR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CG").field("ena", &self.ena()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - ADV_TIMER clock gating configuration bitfield. - ENA\\[i\\]=0: clock gate ADV_TIMERi. - ENA\\[i\\]=1: enable ADV_TIMERi."]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> EnaW<CgSpec> {
        EnaW::new(self, 0)
    }
}
#[doc = "ADV_TIMERS channels clock gating configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgSpec;
impl crate::RegisterSpec for CgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cg::R`](R) reader structure"]
impl crate::Readable for CgSpec {}
#[doc = "`write(|w| ..)` method takes [`cg::W`](W) writer structure"]
impl crate::Writable for CgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CG to value 0"]
impl crate::Resettable for CgSpec {
    const RESET_VALUE: u32 = 0;
}
