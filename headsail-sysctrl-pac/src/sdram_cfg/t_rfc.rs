#[doc = "Register `t_rfc` reader"]
pub type R = crate::R<TRfcSpec>;
#[doc = "Register `t_rfc` writer"]
pub type W = crate::W<TRfcSpec>;
#[doc = "Field `t_rfc` reader - "]
pub type TRfcR = crate::FieldReader<u32>;
#[doc = "Field `t_rfc` writer - "]
pub type TRfcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_rfc(&self) -> TRfcR {
        TRfcR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_rfc")
            .field("t_rfc", &self.t_rfc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_rfc(&mut self) -> TRfcW<TRfcSpec> {
        TRfcW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRfcSpec;
impl crate::RegisterSpec for TRfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_rfc::R`](R) reader structure"]
impl crate::Readable for TRfcSpec {}
#[doc = "`write(|w| ..)` method takes [`t_rfc::W`](W) writer structure"]
impl crate::Writable for TRfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_rfc to value 0x04"]
impl crate::Resettable for TRfcSpec {
    const RESET_VALUE: u32 = 0x04;
}
