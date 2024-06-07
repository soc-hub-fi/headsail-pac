#[doc = "Register `CTRL_CFG_CG` reader"]
pub type R = crate::R<CtrlCfgCgSpec>;
#[doc = "Register `CTRL_CFG_CG` writer"]
pub type W = crate::W<CtrlCfgCgSpec>;
#[doc = "Field `CG_UART` reader - "]
pub type CgUartR = crate::BitReader;
#[doc = "Field `CG_UART` writer - "]
pub type CgUartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_SPIM` reader - "]
pub type CgSpimR = crate::BitReader;
#[doc = "Field `CG_SPIM` writer - "]
pub type CgSpimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_I2C0` reader - "]
pub type CgI2c0R = crate::BitReader;
#[doc = "Field `CG_I2C0` writer - "]
pub type CgI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_I2C1` reader - "]
pub type CgI2c1R = crate::BitReader;
#[doc = "Field `CG_I2C1` writer - "]
pub type CgI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_SDIO` reader - "]
pub type CgSdioR = crate::BitReader;
#[doc = "Field `CG_SDIO` writer - "]
pub type CgSdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_I2S` reader - "]
pub type CgI2sR = crate::BitReader;
#[doc = "Field `CG_I2S` writer - "]
pub type CgI2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_CAM` reader - "]
pub type CgCamR = crate::BitReader;
#[doc = "Field `CG_CAM` writer - "]
pub type CgCamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_FILTER` reader - "]
pub type CgFilterR = crate::BitReader;
#[doc = "Field `CG_FILTER` writer - "]
pub type CgFilterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cg_uart(&self) -> CgUartR {
        CgUartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cg_spim(&self) -> CgSpimR {
        CgSpimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cg_i2c0(&self) -> CgI2c0R {
        CgI2c0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cg_i2c1(&self) -> CgI2c1R {
        CgI2c1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cg_sdio(&self) -> CgSdioR {
        CgSdioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cg_i2s(&self) -> CgI2sR {
        CgI2sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cg_cam(&self) -> CgCamR {
        CgCamR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cg_filter(&self) -> CgFilterR {
        CgFilterR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_CFG_CG")
            .field("cg_uart", &self.cg_uart())
            .field("cg_spim", &self.cg_spim())
            .field("cg_i2c0", &self.cg_i2c0())
            .field("cg_i2c1", &self.cg_i2c1())
            .field("cg_sdio", &self.cg_sdio())
            .field("cg_i2s", &self.cg_i2s())
            .field("cg_cam", &self.cg_cam())
            .field("cg_filter", &self.cg_filter())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cg_uart(&mut self) -> CgUartW<CtrlCfgCgSpec> {
        CgUartW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cg_spim(&mut self) -> CgSpimW<CtrlCfgCgSpec> {
        CgSpimW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2c0(&mut self) -> CgI2c0W<CtrlCfgCgSpec> {
        CgI2c0W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2c1(&mut self) -> CgI2c1W<CtrlCfgCgSpec> {
        CgI2c1W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cg_sdio(&mut self) -> CgSdioW<CtrlCfgCgSpec> {
        CgSdioW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2s(&mut self) -> CgI2sW<CtrlCfgCgSpec> {
        CgI2sW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cg_cam(&mut self) -> CgCamW<CtrlCfgCgSpec> {
        CgCamW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cg_filter(&mut self) -> CgFilterW<CtrlCfgCgSpec> {
        CgFilterW::new(self, 7)
    }
}
#[doc = "uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_cg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_cg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlCfgCgSpec;
impl crate::RegisterSpec for CtrlCfgCgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_cfg_cg::R`](R) reader structure"]
impl crate::Readable for CtrlCfgCgSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_cfg_cg::W`](W) writer structure"]
impl crate::Writable for CtrlCfgCgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_CFG_CG to value 0"]
impl crate::Resettable for CtrlCfgCgSpec {
    const RESET_VALUE: u32 = 0;
}
