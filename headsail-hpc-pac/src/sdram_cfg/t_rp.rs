#[doc = "Register `t_rp` reader"]
pub type R = crate::R<TRpSpec>;
#[doc = "Register `t_rp` writer"]
pub type W = crate::W<TRpSpec>;
#[doc = "Field `t_rp` reader - "]
pub type TRpR = crate::FieldReader<u32>;
#[doc = "Field `t_rp` writer - "]
pub type TRpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_rp(&self) -> TRpR {
        TRpR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_rp").field("t_rp", &self.t_rp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_rp(&mut self) -> TRpW<TRpSpec> {
        TRpW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRpSpec;
impl crate::RegisterSpec for TRpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_rp::R`](R) reader structure"]
impl crate::Readable for TRpSpec {}
#[doc = "`write(|w| ..)` method takes [`t_rp::W`](W) writer structure"]
impl crate::Writable for TRpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_rp to value 0x02"]
impl crate::Resettable for TRpSpec {
    const RESET_VALUE: u32 = 0x02;
}
