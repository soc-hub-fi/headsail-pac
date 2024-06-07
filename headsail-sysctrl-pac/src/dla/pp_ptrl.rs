#[doc = "Register `pp_ptrl` reader"]
pub type R = crate::R<PpPtrlSpec>;
#[doc = "Register `pp_ptrl` writer"]
pub type W = crate::W<PpPtrlSpec>;
#[doc = "Field `active_mode` reader - "]
pub type ActiveModeR = crate::FieldReader;
#[doc = "Field `active_mode` writer - "]
pub type ActiveModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reLu` reader - "]
pub type ReLuR = crate::FieldReader;
#[doc = "Field `reLu` writer - "]
pub type ReLuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `max` reader - "]
pub type MaxR = crate::FieldReader;
#[doc = "Field `max` writer - "]
pub type MaxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pp_select` reader - "]
pub type PpSelectR = crate::BitReader;
#[doc = "Field `pp_select` writer - "]
pub type PpSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pool_mode` reader - "]
pub type PoolModeR = crate::FieldReader;
#[doc = "Field `pool_mode` writer - "]
pub type PoolModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clip` reader - "]
pub type ClipR = crate::FieldReader;
#[doc = "Field `clip` writer - "]
pub type ClipW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn active_mode(&self) -> ActiveModeR {
        ActiveModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn re_lu(&self) -> ReLuR {
        ReLuR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn max(&self) -> MaxR {
        MaxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pp_select(&self) -> PpSelectR {
        PpSelectR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn pool_mode(&self) -> PoolModeR {
        PoolModeR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn clip(&self) -> ClipR {
        ClipR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pp_ptrl")
            .field("active_mode", &self.active_mode())
            .field("re_lu", &self.re_lu())
            .field("max", &self.max())
            .field("pp_select", &self.pp_select())
            .field("pool_mode", &self.pool_mode())
            .field("clip", &self.clip())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn active_mode(&mut self) -> ActiveModeW<PpPtrlSpec> {
        ActiveModeW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn re_lu(&mut self) -> ReLuW<PpPtrlSpec> {
        ReLuW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn max(&mut self) -> MaxW<PpPtrlSpec> {
        MaxW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pp_select(&mut self) -> PpSelectW<PpPtrlSpec> {
        PpSelectW::new(self, 6)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn pool_mode(&mut self) -> PoolModeW<PpPtrlSpec> {
        PoolModeW::new(self, 7)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn clip(&mut self) -> ClipW<PpPtrlSpec> {
        ClipW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_ptrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_ptrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpPtrlSpec;
impl crate::RegisterSpec for PpPtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp_ptrl::R`](R) reader structure"]
impl crate::Readable for PpPtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pp_ptrl::W`](W) writer structure"]
impl crate::Writable for PpPtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pp_ptrl to value 0"]
impl crate::Resettable for PpPtrlSpec {
    const RESET_VALUE: u32 = 0;
}
