#[doc = r"Register block"]
#[repr(C)]
pub struct ADVANCED_TIMER {
    #[doc = "0x00 - ADV_TIMER0 command register"]
    pub t0_cmd: T0_CMD,
    #[doc = "0x04 - ADV_TIMER0 configuration register."]
    pub t0_config: T0_CONFIG,
    #[doc = "0x08 - ADV_TIMER0 threshold configuration register."]
    pub t0_threshold: T0_THRESHOLD,
    #[doc = "0x0c - ADV_TIMER0 channel 0 threshold configuration register"]
    pub t0_th_channel0: T0_TH_CHANNEL0,
    #[doc = "0x10 - ADV_TIMER0 channel 1 threshold configuration register"]
    pub t0_th_channel1: T0_TH_CHANNEL1,
    #[doc = "0x14 - ADV_TIMER0 channel 2 threshold configuration register"]
    pub t0_th_channel2: T0_TH_CHANNEL2,
    #[doc = "0x18 - ADV_TIMER0 channel 3 threshold configuration register"]
    pub t0_th_channel3: T0_TH_CHANNEL3,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - ADV_TIMER0 counter register"]
    pub t0_counter: T0_COUNTER,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - ADV_TIMER1 command register"]
    pub t1_cmd: T1_CMD,
    #[doc = "0x44 - ADV_TIMER1 configuration register"]
    pub t1_config: T1_CONFIG,
    #[doc = "0x48 - ADV_TIMER1 threshold configuration register"]
    pub t1_threshold: T1_THRESHOLD,
    #[doc = "0x4c - ADV_TIMER1 channel 0 threshold configuration register"]
    pub t1_th_channel0: T1_TH_CHANNEL0,
    #[doc = "0x50 - ADV_TIMER1 channel 1 threshold configuration register"]
    pub t1_th_channel1: T1_TH_CHANNEL1,
    #[doc = "0x54 - ADV_TIMER1 channel 2 threshold configuration register"]
    pub t1_th_channel2: T1_TH_CHANNEL2,
    #[doc = "0x58 - ADV_TIMER1 channel 3 threshold configuration register"]
    pub t1_th_channel3: T1_TH_CHANNEL3,
    _reserved15: [u8; 0x10],
    #[doc = "0x6c - ADV_TIMER1 counter register"]
    pub t1_counter: T1_COUNTER,
    _reserved16: [u8; 0x10],
    #[doc = "0x80 - ADV_TIMER2 command register"]
    pub t2_cmd: T2_CMD,
    #[doc = "0x84 - ADV_TIMER2 configuration register"]
    pub t2_config: T2_CONFIG,
    #[doc = "0x88 - ADV_TIMER2 threshold configuration register"]
    pub t2_threshold: T2_THRESHOLD,
    #[doc = "0x8c - ADV_TIMER2 channel 0 threshold configuration register"]
    pub t2_th_channel0: T2_TH_CHANNEL0,
    #[doc = "0x90 - ADV_TIMER2 channel 1 threshold configuration register"]
    pub t2_th_channel1: T2_TH_CHANNEL1,
    #[doc = "0x94 - ADV_TIMER2 channel 2 threshold configuration register"]
    pub t2_th_channel2: T2_TH_CHANNEL2,
    #[doc = "0x98 - ADV_TIMER2 channel 3 threshold configuration register"]
    pub t2_th_channel3: T2_TH_CHANNEL3,
    _reserved23: [u8; 0x10],
    #[doc = "0xac - ADV_TIMER2 counter register"]
    pub t2_counter: T2_COUNTER,
    _reserved24: [u8; 0x10],
    #[doc = "0xc0 - ADV_TIMER3 command register"]
    pub t3_cmd: T3_CMD,
    #[doc = "0xc4 - ADV_TIMER3 configuration register"]
    pub t3_config: T3_CONFIG,
    #[doc = "0xc8 - ADV_TIMER3 threshold configuration register"]
    pub t3_threshold: T3_THRESHOLD,
    #[doc = "0xcc - ADV_TIMER3 channel 0 threshold configuration register"]
    pub t3_th_channel0: T3_TH_CHANNEL0,
    #[doc = "0xd0 - ADV_TIMER3 channel 1 threshold configuration register"]
    pub t3_th_channel1: T3_TH_CHANNEL1,
    #[doc = "0xd4 - ADV_TIMER3 channel 2 threshold configuration register"]
    pub t3_th_channel2: T3_TH_CHANNEL2,
    #[doc = "0xd8 - ADV_TIMER3 channel 3 threshold configuration register"]
    pub t3_th_channel3: T3_TH_CHANNEL3,
    _reserved31: [u8; 0x10],
    #[doc = "0xec - ADV_TIMER3 counter register"]
    pub t3_counter: T3_COUNTER,
    _reserved32: [u8; 0x10],
    #[doc = "0x100 - ADV_TIMERS events configuration register."]
    pub event_cfg: EVENT_CFG,
    #[doc = "0x104 - ADV_TIMERS channels clock gating configuration register."]
    pub cg: CG,
}
#[doc = "T0_CMD (rw) register accessor: an alias for `Reg<T0_CMD_SPEC>`"]
pub type T0_CMD = crate::Reg<t0_cmd::T0_CMD_SPEC>;
#[doc = "ADV_TIMER0 command register"]
pub mod t0_cmd;
#[doc = "T0_CONFIG (rw) register accessor: an alias for `Reg<T0_CONFIG_SPEC>`"]
pub type T0_CONFIG = crate::Reg<t0_config::T0_CONFIG_SPEC>;
#[doc = "ADV_TIMER0 configuration register."]
pub mod t0_config;
#[doc = "T0_THRESHOLD (rw) register accessor: an alias for `Reg<T0_THRESHOLD_SPEC>`"]
pub type T0_THRESHOLD = crate::Reg<t0_threshold::T0_THRESHOLD_SPEC>;
#[doc = "ADV_TIMER0 threshold configuration register."]
pub mod t0_threshold;
#[doc = "T0_TH_CHANNEL0 (rw) register accessor: an alias for `Reg<T0_TH_CHANNEL0_SPEC>`"]
pub type T0_TH_CHANNEL0 = crate::Reg<t0_th_channel0::T0_TH_CHANNEL0_SPEC>;
#[doc = "ADV_TIMER0 channel 0 threshold configuration register"]
pub mod t0_th_channel0;
#[doc = "T0_TH_CHANNEL1 (rw) register accessor: an alias for `Reg<T0_TH_CHANNEL1_SPEC>`"]
pub type T0_TH_CHANNEL1 = crate::Reg<t0_th_channel1::T0_TH_CHANNEL1_SPEC>;
#[doc = "ADV_TIMER0 channel 1 threshold configuration register"]
pub mod t0_th_channel1;
#[doc = "T0_TH_CHANNEL2 (rw) register accessor: an alias for `Reg<T0_TH_CHANNEL2_SPEC>`"]
pub type T0_TH_CHANNEL2 = crate::Reg<t0_th_channel2::T0_TH_CHANNEL2_SPEC>;
#[doc = "ADV_TIMER0 channel 2 threshold configuration register"]
pub mod t0_th_channel2;
#[doc = "T0_TH_CHANNEL3 (rw) register accessor: an alias for `Reg<T0_TH_CHANNEL3_SPEC>`"]
pub type T0_TH_CHANNEL3 = crate::Reg<t0_th_channel3::T0_TH_CHANNEL3_SPEC>;
#[doc = "ADV_TIMER0 channel 3 threshold configuration register"]
pub mod t0_th_channel3;
#[doc = "T0_COUNTER (r) register accessor: an alias for `Reg<T0_COUNTER_SPEC>`"]
pub type T0_COUNTER = crate::Reg<t0_counter::T0_COUNTER_SPEC>;
#[doc = "ADV_TIMER0 counter register"]
pub mod t0_counter;
#[doc = "T1_CMD (rw) register accessor: an alias for `Reg<T1_CMD_SPEC>`"]
pub type T1_CMD = crate::Reg<t1_cmd::T1_CMD_SPEC>;
#[doc = "ADV_TIMER1 command register"]
pub mod t1_cmd;
#[doc = "T1_CONFIG (rw) register accessor: an alias for `Reg<T1_CONFIG_SPEC>`"]
pub type T1_CONFIG = crate::Reg<t1_config::T1_CONFIG_SPEC>;
#[doc = "ADV_TIMER1 configuration register"]
pub mod t1_config;
#[doc = "T1_THRESHOLD (rw) register accessor: an alias for `Reg<T1_THRESHOLD_SPEC>`"]
pub type T1_THRESHOLD = crate::Reg<t1_threshold::T1_THRESHOLD_SPEC>;
#[doc = "ADV_TIMER1 threshold configuration register"]
pub mod t1_threshold;
#[doc = "T1_TH_CHANNEL0 (rw) register accessor: an alias for `Reg<T1_TH_CHANNEL0_SPEC>`"]
pub type T1_TH_CHANNEL0 = crate::Reg<t1_th_channel0::T1_TH_CHANNEL0_SPEC>;
#[doc = "ADV_TIMER1 channel 0 threshold configuration register"]
pub mod t1_th_channel0;
#[doc = "T1_TH_CHANNEL1 (rw) register accessor: an alias for `Reg<T1_TH_CHANNEL1_SPEC>`"]
pub type T1_TH_CHANNEL1 = crate::Reg<t1_th_channel1::T1_TH_CHANNEL1_SPEC>;
#[doc = "ADV_TIMER1 channel 1 threshold configuration register"]
pub mod t1_th_channel1;
#[doc = "T1_TH_CHANNEL2 (rw) register accessor: an alias for `Reg<T1_TH_CHANNEL2_SPEC>`"]
pub type T1_TH_CHANNEL2 = crate::Reg<t1_th_channel2::T1_TH_CHANNEL2_SPEC>;
#[doc = "ADV_TIMER1 channel 2 threshold configuration register"]
pub mod t1_th_channel2;
#[doc = "T1_TH_CHANNEL3 (rw) register accessor: an alias for `Reg<T1_TH_CHANNEL3_SPEC>`"]
pub type T1_TH_CHANNEL3 = crate::Reg<t1_th_channel3::T1_TH_CHANNEL3_SPEC>;
#[doc = "ADV_TIMER1 channel 3 threshold configuration register"]
pub mod t1_th_channel3;
#[doc = "T1_COUNTER (r) register accessor: an alias for `Reg<T1_COUNTER_SPEC>`"]
pub type T1_COUNTER = crate::Reg<t1_counter::T1_COUNTER_SPEC>;
#[doc = "ADV_TIMER1 counter register"]
pub mod t1_counter;
#[doc = "T2_CMD (rw) register accessor: an alias for `Reg<T2_CMD_SPEC>`"]
pub type T2_CMD = crate::Reg<t2_cmd::T2_CMD_SPEC>;
#[doc = "ADV_TIMER2 command register"]
pub mod t2_cmd;
#[doc = "T2_CONFIG (rw) register accessor: an alias for `Reg<T2_CONFIG_SPEC>`"]
pub type T2_CONFIG = crate::Reg<t2_config::T2_CONFIG_SPEC>;
#[doc = "ADV_TIMER2 configuration register"]
pub mod t2_config;
#[doc = "T2_THRESHOLD (rw) register accessor: an alias for `Reg<T2_THRESHOLD_SPEC>`"]
pub type T2_THRESHOLD = crate::Reg<t2_threshold::T2_THRESHOLD_SPEC>;
#[doc = "ADV_TIMER2 threshold configuration register"]
pub mod t2_threshold;
#[doc = "T2_TH_CHANNEL0 (rw) register accessor: an alias for `Reg<T2_TH_CHANNEL0_SPEC>`"]
pub type T2_TH_CHANNEL0 = crate::Reg<t2_th_channel0::T2_TH_CHANNEL0_SPEC>;
#[doc = "ADV_TIMER2 channel 0 threshold configuration register"]
pub mod t2_th_channel0;
#[doc = "T2_TH_CHANNEL1 (rw) register accessor: an alias for `Reg<T2_TH_CHANNEL1_SPEC>`"]
pub type T2_TH_CHANNEL1 = crate::Reg<t2_th_channel1::T2_TH_CHANNEL1_SPEC>;
#[doc = "ADV_TIMER2 channel 1 threshold configuration register"]
pub mod t2_th_channel1;
#[doc = "T2_TH_CHANNEL2 (rw) register accessor: an alias for `Reg<T2_TH_CHANNEL2_SPEC>`"]
pub type T2_TH_CHANNEL2 = crate::Reg<t2_th_channel2::T2_TH_CHANNEL2_SPEC>;
#[doc = "ADV_TIMER2 channel 2 threshold configuration register"]
pub mod t2_th_channel2;
#[doc = "T2_TH_CHANNEL3 (rw) register accessor: an alias for `Reg<T2_TH_CHANNEL3_SPEC>`"]
pub type T2_TH_CHANNEL3 = crate::Reg<t2_th_channel3::T2_TH_CHANNEL3_SPEC>;
#[doc = "ADV_TIMER2 channel 3 threshold configuration register"]
pub mod t2_th_channel3;
#[doc = "T2_COUNTER (rw) register accessor: an alias for `Reg<T2_COUNTER_SPEC>`"]
pub type T2_COUNTER = crate::Reg<t2_counter::T2_COUNTER_SPEC>;
#[doc = "ADV_TIMER2 counter register"]
pub mod t2_counter;
#[doc = "T3_CMD (rw) register accessor: an alias for `Reg<T3_CMD_SPEC>`"]
pub type T3_CMD = crate::Reg<t3_cmd::T3_CMD_SPEC>;
#[doc = "ADV_TIMER3 command register"]
pub mod t3_cmd;
#[doc = "T3_CONFIG (rw) register accessor: an alias for `Reg<T3_CONFIG_SPEC>`"]
pub type T3_CONFIG = crate::Reg<t3_config::T3_CONFIG_SPEC>;
#[doc = "ADV_TIMER3 configuration register"]
pub mod t3_config;
#[doc = "T3_THRESHOLD (rw) register accessor: an alias for `Reg<T3_THRESHOLD_SPEC>`"]
pub type T3_THRESHOLD = crate::Reg<t3_threshold::T3_THRESHOLD_SPEC>;
#[doc = "ADV_TIMER3 threshold configuration register"]
pub mod t3_threshold;
#[doc = "T3_TH_CHANNEL0 (rw) register accessor: an alias for `Reg<T3_TH_CHANNEL0_SPEC>`"]
pub type T3_TH_CHANNEL0 = crate::Reg<t3_th_channel0::T3_TH_CHANNEL0_SPEC>;
#[doc = "ADV_TIMER3 channel 0 threshold configuration register"]
pub mod t3_th_channel0;
#[doc = "T3_TH_CHANNEL1 (rw) register accessor: an alias for `Reg<T3_TH_CHANNEL1_SPEC>`"]
pub type T3_TH_CHANNEL1 = crate::Reg<t3_th_channel1::T3_TH_CHANNEL1_SPEC>;
#[doc = "ADV_TIMER3 channel 1 threshold configuration register"]
pub mod t3_th_channel1;
#[doc = "T3_TH_CHANNEL2 (rw) register accessor: an alias for `Reg<T3_TH_CHANNEL2_SPEC>`"]
pub type T3_TH_CHANNEL2 = crate::Reg<t3_th_channel2::T3_TH_CHANNEL2_SPEC>;
#[doc = "ADV_TIMER3 channel 2 threshold configuration register"]
pub mod t3_th_channel2;
#[doc = "T3_TH_CHANNEL3 (rw) register accessor: an alias for `Reg<T3_TH_CHANNEL3_SPEC>`"]
pub type T3_TH_CHANNEL3 = crate::Reg<t3_th_channel3::T3_TH_CHANNEL3_SPEC>;
#[doc = "ADV_TIMER3 channel 3 threshold configuration register"]
pub mod t3_th_channel3;
#[doc = "CG (rw) register accessor: an alias for `Reg<CG_SPEC>`"]
pub type CG = crate::Reg<cg::CG_SPEC>;
#[doc = "ADV_TIMERS channels clock gating configuration register."]
pub mod cg;
#[doc = "T3_COUNTER (r) register accessor: an alias for `Reg<T3_COUNTER_SPEC>`"]
pub type T3_COUNTER = crate::Reg<t3_counter::T3_COUNTER_SPEC>;
#[doc = "ADV_TIMER3 counter register"]
pub mod t3_counter;
#[doc = "EVENT_CFG (rw) register accessor: an alias for `Reg<EVENT_CFG_SPEC>`"]
pub type EVENT_CFG = crate::Reg<event_cfg::EVENT_CFG_SPEC>;
#[doc = "ADV_TIMERS events configuration register."]
pub mod event_cfg;
