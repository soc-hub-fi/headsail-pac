#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - dma_status"]
    pub dma_status: DMA_STATUS,
    #[doc = "0x40..0xb4 - channel0"]
    pub channel0: CHANNEL0,
    _reserved2: [u8; 0x0c],
    #[doc = "0xc0..0x134 - channel1"]
    pub channel1: CHANNEL1,
    _reserved3: [u8; 0x0c],
    #[doc = "0x140..0x1b4 - channel2"]
    pub channel2: CHANNEL2,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c0..0x234 - channel3"]
    pub channel3: CHANNEL3,
}
#[doc = "dma_status"]
pub use self::dma_status::DMA_STATUS;
#[doc = r"Cluster"]
#[doc = "dma_status"]
pub mod dma_status;
#[doc = "channel0"]
pub use self::channel0::CHANNEL0;
#[doc = r"Cluster"]
#[doc = "channel0"]
pub mod channel0;
#[doc = "channel1"]
pub use self::channel1::CHANNEL1;
#[doc = r"Cluster"]
#[doc = "channel1"]
pub mod channel1;
#[doc = "channel3"]
pub use self::channel3::CHANNEL3;
#[doc = r"Cluster"]
#[doc = "channel3"]
pub mod channel3;
#[doc = "channel2"]
pub use self::channel2::CHANNEL2;
#[doc = r"Cluster"]
#[doc = "channel2"]
pub mod channel2;
