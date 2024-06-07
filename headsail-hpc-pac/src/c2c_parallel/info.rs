#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Field `version` reader - Version number"]
pub type VersionR = crate::FieldReader;
#[doc = "Field `axi_addr_width` reader - AXI Address Width"]
pub type AxiAddrWidthR = crate::FieldReader;
#[doc = "Field `axi_data_width` reader - AXI Data Width"]
pub type AxiDataWidthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Version number"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AXI Address Width"]
    #[inline(always)]
    pub fn axi_addr_width(&self) -> AxiAddrWidthR {
        AxiAddrWidthR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AXI Data Width"]
    #[inline(always)]
    pub fn axi_data_width(&self) -> AxiDataWidthR {
        AxiDataWidthR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFO")
            .field("version", &self.version())
            .field("axi_addr_width", &self.axi_addr_width())
            .field("axi_data_width", &self.axi_data_width())
            .finish()
    }
}
#[doc = "Version number - AXI Address width - AXI Data width-Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`info::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`reset()` method sets INFO to value 0x2000_2000"]
impl crate::Resettable for InfoSpec {
    const RESET_VALUE: u32 = 0x2000_2000;
}
