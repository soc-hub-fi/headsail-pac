#[doc = "Register `execute_region_addr_base0` reader"]
pub type R = crate::R<EXECUTE_REGION_ADDR_BASE0_SPEC>;
#[doc = "Register `execute_region_addr_base0` writer"]
pub type W = crate::W<EXECUTE_REGION_ADDR_BASE0_SPEC>;
#[doc = "Field `base` reader - "]
pub type BASE_R = crate::FieldReader<u64>;
#[doc = "Field `base` writer - "]
pub type BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("execute_region_addr_base0")
            .field("base", &self.base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<EXECUTE_REGION_ADDR_BASE0_SPEC> {
        BASE_W::new(self, 0)
    }
}
#[doc = "Base address for execute region #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXECUTE_REGION_ADDR_BASE0_SPEC;
impl crate::RegisterSpec for EXECUTE_REGION_ADDR_BASE0_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`execute_region_addr_base0::R`](R) reader structure"]
impl crate::Readable for EXECUTE_REGION_ADDR_BASE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`execute_region_addr_base0::W`](W) writer structure"]
impl crate::Writable for EXECUTE_REGION_ADDR_BASE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets execute_region_addr_base0 to value 0"]
impl crate::Resettable for EXECUTE_REGION_ADDR_BASE0_SPEC {
    const RESET_VALUE: u64 = 0;
}
