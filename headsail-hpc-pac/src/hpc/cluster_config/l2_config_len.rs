#[doc = "Register `l2_config_len` reader"]
pub type R = crate::R<L2ConfigLenSpec>;
#[doc = "Register `l2_config_len` writer"]
pub type W = crate::W<L2ConfigLenSpec>;
#[doc = "Field `len` reader - "]
pub type LenR = crate::FieldReader<u64>;
#[doc = "Field `len` writer - "]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("l2_config_len")
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<L2ConfigLenSpec> {
        LenW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_config_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_config_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2ConfigLenSpec;
impl crate::RegisterSpec for L2ConfigLenSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`l2_config_len::R`](R) reader structure"]
impl crate::Readable for L2ConfigLenSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_config_len::W`](W) writer structure"]
impl crate::Writable for L2ConfigLenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets l2_config_len to value 0x1000"]
impl crate::Resettable for L2ConfigLenSpec {
    const RESET_VALUE: u64 = 0x1000;
}
