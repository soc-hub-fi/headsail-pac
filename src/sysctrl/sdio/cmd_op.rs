#[doc = "Register `CMD_OP` writer"]
pub struct W(crate::W<CMD_OP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_OP_SPEC>;
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
impl From<crate::W<CMD_OP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_OP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSP_TYPE_AW {
    #[doc = "0: `0`"]
    RSP_TYPE_NULL = 0,
    #[doc = "1: `1`"]
    RSP_TYPE_48_CRC = 1,
    #[doc = "2: `10`"]
    RSP_TYPE_48_NOCRC = 2,
    #[doc = "3: `11`"]
    RSP_TYPE_136 = 3,
    #[doc = "4: `100`"]
    RSP_TYPE_48_BSY = 4,
}
impl From<RSP_TYPE_AW> for u8 {
    #[inline(always)]
    fn from(variant: RSP_TYPE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RSP_TYPE` writer - Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY"]
pub type RSP_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMD_OP_SPEC, u8, RSP_TYPE_AW, 3, O>;
impl<'a, const O: u8> RSP_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rsp_type_null(self) -> &'a mut W {
        self.variant(RSP_TYPE_AW::RSP_TYPE_NULL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rsp_type_48_crc(self) -> &'a mut W {
        self.variant(RSP_TYPE_AW::RSP_TYPE_48_CRC)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rsp_type_48_nocrc(self) -> &'a mut W {
        self.variant(RSP_TYPE_AW::RSP_TYPE_48_NOCRC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rsp_type_136(self) -> &'a mut W {
        self.variant(RSP_TYPE_AW::RSP_TYPE_136)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rsp_type_48_bsy(self) -> &'a mut W {
        self.variant(RSP_TYPE_AW::RSP_TYPE_48_BSY)
    }
}
#[doc = "Field `CMD_OPCODE` writer - "]
pub type CMD_OPCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_OP_SPEC, u8, u8, 6, O>;
impl W {
    #[doc = "Bits 0:2 - Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY"]
    #[inline(always)]
    #[must_use]
    pub fn rsp_type(&mut self) -> RSP_TYPE_W<0> {
        RSP_TYPE_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_opcode(&mut self) -> CMD_OPCODE_W<8> {
        CMD_OPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_op](index.html) module"]
pub struct CMD_OP_SPEC;
impl crate::RegisterSpec for CMD_OP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd_op::W](W) writer structure"]
impl crate::Writable for CMD_OP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_OP to value 0"]
impl crate::Resettable for CMD_OP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
