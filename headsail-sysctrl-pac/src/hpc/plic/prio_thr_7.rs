#[doc = "Register `prio_thr_7` reader"]
pub type R = crate::R<PRIO_THR_7_SPEC>;
#[doc = "Register `prio_thr_7` writer"]
pub type W = crate::W<PRIO_THR_7_SPEC>;
#[doc = "Field `thr` reader - "]
pub type THR_R = crate::FieldReader;
#[doc = "Field `thr` writer - "]
pub type THR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn thr(&self) -> THR_R {
        THR_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("prio_thr_7")
            .field("thr", &self.thr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn thr(&mut self) -> THR_W<PRIO_THR_7_SPEC> {
        THR_W::new(self, 0)
    }
}
#[doc = "Priority threshold for Hart 3 S-Mode (context #7) HPC masks all PLIC interrupts of a priority less than or equal to threshold. E.g., threshold zero permits all interrupts with non-zero priority, while 7 masks all interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prio_thr_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prio_thr_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIO_THR_7_SPEC;
impl crate::RegisterSpec for PRIO_THR_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prio_thr_7::R`](R) reader structure"]
impl crate::Readable for PRIO_THR_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prio_thr_7::W`](W) writer structure"]
impl crate::Writable for PRIO_THR_7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prio_thr_7 to value 0"]
impl crate::Resettable for PRIO_THR_7_SPEC {
    const RESET_VALUE: u32 = 0;
}
