#[doc = "Register `buf_ctrl` reader"]
pub struct R(crate::R<BUF_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `buf_ctrl` writer"]
pub struct W(crate::W<BUF_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BUF_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `conv_mode` reader - "]
pub type CONV_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `conv_mode` writer - "]
pub type CONV_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `read_a_valid` reader - "]
pub type READ_A_VALID_R = crate::BitReader<bool>;
#[doc = "Field `read_a_valid` writer - "]
pub type READ_A_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CTRL_SPEC, bool, O>;
#[doc = "Field `read_b_valid` reader - "]
pub type READ_B_VALID_R = crate::BitReader<bool>;
#[doc = "Field `read_b_valid` writer - "]
pub type READ_B_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CTRL_SPEC, bool, O>;
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
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn conv_mode(&mut self) -> CONV_MODE_W<0> {
        CONV_MODE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn read_a_valid(&mut self) -> READ_A_VALID_W<4> {
        READ_A_VALID_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn read_b_valid(&mut self) -> READ_B_VALID_W<5> {
        READ_B_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_ctrl](index.html) module"]
pub struct BUF_CTRL_SPEC;
impl crate::RegisterSpec for BUF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_ctrl::R](R) reader structure"]
impl crate::Readable for BUF_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_ctrl::W](W) writer structure"]
impl crate::Writable for BUF_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets buf_ctrl to value 0"]
impl crate::Resettable for BUF_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
