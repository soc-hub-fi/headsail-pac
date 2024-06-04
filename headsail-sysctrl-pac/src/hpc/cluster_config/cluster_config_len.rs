#[doc = "Register `cluster_config_len` reader"]
pub type R = crate::R<CLUSTER_CONFIG_LEN_SPEC>;
#[doc = "Register `cluster_config_len` writer"]
pub type W = crate::W<CLUSTER_CONFIG_LEN_SPEC>;
#[doc = "Field `len` reader - "]
pub type LEN_R = crate::FieldReader<u64>;
#[doc = "Field `len` writer - "]
pub type LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cluster_config_len")
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<CLUSTER_CONFIG_LEN_SPEC> {
        LEN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cluster_config_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cluster_config_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLUSTER_CONFIG_LEN_SPEC;
impl crate::RegisterSpec for CLUSTER_CONFIG_LEN_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`cluster_config_len::R`](R) reader structure"]
impl crate::Readable for CLUSTER_CONFIG_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cluster_config_len::W`](W) writer structure"]
impl crate::Writable for CLUSTER_CONFIG_LEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets cluster_config_len to value 0x1000"]
impl crate::Resettable for CLUSTER_CONFIG_LEN_SPEC {
    const RESET_VALUE: u64 = 0x1000;
}
