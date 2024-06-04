#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "AdvancedTimer"]
#[doc(alias = "AdvancedTimer")]
pub struct ADVANCED_TIMER {
    t0_cmd: T0_CMD,
    t0_config: T0_CONFIG,
    t0_threshold: T0_THRESHOLD,
    t0_th_channel0: T0_TH_CHANNEL0,
    t0_th_channel1: T0_TH_CHANNEL1,
    t0_th_channel2: T0_TH_CHANNEL2,
    t0_th_channel3: T0_TH_CHANNEL3,
    _reserved7: [u8; 0x10],
    t0_counter: T0_COUNTER,
    _reserved8: [u8; 0x10],
    t1_cmd: T1_CMD,
    t1_config: T1_CONFIG,
    t1_threshold: T1_THRESHOLD,
    t1_th_channel0: T1_TH_CHANNEL0,
    t1_th_channel1: T1_TH_CHANNEL1,
    t1_th_channel2: T1_TH_CHANNEL2,
    t1_th_channel3: T1_TH_CHANNEL3,
    _reserved15: [u8; 0x10],
    t1_counter: T1_COUNTER,
    _reserved16: [u8; 0x10],
    t2_cmd: T2_CMD,
    t2_config: T2_CONFIG,
    t2_threshold: T2_THRESHOLD,
    t2_th_channel0: T2_TH_CHANNEL0,
    t2_th_channel1: T2_TH_CHANNEL1,
    t2_th_channel2: T2_TH_CHANNEL2,
    t2_th_channel3: T2_TH_CHANNEL3,
    _reserved23: [u8; 0x10],
    t2_counter: T2_COUNTER,
    _reserved24: [u8; 0x10],
    t3_cmd: T3_CMD,
    t3_config: T3_CONFIG,
    t3_threshold: T3_THRESHOLD,
    t3_th_channel0: T3_TH_CHANNEL0,
    t3_th_channel1: T3_TH_CHANNEL1,
    t3_th_channel2: T3_TH_CHANNEL2,
    t3_th_channel3: T3_TH_CHANNEL3,
    _reserved31: [u8; 0x10],
    t3_counter: T3_COUNTER,
    _reserved32: [u8; 0x10],
    event_cfg: EVENT_CFG,
    cg: CG,
}
impl ADVANCED_TIMER {
    #[doc = "0x00 - ADV_TIMER0 command register"]
    #[inline(always)]
    pub const fn t0_cmd(&self) -> &T0_CMD {
        &self.t0_cmd
    }
    #[doc = "0x04 - ADV_TIMER0 configuration register."]
    #[inline(always)]
    pub const fn t0_config(&self) -> &T0_CONFIG {
        &self.t0_config
    }
    #[doc = "0x08 - ADV_TIMER0 threshold configuration register."]
    #[inline(always)]
    pub const fn t0_threshold(&self) -> &T0_THRESHOLD {
        &self.t0_threshold
    }
    #[doc = "0x0c - ADV_TIMER0 channel 0 threshold configuration register"]
    #[inline(always)]
    pub const fn t0_th_channel0(&self) -> &T0_TH_CHANNEL0 {
        &self.t0_th_channel0
    }
    #[doc = "0x10 - ADV_TIMER0 channel 1 threshold configuration register"]
    #[inline(always)]
    pub const fn t0_th_channel1(&self) -> &T0_TH_CHANNEL1 {
        &self.t0_th_channel1
    }
    #[doc = "0x14 - ADV_TIMER0 channel 2 threshold configuration register"]
    #[inline(always)]
    pub const fn t0_th_channel2(&self) -> &T0_TH_CHANNEL2 {
        &self.t0_th_channel2
    }
    #[doc = "0x18 - ADV_TIMER0 channel 3 threshold configuration register"]
    #[inline(always)]
    pub const fn t0_th_channel3(&self) -> &T0_TH_CHANNEL3 {
        &self.t0_th_channel3
    }
    #[doc = "0x2c - ADV_TIMER0 counter register"]
    #[inline(always)]
    pub const fn t0_counter(&self) -> &T0_COUNTER {
        &self.t0_counter
    }
    #[doc = "0x40 - ADV_TIMER1 command register"]
    #[inline(always)]
    pub const fn t1_cmd(&self) -> &T1_CMD {
        &self.t1_cmd
    }
    #[doc = "0x44 - ADV_TIMER1 configuration register"]
    #[inline(always)]
    pub const fn t1_config(&self) -> &T1_CONFIG {
        &self.t1_config
    }
    #[doc = "0x48 - ADV_TIMER1 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_threshold(&self) -> &T1_THRESHOLD {
        &self.t1_threshold
    }
    #[doc = "0x4c - ADV_TIMER1 channel 0 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_th_channel0(&self) -> &T1_TH_CHANNEL0 {
        &self.t1_th_channel0
    }
    #[doc = "0x50 - ADV_TIMER1 channel 1 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_th_channel1(&self) -> &T1_TH_CHANNEL1 {
        &self.t1_th_channel1
    }
    #[doc = "0x54 - ADV_TIMER1 channel 2 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_th_channel2(&self) -> &T1_TH_CHANNEL2 {
        &self.t1_th_channel2
    }
    #[doc = "0x58 - ADV_TIMER1 channel 3 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_th_channel3(&self) -> &T1_TH_CHANNEL3 {
        &self.t1_th_channel3
    }
    #[doc = "0x6c - ADV_TIMER1 counter register"]
    #[inline(always)]
    pub const fn t1_counter(&self) -> &T1_COUNTER {
        &self.t1_counter
    }
    #[doc = "0x80 - ADV_TIMER2 command register"]
    #[inline(always)]
    pub const fn t2_cmd(&self) -> &T2_CMD {
        &self.t2_cmd
    }
    #[doc = "0x84 - ADV_TIMER2 configuration register"]
    #[inline(always)]
    pub const fn t2_config(&self) -> &T2_CONFIG {
        &self.t2_config
    }
    #[doc = "0x88 - ADV_TIMER2 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_threshold(&self) -> &T2_THRESHOLD {
        &self.t2_threshold
    }
    #[doc = "0x8c - ADV_TIMER2 channel 0 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_th_channel0(&self) -> &T2_TH_CHANNEL0 {
        &self.t2_th_channel0
    }
    #[doc = "0x90 - ADV_TIMER2 channel 1 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_th_channel1(&self) -> &T2_TH_CHANNEL1 {
        &self.t2_th_channel1
    }
    #[doc = "0x94 - ADV_TIMER2 channel 2 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_th_channel2(&self) -> &T2_TH_CHANNEL2 {
        &self.t2_th_channel2
    }
    #[doc = "0x98 - ADV_TIMER2 channel 3 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_th_channel3(&self) -> &T2_TH_CHANNEL3 {
        &self.t2_th_channel3
    }
    #[doc = "0xac - ADV_TIMER2 counter register"]
    #[inline(always)]
    pub const fn t2_counter(&self) -> &T2_COUNTER {
        &self.t2_counter
    }
    #[doc = "0xc0 - ADV_TIMER3 command register"]
    #[inline(always)]
    pub const fn t3_cmd(&self) -> &T3_CMD {
        &self.t3_cmd
    }
    #[doc = "0xc4 - ADV_TIMER3 configuration register"]
    #[inline(always)]
    pub const fn t3_config(&self) -> &T3_CONFIG {
        &self.t3_config
    }
    #[doc = "0xc8 - ADV_TIMER3 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_threshold(&self) -> &T3_THRESHOLD {
        &self.t3_threshold
    }
    #[doc = "0xcc - ADV_TIMER3 channel 0 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_th_channel0(&self) -> &T3_TH_CHANNEL0 {
        &self.t3_th_channel0
    }
    #[doc = "0xd0 - ADV_TIMER3 channel 1 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_th_channel1(&self) -> &T3_TH_CHANNEL1 {
        &self.t3_th_channel1
    }
    #[doc = "0xd4 - ADV_TIMER3 channel 2 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_th_channel2(&self) -> &T3_TH_CHANNEL2 {
        &self.t3_th_channel2
    }
    #[doc = "0xd8 - ADV_TIMER3 channel 3 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_th_channel3(&self) -> &T3_TH_CHANNEL3 {
        &self.t3_th_channel3
    }
    #[doc = "0xec - ADV_TIMER3 counter register"]
    #[inline(always)]
    pub const fn t3_counter(&self) -> &T3_COUNTER {
        &self.t3_counter
    }
    #[doc = "0x100 - ADV_TIMERS events configuration register."]
    #[inline(always)]
    pub const fn event_cfg(&self) -> &EVENT_CFG {
        &self.event_cfg
    }
    #[doc = "0x104 - ADV_TIMERS channels clock gating configuration register."]
    #[inline(always)]
    pub const fn cg(&self) -> &CG {
        &self.cg
    }
}
#[doc = "T0_CMD (rw) register accessor: ADV_TIMER0 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_cmd`]
module"]
pub type T0_CMD = crate::Reg<t0_cmd::T0_CMD_SPEC>;
#[doc = "ADV_TIMER0 command register"]
pub mod t0_cmd;
#[doc = "T0_CONFIG (rw) register accessor: ADV_TIMER0 configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_config`]
module"]
pub type T0_CONFIG = crate::Reg<t0_config::T0_CONFIG_SPEC>;
#[doc = "ADV_TIMER0 configuration register."]
pub mod t0_config;
#[doc = "T0_THRESHOLD (rw) register accessor: ADV_TIMER0 threshold configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_threshold`]
module"]
pub type T0_THRESHOLD = crate::Reg<t0_threshold::T0_THRESHOLD_SPEC>;
#[doc = "ADV_TIMER0 threshold configuration register."]
pub mod t0_threshold;
#[doc = "T0_TH_CHANNEL0 (rw) register accessor: ADV_TIMER0 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_th_channel0`]
module"]
pub type T0_TH_CHANNEL0 = crate::Reg<t0_th_channel0::T0_TH_CHANNEL0_SPEC>;
#[doc = "ADV_TIMER0 channel 0 threshold configuration register"]
pub mod t0_th_channel0;
#[doc = "T0_TH_CHANNEL1 (rw) register accessor: ADV_TIMER0 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_th_channel1`]
module"]
pub type T0_TH_CHANNEL1 = crate::Reg<t0_th_channel1::T0_TH_CHANNEL1_SPEC>;
#[doc = "ADV_TIMER0 channel 1 threshold configuration register"]
pub mod t0_th_channel1;
#[doc = "T0_TH_CHANNEL2 (rw) register accessor: ADV_TIMER0 channel 2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_th_channel2`]
module"]
pub type T0_TH_CHANNEL2 = crate::Reg<t0_th_channel2::T0_TH_CHANNEL2_SPEC>;
#[doc = "ADV_TIMER0 channel 2 threshold configuration register"]
pub mod t0_th_channel2;
#[doc = "T0_TH_CHANNEL3 (rw) register accessor: ADV_TIMER0 channel 3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_th_channel3`]
module"]
pub type T0_TH_CHANNEL3 = crate::Reg<t0_th_channel3::T0_TH_CHANNEL3_SPEC>;
#[doc = "ADV_TIMER0 channel 3 threshold configuration register"]
pub mod t0_th_channel3;
#[doc = "T0_COUNTER (r) register accessor: ADV_TIMER0 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_counter`]
module"]
pub type T0_COUNTER = crate::Reg<t0_counter::T0_COUNTER_SPEC>;
#[doc = "ADV_TIMER0 counter register"]
pub mod t0_counter;
#[doc = "T1_CMD (rw) register accessor: ADV_TIMER1 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_cmd`]
module"]
pub type T1_CMD = crate::Reg<t1_cmd::T1_CMD_SPEC>;
#[doc = "ADV_TIMER1 command register"]
pub mod t1_cmd;
#[doc = "T1_CONFIG (rw) register accessor: ADV_TIMER1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_config`]
module"]
pub type T1_CONFIG = crate::Reg<t1_config::T1_CONFIG_SPEC>;
#[doc = "ADV_TIMER1 configuration register"]
pub mod t1_config;
#[doc = "T1_THRESHOLD (rw) register accessor: ADV_TIMER1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_threshold`]
module"]
pub type T1_THRESHOLD = crate::Reg<t1_threshold::T1_THRESHOLD_SPEC>;
#[doc = "ADV_TIMER1 threshold configuration register"]
pub mod t1_threshold;
#[doc = "T1_TH_CHANNEL0 (rw) register accessor: ADV_TIMER1 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_th_channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_th_channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_th_channel0`]
module"]
pub type T1_TH_CHANNEL0 = crate::Reg<t1_th_channel0::T1_TH_CHANNEL0_SPEC>;
#[doc = "ADV_TIMER1 channel 0 threshold configuration register"]
pub mod t1_th_channel0;
#[doc = "T1_TH_CHANNEL1 (rw) register accessor: ADV_TIMER1 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_th_channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_th_channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_th_channel1`]
module"]
pub type T1_TH_CHANNEL1 = crate::Reg<t1_th_channel1::T1_TH_CHANNEL1_SPEC>;
#[doc = "ADV_TIMER1 channel 1 threshold configuration register"]
pub mod t1_th_channel1;
#[doc = "T1_TH_CHANNEL2 (rw) register accessor: ADV_TIMER1 channel 2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_th_channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_th_channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_th_channel2`]
module"]
pub type T1_TH_CHANNEL2 = crate::Reg<t1_th_channel2::T1_TH_CHANNEL2_SPEC>;
#[doc = "ADV_TIMER1 channel 2 threshold configuration register"]
pub mod t1_th_channel2;
#[doc = "T1_TH_CHANNEL3 (rw) register accessor: ADV_TIMER1 channel 3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_th_channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_th_channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_th_channel3`]
module"]
pub type T1_TH_CHANNEL3 = crate::Reg<t1_th_channel3::T1_TH_CHANNEL3_SPEC>;
#[doc = "ADV_TIMER1 channel 3 threshold configuration register"]
pub mod t1_th_channel3;
#[doc = "T1_COUNTER (r) register accessor: ADV_TIMER1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_counter`]
module"]
pub type T1_COUNTER = crate::Reg<t1_counter::T1_COUNTER_SPEC>;
#[doc = "ADV_TIMER1 counter register"]
pub mod t1_counter;
#[doc = "T2_CMD (rw) register accessor: ADV_TIMER2 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_cmd`]
module"]
pub type T2_CMD = crate::Reg<t2_cmd::T2_CMD_SPEC>;
#[doc = "ADV_TIMER2 command register"]
pub mod t2_cmd;
#[doc = "T2_CONFIG (rw) register accessor: ADV_TIMER2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_config`]
module"]
pub type T2_CONFIG = crate::Reg<t2_config::T2_CONFIG_SPEC>;
#[doc = "ADV_TIMER2 configuration register"]
pub mod t2_config;
#[doc = "T2_THRESHOLD (rw) register accessor: ADV_TIMER2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_threshold`]
module"]
pub type T2_THRESHOLD = crate::Reg<t2_threshold::T2_THRESHOLD_SPEC>;
#[doc = "ADV_TIMER2 threshold configuration register"]
pub mod t2_threshold;
#[doc = "T2_TH_CHANNEL0 (rw) register accessor: ADV_TIMER2 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_th_channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_th_channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_th_channel0`]
module"]
pub type T2_TH_CHANNEL0 = crate::Reg<t2_th_channel0::T2_TH_CHANNEL0_SPEC>;
#[doc = "ADV_TIMER2 channel 0 threshold configuration register"]
pub mod t2_th_channel0;
#[doc = "T2_TH_CHANNEL1 (rw) register accessor: ADV_TIMER2 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_th_channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_th_channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_th_channel1`]
module"]
pub type T2_TH_CHANNEL1 = crate::Reg<t2_th_channel1::T2_TH_CHANNEL1_SPEC>;
#[doc = "ADV_TIMER2 channel 1 threshold configuration register"]
pub mod t2_th_channel1;
#[doc = "T2_TH_CHANNEL2 (rw) register accessor: ADV_TIMER2 channel 2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_th_channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_th_channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_th_channel2`]
module"]
pub type T2_TH_CHANNEL2 = crate::Reg<t2_th_channel2::T2_TH_CHANNEL2_SPEC>;
#[doc = "ADV_TIMER2 channel 2 threshold configuration register"]
pub mod t2_th_channel2;
#[doc = "T2_TH_CHANNEL3 (rw) register accessor: ADV_TIMER2 channel 3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_th_channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_th_channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_th_channel3`]
module"]
pub type T2_TH_CHANNEL3 = crate::Reg<t2_th_channel3::T2_TH_CHANNEL3_SPEC>;
#[doc = "ADV_TIMER2 channel 3 threshold configuration register"]
pub mod t2_th_channel3;
#[doc = "T2_COUNTER (rw) register accessor: ADV_TIMER2 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_counter`]
module"]
pub type T2_COUNTER = crate::Reg<t2_counter::T2_COUNTER_SPEC>;
#[doc = "ADV_TIMER2 counter register"]
pub mod t2_counter;
#[doc = "T3_CMD (rw) register accessor: ADV_TIMER3 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_cmd`]
module"]
pub type T3_CMD = crate::Reg<t3_cmd::T3_CMD_SPEC>;
#[doc = "ADV_TIMER3 command register"]
pub mod t3_cmd;
#[doc = "T3_CONFIG (rw) register accessor: ADV_TIMER3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_config`]
module"]
pub type T3_CONFIG = crate::Reg<t3_config::T3_CONFIG_SPEC>;
#[doc = "ADV_TIMER3 configuration register"]
pub mod t3_config;
#[doc = "T3_THRESHOLD (rw) register accessor: ADV_TIMER3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_threshold`]
module"]
pub type T3_THRESHOLD = crate::Reg<t3_threshold::T3_THRESHOLD_SPEC>;
#[doc = "ADV_TIMER3 threshold configuration register"]
pub mod t3_threshold;
#[doc = "T3_TH_CHANNEL0 (rw) register accessor: ADV_TIMER3 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_th_channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_th_channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_th_channel0`]
module"]
pub type T3_TH_CHANNEL0 = crate::Reg<t3_th_channel0::T3_TH_CHANNEL0_SPEC>;
#[doc = "ADV_TIMER3 channel 0 threshold configuration register"]
pub mod t3_th_channel0;
#[doc = "T3_TH_CHANNEL1 (rw) register accessor: ADV_TIMER3 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_th_channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_th_channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_th_channel1`]
module"]
pub type T3_TH_CHANNEL1 = crate::Reg<t3_th_channel1::T3_TH_CHANNEL1_SPEC>;
#[doc = "ADV_TIMER3 channel 1 threshold configuration register"]
pub mod t3_th_channel1;
#[doc = "T3_TH_CHANNEL2 (rw) register accessor: ADV_TIMER3 channel 2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_th_channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_th_channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_th_channel2`]
module"]
pub type T3_TH_CHANNEL2 = crate::Reg<t3_th_channel2::T3_TH_CHANNEL2_SPEC>;
#[doc = "ADV_TIMER3 channel 2 threshold configuration register"]
pub mod t3_th_channel2;
#[doc = "T3_TH_CHANNEL3 (rw) register accessor: ADV_TIMER3 channel 3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_th_channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_th_channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_th_channel3`]
module"]
pub type T3_TH_CHANNEL3 = crate::Reg<t3_th_channel3::T3_TH_CHANNEL3_SPEC>;
#[doc = "ADV_TIMER3 channel 3 threshold configuration register"]
pub mod t3_th_channel3;
#[doc = "CG (rw) register accessor: ADV_TIMERS channels clock gating configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cg`]
module"]
pub type CG = crate::Reg<cg::CG_SPEC>;
#[doc = "ADV_TIMERS channels clock gating configuration register."]
pub mod cg;
#[doc = "T3_COUNTER (r) register accessor: ADV_TIMER3 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_counter`]
module"]
pub type T3_COUNTER = crate::Reg<t3_counter::T3_COUNTER_SPEC>;
#[doc = "ADV_TIMER3 counter register"]
pub mod t3_counter;
#[doc = "EVENT_CFG (rw) register accessor: ADV_TIMERS events configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_cfg`]
module"]
pub type EVENT_CFG = crate::Reg<event_cfg::EVENT_CFG_SPEC>;
#[doc = "ADV_TIMERS events configuration register."]
pub mod event_cfg;
