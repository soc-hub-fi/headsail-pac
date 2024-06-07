#[doc = "Register `TX` reader"]
pub type R = crate::R<TxSpec>;
#[doc = "Register `TX` writer"]
pub type W = crate::W<TxSpec>;
#[doc = "Field `TX` reader - Transmit Register"]
pub type TxR = crate::FieldReader;
#[doc = "Field `TX` writer - Transmit Register"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Register"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new((self.bits & 0xff) as u8)
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
    pub fn tx(&mut self) -> TxW<TxSpec> {
        TxW::new(self, 0)
    }
}
#[doc = "Transmit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxSpec;
impl crate::RegisterSpec for TxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx::R`](R) reader structure"]
impl crate::Readable for TxSpec {}
#[doc = "`write(|w| ..)` method takes [`tx::W`](W) writer structure"]
impl crate::Writable for TxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX to value 0"]
impl crate::Resettable for TxSpec {
    const RESET_VALUE: u32 = 0;
}
