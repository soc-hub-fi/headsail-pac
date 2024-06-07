#[doc = "Register `c_wr_lat` reader"]
pub type R = crate::R<CWrLatSpec>;
#[doc = "Register `c_wr_lat` writer"]
pub type W = crate::W<CWrLatSpec>;
#[doc = "Field `c_wr_lat` reader - "]
pub type CWrLatR = crate::FieldReader<u32>;
#[doc = "Field `c_wr_lat` writer - "]
pub type CWrLatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_wr_lat(&self) -> CWrLatR {
        CWrLatR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_wr_lat")
            .field("c_wr_lat", &self.c_wr_lat())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_wr_lat(&mut self) -> CWrLatW<CWrLatSpec> {
        CWrLatW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_wr_lat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_wr_lat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWrLatSpec;
impl crate::RegisterSpec for CWrLatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_wr_lat::R`](R) reader structure"]
impl crate::Readable for CWrLatSpec {}
#[doc = "`write(|w| ..)` method takes [`c_wr_lat::W`](W) writer structure"]
impl crate::Writable for CWrLatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_wr_lat to value 0x01"]
impl crate::Resettable for CWrLatSpec {
    const RESET_VALUE: u32 = 0x01;
}
