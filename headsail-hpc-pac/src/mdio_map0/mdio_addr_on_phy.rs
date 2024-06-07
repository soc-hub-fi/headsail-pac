#[doc = "Register `MDIO_ADDR_ON_PHY` reader"]
pub type R = crate::R<MdioAddrOnPhySpec>;
#[doc = "Register `MDIO_ADDR_ON_PHY` writer"]
pub type W = crate::W<MdioAddrOnPhySpec>;
#[doc = "Field `MDIO_ADDR_ON_PHY` reader - "]
pub type MdioAddrOnPhyR = crate::FieldReader;
#[doc = "Field `MDIO_ADDR_ON_PHY` writer - "]
pub type MdioAddrOnPhyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn mdio_addr_on_phy(&self) -> MdioAddrOnPhyR {
        MdioAddrOnPhyR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIO_ADDR_ON_PHY")
            .field("mdio_addr_on_phy", &self.mdio_addr_on_phy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_addr_on_phy(&mut self) -> MdioAddrOnPhyW<MdioAddrOnPhySpec> {
        MdioAddrOnPhyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_addr_on_phy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_addr_on_phy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioAddrOnPhySpec;
impl crate::RegisterSpec for MdioAddrOnPhySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_addr_on_phy::R`](R) reader structure"]
impl crate::Readable for MdioAddrOnPhySpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_addr_on_phy::W`](W) writer structure"]
impl crate::Writable for MdioAddrOnPhySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_ADDR_ON_PHY to value 0"]
impl crate::Resettable for MdioAddrOnPhySpec {
    const RESET_VALUE: u32 = 0;
}
