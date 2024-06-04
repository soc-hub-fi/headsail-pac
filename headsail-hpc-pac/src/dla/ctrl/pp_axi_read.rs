#[doc = "Register `pp_axi_read` reader"]
pub type R = crate::R<PP_AXI_READ_SPEC>;
#[doc = "Register `pp_axi_read` writer"]
pub type W = crate::W<PP_AXI_READ_SPEC>;
#[doc = "Field `addr` reader - "]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `addr` writer - "]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pp_axi_read")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<PP_AXI_READ_SPEC> {
        ADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_axi_read::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_axi_read::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_AXI_READ_SPEC;
impl crate::RegisterSpec for PP_AXI_READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp_axi_read::R`](R) reader structure"]
impl crate::Readable for PP_AXI_READ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp_axi_read::W`](W) writer structure"]
impl crate::Writable for PP_AXI_READ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pp_axi_read to value 0"]
impl crate::Resettable for PP_AXI_READ_SPEC {
    const RESET_VALUE: u32 = 0;
}
