#[doc = "Register `t_rrd` reader"]
pub type R = crate::R<TRrdSpec>;
#[doc = "Register `t_rrd` writer"]
pub type W = crate::W<TRrdSpec>;
#[doc = "Field `t_rrd` reader - "]
pub type TRrdR = crate::FieldReader<u32>;
#[doc = "Field `t_rrd` writer - "]
pub type TRrdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_rrd(&self) -> TRrdR {
        TRrdR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_rrd")
            .field("t_rrd", &self.t_rrd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_rrd(&mut self) -> TRrdW<TRrdSpec> {
        TRrdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRrdSpec;
impl crate::RegisterSpec for TRrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_rrd::R`](R) reader structure"]
impl crate::Readable for TRrdSpec {}
#[doc = "`write(|w| ..)` method takes [`t_rrd::W`](W) writer structure"]
impl crate::Writable for TRrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_rrd to value 0x01"]
impl crate::Resettable for TRrdSpec {
    const RESET_VALUE: u32 = 0x01;
}
