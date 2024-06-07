#[doc = "Register `ddr_refresh_ack` reader"]
pub type R = crate::R<DDR_REFRESH_ACK_SPEC>;
#[doc = "Register `ddr_refresh_ack` writer"]
pub type W = crate::W<DDR_REFRESH_ACK_SPEC>;
#[doc = "Field `ddr_refresh_ack` reader - "]
pub type DDR_REFRESH_ACK_R = crate::FieldReader<u32>;
#[doc = "Field `ddr_refresh_ack` writer - "]
pub type DDR_REFRESH_ACK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ddr_refresh_ack(&self) -> DDR_REFRESH_ACK_R {
        DDR_REFRESH_ACK_R::new(self.bits)
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
    pub fn ddr_refresh_ack(&mut self) -> DDR_REFRESH_ACK_W<DDR_REFRESH_ACK_SPEC> {
        DDR_REFRESH_ACK_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_refresh_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_refresh_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDR_REFRESH_ACK_SPEC;
impl crate::RegisterSpec for DDR_REFRESH_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_refresh_ack::R`](R) reader structure"]
impl crate::Readable for DDR_REFRESH_ACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddr_refresh_ack::W`](W) writer structure"]
impl crate::Writable for DDR_REFRESH_ACK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ddr_refresh_ack to value 0"]
impl crate::Resettable for DDR_REFRESH_ACK_SPEC {
    const RESET_VALUE: u32 = 0;
}
