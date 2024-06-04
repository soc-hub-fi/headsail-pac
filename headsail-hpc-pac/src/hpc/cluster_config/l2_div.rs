#[doc = "Register `l2_div` reader"]
pub type R = crate::R<L2_DIV_SPEC>;
#[doc = "Register `l2_div` writer"]
pub type W = crate::W<L2_DIV_SPEC>;
#[doc = "Field `l2_div` reader - "]
pub type L2_DIV_R = crate::FieldReader<u64>;
#[doc = "Field `l2_div` writer - "]
pub type L2_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn l2_div(&self) -> L2_DIV_R {
        L2_DIV_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("l2_div")
            .field("l2_div", &self.l2_div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn l2_div(&mut self) -> L2_DIV_W<L2_DIV_SPEC> {
        L2_DIV_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_DIV_SPEC;
impl crate::RegisterSpec for L2_DIV_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`l2_div::R`](R) reader structure"]
impl crate::Readable for L2_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_div::W`](W) writer structure"]
impl crate::Writable for L2_DIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets l2_div to value 0x01"]
impl crate::Resettable for L2_DIV_SPEC {
    const RESET_VALUE: u64 = 0x01;
}
