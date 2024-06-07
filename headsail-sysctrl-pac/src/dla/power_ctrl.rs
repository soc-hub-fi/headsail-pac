#[doc = "Register `power_ctrl` reader"]
pub type R = crate::R<POWER_CTRL_SPEC>;
#[doc = "Register `power_ctrl` writer"]
pub type W = crate::W<POWER_CTRL_SPEC>;
#[doc = "Field `down1` reader - "]
pub type DOWN1_R = crate::BitReader;
#[doc = "Field `down1` writer - "]
pub type DOWN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `down2` reader - "]
pub type DOWN2_R = crate::BitReader;
#[doc = "Field `down2` writer - "]
pub type DOWN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iso` reader - "]
pub type ISO_R = crate::BitReader;
#[doc = "Field `iso` writer - "]
pub type ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn down1(&self) -> DOWN1_R {
        DOWN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn down2(&self) -> DOWN2_R {
        DOWN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("power_ctrl")
            .field("down1", &self.down1())
            .field("down2", &self.down2())
            .field("iso", &self.iso())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn down1(&mut self) -> DOWN1_W<POWER_CTRL_SPEC> {
        DOWN1_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn down2(&mut self) -> DOWN2_W<POWER_CTRL_SPEC> {
        DOWN2_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> ISO_W<POWER_CTRL_SPEC> {
        ISO_W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_CTRL_SPEC;
impl crate::RegisterSpec for POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_ctrl::R`](R) reader structure"]
impl crate::Readable for POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_ctrl::W`](W) writer structure"]
impl crate::Writable for POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets power_ctrl to value 0"]
impl crate::Resettable for POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
