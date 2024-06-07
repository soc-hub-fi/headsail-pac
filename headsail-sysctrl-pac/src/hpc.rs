#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0005_0000],
    timer: Timer,
    _reserved1: [u8; 0x1fc4],
    l2_config: L2Config,
    _reserved2: [u8; 0x1ff8],
    cluster_config: ClusterConfig,
    _reserved3: [u8; 0xbda0],
    clint: Clint,
    _reserved4: [u8; 0x0001_4000],
    plic: Plic,
}
impl RegisterBlock {
    #[doc = "0x50000..0x5003c - timer"]
    #[inline(always)]
    pub const fn timer(&self) -> &Timer {
        &self.timer
    }
    #[doc = "0x52000..0x52008 - l2_config"]
    #[inline(always)]
    pub const fn l2_config(&self) -> &L2Config {
        &self.l2_config
    }
    #[doc = "0x54000..0x54260 - cluster_config"]
    #[inline(always)]
    pub const fn cluster_config(&self) -> &ClusterConfig {
        &self.cluster_config
    }
    #[doc = "0x60000..0x6c000 - clint"]
    #[inline(always)]
    pub const fn clint(&self) -> &Clint {
        &self.clint
    }
    #[doc = "0x80000..0x8b008 - plic"]
    #[inline(always)]
    pub const fn plic(&self) -> &Plic {
        &self.plic
    }
}
#[doc = "clint"]
pub use self::clint::Clint;
#[doc = r"Cluster"]
#[doc = "clint"]
pub mod clint;
#[doc = "timer"]
pub use self::timer::Timer;
#[doc = r"Cluster"]
#[doc = "timer"]
pub mod timer;
#[doc = "l2_config"]
pub use self::l2_config::L2Config;
#[doc = r"Cluster"]
#[doc = "l2_config"]
pub mod l2_config;
#[doc = "plic"]
pub use self::plic::Plic;
#[doc = r"Cluster"]
#[doc = "plic"]
pub mod plic;
#[doc = "cluster_config"]
pub use self::cluster_config::ClusterConfig;
#[doc = r"Cluster"]
#[doc = "cluster_config"]
pub mod cluster_config;
