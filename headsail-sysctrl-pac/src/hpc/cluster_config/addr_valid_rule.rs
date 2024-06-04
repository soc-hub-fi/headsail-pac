#[doc = "Register `addr_valid_rule` reader"]
pub type R = crate::R<ADDR_VALID_RULE_SPEC>;
#[doc = "Register `addr_valid_rule` writer"]
pub type W = crate::W<ADDR_VALID_RULE_SPEC>;
#[doc = "Field `addr_valid_rule` reader - "]
pub type ADDR_VALID_RULE_R = crate::FieldReader<u64>;
#[doc = "Field `addr_valid_rule` writer - "]
pub type ADDR_VALID_RULE_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn addr_valid_rule(&self) -> ADDR_VALID_RULE_R {
        ADDR_VALID_RULE_R::new(self.bits)
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
    pub fn addr_valid_rule(&mut self) -> ADDR_VALID_RULE_W<ADDR_VALID_RULE_SPEC> {
        ADDR_VALID_RULE_W::new(self, 0)
    }
}
#[doc = "Valid address space flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr_valid_rule::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_valid_rule::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_VALID_RULE_SPEC;
impl crate::RegisterSpec for ADDR_VALID_RULE_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`addr_valid_rule::R`](R) reader structure"]
impl crate::Readable for ADDR_VALID_RULE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_valid_rule::W`](W) writer structure"]
impl crate::Writable for ADDR_VALID_RULE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets addr_valid_rule to value 0x01"]
impl crate::Resettable for ADDR_VALID_RULE_SPEC {
    const RESET_VALUE: u64 = 0x01;
}
