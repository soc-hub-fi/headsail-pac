#[doc = "Register `DATA_SETUP` writer"]
pub type W = crate::W<DataSetupSpec>;
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWN` writer - "]
pub type RwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUAD` writer - "]
pub type QuadW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Block_Num` writer - "]
pub type BlockNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLOCK_SIZE` writer - "]
pub type BlockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<DataSetupSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<DataSetupSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rwn(&mut self) -> RwnW<DataSetupSpec> {
        RwnW::new(self, 1)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    #[must_use]
    pub fn quad(&mut self) -> QuadW<DataSetupSpec> {
        QuadW::new(self, 2)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn block_num(&mut self) -> BlockNumW<DataSetupSpec> {
        BlockNumW::new(self, 8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn block_size(&mut self) -> BlockSizeW<DataSetupSpec> {
        BlockSizeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_setup::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSetupSpec;
impl crate::RegisterSpec for DataSetupSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data_setup::W`](W) writer structure"]
impl crate::Writable for DataSetupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_SETUP to value 0"]
impl crate::Resettable for DataSetupSpec {
    const RESET_VALUE: u32 = 0;
}
