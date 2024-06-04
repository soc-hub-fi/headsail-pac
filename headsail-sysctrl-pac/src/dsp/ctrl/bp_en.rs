#[doc = "Register `bp_en` reader"]
pub type R = crate::R<BP_EN_SPEC>;
#[doc = "Register `bp_en` writer"]
pub type W = crate::W<BP_EN_SPEC>;
#[doc = "Field `single_step_bp_en` reader - "]
pub type SINGLE_STEP_BP_EN_R = crate::BitReader;
#[doc = "Field `single_step_bp_en` writer - "]
pub type SINGLE_STEP_BP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable_breakpoint_1` reader - "]
pub type ENABLE_BREAKPOINT_1_R = crate::BitReader;
#[doc = "Field `enable_breakpoint_1` writer - "]
pub type ENABLE_BREAKPOINT_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable_breakpoint_2` reader - "]
pub type ENABLE_BREAKPOINT_2_R = crate::BitReader;
#[doc = "Field `enable_breakpoint_2` writer - "]
pub type ENABLE_BREAKPOINT_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn single_step_bp_en(&self) -> SINGLE_STEP_BP_EN_R {
        SINGLE_STEP_BP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable_breakpoint_1(&self) -> ENABLE_BREAKPOINT_1_R {
        ENABLE_BREAKPOINT_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn enable_breakpoint_2(&self) -> ENABLE_BREAKPOINT_2_R {
        ENABLE_BREAKPOINT_2_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("bp_en")
            .field("single_step_bp_en", &self.single_step_bp_en())
            .field("enable_breakpoint_1", &self.enable_breakpoint_1())
            .field("enable_breakpoint_2", &self.enable_breakpoint_2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn single_step_bp_en(&mut self) -> SINGLE_STEP_BP_EN_W<BP_EN_SPEC> {
        SINGLE_STEP_BP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn enable_breakpoint_1(&mut self) -> ENABLE_BREAKPOINT_1_W<BP_EN_SPEC> {
        ENABLE_BREAKPOINT_1_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn enable_breakpoint_2(&mut self) -> ENABLE_BREAKPOINT_2_W<BP_EN_SPEC> {
        ENABLE_BREAKPOINT_2_W::new(self, 4)
    }
}
#[doc = "Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BP_EN_SPEC;
impl crate::RegisterSpec for BP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bp_en::R`](R) reader structure"]
impl crate::Readable for BP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bp_en::W`](W) writer structure"]
impl crate::Writable for BP_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bp_en to value 0"]
impl crate::Resettable for BP_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
