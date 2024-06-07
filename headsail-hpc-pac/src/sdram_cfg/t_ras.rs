#[doc = "Register `t_ras` reader"]
pub type R = crate::R<TRasSpec>;
#[doc = "Register `t_ras` writer"]
pub type W = crate::W<TRasSpec>;
#[doc = "Field `t_ras` reader - "]
pub type TRasR = crate::FieldReader<u32>;
#[doc = "Field `t_ras` writer - "]
pub type TRasW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_ras(&self) -> TRasR {
        TRasR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_ras")
            .field("t_ras", &self.t_ras())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_ras(&mut self) -> TRasW<TRasSpec> {
        TRasW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_ras::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_ras::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRasSpec;
impl crate::RegisterSpec for TRasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_ras::R`](R) reader structure"]
impl crate::Readable for TRasSpec {}
#[doc = "`write(|w| ..)` method takes [`t_ras::W`](W) writer structure"]
impl crate::Writable for TRasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_ras to value 0x03"]
impl crate::Resettable for TRasSpec {
    const RESET_VALUE: u32 = 0x03;
}
