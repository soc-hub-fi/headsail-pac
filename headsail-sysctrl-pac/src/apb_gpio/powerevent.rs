#[doc = "Register `POWEREVENT` reader"]
pub type R = crate::R<PowereventSpec>;
#[doc = "Register `POWEREVENT` writer"]
pub type W = crate::W<PowereventSpec>;
#[doc = "Field `POWEREVENT` reader - "]
pub type PowereventR = crate::FieldReader<u32>;
#[doc = "Field `POWEREVENT` writer - "]
pub type PowereventW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn powerevent(&self) -> PowereventR {
        PowereventR::new(self.bits)
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
    pub fn powerevent(&mut self) -> PowereventW<PowereventSpec> {
        PowereventW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerevent::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerevent::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowereventSpec;
impl crate::RegisterSpec for PowereventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerevent::R`](R) reader structure"]
impl crate::Readable for PowereventSpec {}
#[doc = "`write(|w| ..)` method takes [`powerevent::W`](W) writer structure"]
impl crate::Writable for PowereventSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWEREVENT to value 0"]
impl crate::Resettable for PowereventSpec {
    const RESET_VALUE: u32 = 0;
}
