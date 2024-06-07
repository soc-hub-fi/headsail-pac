#[doc = "Register `t_rrd` reader"]
pub type R = crate::R<T_RRD_SPEC>;
#[doc = "Register `t_rrd` writer"]
pub type W = crate::W<T_RRD_SPEC>;
#[doc = "Field `t_rrd` reader - "]
pub type T_RRD_R = crate::FieldReader<u32>;
#[doc = "Field `t_rrd` writer - "]
pub type T_RRD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_rrd(&self) -> T_RRD_R {
        T_RRD_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_rrd")
            .field("t_rrd", &self.t_rrd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_rrd(&mut self) -> T_RRD_W<T_RRD_SPEC> {
        T_RRD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_RRD_SPEC;
impl crate::RegisterSpec for T_RRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_rrd::R`](R) reader structure"]
impl crate::Readable for T_RRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_rrd::W`](W) writer structure"]
impl crate::Writable for T_RRD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_rrd to value 0x01"]
impl crate::Resettable for T_RRD_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
