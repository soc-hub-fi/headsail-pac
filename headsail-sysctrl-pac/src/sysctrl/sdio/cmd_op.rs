#[doc = "Register `CMD_OP` writer"]
pub type W = crate::W<CmdOpSpec>;
#[doc = "Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RspType {
    #[doc = "0: `0`"]
    RspTypeNull = 0,
    #[doc = "1: `1`"]
    RspType48Crc = 1,
    #[doc = "2: `10`"]
    RspType48Nocrc = 2,
    #[doc = "3: `11`"]
    RspType136 = 3,
    #[doc = "4: `100`"]
    RspType48Bsy = 4,
}
impl From<RspType> for u8 {
    #[inline(always)]
    fn from(variant: RspType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RspType {
    type Ux = u8;
}
impl crate::IsEnum for RspType {}
#[doc = "Field `RSP_TYPE` writer - Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY"]
pub type RspTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3, RspType>;
impl<'a, REG> RspTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rsp_type_null(self) -> &'a mut crate::W<REG> {
        self.variant(RspType::RspTypeNull)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rsp_type_48_crc(self) -> &'a mut crate::W<REG> {
        self.variant(RspType::RspType48Crc)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rsp_type_48_nocrc(self) -> &'a mut crate::W<REG> {
        self.variant(RspType::RspType48Nocrc)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rsp_type_136(self) -> &'a mut crate::W<REG> {
        self.variant(RspType::RspType136)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rsp_type_48_bsy(self) -> &'a mut crate::W<REG> {
        self.variant(RspType::RspType48Bsy)
    }
}
#[doc = "Field `CMD_OPCODE` writer - "]
pub type CmdOpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<CmdOpSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:2 - Values for RSP_TYPE: 3'b000: RSP_TYPE_NULL 3'b001: RSP_TYPE_48_CRC 3'b010: RSP_TYPE_48_NOCRC 3'b011: RSP_TYPE_136 3'b100: RSP_TYPE_48_BSY"]
    #[inline(always)]
    #[must_use]
    pub fn rsp_type(&mut self) -> RspTypeW<CmdOpSpec> {
        RspTypeW::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_opcode(&mut self) -> CmdOpcodeW<CmdOpSpec> {
        CmdOpcodeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_op::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdOpSpec;
impl crate::RegisterSpec for CmdOpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd_op::W`](W) writer structure"]
impl crate::Writable for CmdOpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_OP to value 0"]
impl crate::Resettable for CmdOpSpec {
    const RESET_VALUE: u32 = 0;
}
