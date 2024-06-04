#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: STATUS,
    _reserved1: [u8; 0x28],
    bus_trace: BUS_TRACE,
    _reserved2: [u8; 0x01ac],
    ctrl: CTRL,
    _reserved3: [u8; 0xe8],
    processor_info: PROCESSOR_INFO,
}
impl RegisterBlock {
    #[doc = "0x00..0x18 - status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x40..0x54 - bus_trace"]
    #[inline(always)]
    pub const fn bus_trace(&self) -> &BUS_TRACE {
        &self.bus_trace
    }
    #[doc = "0x200..0x218 - ctrl"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x300..0x354 - processor_info"]
    #[inline(always)]
    pub const fn processor_info(&self) -> &PROCESSOR_INFO {
        &self.processor_info
    }
}
#[doc = "status"]
pub use self::status::STATUS;
#[doc = r"Cluster"]
#[doc = "status"]
pub mod status;
#[doc = "ctrl"]
pub use self::ctrl::CTRL;
#[doc = r"Cluster"]
#[doc = "ctrl"]
pub mod ctrl;
#[doc = "processor_info"]
pub use self::processor_info::PROCESSOR_INFO;
#[doc = r"Cluster"]
#[doc = "processor_info"]
pub mod processor_info;
#[doc = "bus_trace"]
pub use self::bus_trace::BUS_TRACE;
#[doc = r"Cluster"]
#[doc = "bus_trace"]
pub mod bus_trace;
