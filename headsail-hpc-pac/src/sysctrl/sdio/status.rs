#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `EOT` reader - "]
pub type EotR = crate::BitReader;
#[doc = "Field `EOT` writer - "]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - "]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - "]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_ERR` reader - "]
pub type CardErrR = crate::FieldReader<u16>;
#[doc = "Field `CARD_ERR` writer - "]
pub type CardErrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VHS` reader - Note: Is reset value correct?"]
pub type VhsR = crate::FieldReader;
#[doc = "Field `VHS` writer - Note: Is reset value correct?"]
pub type VhsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Status` reader - "]
pub type StatusR = crate::FieldReader<u16>;
#[doc = "Field `Status` writer - "]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn card_err(&self) -> CardErrR {
        CardErrR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - Note: Is reset value correct?"]
    #[inline(always)]
    pub fn vhs(&self) -> VhsR {
        VhsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("eot", &self.eot())
            .field("err", &self.err())
            .field("card_err", &self.card_err())
            .field("vhs", &self.vhs())
            .field("status", &self.status())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EotW<StatusSpec> {
        EotW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<StatusSpec> {
        ErrW::new(self, 1)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    #[must_use]
    pub fn card_err(&mut self) -> CardErrW<StatusSpec> {
        CardErrW::new(self, 2)
    }
    #[doc = "Bits 12:15 - Note: Is reset value correct?"]
    #[inline(always)]
    #[must_use]
    pub fn vhs(&mut self) -> VhsW<StatusSpec> {
        VhsW::new(self, 12)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<StatusSpec> {
        StatusW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x10"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x10;
}
