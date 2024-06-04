#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "bus_trace"]
#[doc(alias = "bus_trace")]
pub struct BUS_TRACE {
    bus_trace0: BUS_TRACE0,
    bus_trace1: BUS_TRACE1,
    bus_trace2: BUS_TRACE2,
    bus_trace3: BUS_TRACE3,
    bus_trace4: BUS_TRACE4,
}
impl BUS_TRACE {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn bus_trace0(&self) -> &BUS_TRACE0 {
        &self.bus_trace0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn bus_trace1(&self) -> &BUS_TRACE1 {
        &self.bus_trace1
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn bus_trace2(&self) -> &BUS_TRACE2 {
        &self.bus_trace2
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn bus_trace3(&self) -> &BUS_TRACE3 {
        &self.bus_trace3
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn bus_trace4(&self) -> &BUS_TRACE4 {
        &self.bus_trace4
    }
}
#[doc = "bus_trace0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_trace0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_trace0`]
module"]
#[doc(alias = "bus_trace0")]
pub type BUS_TRACE0 = crate::Reg<bus_trace0::BUS_TRACE0_SPEC>;
#[doc = ""]
pub mod bus_trace0;
#[doc = "bus_trace1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_trace1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_trace1`]
module"]
#[doc(alias = "bus_trace1")]
pub type BUS_TRACE1 = crate::Reg<bus_trace1::BUS_TRACE1_SPEC>;
#[doc = ""]
pub mod bus_trace1;
#[doc = "bus_trace2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_trace2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_trace2`]
module"]
#[doc(alias = "bus_trace2")]
pub type BUS_TRACE2 = crate::Reg<bus_trace2::BUS_TRACE2_SPEC>;
#[doc = ""]
pub mod bus_trace2;
#[doc = "bus_trace3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_trace3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_trace3`]
module"]
#[doc(alias = "bus_trace3")]
pub type BUS_TRACE3 = crate::Reg<bus_trace3::BUS_TRACE3_SPEC>;
#[doc = ""]
pub mod bus_trace3;
#[doc = "bus_trace4 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_trace4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_trace4`]
module"]
#[doc(alias = "bus_trace4")]
pub type BUS_TRACE4 = crate::Reg<bus_trace4::BUS_TRACE4_SPEC>;
#[doc = ""]
pub mod bus_trace4;
