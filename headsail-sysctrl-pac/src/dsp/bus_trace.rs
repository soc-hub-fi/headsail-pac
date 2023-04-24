#[doc = r"Register block"]
#[repr(C)]
pub struct BUS_TRACE {
    #[doc = "0x00 - "]
    pub bus_trace0: BUS_TRACE0,
    #[doc = "0x04 - "]
    pub bus_trace1: BUS_TRACE1,
    #[doc = "0x08 - "]
    pub bus_trace2: BUS_TRACE2,
    #[doc = "0x0c - "]
    pub bus_trace3: BUS_TRACE3,
    #[doc = "0x10 - "]
    pub bus_trace4: BUS_TRACE4,
}
#[doc = "bus_trace0 (r) register accessor: an alias for `Reg<BUS_TRACE0_SPEC>`"]
pub type BUS_TRACE0 = crate::Reg<bus_trace0::BUS_TRACE0_SPEC>;
#[doc = ""]
pub mod bus_trace0;
#[doc = "bus_trace1 (r) register accessor: an alias for `Reg<BUS_TRACE1_SPEC>`"]
pub type BUS_TRACE1 = crate::Reg<bus_trace1::BUS_TRACE1_SPEC>;
#[doc = ""]
pub mod bus_trace1;
#[doc = "bus_trace2 (r) register accessor: an alias for `Reg<BUS_TRACE2_SPEC>`"]
pub type BUS_TRACE2 = crate::Reg<bus_trace2::BUS_TRACE2_SPEC>;
#[doc = ""]
pub mod bus_trace2;
#[doc = "bus_trace3 (r) register accessor: an alias for `Reg<BUS_TRACE3_SPEC>`"]
pub type BUS_TRACE3 = crate::Reg<bus_trace3::BUS_TRACE3_SPEC>;
#[doc = ""]
pub mod bus_trace3;
#[doc = "bus_trace4 (r) register accessor: an alias for `Reg<BUS_TRACE4_SPEC>`"]
pub type BUS_TRACE4 = crate::Reg<bus_trace4::BUS_TRACE4_SPEC>;
#[doc = ""]
pub mod bus_trace4;
