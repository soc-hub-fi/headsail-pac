#[doc = "Register `prio_thr_2` reader"]
pub type R = crate::R<PrioThr2Spec>;
#[doc = "Register `prio_thr_2` writer"]
pub type W = crate::W<PrioThr2Spec>;
#[doc = "Field `thr` reader - "]
pub type ThrR = crate::FieldReader;
#[doc = "Field `thr` writer - "]
pub type ThrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn thr(&self) -> ThrR {
        ThrR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("prio_thr_2")
            .field("thr", &self.thr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn thr(&mut self) -> ThrW<PrioThr2Spec> {
        ThrW::new(self, 0)
    }
}
#[doc = "Priority threshold for Hart 1 M-Mode (context #2) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrioThr2Spec;
impl crate::RegisterSpec for PrioThr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prio_thr_2::R`](R) reader structure"]
impl crate::Readable for PrioThr2Spec {}
#[doc = "`write(|w| ..)` method takes [`prio_thr_2::W`](W) writer structure"]
impl crate::Writable for PrioThr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prio_thr_2 to value 0"]
impl crate::Resettable for PrioThr2Spec {
    const RESET_VALUE: u32 = 0;
}
