#[doc = "Register `PADOUT` reader"]
pub type R = crate::R<PADOUT_SPEC>;
#[doc = "Register `PADOUT` writer"]
pub type W = crate::W<PADOUT_SPEC>;
#[doc = "Field `PADOUT` reader - "]
pub type PADOUT_R = crate::FieldReader<u32>;
#[doc = "Field `PADOUT` writer - "]
pub type PADOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padout(&self) -> PADOUT_R {
        PADOUT_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADOUT")
            .field("padout", &self.padout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn padout(&mut self) -> PADOUT_W<PADOUT_SPEC> {
        PADOUT_W::new(self, 0)
    }
}
#[doc = "Output Values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADOUT_SPEC;
impl crate::RegisterSpec for PADOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padout::R`](R) reader structure"]
impl crate::Readable for PADOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padout::W`](W) writer structure"]
impl crate::Writable for PADOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADOUT to value 0"]
impl crate::Resettable for PADOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
