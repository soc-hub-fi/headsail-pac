#[doc = "Register `CMD_OP` writer"]
pub type W = crate::W<CMD_OP_SPEC>;
#[doc = "Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSP_TYPE_A {
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
impl From<RSP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: RSP_TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSP_TYPE_A {
    type Ux = u8;
}
impl crate::IsEnum for RSP_TYPE_A {}
#[doc = "Field `RSP_TYPE` writer - Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY"]
pub type RSP_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RSP_TYPE_A>;
impl<'a, REG> RSP_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rsp_type_null(self) -> &'a mut crate::W<REG> {
        self.variant(RSP_TYPE_A::RSP_TYPE_NULL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rsp_type_48_crc(self) -> &'a mut crate::W<REG> {
        self.variant(RSP_TYPE_A::RSP_TYPE_48_CRC)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rsp_type_48_nocrc(self) -> &'a mut crate::W<REG> {
        self.variant(RSP_TYPE_A::RSP_TYPE_48_NOCRC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rsp_type_136(self) -> &'a mut crate::W<REG> {
        self.variant(RSP_TYPE_A::RSP_TYPE_136)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rsp_type_48_bsy(self) -> &'a mut crate::W<REG> {
        self.variant(RSP_TYPE_A::RSP_TYPE_48_BSY)
    }
}
#[doc = "Field `CMD_OPCODE` writer - "]
pub type CMD_OPCODE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_OP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:2 - Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY"]
    #[inline(always)]
    #[must_use]
    pub fn rsp_type(&mut self) -> RSP_TYPE_W<CMD_OP_SPEC> {
        RSP_TYPE_W::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_opcode(&mut self) -> CMD_OPCODE_W<CMD_OP_SPEC> {
        CMD_OPCODE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_op::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_OP_SPEC;
impl crate::RegisterSpec for CMD_OP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd_op::W`](W) writer structure"]
impl crate::Writable for CMD_OP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_OP to value 0"]
impl crate::Resettable for CMD_OP_SPEC {
    const RESET_VALUE: u32 = 0;
}
