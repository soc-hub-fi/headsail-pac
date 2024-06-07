#[doc = "Register `buf_ctrl` reader"]
pub type R = crate::R<BufCtrlSpec>;
#[doc = "Register `buf_ctrl` writer"]
pub type W = crate::W<BufCtrlSpec>;
#[doc = "Field `conv_mode` reader - "]
pub type ConvModeR = crate::FieldReader;
#[doc = "Field `conv_mode` writer - "]
pub type ConvModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `read_a_valid` reader - "]
pub type ReadAValidR = crate::BitReader;
#[doc = "Field `read_a_valid` writer - "]
pub type ReadAValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `read_b_valid` reader - "]
pub type ReadBValidR = crate::BitReader;
#[doc = "Field `read_b_valid` writer - "]
pub type ReadBValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn conv_mode(&self) -> ConvModeR {
        ConvModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn read_a_valid(&self) -> ReadAValidR {
        ReadAValidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn read_b_valid(&self) -> ReadBValidR {
        ReadBValidR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_ctrl")
            .field("conv_mode", &self.conv_mode())
            .field("read_a_valid", &self.read_a_valid())
            .field("read_b_valid", &self.read_b_valid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn conv_mode(&mut self) -> ConvModeW<BufCtrlSpec> {
        ConvModeW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn read_a_valid(&mut self) -> ReadAValidW<BufCtrlSpec> {
        ReadAValidW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn read_b_valid(&mut self) -> ReadBValidW<BufCtrlSpec> {
        ReadBValidW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufCtrlSpec;
impl crate::RegisterSpec for BufCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_ctrl::R`](R) reader structure"]
impl crate::Readable for BufCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`buf_ctrl::W`](W) writer structure"]
impl crate::Writable for BufCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_ctrl to value 0"]
impl crate::Resettable for BufCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
