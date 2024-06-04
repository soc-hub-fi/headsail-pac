#[doc = "Register `CTRL_CFG_CG` reader"]
pub type R = crate::R<CTRL_CFG_CG_SPEC>;
#[doc = "Register `CTRL_CFG_CG` writer"]
pub type W = crate::W<CTRL_CFG_CG_SPEC>;
#[doc = "Field `CG_UART` reader - "]
pub type CG_UART_R = crate::BitReader;
#[doc = "Field `CG_UART` writer - "]
pub type CG_UART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_SPIM` reader - "]
pub type CG_SPIM_R = crate::BitReader;
#[doc = "Field `CG_SPIM` writer - "]
pub type CG_SPIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_I2C0` reader - "]
pub type CG_I2C0_R = crate::BitReader;
#[doc = "Field `CG_I2C0` writer - "]
pub type CG_I2C0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_I2C1` reader - "]
pub type CG_I2C1_R = crate::BitReader;
#[doc = "Field `CG_I2C1` writer - "]
pub type CG_I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_SDIO` reader - "]
pub type CG_SDIO_R = crate::BitReader;
#[doc = "Field `CG_SDIO` writer - "]
pub type CG_SDIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_I2S` reader - "]
pub type CG_I2S_R = crate::BitReader;
#[doc = "Field `CG_I2S` writer - "]
pub type CG_I2S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_CAM` reader - "]
pub type CG_CAM_R = crate::BitReader;
#[doc = "Field `CG_CAM` writer - "]
pub type CG_CAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CG_FILTER` reader - "]
pub type CG_FILTER_R = crate::BitReader;
#[doc = "Field `CG_FILTER` writer - "]
pub type CG_FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cg_uart(&self) -> CG_UART_R {
        CG_UART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cg_spim(&self) -> CG_SPIM_R {
        CG_SPIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cg_i2c0(&self) -> CG_I2C0_R {
        CG_I2C0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cg_i2c1(&self) -> CG_I2C1_R {
        CG_I2C1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cg_sdio(&self) -> CG_SDIO_R {
        CG_SDIO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cg_i2s(&self) -> CG_I2S_R {
        CG_I2S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cg_cam(&self) -> CG_CAM_R {
        CG_CAM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cg_filter(&self) -> CG_FILTER_R {
        CG_FILTER_R::new(((self.bits >> 7) & 1) != 0)
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
    pub fn cg_uart(&mut self) -> CG_UART_W<CTRL_CFG_CG_SPEC> {
        CG_UART_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cg_spim(&mut self) -> CG_SPIM_W<CTRL_CFG_CG_SPEC> {
        CG_SPIM_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2c0(&mut self) -> CG_I2C0_W<CTRL_CFG_CG_SPEC> {
        CG_I2C0_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2c1(&mut self) -> CG_I2C1_W<CTRL_CFG_CG_SPEC> {
        CG_I2C1_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cg_sdio(&mut self) -> CG_SDIO_W<CTRL_CFG_CG_SPEC> {
        CG_SDIO_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2s(&mut self) -> CG_I2S_W<CTRL_CFG_CG_SPEC> {
        CG_I2S_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cg_cam(&mut self) -> CG_CAM_W<CTRL_CFG_CG_SPEC> {
        CG_CAM_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cg_filter(&mut self) -> CG_FILTER_W<CTRL_CFG_CG_SPEC> {
        CG_FILTER_W::new(self, 7)
    }
}
#[doc = "uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_cg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_cg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_CFG_CG_SPEC;
impl crate::RegisterSpec for CTRL_CFG_CG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_cfg_cg::R`](R) reader structure"]
impl crate::Readable for CTRL_CFG_CG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_cfg_cg::W`](W) writer structure"]
impl crate::Writable for CTRL_CFG_CG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_CFG_CG to value 0"]
impl crate::Resettable for CTRL_CFG_CG_SPEC {
    const RESET_VALUE: u32 = 0;
}
