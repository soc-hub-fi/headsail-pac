#[doc = "Register `BOOT_STATUS` reader"]
pub type R = crate::R<BootStatusSpec>;
#[doc = "Register `BOOT_STATUS` writer"]
pub type W = crate::W<BootStatusSpec>;
#[doc = "Field `REG_BOOT_STATUS` reader - "]
pub type RegBootStatusR = crate::FieldReader<u32>;
#[doc = "Field `REG_BOOT_STATUS` writer - "]
pub type RegBootStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_boot_status(&self) -> RegBootStatusR {
        RegBootStatusR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_STATUS")
            .field("reg_boot_status", &self.reg_boot_status())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_boot_status(&mut self) -> RegBootStatusW<BootStatusSpec> {
        RegBootStatusW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootStatusSpec;
impl crate::RegisterSpec for BootStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_status::R`](R) reader structure"]
impl crate::Readable for BootStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`boot_status::W`](W) writer structure"]
impl crate::Writable for BootStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_STATUS to value 0"]
impl crate::Resettable for BootStatusSpec {
    const RESET_VALUE: u32 = 0;
}
