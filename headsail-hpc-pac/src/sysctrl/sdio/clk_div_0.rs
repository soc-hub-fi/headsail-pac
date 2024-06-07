#[doc = "Register `CLK_DIV_0` reader"]
pub type R = crate::R<ClkDiv0Spec>;
#[doc = "Register `CLK_DIV_0` writer"]
pub type W = crate::W<ClkDiv0Spec>;
#[doc = "Field `Div_count` reader - "]
pub type DivCountR = crate::FieldReader;
#[doc = "Field `Div_count` writer - "]
pub type DivCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Valid` reader - "]
pub type ValidR = crate::BitReader;
#[doc = "Field `Valid` writer - "]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn div_count(&self) -> DivCountR {
        DivCountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_DIV_0")
            .field("div_count", &self.div_count())
            .field("valid", &self.valid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn div_count(&mut self) -> DivCountW<ClkDiv0Spec> {
        DivCountW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<ClkDiv0Spec> {
        ValidW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_div_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_div_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDiv0Spec;
impl crate::RegisterSpec for ClkDiv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_div_0::R`](R) reader structure"]
impl crate::Readable for ClkDiv0Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_div_0::W`](W) writer structure"]
impl crate::Writable for ClkDiv0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_DIV_0 to value 0"]
impl crate::Resettable for ClkDiv0Spec {
    const RESET_VALUE: u32 = 0;
}
