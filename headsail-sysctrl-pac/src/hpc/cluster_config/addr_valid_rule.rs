#[doc = "Register `addr_valid_rule` reader"]
pub type R = crate::R<AddrValidRuleSpec>;
#[doc = "Register `addr_valid_rule` writer"]
pub type W = crate::W<AddrValidRuleSpec>;
#[doc = "Field `addr_valid_rule` reader - "]
pub type AddrValidRuleR = crate::FieldReader<u64>;
#[doc = "Field `addr_valid_rule` writer - "]
pub type AddrValidRuleW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn addr_valid_rule(&self) -> AddrValidRuleR {
        AddrValidRuleR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("addr_valid_rule")
            .field("addr_valid_rule", &self.addr_valid_rule())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn addr_valid_rule(&mut self) -> AddrValidRuleW<AddrValidRuleSpec> {
        AddrValidRuleW::new(self, 0)
    }
}
#[doc = "Valid address space flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr_valid_rule::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_valid_rule::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrValidRuleSpec;
impl crate::RegisterSpec for AddrValidRuleSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`addr_valid_rule::R`](R) reader structure"]
impl crate::Readable for AddrValidRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`addr_valid_rule::W`](W) writer structure"]
impl crate::Writable for AddrValidRuleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets addr_valid_rule to value 0x01"]
impl crate::Resettable for AddrValidRuleSpec {
    const RESET_VALUE: u64 = 0x01;
}
