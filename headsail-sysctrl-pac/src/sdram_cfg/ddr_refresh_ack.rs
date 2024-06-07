#[doc = "Register `ddr_refresh_ack` reader"]
pub type R = crate::R<DdrRefreshAckSpec>;
#[doc = "Register `ddr_refresh_ack` writer"]
pub type W = crate::W<DdrRefreshAckSpec>;
#[doc = "Field `ddr_refresh_ack` reader - "]
pub type DdrRefreshAckR = crate::FieldReader<u32>;
#[doc = "Field `ddr_refresh_ack` writer - "]
pub type DdrRefreshAckW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ddr_refresh_ack(&self) -> DdrRefreshAckR {
        DdrRefreshAckR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ddr_refresh_ack")
            .field("ddr_refresh_ack", &self.ddr_refresh_ack())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_refresh_ack(&mut self) -> DdrRefreshAckW<DdrRefreshAckSpec> {
        DdrRefreshAckW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_refresh_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_refresh_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrRefreshAckSpec;
impl crate::RegisterSpec for DdrRefreshAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_refresh_ack::R`](R) reader structure"]
impl crate::Readable for DdrRefreshAckSpec {}
#[doc = "`write(|w| ..)` method takes [`ddr_refresh_ack::W`](W) writer structure"]
impl crate::Writable for DdrRefreshAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ddr_refresh_ack to value 0"]
impl crate::Resettable for DdrRefreshAckSpec {
    const RESET_VALUE: u32 = 0;
}
