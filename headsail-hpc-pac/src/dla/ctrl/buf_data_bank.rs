#[doc = "Register `buf_data_bank` reader"]
pub type R = crate::R<BUF_DATA_BANK_SPEC>;
#[doc = "Register `buf_data_bank` writer"]
pub type W = crate::W<BUF_DATA_BANK_SPEC>;
#[doc = "Field `a` reader - "]
pub type A_R = crate::FieldReader;
#[doc = "Field `a` writer - "]
pub type A_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `b` reader - "]
pub type B_R = crate::FieldReader;
#[doc = "Field `b` writer - "]
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_data_bank")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<BUF_DATA_BANK_SPEC> {
        A_W::new(self, 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<BUF_DATA_BANK_SPEC> {
        B_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_bank::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_data_bank::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_DATA_BANK_SPEC;
impl crate::RegisterSpec for BUF_DATA_BANK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_data_bank::R`](R) reader structure"]
impl crate::Readable for BUF_DATA_BANK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_data_bank::W`](W) writer structure"]
impl crate::Writable for BUF_DATA_BANK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_data_bank to value 0"]
impl crate::Resettable for BUF_DATA_BANK_SPEC {
    const RESET_VALUE: u32 = 0;
}
