#[doc = "Register `c_rd_lat` reader"]
pub type R = crate::R<CRdLatSpec>;
#[doc = "Register `c_rd_lat` writer"]
pub type W = crate::W<CRdLatSpec>;
#[doc = "Field `c_rd_lat` reader - "]
pub type CRdLatR = crate::FieldReader<u32>;
#[doc = "Field `c_rd_lat` writer - "]
pub type CRdLatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_rd_lat(&self) -> CRdLatR {
        CRdLatR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_rd_lat")
            .field("c_rd_lat", &self.c_rd_lat())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_rd_lat(&mut self) -> CRdLatW<CRdLatSpec> {
        CRdLatW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_rd_lat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_rd_lat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRdLatSpec;
impl crate::RegisterSpec for CRdLatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_rd_lat::R`](R) reader structure"]
impl crate::Readable for CRdLatSpec {}
#[doc = "`write(|w| ..)` method takes [`c_rd_lat::W`](W) writer structure"]
impl crate::Writable for CRdLatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_rd_lat to value 0x03"]
impl crate::Resettable for CRdLatSpec {
    const RESET_VALUE: u32 = 0x03;
}
