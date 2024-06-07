#[doc = "Register `MDIO_ADDR_PHY` reader"]
pub type R = crate::R<MdioAddrPhySpec>;
#[doc = "Register `MDIO_ADDR_PHY` writer"]
pub type W = crate::W<MdioAddrPhySpec>;
#[doc = "Field `MDIO_ADDR_PHY` reader - "]
pub type MdioAddrPhyR = crate::FieldReader;
#[doc = "Field `MDIO_ADDR_PHY` writer - "]
pub type MdioAddrPhyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn mdio_addr_phy(&self) -> MdioAddrPhyR {
        MdioAddrPhyR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIO_ADDR_PHY")
            .field("mdio_addr_phy", &self.mdio_addr_phy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_addr_phy(&mut self) -> MdioAddrPhyW<MdioAddrPhySpec> {
        MdioAddrPhyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_addr_phy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_addr_phy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioAddrPhySpec;
impl crate::RegisterSpec for MdioAddrPhySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_addr_phy::R`](R) reader structure"]
impl crate::Readable for MdioAddrPhySpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_addr_phy::W`](W) writer structure"]
impl crate::Writable for MdioAddrPhySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_ADDR_PHY to value 0"]
impl crate::Resettable for MdioAddrPhySpec {
    const RESET_VALUE: u32 = 0;
}
