#[doc = "Register `power_ctrl` reader"]
pub type R = crate::R<PowerCtrlSpec>;
#[doc = "Register `power_ctrl` writer"]
pub type W = crate::W<PowerCtrlSpec>;
#[doc = "Field `down1` reader - "]
pub type Down1R = crate::BitReader;
#[doc = "Field `down1` writer - "]
pub type Down1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `down2` reader - "]
pub type Down2R = crate::BitReader;
#[doc = "Field `down2` writer - "]
pub type Down2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iso` reader - "]
pub type IsoR = crate::BitReader;
#[doc = "Field `iso` writer - "]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn down1(&self) -> Down1R {
        Down1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn down2(&self) -> Down2R {
        Down2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn down1(&mut self) -> Down1W<PowerCtrlSpec> {
        Down1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn down2(&mut self) -> Down2W<PowerCtrlSpec> {
        Down2W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> IsoW<PowerCtrlSpec> {
        IsoW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerCtrlSpec;
impl crate::RegisterSpec for PowerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_ctrl::R`](R) reader structure"]
impl crate::Readable for PowerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_ctrl::W`](W) writer structure"]
impl crate::Writable for PowerCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets power_ctrl to value 0"]
impl crate::Resettable for PowerCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
