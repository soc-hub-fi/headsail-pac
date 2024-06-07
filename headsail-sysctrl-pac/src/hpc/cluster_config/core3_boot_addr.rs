#[doc = "Register `core3_boot_addr` reader"]
pub type R = crate::R<Core3BootAddrSpec>;
#[doc = "Register `core3_boot_addr` writer"]
pub type W = crate::W<Core3BootAddrSpec>;
#[doc = "Field `boot_addr` reader - "]
pub type BootAddrR = crate::FieldReader<u64>;
#[doc = "Field `boot_addr` writer - "]
pub type BootAddrW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn boot_addr(&self) -> BootAddrR {
        BootAddrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("core3_boot_addr")
            .field("boot_addr", &self.boot_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn boot_addr(&mut self) -> BootAddrW<Core3BootAddrSpec> {
        BootAddrW::new(self, 0)
    }
}
#[doc = "Core #3 boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core3_boot_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core3_boot_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core3BootAddrSpec;
impl crate::RegisterSpec for Core3BootAddrSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`core3_boot_addr::R`](R) reader structure"]
impl crate::Readable for Core3BootAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`core3_boot_addr::W`](W) writer structure"]
impl crate::Writable for Core3BootAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets core3_boot_addr to value 0x0001_0000"]
impl crate::Resettable for Core3BootAddrSpec {
    const RESET_VALUE: u64 = 0x0001_0000;
}
