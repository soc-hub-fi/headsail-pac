#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x40],
    channel0: Channel0,
    _reserved2: [u8; 0x0c],
    channel1: Channel1,
    _reserved3: [u8; 0x0c],
    channel2: Channel2,
    _reserved4: [u8; 0x0c],
    channel3: Channel3,
}
impl RegisterBlock {
    #[doc = "0x00 - ctrl"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x40..0xb4 - channel0"]
    #[inline(always)]
    pub const fn channel0(&self) -> &Channel0 {
        &self.channel0
    }
    #[doc = "0xc0..0x134 - channel1"]
    #[inline(always)]
    pub const fn channel1(&self) -> &Channel1 {
        &self.channel1
    }
    #[doc = "0x140..0x1b4 - channel2"]
    #[inline(always)]
    pub const fn channel2(&self) -> &Channel2 {
        &self.channel2
    }
    #[doc = "0x1c0..0x234 - channel3"]
    #[inline(always)]
    pub const fn channel3(&self) -> &Channel3 {
        &self.channel3
    }
}
#[doc = "ctrl"]
pub use self::ctrl::Ctrl;
#[doc = r"Cluster"]
#[doc = "ctrl"]
pub mod ctrl;
#[doc = "channel0"]
pub use self::channel0::Channel0;
#[doc = r"Cluster"]
#[doc = "channel0"]
pub mod channel0;
#[doc = "channel1"]
pub use self::channel1::Channel1;
#[doc = r"Cluster"]
#[doc = "channel1"]
pub mod channel1;
#[doc = "channel3"]
pub use self::channel3::Channel3;
#[doc = r"Cluster"]
#[doc = "channel3"]
pub mod channel3;
#[doc = "channel2"]
pub use self::channel2::Channel2;
#[doc = r"Cluster"]
#[doc = "channel2"]
pub mod channel2;
