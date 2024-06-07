#[doc = "Register `buf_ctrl` reader"]
pub type R = crate::R<BUF_CTRL_SPEC>;
#[doc = "Register `buf_ctrl` writer"]
pub type W = crate::W<BUF_CTRL_SPEC>;
#[doc = "Field `conv_mode` reader - "]
pub type CONV_MODE_R = crate::FieldReader;
#[doc = "Field `conv_mode` writer - "]
pub type CONV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `read_a_valid` reader - "]
pub type READ_A_VALID_R = crate::BitReader;
#[doc = "Field `read_a_valid` writer - "]
pub type READ_A_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `read_b_valid` reader - "]
pub type READ_B_VALID_R = crate::BitReader;
#[doc = "Field `read_b_valid` writer - "]
pub type READ_B_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn conv_mode(&self) -> CONV_MODE_R {
        CONV_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn read_a_valid(&self) -> READ_A_VALID_R {
        READ_A_VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn read_b_valid(&self) -> READ_B_VALID_R {
        READ_B_VALID_R::new(((self.bits >> 5) & 1) != 0)
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
    pub fn conv_mode(&mut self) -> CONV_MODE_W<BUF_CTRL_SPEC> {
        CONV_MODE_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn read_a_valid(&mut self) -> READ_A_VALID_W<BUF_CTRL_SPEC> {
        READ_A_VALID_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn read_b_valid(&mut self) -> READ_B_VALID_W<BUF_CTRL_SPEC> {
        READ_B_VALID_W::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_CTRL_SPEC;
impl crate::RegisterSpec for BUF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_ctrl::R`](R) reader structure"]
impl crate::Readable for BUF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_ctrl::W`](W) writer structure"]
impl crate::Writable for BUF_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_ctrl to value 0"]
impl crate::Resettable for BUF_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
