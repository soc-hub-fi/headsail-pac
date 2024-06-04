#[doc = "Register `SPICMD` reader"]
pub type R = crate::R<SPICMD_SPEC>;
#[doc = "Register `SPICMD` writer"]
pub type W = crate::W<SPICMD_SPEC>;
#[doc = "Field `SPICMD` reader - Insert the command to be transmitted"]
pub type SPICMD_R = crate::FieldReader<u32>;
#[doc = "Field `SPICMD` writer - Insert the command to be transmitted"]
pub type SPICMD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Insert the command to be transmitted"]
    #[inline(always)]
    pub fn spicmd(&self) -> SPICMD_R {
        SPICMD_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPICMD")
            .field("spicmd", &self.spicmd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Insert the command to be transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn spicmd(&mut self) -> SPICMD_W<SPICMD_SPEC> {
        SPICMD_W::new(self, 0)
    }
}
#[doc = "SPI Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spicmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spicmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPICMD_SPEC;
impl crate::RegisterSpec for SPICMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spicmd::R`](R) reader structure"]
impl crate::Readable for SPICMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spicmd::W`](W) writer structure"]
impl crate::Writable for SPICMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPICMD to value 0"]
impl crate::Resettable for SPICMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
