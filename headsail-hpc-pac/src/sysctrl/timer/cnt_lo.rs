#[doc = "Register `CNT_LO` reader"]
pub type R = crate::R<CntLoSpec>;
#[doc = "Register `CNT_LO` writer"]
pub type W = crate::W<CntLoSpec>;
#[doc = "Field `CNT_LO` reader - "]
pub type CntLoR = crate::FieldReader<u32>;
#[doc = "Field `CNT_LO` writer - "]
pub type CntLoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cnt_lo(&self) -> CntLoR {
        CntLoR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT_LO")
            .field("cnt_lo", &self.cnt_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_lo(&mut self) -> CntLoW<CntLoSpec> {
        CntLoW::new(self, 0)
    }
}
#[doc = "Timer Low counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntLoSpec;
impl crate::RegisterSpec for CntLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt_lo::R`](R) reader structure"]
impl crate::Readable for CntLoSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt_lo::W`](W) writer structure"]
impl crate::Writable for CntLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT_LO to value 0"]
impl crate::Resettable for CntLoSpec {
    const RESET_VALUE: u32 = 0;
}
