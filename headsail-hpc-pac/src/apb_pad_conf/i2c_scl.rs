#[doc = "Register `i2c_scl` reader"]
pub type R = crate::R<I2C_SCL_SPEC>;
#[doc = "Register `i2c_scl` writer"]
pub type W = crate::W<I2C_SCL_SPEC>;
#[doc = "Field `DS0` reader - "]
pub type DS0_R = crate::BitReader;
#[doc = "Field `DS0` writer - "]
pub type DS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DS1` reader - "]
pub type DS1_R = crate::BitReader;
#[doc = "Field `DS1` writer - "]
pub type DS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST0` reader - "]
pub type ST0_R = crate::BitReader;
#[doc = "Field `ST0` writer - "]
pub type ST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST1` reader - "]
pub type ST1_R = crate::BitReader;
#[doc = "Field `ST1` writer - "]
pub type ST1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATE_CONTROL` reader - "]
pub type RATE_CONTROL_R = crate::BitReader;
#[doc = "Field `RATE_CONTROL` writer - "]
pub type RATE_CONTROL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_EN` reader - "]
pub type OUTPUT_EN_R = crate::BitReader;
#[doc = "Field `OUTPUT_EN` writer - "]
pub type OUTPUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOLD` reader - "]
pub type HOLD_R = crate::BitReader;
#[doc = "Field `HOLD` writer - "]
pub type HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULL_ENABLE` reader - "]
pub type PULL_ENABLE_R = crate::BitReader;
#[doc = "Field `PULL_ENABLE` writer - "]
pub type PULL_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULL_DIR` reader - "]
pub type PULL_DIR_R = crate::BitReader;
#[doc = "Field `PULL_DIR` writer - "]
pub type PULL_DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_EN` reader - "]
pub type INPUT_EN_R = crate::BitReader;
#[doc = "Field `INPUT_EN` writer - "]
pub type INPUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ds0(&self) -> DS0_R {
        DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ds1(&self) -> DS1_R {
        DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rate_control(&self) -> RATE_CONTROL_R {
        RATE_CONTROL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_en(&self) -> OUTPUT_EN_R {
        OUTPUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PULL_ENABLE_R {
        PULL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pull_dir(&self) -> PULL_DIR_R {
        PULL_DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn input_en(&self) -> INPUT_EN_R {
        INPUT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("i2c_scl")
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("st0", &self.st0())
            .field("st1", &self.st1())
            .field("rate_control", &self.rate_control())
            .field("output_en", &self.output_en())
            .field("hold", &self.hold())
            .field("pull_enable", &self.pull_enable())
            .field("pull_dir", &self.pull_dir())
            .field("input_en", &self.input_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ds0(&mut self) -> DS0_W<I2C_SCL_SPEC> {
        DS0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ds1(&mut self) -> DS1_W<I2C_SCL_SPEC> {
        DS1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> ST0_W<I2C_SCL_SPEC> {
        ST0_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> ST1_W<I2C_SCL_SPEC> {
        ST1_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rate_control(&mut self) -> RATE_CONTROL_W<I2C_SCL_SPEC> {
        RATE_CONTROL_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn output_en(&mut self) -> OUTPUT_EN_W<I2C_SCL_SPEC> {
        OUTPUT_EN_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hold(&mut self) -> HOLD_W<I2C_SCL_SPEC> {
        HOLD_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pull_enable(&mut self) -> PULL_ENABLE_W<I2C_SCL_SPEC> {
        PULL_ENABLE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pull_dir(&mut self) -> PULL_DIR_W<I2C_SCL_SPEC> {
        PULL_DIR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn input_en(&mut self) -> INPUT_EN_W<I2C_SCL_SPEC> {
        INPUT_EN_W::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_SCL_SPEC;
impl crate::RegisterSpec for I2C_SCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_scl::R`](R) reader structure"]
impl crate::Readable for I2C_SCL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_scl::W`](W) writer structure"]
impl crate::Writable for I2C_SCL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets i2c_scl to value 0"]
impl crate::Resettable for I2C_SCL_SPEC {
    const RESET_VALUE: u32 = 0;
}
