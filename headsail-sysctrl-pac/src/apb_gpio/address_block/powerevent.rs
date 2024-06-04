#[doc = "Register `POWEREVENT` reader"]
pub type R = crate::R<POWEREVENT_SPEC>;
#[doc = "Register `POWEREVENT` writer"]
pub type W = crate::W<POWEREVENT_SPEC>;
#[doc = "Field `POWEREVENT` reader - "]
pub type POWEREVENT_R = crate::FieldReader<u32>;
#[doc = "Field `POWEREVENT` writer - "]
pub type POWEREVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn powerevent(&self) -> POWEREVENT_R {
        POWEREVENT_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWEREVENT")
            .field("powerevent", &self.powerevent())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn powerevent(&mut self) -> POWEREVENT_W<POWEREVENT_SPEC> {
        POWEREVENT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerevent::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerevent::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWEREVENT_SPEC;
impl crate::RegisterSpec for POWEREVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerevent::R`](R) reader structure"]
impl crate::Readable for POWEREVENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`powerevent::W`](W) writer structure"]
impl crate::Writable for POWEREVENT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWEREVENT to value 0"]
impl crate::Resettable for POWEREVENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
