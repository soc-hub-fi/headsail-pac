#[doc = "Register `T3_THRESHOLD` reader"]
pub type R = crate::R<T3_THRESHOLD_SPEC>;
#[doc = "Register `T3_THRESHOLD` writer"]
pub type W = crate::W<T3_THRESHOLD_SPEC>;
#[doc = "Field `TH_LO` reader - ADV_TIMER3 threshold low part configuration bitfield. It defines start counter value"]
pub type TH_LO_R = crate::FieldReader<u16>;
#[doc = "Field `TH_LO` writer - ADV_TIMER3 threshold low part configuration bitfield. It defines start counter value"]
pub type TH_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TH_HI` reader - ADV_TIMER3 threshold high part configuration bitfield. It defines end counter value."]
pub type TH_HI_R = crate::FieldReader<u16>;
#[doc = "Field `TH_HI` writer - ADV_TIMER3 threshold high part configuration bitfield. It defines end counter value."]
pub type TH_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADV_TIMER3 threshold low part configuration bitfield. It defines start counter value"]
    #[inline(always)]
    pub fn th_lo(&self) -> TH_LO_R {
        TH_LO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADV_TIMER3 threshold high part configuration bitfield. It defines end counter value."]
    #[inline(always)]
    pub fn th_hi(&self) -> TH_HI_R {
        TH_HI_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T3_THRESHOLD")
            .field("th_lo", &self.th_lo())
            .field("th_hi", &self.th_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - ADV_TIMER3 threshold low part configuration bitfield. It defines start counter value"]
    #[inline(always)]
    #[must_use]
    pub fn th_lo(&mut self) -> TH_LO_W<T3_THRESHOLD_SPEC> {
        TH_LO_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADV_TIMER3 threshold high part configuration bitfield. It defines end counter value."]
    #[inline(always)]
    #[must_use]
    pub fn th_hi(&mut self) -> TH_HI_W<T3_THRESHOLD_SPEC> {
        TH_HI_W::new(self, 16)
    }
}
#[doc = "ADV_TIMER3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T3_THRESHOLD_SPEC;
impl crate::RegisterSpec for T3_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t3_threshold::R`](R) reader structure"]
impl crate::Readable for T3_THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t3_threshold::W`](W) writer structure"]
impl crate::Writable for T3_THRESHOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T3_THRESHOLD to value 0"]
impl crate::Resettable for T3_THRESHOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
