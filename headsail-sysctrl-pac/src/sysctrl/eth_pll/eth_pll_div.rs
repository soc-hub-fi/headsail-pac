#[doc = "Register `ETH_PLL_DIV` reader"]
pub type R = crate::R<EthPllDivSpec>;
#[doc = "Register `ETH_PLL_DIV` writer"]
pub type W = crate::W<EthPllDivSpec>;
#[doc = "Field `r_div` reader - "]
pub type RDivR = crate::FieldReader;
#[doc = "Field `r_div` writer - "]
pub type RDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `n_div` reader - "]
pub type NDivR = crate::FieldReader<u16>;
#[doc = "Field `n_div` writer - "]
pub type NDivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `m_div` reader - "]
pub type MDivR = crate::FieldReader;
#[doc = "Field `m_div` writer - "]
pub type MDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn r_div(&self) -> RDivR {
        RDivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    pub fn n_div(&self) -> NDivR {
        NDivR::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn m_div(&self) -> MDivR {
        MDivR::new(((self.bits >> 14) & 7) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_PLL_DIV")
            .field("r_div", &self.r_div())
            .field("n_div", &self.n_div())
            .field("m_div", &self.m_div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn r_div(&mut self) -> RDivW<EthPllDivSpec> {
        RDivW::new(self, 0)
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    #[must_use]
    pub fn n_div(&mut self) -> NDivW<EthPllDivSpec> {
        NDivW::new(self, 4)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    #[must_use]
    pub fn m_div(&mut self) -> MDivW<EthPllDivSpec> {
        MDivW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_pll_div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_pll_div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EthPllDivSpec;
impl crate::RegisterSpec for EthPllDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_pll_div::R`](R) reader structure"]
impl crate::Readable for EthPllDivSpec {}
#[doc = "`write(|w| ..)` method takes [`eth_pll_div::W`](W) writer structure"]
impl crate::Writable for EthPllDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_PLL_DIV to value 0"]
impl crate::Resettable for EthPllDivSpec {
    const RESET_VALUE: u32 = 0;
}
