#[doc = "Register `t_ras` reader"]
pub type R = crate::R<T_RAS_SPEC>;
#[doc = "Register `t_ras` writer"]
pub type W = crate::W<T_RAS_SPEC>;
#[doc = "Field `t_ras` reader - "]
pub type T_RAS_R = crate::FieldReader<u32>;
#[doc = "Field `t_ras` writer - "]
pub type T_RAS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_ras(&self) -> T_RAS_R {
        T_RAS_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_ras")
            .field("t_ras", &self.t_ras())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_ras(&mut self) -> T_RAS_W<T_RAS_SPEC> {
        T_RAS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_ras::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_ras::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_RAS_SPEC;
impl crate::RegisterSpec for T_RAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_ras::R`](R) reader structure"]
impl crate::Readable for T_RAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_ras::W`](W) writer structure"]
impl crate::Writable for T_RAS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_ras to value 0x03"]
impl crate::Resettable for T_RAS_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
