#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ddr_refresh_ack: DDR_REFRESH_ACK,
    ddr_init_done: DDR_INIT_DONE,
    axi_enabled: AXI_ENABLED,
    t_init: T_INIT,
    t_arf: T_ARF,
    t_mrd: T_MRD,
    t_ras: T_RAS,
    t_rc: T_RC,
    t_rcd: T_RCD,
    t_rfc: T_RFC,
    t_rp: T_RP,
    t_rrd: T_RRD,
    t_wr: T_WR,
    t_wtr: T_WTR,
    c_burst_len: C_BURST_LEN,
    c_burst_cycles: C_BURST_CYCLES,
    c_rd_lat: C_RD_LAT,
    c_wr_lat: C_WR_LAT,
    c_col_shift: C_COL_SHIFT,
    c_col_mask: C_COL_MASK,
    c_row_shift: C_ROW_SHIFT,
    c_row_mask: C_ROW_MASK,
    c_bank_shift: C_BANK_SHIFT,
    c_bank_mask: C_BANK_MASK,
    c_offset: C_OFFSET,
    c_start_addr: C_START_ADDR,
    c_end_addr: C_END_ADDR,
    d_wrseparation: D_WRSEPARATION,
    d_wrlatcorr: D_WRLATCORR,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn ddr_refresh_ack(&self) -> &DDR_REFRESH_ACK {
        &self.ddr_refresh_ack
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn ddr_init_done(&self) -> &DDR_INIT_DONE {
        &self.ddr_init_done
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn axi_enabled(&self) -> &AXI_ENABLED {
        &self.axi_enabled
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn t_init(&self) -> &T_INIT {
        &self.t_init
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn t_arf(&self) -> &T_ARF {
        &self.t_arf
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn t_mrd(&self) -> &T_MRD {
        &self.t_mrd
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn t_ras(&self) -> &T_RAS {
        &self.t_ras
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn t_rc(&self) -> &T_RC {
        &self.t_rc
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn t_rcd(&self) -> &T_RCD {
        &self.t_rcd
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn t_rfc(&self) -> &T_RFC {
        &self.t_rfc
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn t_rp(&self) -> &T_RP {
        &self.t_rp
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn t_rrd(&self) -> &T_RRD {
        &self.t_rrd
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn t_wr(&self) -> &T_WR {
        &self.t_wr
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn t_wtr(&self) -> &T_WTR {
        &self.t_wtr
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn c_burst_len(&self) -> &C_BURST_LEN {
        &self.c_burst_len
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn c_burst_cycles(&self) -> &C_BURST_CYCLES {
        &self.c_burst_cycles
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn c_rd_lat(&self) -> &C_RD_LAT {
        &self.c_rd_lat
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn c_wr_lat(&self) -> &C_WR_LAT {
        &self.c_wr_lat
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn c_col_shift(&self) -> &C_COL_SHIFT {
        &self.c_col_shift
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn c_col_mask(&self) -> &C_COL_MASK {
        &self.c_col_mask
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn c_row_shift(&self) -> &C_ROW_SHIFT {
        &self.c_row_shift
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn c_row_mask(&self) -> &C_ROW_MASK {
        &self.c_row_mask
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn c_bank_shift(&self) -> &C_BANK_SHIFT {
        &self.c_bank_shift
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn c_bank_mask(&self) -> &C_BANK_MASK {
        &self.c_bank_mask
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn c_offset(&self) -> &C_OFFSET {
        &self.c_offset
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn c_start_addr(&self) -> &C_START_ADDR {
        &self.c_start_addr
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn c_end_addr(&self) -> &C_END_ADDR {
        &self.c_end_addr
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn d_wrseparation(&self) -> &D_WRSEPARATION {
        &self.d_wrseparation
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn d_wrlatcorr(&self) -> &D_WRLATCORR {
        &self.d_wrlatcorr
    }
}
#[doc = "ddr_refresh_ack (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_refresh_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_refresh_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_refresh_ack`]
module"]
#[doc(alias = "ddr_refresh_ack")]
pub type DDR_REFRESH_ACK = crate::Reg<ddr_refresh_ack::DDR_REFRESH_ACK_SPEC>;
#[doc = ""]
pub mod ddr_refresh_ack;
#[doc = "ddr_init_done (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_init_done::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_init_done::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_init_done`]
module"]
#[doc(alias = "ddr_init_done")]
pub type DDR_INIT_DONE = crate::Reg<ddr_init_done::DDR_INIT_DONE_SPEC>;
#[doc = ""]
pub mod ddr_init_done;
#[doc = "axi_enabled (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_enabled::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_enabled::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_enabled`]
module"]
#[doc(alias = "axi_enabled")]
pub type AXI_ENABLED = crate::Reg<axi_enabled::AXI_ENABLED_SPEC>;
#[doc = ""]
pub mod axi_enabled;
#[doc = "t_init (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_init`]
module"]
#[doc(alias = "t_init")]
pub type T_INIT = crate::Reg<t_init::T_INIT_SPEC>;
#[doc = ""]
pub mod t_init;
#[doc = "t_arf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_arf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_arf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_arf`]
module"]
#[doc(alias = "t_arf")]
pub type T_ARF = crate::Reg<t_arf::T_ARF_SPEC>;
#[doc = ""]
pub mod t_arf;
#[doc = "t_mrd (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_mrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_mrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_mrd`]
module"]
#[doc(alias = "t_mrd")]
pub type T_MRD = crate::Reg<t_mrd::T_MRD_SPEC>;
#[doc = ""]
pub mod t_mrd;
#[doc = "t_ras (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_ras::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_ras::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_ras`]
module"]
#[doc(alias = "t_ras")]
pub type T_RAS = crate::Reg<t_ras::T_RAS_SPEC>;
#[doc = ""]
pub mod t_ras;
#[doc = "t_rc (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rc`]
module"]
#[doc(alias = "t_rc")]
pub type T_RC = crate::Reg<t_rc::T_RC_SPEC>;
#[doc = ""]
pub mod t_rc;
#[doc = "t_rcd (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rcd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rcd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rcd`]
module"]
#[doc(alias = "t_rcd")]
pub type T_RCD = crate::Reg<t_rcd::T_RCD_SPEC>;
#[doc = ""]
pub mod t_rcd;
#[doc = "t_rfc (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rfc`]
module"]
#[doc(alias = "t_rfc")]
pub type T_RFC = crate::Reg<t_rfc::T_RFC_SPEC>;
#[doc = ""]
pub mod t_rfc;
#[doc = "t_rp (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rp`]
module"]
#[doc(alias = "t_rp")]
pub type T_RP = crate::Reg<t_rp::T_RP_SPEC>;
#[doc = ""]
pub mod t_rp;
#[doc = "t_rrd (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rrd`]
module"]
#[doc(alias = "t_rrd")]
pub type T_RRD = crate::Reg<t_rrd::T_RRD_SPEC>;
#[doc = ""]
pub mod t_rrd;
#[doc = "t_wr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_wr`]
module"]
#[doc(alias = "t_wr")]
pub type T_WR = crate::Reg<t_wr::T_WR_SPEC>;
#[doc = ""]
pub mod t_wr;
#[doc = "t_wtr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_wtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_wtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_wtr`]
module"]
#[doc(alias = "t_wtr")]
pub type T_WTR = crate::Reg<t_wtr::T_WTR_SPEC>;
#[doc = ""]
pub mod t_wtr;
#[doc = "c_burst_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_burst_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_burst_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_burst_len`]
module"]
#[doc(alias = "c_burst_len")]
pub type C_BURST_LEN = crate::Reg<c_burst_len::C_BURST_LEN_SPEC>;
#[doc = ""]
pub mod c_burst_len;
#[doc = "c_burst_cycles (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_burst_cycles::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_burst_cycles::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_burst_cycles`]
module"]
#[doc(alias = "c_burst_cycles")]
pub type C_BURST_CYCLES = crate::Reg<c_burst_cycles::C_BURST_CYCLES_SPEC>;
#[doc = ""]
pub mod c_burst_cycles;
#[doc = "c_rd_lat (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_rd_lat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_rd_lat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_rd_lat`]
module"]
#[doc(alias = "c_rd_lat")]
pub type C_RD_LAT = crate::Reg<c_rd_lat::C_RD_LAT_SPEC>;
#[doc = ""]
pub mod c_rd_lat;
#[doc = "c_wr_lat (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_wr_lat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_wr_lat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_wr_lat`]
module"]
#[doc(alias = "c_wr_lat")]
pub type C_WR_LAT = crate::Reg<c_wr_lat::C_WR_LAT_SPEC>;
#[doc = ""]
pub mod c_wr_lat;
#[doc = "c_col_shift (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_col_shift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_col_shift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_col_shift`]
module"]
#[doc(alias = "c_col_shift")]
pub type C_COL_SHIFT = crate::Reg<c_col_shift::C_COL_SHIFT_SPEC>;
#[doc = ""]
pub mod c_col_shift;
#[doc = "c_col_mask (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_col_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_col_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_col_mask`]
module"]
#[doc(alias = "c_col_mask")]
pub type C_COL_MASK = crate::Reg<c_col_mask::C_COL_MASK_SPEC>;
#[doc = ""]
pub mod c_col_mask;
#[doc = "c_row_shift (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_row_shift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_row_shift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_row_shift`]
module"]
#[doc(alias = "c_row_shift")]
pub type C_ROW_SHIFT = crate::Reg<c_row_shift::C_ROW_SHIFT_SPEC>;
#[doc = ""]
pub mod c_row_shift;
#[doc = "c_row_mask (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_row_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_row_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_row_mask`]
module"]
#[doc(alias = "c_row_mask")]
pub type C_ROW_MASK = crate::Reg<c_row_mask::C_ROW_MASK_SPEC>;
#[doc = ""]
pub mod c_row_mask;
#[doc = "c_bank_shift (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_bank_shift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_bank_shift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_bank_shift`]
module"]
#[doc(alias = "c_bank_shift")]
pub type C_BANK_SHIFT = crate::Reg<c_bank_shift::C_BANK_SHIFT_SPEC>;
#[doc = ""]
pub mod c_bank_shift;
#[doc = "c_bank_mask (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_bank_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_bank_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_bank_mask`]
module"]
#[doc(alias = "c_bank_mask")]
pub type C_BANK_MASK = crate::Reg<c_bank_mask::C_BANK_MASK_SPEC>;
#[doc = ""]
pub mod c_bank_mask;
#[doc = "c_offset (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_offset`]
module"]
#[doc(alias = "c_offset")]
pub type C_OFFSET = crate::Reg<c_offset::C_OFFSET_SPEC>;
#[doc = ""]
pub mod c_offset;
#[doc = "c_start_addr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_start_addr`]
module"]
#[doc(alias = "c_start_addr")]
pub type C_START_ADDR = crate::Reg<c_start_addr::C_START_ADDR_SPEC>;
#[doc = ""]
pub mod c_start_addr;
#[doc = "c_end_addr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_end_addr`]
module"]
#[doc(alias = "c_end_addr")]
pub type C_END_ADDR = crate::Reg<c_end_addr::C_END_ADDR_SPEC>;
#[doc = ""]
pub mod c_end_addr;
#[doc = "d_wrseparation (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_wrseparation::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_wrseparation::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_wrseparation`]
module"]
#[doc(alias = "d_wrseparation")]
pub type D_WRSEPARATION = crate::Reg<d_wrseparation::D_WRSEPARATION_SPEC>;
#[doc = ""]
pub mod d_wrseparation;
#[doc = "d_wrlatcorr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_wrlatcorr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_wrlatcorr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_wrlatcorr`]
module"]
#[doc(alias = "d_wrlatcorr")]
pub type D_WRLATCORR = crate::Reg<d_wrlatcorr::D_WRLATCORR_SPEC>;
#[doc = ""]
pub mod d_wrlatcorr;
