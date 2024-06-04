#[doc = "Register `dla_ctrl` reader"]
pub type R = crate::R<DLA_CTRL_SPEC>;
#[doc = "Register `dla_ctrl` writer"]
pub type W = crate::W<DLA_CTRL_SPEC>;
#[doc = "Field `cpu_fe` reader - "]
pub type CPU_FE_R = crate::BitReader;
#[doc = "Field `cpu_fe` writer - "]
pub type CPU_FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hpRst` reader - "]
pub type HP_RST_R = crate::BitReader;
#[doc = "Field `hpRst` writer - "]
pub type HP_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_irq` reader - "]
pub type SW_IRQ_R = crate::BitReader;
#[doc = "Field `sw_irq` writer - "]
pub type SW_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpu_fe(&self) -> CPU_FE_R {
        CPU_FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hp_rst(&self) -> HP_RST_R {
        HP_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sw_irq(&self) -> SW_IRQ_R {
        SW_IRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("dla_ctrl")
            .field("cpu_fe", &self.cpu_fe())
            .field("hp_rst", &self.hp_rst())
            .field("sw_irq", &self.sw_irq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_fe(&mut self) -> CPU_FE_W<DLA_CTRL_SPEC> {
        CPU_FE_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn hp_rst(&mut self) -> HP_RST_W<DLA_CTRL_SPEC> {
        HP_RST_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irq(&mut self) -> SW_IRQ_W<DLA_CTRL_SPEC> {
        SW_IRQ_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLA_CTRL_SPEC;
impl crate::RegisterSpec for DLA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dla_ctrl::R`](R) reader structure"]
impl crate::Readable for DLA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dla_ctrl::W`](W) writer structure"]
impl crate::Writable for DLA_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dla_ctrl to value 0"]
impl crate::Resettable for DLA_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
