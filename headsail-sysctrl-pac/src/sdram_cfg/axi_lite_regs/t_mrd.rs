#[doc = "Register `t_mrd` reader"]
pub type R = crate::R<T_MRD_SPEC>;
#[doc = "Register `t_mrd` writer"]
pub type W = crate::W<T_MRD_SPEC>;
#[doc = "Field `t_mrd` reader - "]
pub type T_MRD_R = crate::FieldReader<u32>;
#[doc = "Field `t_mrd` writer - "]
pub type T_MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_mrd(&self) -> T_MRD_R {
        T_MRD_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_mrd")
            .field("t_mrd", &self.t_mrd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrd(&mut self) -> T_MRD_W<T_MRD_SPEC> {
        T_MRD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_mrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_mrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_MRD_SPEC;
impl crate::RegisterSpec for T_MRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_mrd::R`](R) reader structure"]
impl crate::Readable for T_MRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_mrd::W`](W) writer structure"]
impl crate::Writable for T_MRD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_mrd to value 0x02"]
impl crate::Resettable for T_MRD_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
