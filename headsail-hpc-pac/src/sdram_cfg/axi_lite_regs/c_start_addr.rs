#[doc = "Register `c_start_addr` reader"]
pub type R = crate::R<C_START_ADDR_SPEC>;
#[doc = "Register `c_start_addr` writer"]
pub type W = crate::W<C_START_ADDR_SPEC>;
#[doc = "Field `c_start_addr` reader - "]
pub type C_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `c_start_addr` writer - "]
pub type C_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_start_addr(&self) -> C_START_ADDR_R {
        C_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_start_addr")
            .field("c_start_addr", &self.c_start_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_start_addr(&mut self) -> C_START_ADDR_W<C_START_ADDR_SPEC> {
        C_START_ADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_start_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_start_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C_START_ADDR_SPEC;
impl crate::RegisterSpec for C_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_start_addr::R`](R) reader structure"]
impl crate::Readable for C_START_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c_start_addr::W`](W) writer structure"]
impl crate::Writable for C_START_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_start_addr to value 0"]
impl crate::Resettable for C_START_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
