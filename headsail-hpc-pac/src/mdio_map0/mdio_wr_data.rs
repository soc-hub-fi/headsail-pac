#[doc = "Register `MDIO_WR_DATA` reader"]
pub type R = crate::R<MdioWrDataSpec>;
#[doc = "Register `MDIO_WR_DATA` writer"]
pub type W = crate::W<MdioWrDataSpec>;
#[doc = "Field `MDIO_WR_DATA` reader - "]
pub type MdioWrDataR = crate::FieldReader<u16>;
#[doc = "Field `MDIO_WR_DATA` writer - "]
pub type MdioWrDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mdio_wr_data(&self) -> MdioWrDataR {
        MdioWrDataR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIO_WR_DATA")
            .field("mdio_wr_data", &self.mdio_wr_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_wr_data(&mut self) -> MdioWrDataW<MdioWrDataSpec> {
        MdioWrDataW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_wr_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_wr_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioWrDataSpec;
impl crate::RegisterSpec for MdioWrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_wr_data::R`](R) reader structure"]
impl crate::Readable for MdioWrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_wr_data::W`](W) writer structure"]
impl crate::Writable for MdioWrDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_WR_DATA to value 0"]
impl crate::Resettable for MdioWrDataSpec {
    const RESET_VALUE: u32 = 0;
}
