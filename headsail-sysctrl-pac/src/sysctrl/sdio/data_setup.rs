#[doc = "Register `DATA_SETUP` writer"]
pub type W = crate::W<DATA_SETUP_SPEC>;
#[doc = "Field `EN` writer - "]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWN` writer - "]
pub type RWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUAD` writer - "]
pub type QUAD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Block_Num` writer - "]
pub type BLOCK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLOCK_SIZE` writer - "]
pub type BLOCK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_SETUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DATA_SETUP_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rwn(&mut self) -> RWN_W<DATA_SETUP_SPEC> {
        RWN_W::new(self, 1)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    #[must_use]
    pub fn quad(&mut self) -> QUAD_W<DATA_SETUP_SPEC> {
        QUAD_W::new(self, 2)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn block_num(&mut self) -> BLOCK_NUM_W<DATA_SETUP_SPEC> {
        BLOCK_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W<DATA_SETUP_SPEC> {
        BLOCK_SIZE_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_setup::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SETUP_SPEC;
impl crate::RegisterSpec for DATA_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data_setup::W`](W) writer structure"]
impl crate::Writable for DATA_SETUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_SETUP to value 0"]
impl crate::Resettable for DATA_SETUP_SPEC {
    const RESET_VALUE: u32 = 0;
}
