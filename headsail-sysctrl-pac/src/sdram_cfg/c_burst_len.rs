#[doc = "Register `c_burst_len` reader"]
pub type R = crate::R<C_BURST_LEN_SPEC>;
#[doc = "Register `c_burst_len` writer"]
pub type W = crate::W<C_BURST_LEN_SPEC>;
#[doc = "Field `c_burst_len` reader - "]
pub type C_BURST_LEN_R = crate::FieldReader<u32>;
#[doc = "Field `c_burst_len` writer - "]
pub type C_BURST_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_burst_len(&self) -> C_BURST_LEN_R {
        C_BURST_LEN_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_burst_len")
            .field("c_burst_len", &self.c_burst_len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_burst_len(&mut self) -> C_BURST_LEN_W<C_BURST_LEN_SPEC> {
        C_BURST_LEN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_burst_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_burst_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C_BURST_LEN_SPEC;
impl crate::RegisterSpec for C_BURST_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_burst_len::R`](R) reader structure"]
impl crate::Readable for C_BURST_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c_burst_len::W`](W) writer structure"]
impl crate::Writable for C_BURST_LEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_burst_len to value 0x01"]
impl crate::Resettable for C_BURST_LEN_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
