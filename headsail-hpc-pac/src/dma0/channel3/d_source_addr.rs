#[doc = "Register `D_SOURCE_ADDR` reader"]
pub type R = crate::R<D_SOURCE_ADDR_SPEC>;
#[doc = "Register `D_SOURCE_ADDR` writer"]
pub type W = crate::W<D_SOURCE_ADDR_SPEC>;
#[doc = "Field `Source_Address_0` reader - "]
pub type SOURCE_ADDRESS_0_R = crate::FieldReader<u32>;
#[doc = "Field `Source_Address_0` writer - "]
pub type SOURCE_ADDRESS_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn source_address_0(&self) -> SOURCE_ADDRESS_0_R {
        SOURCE_ADDRESS_0_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D_SOURCE_ADDR")
            .field("source_address_0", &self.source_address_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_0(&mut self) -> SOURCE_ADDRESS_0_W<D_SOURCE_ADDR_SPEC> {
        SOURCE_ADDRESS_0_W::new(self, 0)
    }
}
#[doc = "Start address for read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D_SOURCE_ADDR_SPEC;
impl crate::RegisterSpec for D_SOURCE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_source_addr::R`](R) reader structure"]
impl crate::Readable for D_SOURCE_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d_source_addr::W`](W) writer structure"]
impl crate::Writable for D_SOURCE_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D_SOURCE_ADDR to value 0"]
impl crate::Resettable for D_SOURCE_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
