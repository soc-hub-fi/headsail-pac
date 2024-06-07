#[doc = "Register `RGMII_SEL` reader"]
pub type R = crate::R<RgmiiSelSpec>;
#[doc = "Register `RGMII_SEL` writer"]
pub type W = crate::W<RgmiiSelSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgmiiSel {
    #[doc = "0: `0`"]
    SelTsnIp = 0,
    #[doc = "1: `1`"]
    SelEthIp = 1,
}
impl From<RgmiiSel> for bool {
    #[inline(always)]
    fn from(variant: RgmiiSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGMII_SEL` reader - "]
pub type RgmiiSelR = crate::BitReader<RgmiiSel>;
impl RgmiiSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgmiiSel {
        match self.bits {
            false => RgmiiSel::SelTsnIp,
            true => RgmiiSel::SelEthIp,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_sel_tsn_ip(&self) -> bool {
        *self == RgmiiSel::SelTsnIp
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sel_eth_ip(&self) -> bool {
        *self == RgmiiSel::SelEthIp
    }
}
#[doc = "Field `RGMII_SEL` writer - "]
pub type RgmiiSelW<'a, REG> = crate::BitWriter<'a, REG, RgmiiSel>;
impl<'a, REG> RgmiiSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_tsn_ip(self) -> &'a mut crate::W<REG> {
        self.variant(RgmiiSel::SelTsnIp)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_eth_ip(self) -> &'a mut crate::W<REG> {
        self.variant(RgmiiSel::SelEthIp)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rgmii_sel(&self) -> RgmiiSelR {
        RgmiiSelR::new((self.bits & 1) != 0)
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
    pub fn rgmii_sel(&mut self) -> RgmiiSelW<RgmiiSelSpec> {
        RgmiiSelW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgmii_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgmii_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgmiiSelSpec;
impl crate::RegisterSpec for RgmiiSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgmii_sel::R`](R) reader structure"]
impl crate::Readable for RgmiiSelSpec {}
#[doc = "`write(|w| ..)` method takes [`rgmii_sel::W`](W) writer structure"]
impl crate::Writable for RgmiiSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGMII_SEL to value 0"]
impl crate::Resettable for RgmiiSelSpec {
    const RESET_VALUE: u32 = 0;
}
