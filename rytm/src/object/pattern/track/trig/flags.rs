// All casts in this file are intended or safe within the context of this library.
#![allow(clippy::cast_possible_truncation)]
use rytm_sys::{
    AR_TRIG_ACCENT, AR_TRIG_ENABLE, AR_TRIG_ENV_PL_EN, AR_TRIG_ENV_PL_SW, AR_TRIG_LFO_PL_EN,
    AR_TRIG_LFO_PL_SW, AR_TRIG_MUTE, AR_TRIG_RETRIG, AR_TRIG_SLIDE, AR_TRIG_SMP_PL_EN,
    AR_TRIG_SMP_PL_SW, AR_TRIG_SWING, AR_TRIG_SYN_PL_EN, AR_TRIG_SYN_PL_SW,
};

/// Trig is enabled or disabled.
pub const ENABLE: u16 = AR_TRIG_ENABLE as u16;
/// Trig's retrig option is enabled or disabled.
pub const RETRIG: u16 = AR_TRIG_RETRIG as u16;
/// Trig's mute option is enabled or disabled.
pub const MUTE: u16 = AR_TRIG_MUTE as u16;
/// Trig's accent option is enabled or disabled.
pub const ACCENT: u16 = AR_TRIG_ACCENT as u16;
/// Trig's swing option is enabled or disabled.
pub const SWING: u16 = AR_TRIG_SWING as u16;
/// Trig's slide option is enabled or disabled.
pub const SLIDE: u16 = AR_TRIG_SLIDE as u16;

pub const LFO_PL_SW: u16 = AR_TRIG_LFO_PL_SW as u16;
pub const SYN_PL_SW: u16 = AR_TRIG_SYN_PL_SW as u16;
pub const SMP_PL_SW: u16 = AR_TRIG_SMP_PL_SW as u16;
pub const ENV_PL_SW: u16 = AR_TRIG_ENV_PL_SW as u16;
pub const LFO_PL_EN: u16 = AR_TRIG_LFO_PL_EN as u16;
pub const SYN_PL_EN: u16 = AR_TRIG_SYN_PL_EN as u16;
pub const SMP_PL_EN: u16 = AR_TRIG_SMP_PL_EN as u16;
pub const ENV_PL_EN: u16 = AR_TRIG_ENV_PL_EN as u16;
