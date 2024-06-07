#[doc = "Register `c_burst_cycles` reader"]
pub type R = crate::R<CBurstCyclesSpec>;
#[doc = "Register `c_burst_cycles` writer"]
pub type W = crate::W<CBurstCyclesSpec>;
#[doc = "Field `c_burst_cycles` reader - "]
pub type CBurstCyclesR = crate::FieldReader<u32>;
#[doc = "Field `c_burst_cycles` writer - "]
pub type CBurstCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_burst_cycles(&self) -> CBurstCyclesR {
        CBurstCyclesR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_burst_cycles")
            .field("c_burst_cycles", &self.c_burst_cycles())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_burst_cycles(&mut self) -> CBurstCyclesW<CBurstCyclesSpec> {
        CBurstCyclesW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_burst_cycles::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_burst_cycles::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CBurstCyclesSpec;
impl crate::RegisterSpec for CBurstCyclesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_burst_cycles::R`](R) reader structure"]
impl crate::Readable for CBurstCyclesSpec {}
#[doc = "`write(|w| ..)` method takes [`c_burst_cycles::W`](W) writer structure"]
impl crate::Writable for CBurstCyclesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_burst_cycles to value 0x01"]
impl crate::Resettable for CBurstCyclesSpec {
    const RESET_VALUE: u32 = 0x01;
}
