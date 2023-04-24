#[doc = "Register `handshake` reader"]
pub struct R(crate::R<HANDSHAKE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HANDSHAKE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HANDSHAKE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HANDSHAKE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `handshake` writer"]
pub struct W(crate::W<HANDSHAKE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HANDSHAKE_SPEC>;
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
impl From<crate::W<HANDSHAKE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HANDSHAKE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `buffer_valid` reader - "]
pub type BUFFER_VALID_R = crate::BitReader<bool>;
#[doc = "Field `buffer_valid` writer - "]
pub type BUFFER_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `mac_valid` reader - "]
pub type MAC_VALID_R = crate::BitReader<bool>;
#[doc = "Field `mac_valid` writer - "]
pub type MAC_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `pool_valid` reader - "]
pub type POOL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `pool_valid` writer - "]
pub type POOL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `active_valid` reader - "]
pub type ACTIVE_VALID_R = crate::BitReader<bool>;
#[doc = "Field `active_valid` writer - "]
pub type ACTIVE_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `bufferEnable` reader - "]
pub type BUFFER_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `bufferEnable` writer - "]
pub type BUFFER_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `MACEnable` reader - "]
pub type MACENABLE_R = crate::BitReader<bool>;
#[doc = "Field `MACEnable` writer - "]
pub type MACENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `activeEnable` reader - "]
pub type ACTIVE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `activeEnable` writer - "]
pub type ACTIVE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `poolEnable` reader - "]
pub type POOL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `poolEnable` writer - "]
pub type POOL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `biasEnable` reader - "]
pub type BIAS_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `biasEnable` writer - "]
pub type BIAS_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
#[doc = "Field `bypassEnable` reader - "]
pub type BYPASS_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `bypassEnable` writer - "]
pub type BYPASS_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HANDSHAKE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn buffer_valid(&self) -> BUFFER_VALID_R {
        BUFFER_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mac_valid(&self) -> MAC_VALID_R {
        MAC_VALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pool_valid(&self) -> POOL_VALID_R {
        POOL_VALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn active_valid(&self) -> ACTIVE_VALID_R {
        ACTIVE_VALID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn buffer_enable(&self) -> BUFFER_ENABLE_R {
        BUFFER_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn macenable(&self) -> MACENABLE_R {
        MACENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn active_enable(&self) -> ACTIVE_ENABLE_R {
        ACTIVE_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pool_enable(&self) -> POOL_ENABLE_R {
        POOL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bias_enable(&self) -> BIAS_ENABLE_R {
        BIAS_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bypass_enable(&self) -> BYPASS_ENABLE_R {
        BYPASS_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_valid(&mut self) -> BUFFER_VALID_W<0> {
        BUFFER_VALID_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mac_valid(&mut self) -> MAC_VALID_W<1> {
        MAC_VALID_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pool_valid(&mut self) -> POOL_VALID_W<2> {
        POOL_VALID_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn active_valid(&mut self) -> ACTIVE_VALID_W<3> {
        ACTIVE_VALID_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_enable(&mut self) -> BUFFER_ENABLE_W<4> {
        BUFFER_ENABLE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn macenable(&mut self) -> MACENABLE_W<5> {
        MACENABLE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn active_enable(&mut self) -> ACTIVE_ENABLE_W<6> {
        ACTIVE_ENABLE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pool_enable(&mut self) -> POOL_ENABLE_W<7> {
        POOL_ENABLE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bias_enable(&mut self) -> BIAS_ENABLE_W<8> {
        BIAS_ENABLE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_enable(&mut self) -> BYPASS_ENABLE_W<9> {
        BYPASS_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [handshake](index.html) module"]
pub struct HANDSHAKE_SPEC;
impl crate::RegisterSpec for HANDSHAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [handshake::R](R) reader structure"]
impl crate::Readable for HANDSHAKE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [handshake::W](W) writer structure"]
impl crate::Writable for HANDSHAKE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets handshake to value 0"]
impl crate::Resettable for HANDSHAKE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
