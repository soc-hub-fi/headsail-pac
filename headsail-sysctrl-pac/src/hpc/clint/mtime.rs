#[doc = "Register `mtime` reader"]
pub type R = crate::R<MTIME_SPEC>;
#[doc = "Register `mtime` writer"]
pub type W = crate::W<MTIME_SPEC>;
#[doc = "Field `mtime` reader - "]
pub type MTIME_R = crate::FieldReader<u64>;
#[doc = "Field `mtime` writer - "]
pub type MTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn mtime(&self) -> MTIME_R {
        MTIME_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("mtime")
            .field("mtime", &self.mtime())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn mtime(&mut self) -> MTIME_W<MTIME_SPEC> {
        MTIME_W::new(self, 0)
    }
}
#[doc = "Cycle counter. A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtime::R`](R) reader structure"]
impl crate::Readable for MTIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtime::W`](W) writer structure"]
impl crate::Writable for MTIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets mtime to value 0"]
impl crate::Resettable for MTIME_SPEC {
    const RESET_VALUE: u64 = 0;
}
