#[doc = "Register `t_rfc` reader"]
pub type R = crate::R<T_RFC_SPEC>;
#[doc = "Register `t_rfc` writer"]
pub type W = crate::W<T_RFC_SPEC>;
#[doc = "Field `t_rfc` reader - "]
pub type T_RFC_R = crate::FieldReader<u32>;
#[doc = "Field `t_rfc` writer - "]
pub type T_RFC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_rfc(&self) -> T_RFC_R {
        T_RFC_R::new(self.bits)
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
    pub fn t_rfc(&mut self) -> T_RFC_W<T_RFC_SPEC> {
        T_RFC_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_RFC_SPEC;
impl crate::RegisterSpec for T_RFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_rfc::R`](R) reader structure"]
impl crate::Readable for T_RFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_rfc::W`](W) writer structure"]
impl crate::Writable for T_RFC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_rfc to value 0x04"]
impl crate::Resettable for T_RFC_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
