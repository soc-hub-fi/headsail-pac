#[doc = "Register `handshake` reader"]
pub type R = crate::R<HandshakeSpec>;
#[doc = "Register `handshake` writer"]
pub type W = crate::W<HandshakeSpec>;
#[doc = "Field `buffer_valid` reader - "]
pub type BufferValidR = crate::BitReader;
#[doc = "Field `buffer_valid` writer - "]
pub type BufferValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mac_valid` reader - "]
pub type MacValidR = crate::BitReader;
#[doc = "Field `mac_valid` writer - "]
pub type MacValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pool_valid` reader - "]
pub type PoolValidR = crate::BitReader;
#[doc = "Field `pool_valid` writer - "]
pub type PoolValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `active_valid` reader - "]
pub type ActiveValidR = crate::BitReader;
#[doc = "Field `active_valid` writer - "]
pub type ActiveValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bufferEnable` reader - "]
pub type BufferEnableR = crate::BitReader;
#[doc = "Field `bufferEnable` writer - "]
pub type BufferEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACEnable` reader - "]
pub type MacenableR = crate::BitReader;
#[doc = "Field `MACEnable` writer - "]
pub type MacenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `activeEnable` reader - "]
pub type ActiveEnableR = crate::BitReader;
#[doc = "Field `activeEnable` writer - "]
pub type ActiveEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `poolEnable` reader - "]
pub type PoolEnableR = crate::BitReader;
#[doc = "Field `poolEnable` writer - "]
pub type PoolEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `biasEnable` reader - "]
pub type BiasEnableR = crate::BitReader;
#[doc = "Field `biasEnable` writer - "]
pub type BiasEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bypassEnable` reader - "]
pub type BypassEnableR = crate::BitReader;
#[doc = "Field `bypassEnable` writer - "]
pub type BypassEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn buffer_valid(&self) -> BufferValidR {
        BufferValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mac_valid(&self) -> MacValidR {
        MacValidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pool_valid(&self) -> PoolValidR {
        PoolValidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn active_valid(&self) -> ActiveValidR {
        ActiveValidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn buffer_enable(&self) -> BufferEnableR {
        BufferEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn macenable(&self) -> MacenableR {
        MacenableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn active_enable(&self) -> ActiveEnableR {
        ActiveEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pool_enable(&self) -> PoolEnableR {
        PoolEnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bias_enable(&self) -> BiasEnableR {
        BiasEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bypass_enable(&self) -> BypassEnableR {
        BypassEnableR::new(((self.bits >> 9) & 1) != 0)
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
    pub fn buffer_valid(&mut self) -> BufferValidW<HandshakeSpec> {
        BufferValidW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mac_valid(&mut self) -> MacValidW<HandshakeSpec> {
        MacValidW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pool_valid(&mut self) -> PoolValidW<HandshakeSpec> {
        PoolValidW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn active_valid(&mut self) -> ActiveValidW<HandshakeSpec> {
        ActiveValidW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_enable(&mut self) -> BufferEnableW<HandshakeSpec> {
        BufferEnableW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn macenable(&mut self) -> MacenableW<HandshakeSpec> {
        MacenableW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn active_enable(&mut self) -> ActiveEnableW<HandshakeSpec> {
        ActiveEnableW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pool_enable(&mut self) -> PoolEnableW<HandshakeSpec> {
        PoolEnableW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bias_enable(&mut self) -> BiasEnableW<HandshakeSpec> {
        BiasEnableW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_enable(&mut self) -> BypassEnableW<HandshakeSpec> {
        BypassEnableW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`handshake::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`handshake::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HandshakeSpec;
impl crate::RegisterSpec for HandshakeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`handshake::R`](R) reader structure"]
impl crate::Readable for HandshakeSpec {}
#[doc = "`write(|w| ..)` method takes [`handshake::W`](W) writer structure"]
impl crate::Writable for HandshakeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets handshake to value 0"]
impl crate::Resettable for HandshakeSpec {
    const RESET_VALUE: u32 = 0;
}
