#[doc = "Register `start_addr` reader"]
pub type R = crate::R<StartAddrSpec>;
#[doc = "Register `start_addr` writer"]
pub type W = crate::W<StartAddrSpec>;
#[doc = "Field `addr` reader - star"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `addr` writer - star"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - star"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("start_addr")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - star"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<StartAddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`start_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartAddrSpec;
impl crate::RegisterSpec for StartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start_addr::R`](R) reader structure"]
impl crate::Readable for StartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`start_addr::W`](W) writer structure"]
impl crate::Writable for StartAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets start_addr to value 0"]
impl crate::Resettable for StartAddrSpec {
    const RESET_VALUE: u32 = 0;
}
