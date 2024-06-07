#[doc = "Register `TX` reader"]
pub type R = crate::R<TX_SPEC>;
#[doc = "Register `TX` writer"]
pub type W = crate::W<TX_SPEC>;
#[doc = "Field `TX` reader - Transmit Register"]
pub type TX_R = crate::FieldReader;
#[doc = "Field `TX` writer - Transmit Register"]
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Register"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX").field("tx", &self.tx()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Register"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<TX_SPEC> {
        TX_W::new(self, 0)
    }
}
#[doc = "Transmit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_SPEC;
impl crate::RegisterSpec for TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx::R`](R) reader structure"]
impl crate::Readable for TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx::W`](W) writer structure"]
impl crate::Writable for TX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX to value 0"]
impl crate::Resettable for TX_SPEC {
    const RESET_VALUE: u32 = 0;
}
