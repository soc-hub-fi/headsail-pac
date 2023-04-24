#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x18 - status"]
    pub status: STATUS,
    _reserved1: [u8; 0x28],
    #[doc = "0x40..0x54 - bus_trace"]
    pub bus_trace: BUS_TRACE,
    _reserved2: [u8; 0x01ac],
    #[doc = "0x200..0x218 - ctrl"]
    pub ctrl: CTRL,
    _reserved3: [u8; 0xe8],
    #[doc = "0x300..0x354 - processor_info"]
    pub processor_info: PROCESSOR_INFO,
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
