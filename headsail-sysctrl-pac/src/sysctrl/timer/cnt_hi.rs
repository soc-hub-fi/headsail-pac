#[doc = "Register `CNT_HI` reader"]
pub type R = crate::R<CNT_HI_SPEC>;
#[doc = "Register `CNT_HI` writer"]
pub type W = crate::W<CNT_HI_SPEC>;
#[doc = "Field `CNT_HI` reader - "]
pub type CNT_HI_R = crate::FieldReader<u32>;
#[doc = "Field `CNT_HI` writer - "]
pub type CNT_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cnt_hi(&self) -> CNT_HI_R {
        CNT_HI_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT_HI")
            .field("cnt_hi", &self.cnt_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_hi(&mut self) -> CNT_HI_W<CNT_HI_SPEC> {
        CNT_HI_W::new(self, 0)
    }
}
#[doc = "Timer High counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_HI_SPEC;
impl crate::RegisterSpec for CNT_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt_hi::R`](R) reader structure"]
impl crate::Readable for CNT_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt_hi::W`](W) writer structure"]
impl crate::Writable for CNT_HI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT_HI to value 0"]
impl crate::Resettable for CNT_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
