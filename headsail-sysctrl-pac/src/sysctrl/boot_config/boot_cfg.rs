#[doc = "Register `BOOT_CFG` reader"]
pub type R = crate::R<BootCfgSpec>;
#[doc = "Register `BOOT_CFG` writer"]
pub type W = crate::W<BootCfgSpec>;
#[doc = "Field `REG_BOOT_CFG` reader - "]
pub type RegBootCfgR = crate::FieldReader<u32>;
#[doc = "Field `REG_BOOT_CFG` writer - "]
pub type RegBootCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_boot_cfg(&self) -> RegBootCfgR {
        RegBootCfgR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_CFG")
            .field("reg_boot_cfg", &self.reg_boot_cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_boot_cfg(&mut self) -> RegBootCfgW<BootCfgSpec> {
        RegBootCfgW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootCfgSpec;
impl crate::RegisterSpec for BootCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_cfg::R`](R) reader structure"]
impl crate::Readable for BootCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`boot_cfg::W`](W) writer structure"]
impl crate::Writable for BootCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_CFG to value 0"]
impl crate::Resettable for BootCfgSpec {
    const RESET_VALUE: u32 = 0;
}
