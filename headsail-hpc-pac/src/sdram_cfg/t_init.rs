#[doc = "Register `t_init` reader"]
pub type R = crate::R<T_INIT_SPEC>;
#[doc = "Register `t_init` writer"]
pub type W = crate::W<T_INIT_SPEC>;
#[doc = "Field `t_init` reader - "]
pub type T_INIT_R = crate::FieldReader<u32>;
#[doc = "Field `t_init` writer - "]
pub type T_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_init(&self) -> T_INIT_R {
        T_INIT_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_init")
            .field("t_init", &self.t_init())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_init(&mut self) -> T_INIT_W<T_INIT_SPEC> {
        T_INIT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_INIT_SPEC;
impl crate::RegisterSpec for T_INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_init::R`](R) reader structure"]
impl crate::Readable for T_INIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_init::W`](W) writer structure"]
impl crate::Writable for T_INIT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_init to value 0x2710"]
impl crate::Resettable for T_INIT_SPEC {
    const RESET_VALUE: u32 = 0x2710;
}
