#[doc = "Register `c_end_addr` reader"]
pub type R = crate::R<CEndAddrSpec>;
#[doc = "Register `c_end_addr` writer"]
pub type W = crate::W<CEndAddrSpec>;
#[doc = "Field `c_end_addr` reader - "]
pub type CEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `c_end_addr` writer - "]
pub type CEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_end_addr(&self) -> CEndAddrR {
        CEndAddrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_end_addr")
            .field("c_end_addr", &self.c_end_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_end_addr(&mut self) -> CEndAddrW<CEndAddrSpec> {
        CEndAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_end_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_end_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEndAddrSpec;
impl crate::RegisterSpec for CEndAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_end_addr::R`](R) reader structure"]
impl crate::Readable for CEndAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c_end_addr::W`](W) writer structure"]
impl crate::Writable for CEndAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_end_addr to value 0xbfff_ffff"]
impl crate::Resettable for CEndAddrSpec {
    const RESET_VALUE: u32 = 0xbfff_ffff;
}
