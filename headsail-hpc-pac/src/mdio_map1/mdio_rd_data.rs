#[doc = "Register `MDIO_RD_DATA` reader"]
pub type R = crate::R<MdioRdDataSpec>;
#[doc = "Field `MDIO_RD_DATA` reader - "]
pub type MdioRdDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mdio_rd_data(&self) -> MdioRdDataR {
        MdioRdDataR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIO_RD_DATA")
            .field("mdio_rd_data", &self.mdio_rd_data())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_rd_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioRdDataSpec;
impl crate::RegisterSpec for MdioRdDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_rd_data::R`](R) reader structure"]
impl crate::Readable for MdioRdDataSpec {}
#[doc = "`reset()` method sets MDIO_RD_DATA to value 0"]
impl crate::Resettable for MdioRdDataSpec {
    const RESET_VALUE: u32 = 0;
}
