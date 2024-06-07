#[doc = "Register `bp2_addr` reader"]
pub type R = crate::R<Bp2AddrSpec>;
#[doc = "Register `bp2_addr` writer"]
pub type W = crate::W<Bp2AddrSpec>;
#[doc = "Field `addr` reader - "]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `addr` writer - "]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("bp2_addr")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Bp2AddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Breakpoint 2 address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp2_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp2_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bp2AddrSpec;
impl crate::RegisterSpec for Bp2AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bp2_addr::R`](R) reader structure"]
impl crate::Readable for Bp2AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`bp2_addr::W`](W) writer structure"]
impl crate::Writable for Bp2AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bp2_addr to value 0"]
impl crate::Resettable for Bp2AddrSpec {
    const RESET_VALUE: u32 = 0;
}
