#[doc = "Register `t_arf` reader"]
pub type R = crate::R<T_ARF_SPEC>;
#[doc = "Register `t_arf` writer"]
pub type W = crate::W<T_ARF_SPEC>;
#[doc = "Field `t_arf` reader - "]
pub type T_ARF_R = crate::FieldReader<u32>;
#[doc = "Field `t_arf` writer - "]
pub type T_ARF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_arf(&self) -> T_ARF_R {
        T_ARF_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_arf")
            .field("t_arf", &self.t_arf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_arf(&mut self) -> T_ARF_W<T_ARF_SPEC> {
        T_ARF_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_arf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_arf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_ARF_SPEC;
impl crate::RegisterSpec for T_ARF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_arf::R`](R) reader structure"]
impl crate::Readable for T_ARF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_arf::W`](W) writer structure"]
impl crate::Writable for T_ARF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_arf to value 0x017c"]
impl crate::Resettable for T_ARF_SPEC {
    const RESET_VALUE: u32 = 0x017c;
}
