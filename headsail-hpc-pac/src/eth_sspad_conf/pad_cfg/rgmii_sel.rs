#[doc = "Register `RGMII_SEL` reader"]
pub type R = crate::R<RGMII_SEL_SPEC>;
#[doc = "Register `RGMII_SEL` writer"]
pub type W = crate::W<RGMII_SEL_SPEC>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGMII_SEL_A {
    #[doc = "0: `0`"]
    SEL_TSN_IP = 0,
    #[doc = "1: `1`"]
    SEL_ETH_IP = 1,
}
impl From<RGMII_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: RGMII_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGMII_SEL` reader - "]
pub type RGMII_SEL_R = crate::BitReader<RGMII_SEL_A>;
impl RGMII_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RGMII_SEL_A {
        match self.bits {
            false => RGMII_SEL_A::SEL_TSN_IP,
            true => RGMII_SEL_A::SEL_ETH_IP,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_sel_tsn_ip(&self) -> bool {
        *self == RGMII_SEL_A::SEL_TSN_IP
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sel_eth_ip(&self) -> bool {
        *self == RGMII_SEL_A::SEL_ETH_IP
    }
}
#[doc = "Field `RGMII_SEL` writer - "]
pub type RGMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG, RGMII_SEL_A>;
impl<'a, REG> RGMII_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_tsn_ip(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_SEL_A::SEL_TSN_IP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_eth_ip(self) -> &'a mut crate::W<REG> {
        self.variant(RGMII_SEL_A::SEL_ETH_IP)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rgmii_sel(&self) -> RGMII_SEL_R {
        RGMII_SEL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGMII_SEL")
            .field("rgmii_sel", &self.rgmii_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_sel(&mut self) -> RGMII_SEL_W<RGMII_SEL_SPEC> {
        RGMII_SEL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgmii_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgmii_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGMII_SEL_SPEC;
impl crate::RegisterSpec for RGMII_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgmii_sel::R`](R) reader structure"]
impl crate::Readable for RGMII_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rgmii_sel::W`](W) writer structure"]
impl crate::Writable for RGMII_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGMII_SEL to value 0"]
impl crate::Resettable for RGMII_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
