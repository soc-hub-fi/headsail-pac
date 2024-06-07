#[doc = "Register `PADDIR` reader"]
pub type R = crate::R<PADDIR_SPEC>;
#[doc = "Register `PADDIR` writer"]
pub type W = crate::W<PADDIR_SPEC>;
#[doc = "Field `PADDIR` reader - "]
pub type PADDIR_R = crate::FieldReader<u32>;
#[doc = "Field `PADDIR` writer - "]
pub type PADDIR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn paddir(&self) -> PADDIR_R {
        PADDIR_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADDIR")
            .field("paddir", &self.paddir())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn paddir(&mut self) -> PADDIR_W<PADDIR_SPEC> {
        PADDIR_W::new(self, 0)
    }
}
#[doc = "Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADDIR_SPEC;
impl crate::RegisterSpec for PADDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`paddir::R`](R) reader structure"]
impl crate::Readable for PADDIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`paddir::W`](W) writer structure"]
impl crate::Writable for PADDIR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADDIR to value 0"]
impl crate::Resettable for PADDIR_SPEC {
    const RESET_VALUE: u32 = 0;
}
