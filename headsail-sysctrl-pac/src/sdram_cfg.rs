#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ddr_refresh_ack: DdrRefreshAck,
    ddr_init_done: DdrInitDone,
    axi_enabled: AxiEnabled,
    t_init: TInit,
    t_arf: TArf,
    t_mrd: TMrd,
    t_ras: TRas,
    t_rc: TRc,
    t_rcd: TRcd,
    t_rfc: TRfc,
    t_rp: TRp,
    t_rrd: TRrd,
    t_wr: TWr,
    t_wtr: TWtr,
    c_burst_len: CBurstLen,
    c_burst_cycles: CBurstCycles,
    c_rd_lat: CRdLat,
    c_wr_lat: CWrLat,
    c_col_shift: CColShift,
    c_col_mask: CColMask,
    c_row_shift: CRowShift,
    c_row_mask: CRowMask,
    c_bank_shift: CBankShift,
    c_bank_mask: CBankMask,
    c_offset: COffset,
    c_start_addr: CStartAddr,
    c_end_addr: CEndAddr,
    d_wrseparation: DWrseparation,
    d_wrlatcorr: DWrlatcorr,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn ddr_refresh_ack(&self) -> &DdrRefreshAck {
        &self.ddr_refresh_ack
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn ddr_init_done(&self) -> &DdrInitDone {
        &self.ddr_init_done
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn axi_enabled(&self) -> &AxiEnabled {
        &self.axi_enabled
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn t_init(&self) -> &TInit {
        &self.t_init
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn t_arf(&self) -> &TArf {
        &self.t_arf
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn t_mrd(&self) -> &TMrd {
        &self.t_mrd
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn t_ras(&self) -> &TRas {
        &self.t_ras
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn t_rc(&self) -> &TRc {
        &self.t_rc
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn t_rcd(&self) -> &TRcd {
        &self.t_rcd
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn t_rfc(&self) -> &TRfc {
        &self.t_rfc
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn t_rp(&self) -> &TRp {
        &self.t_rp
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn t_rrd(&self) -> &TRrd {
        &self.t_rrd
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn t_wr(&self) -> &TWr {
        &self.t_wr
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn t_wtr(&self) -> &TWtr {
        &self.t_wtr
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn c_burst_len(&self) -> &CBurstLen {
        &self.c_burst_len
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn c_burst_cycles(&self) -> &CBurstCycles {
        &self.c_burst_cycles
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn c_rd_lat(&self) -> &CRdLat {
        &self.c_rd_lat
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn c_wr_lat(&self) -> &CWrLat {
        &self.c_wr_lat
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn c_col_shift(&self) -> &CColShift {
        &self.c_col_shift
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn c_col_mask(&self) -> &CColMask {
        &self.c_col_mask
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn c_row_shift(&self) -> &CRowShift {
        &self.c_row_shift
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn c_row_mask(&self) -> &CRowMask {
        &self.c_row_mask
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn c_bank_shift(&self) -> &CBankShift {
        &self.c_bank_shift
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn c_bank_mask(&self) -> &CBankMask {
        &self.c_bank_mask
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn c_offset(&self) -> &COffset {
        &self.c_offset
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn c_start_addr(&self) -> &CStartAddr {
        &self.c_start_addr
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn c_end_addr(&self) -> &CEndAddr {
        &self.c_end_addr
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn d_wrseparation(&self) -> &DWrseparation {
        &self.d_wrseparation
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn d_wrlatcorr(&self) -> &DWrlatcorr {
        &self.d_wrlatcorr
    }
}
#[doc = "ddr_refresh_ack (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_refresh_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_refresh_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_refresh_ack`]
module"]
#[doc(alias = "ddr_refresh_ack")]
pub type DdrRefreshAck = crate::Reg<ddr_refresh_ack::DdrRefreshAckSpec>;
#[doc = ""]
pub mod ddr_refresh_ack;
#[doc = "ddr_init_done (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_init_done::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_init_done::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_init_done`]
module"]
#[doc(alias = "ddr_init_done")]
pub type DdrInitDone = crate::Reg<ddr_init_done::DdrInitDoneSpec>;
#[doc = ""]
pub mod ddr_init_done;
#[doc = "axi_enabled (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_enabled::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_enabled::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_enabled`]
module"]
#[doc(alias = "axi_enabled")]
pub type AxiEnabled = crate::Reg<axi_enabled::AxiEnabledSpec>;
#[doc = ""]
pub mod axi_enabled;
#[doc = "t_init (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_init`]
module"]
#[doc(alias = "t_init")]
pub type TInit = crate::Reg<t_init::TInitSpec>;
#[doc = ""]
pub mod t_init;
#[doc = "t_arf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_arf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_arf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_arf`]
module"]
#[doc(alias = "t_arf")]
pub type TArf = crate::Reg<t_arf::TArfSpec>;
#[doc = ""]
pub mod t_arf;
#[doc = "t_mrd (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_mrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_mrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_mrd`]
module"]
#[doc(alias = "t_mrd")]
pub type TMrd = crate::Reg<t_mrd::TMrdSpec>;
#[doc = ""]
pub mod t_mrd;
#[doc = "t_ras (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_ras::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_ras::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_ras`]
module"]
#[doc(alias = "t_ras")]
pub type TRas = crate::Reg<t_ras::TRasSpec>;
#[doc = ""]
pub mod t_ras;
#[doc = "t_rc (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rc`]
module"]
#[doc(alias = "t_rc")]
pub type TRc = crate::Reg<t_rc::TRcSpec>;
#[doc = ""]
pub mod t_rc;
#[doc = "t_rcd (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rcd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rcd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rcd`]
module"]
#[doc(alias = "t_rcd")]
pub type TRcd = crate::Reg<t_rcd::TRcdSpec>;
#[doc = ""]
pub mod t_rcd;
#[doc = "t_rfc (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rfc`]
module"]
#[doc(alias = "t_rfc")]
pub type TRfc = crate::Reg<t_rfc::TRfcSpec>;
#[doc = ""]
pub mod t_rfc;
#[doc = "t_rp (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rp`]
module"]
#[doc(alias = "t_rp")]
pub type TRp = crate::Reg<t_rp::TRpSpec>;
#[doc = ""]
pub mod t_rp;
#[doc = "t_rrd (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_rrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_rrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_rrd`]
module"]
#[doc(alias = "t_rrd")]
pub type TRrd = crate::Reg<t_rrd::TRrdSpec>;
#[doc = ""]
pub mod t_rrd;
#[doc = "t_wr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_wr`]
module"]
#[doc(alias = "t_wr")]
pub type TWr = crate::Reg<t_wr::TWrSpec>;
#[doc = ""]
pub mod t_wr;
#[doc = "t_wtr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_wtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_wtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_wtr`]
module"]
#[doc(alias = "t_wtr")]
pub type TWtr = crate::Reg<t_wtr::TWtrSpec>;
#[doc = ""]
pub mod t_wtr;
#[doc = "c_burst_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_burst_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_burst_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_burst_len`]
module"]
#[doc(alias = "c_burst_len")]
pub type CBurstLen = crate::Reg<c_burst_len::CBurstLenSpec>;
#[doc = ""]
pub mod c_burst_len;
#[doc = "c_burst_cycles (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_burst_cycles::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_burst_cycles::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_burst_cycles`]
module"]
#[doc(alias = "c_burst_cycles")]
pub type CBurstCycles = crate::Reg<c_burst_cycles::CBurstCyclesSpec>;
#[doc = ""]
pub mod c_burst_cycles;
#[doc = "c_rd_lat (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_rd_lat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_rd_lat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_rd_lat`]
module"]
#[doc(alias = "c_rd_lat")]
pub type CRdLat = crate::Reg<c_rd_lat::CRdLatSpec>;
#[doc = ""]
pub mod c_rd_lat;
#[doc = "c_wr_lat (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_wr_lat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_wr_lat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_wr_lat`]
module"]
#[doc(alias = "c_wr_lat")]
pub type CWrLat = crate::Reg<c_wr_lat::CWrLatSpec>;
#[doc = ""]
pub mod c_wr_lat;
#[doc = "c_col_shift (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_col_shift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_col_shift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_col_shift`]
module"]
#[doc(alias = "c_col_shift")]
pub type CColShift = crate::Reg<c_col_shift::CColShiftSpec>;
#[doc = ""]
pub mod c_col_shift;
#[doc = "c_col_mask (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_col_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_col_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_col_mask`]
module"]
#[doc(alias = "c_col_mask")]
pub type CColMask = crate::Reg<c_col_mask::CColMaskSpec>;
#[doc = ""]
pub mod c_col_mask;
#[doc = "c_row_shift (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_row_shift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_row_shift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_row_shift`]
module"]
#[doc(alias = "c_row_shift")]
pub type CRowShift = crate::Reg<c_row_shift::CRowShiftSpec>;
#[doc = ""]
pub mod c_row_shift;
#[doc = "c_row_mask (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_row_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_row_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_row_mask`]
module"]
#[doc(alias = "c_row_mask")]
pub type CRowMask = crate::Reg<c_row_mask::CRowMaskSpec>;
#[doc = ""]
pub mod c_row_mask;
#[doc = "c_bank_shift (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_bank_shift::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_bank_shift::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_bank_shift`]
module"]
#[doc(alias = "c_bank_shift")]
pub type CBankShift = crate::Reg<c_bank_shift::CBankShiftSpec>;
#[doc = ""]
pub mod c_bank_shift;
#[doc = "c_bank_mask (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_bank_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_bank_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_bank_mask`]
module"]
#[doc(alias = "c_bank_mask")]
pub type CBankMask = crate::Reg<c_bank_mask::CBankMaskSpec>;
#[doc = ""]
pub mod c_bank_mask;
#[doc = "c_offset (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_offset`]
module"]
#[doc(alias = "c_offset")]
pub type COffset = crate::Reg<c_offset::COffsetSpec>;
#[doc = ""]
pub mod c_offset;
#[doc = "c_start_addr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_start_addr`]
module"]
#[doc(alias = "c_start_addr")]
pub type CStartAddr = crate::Reg<c_start_addr::CStartAddrSpec>;
#[doc = ""]
pub mod c_start_addr;
#[doc = "c_end_addr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c_end_addr`]
module"]
#[doc(alias = "c_end_addr")]
pub type CEndAddr = crate::Reg<c_end_addr::CEndAddrSpec>;
#[doc = ""]
pub mod c_end_addr;
#[doc = "d_wrseparation (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_wrseparation::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_wrseparation::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_wrseparation`]
module"]
#[doc(alias = "d_wrseparation")]
pub type DWrseparation = crate::Reg<d_wrseparation::DWrseparationSpec>;
#[doc = ""]
pub mod d_wrseparation;
#[doc = "d_wrlatcorr (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_wrlatcorr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_wrlatcorr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_wrlatcorr`]
module"]
#[doc(alias = "d_wrlatcorr")]
pub type DWrlatcorr = crate::Reg<d_wrlatcorr::DWrlatcorrSpec>;
#[doc = ""]
pub mod d_wrlatcorr;
