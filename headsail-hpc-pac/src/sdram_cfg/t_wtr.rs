#[doc = "Register `t_wtr` reader"]
pub type R = crate::R<TWtrSpec>;
#[doc = "Register `t_wtr` writer"]
pub type W = crate::W<TWtrSpec>;
#[doc = "Field `t_wtr` reader - "]
pub type TWtrR = crate::FieldReader<u32>;
#[doc = "Field `t_wtr` writer - "]
pub type TWtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_wtr(&self) -> TWtrR {
        TWtrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_wtr")
            .field("t_wtr", &self.t_wtr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_wtr(&mut self) -> TWtrW<TWtrSpec> {
        TWtrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_wtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_wtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWtrSpec;
impl crate::RegisterSpec for TWtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_wtr::R`](R) reader structure"]
impl crate::Readable for TWtrSpec {}
#[doc = "`write(|w| ..)` method takes [`t_wtr::W`](W) writer structure"]
impl crate::Writable for TWtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_wtr to value 0x02"]
impl crate::Resettable for TWtrSpec {
    const RESET_VALUE: u32 = 0x02;
}
