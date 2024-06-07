#[doc = "Register `handshake` reader"]
pub type R = crate::R<HANDSHAKE_SPEC>;
#[doc = "Register `handshake` writer"]
pub type W = crate::W<HANDSHAKE_SPEC>;
#[doc = "Field `buffer_valid` reader - "]
pub type BUFFER_VALID_R = crate::BitReader;
#[doc = "Field `buffer_valid` writer - "]
pub type BUFFER_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mac_valid` reader - "]
pub type MAC_VALID_R = crate::BitReader;
#[doc = "Field `mac_valid` writer - "]
pub type MAC_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pool_valid` reader - "]
pub type POOL_VALID_R = crate::BitReader;
#[doc = "Field `pool_valid` writer - "]
pub type POOL_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `active_valid` reader - "]
pub type ACTIVE_VALID_R = crate::BitReader;
#[doc = "Field `active_valid` writer - "]
pub type ACTIVE_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bufferEnable` reader - "]
pub type BUFFER_ENABLE_R = crate::BitReader;
#[doc = "Field `bufferEnable` writer - "]
pub type BUFFER_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACEnable` reader - "]
pub type MACENABLE_R = crate::BitReader;
#[doc = "Field `MACEnable` writer - "]
pub type MACENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `activeEnable` reader - "]
pub type ACTIVE_ENABLE_R = crate::BitReader;
#[doc = "Field `activeEnable` writer - "]
pub type ACTIVE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `poolEnable` reader - "]
pub type POOL_ENABLE_R = crate::BitReader;
#[doc = "Field `poolEnable` writer - "]
pub type POOL_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `biasEnable` reader - "]
pub type BIAS_ENABLE_R = crate::BitReader;
#[doc = "Field `biasEnable` writer - "]
pub type BIAS_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypassEnable` reader - "]
pub type BYPASS_ENABLE_R = crate::BitReader;
#[doc = "Field `bypassEnable` writer - "]
pub type BYPASS_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("handshake")
            .field("buffer_valid", &self.buffer_valid())
            .field("mac_valid", &self.mac_valid())
            .field("pool_valid", &self.pool_valid())
            .field("active_valid", &self.active_valid())
            .field("buffer_enable", &self.buffer_enable())
            .field("macenable", &self.macenable())
            .field("active_enable", &self.active_enable())
            .field("pool_enable", &self.pool_enable())
            .field("bias_enable", &self.bias_enable())
            .field("bypass_enable", &self.bypass_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_valid(&mut self) -> BUFFER_VALID_W<HANDSHAKE_SPEC> {
        BUFFER_VALID_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mac_valid(&mut self) -> MAC_VALID_W<HANDSHAKE_SPEC> {
        MAC_VALID_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pool_valid(&mut self) -> POOL_VALID_W<HANDSHAKE_SPEC> {
        POOL_VALID_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn active_valid(&mut self) -> ACTIVE_VALID_W<HANDSHAKE_SPEC> {
        ACTIVE_VALID_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_enable(&mut self) -> BUFFER_ENABLE_W<HANDSHAKE_SPEC> {
        BUFFER_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn macenable(&mut self) -> MACENABLE_W<HANDSHAKE_SPEC> {
        MACENABLE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn active_enable(&mut self) -> ACTIVE_ENABLE_W<HANDSHAKE_SPEC> {
        ACTIVE_ENABLE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pool_enable(&mut self) -> POOL_ENABLE_W<HANDSHAKE_SPEC> {
        POOL_ENABLE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bias_enable(&mut self) -> BIAS_ENABLE_W<HANDSHAKE_SPEC> {
        BIAS_ENABLE_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_enable(&mut self) -> BYPASS_ENABLE_W<HANDSHAKE_SPEC> {
        BYPASS_ENABLE_W::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`handshake::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`handshake::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HANDSHAKE_SPEC;
impl crate::RegisterSpec for HANDSHAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`handshake::R`](R) reader structure"]
impl crate::Readable for HANDSHAKE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`handshake::W`](W) writer structure"]
impl crate::Writable for HANDSHAKE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets handshake to value 0"]
impl crate::Resettable for HANDSHAKE_SPEC {
    const RESET_VALUE: u32 = 0;
}
