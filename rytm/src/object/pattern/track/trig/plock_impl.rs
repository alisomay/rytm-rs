// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
// TODO: I'm currently really lazy to write errors doc for this module. Will do it later
#![allow(clippy::missing_errors_doc)]

mod amp;
mod comp;
mod delay;
mod dist;
mod filter;
mod fx_lfo;
mod lfo;
mod reverb;
mod samp;

pub mod fx_plock_types {
    pub const AR_FX_PLOCK_TYPE_DELAY_TIME: u32 = 0;
    pub const AR_FX_PLOCK_TYPE_DELAY_PING_PONG: u32 = 1;
    pub const AR_FX_PLOCK_TYPE_DELAY_WIDTH: u32 = 2;
    pub const AR_FX_PLOCK_TYPE_DELAY_FEEDBACK: u32 = 3;
    pub const AR_FX_PLOCK_TYPE_DELAY_HPF: u32 = 4;
    pub const AR_FX_PLOCK_TYPE_DELAY_LPF: u32 = 5;
    pub const AR_FX_PLOCK_TYPE_DELAY_REV: u32 = 6;
    pub const AR_FX_PLOCK_TYPE_DELAY_VOL: u32 = 7;
    pub const AR_FX_PLOCK_TYPE_REVERB_PRE: u32 = 10;
    pub const AR_FX_PLOCK_TYPE_REVERB_DECAY: u32 = 11;
    pub const AR_FX_PLOCK_TYPE_REVERB_FREQ: u32 = 12;
    pub const AR_FX_PLOCK_TYPE_REVERB_GAIN: u32 = 13;
    pub const AR_FX_PLOCK_TYPE_REVERB_HPF: u32 = 14;
    pub const AR_FX_PLOCK_TYPE_REVERB_LPF: u32 = 15;
    pub const AR_FX_PLOCK_TYPE_REVERB_VOL: u32 = 16;
    pub const AR_FX_PLOCK_TYPE_DIST_AMOUNT: u32 = 18;
    pub const AR_FX_PLOCK_TYPE_DIST_SYM: u32 = 19;
    pub const AR_FX_PLOCK_TYPE_DIST_DOV: u32 = 8;
    pub const AR_FX_PLOCK_TYPE_DIST_DELAY: u32 = 9;
    pub const AR_FX_PLOCK_TYPE_DIST_REV: u32 = 17;
    pub const AR_FX_PLOCK_TYPE_COMP_THRESHOLD: u32 = 21;
    pub const AR_FX_PLOCK_TYPE_COMP_ATTACK: u32 = 22;
    pub const AR_FX_PLOCK_TYPE_COMP_RELEASE: u32 = 23;
    pub const AR_FX_PLOCK_TYPE_COMP_MAKEUP: u32 = 26;
    pub const AR_FX_PLOCK_TYPE_COMP_RATIO: u32 = 24;
    pub const AR_FX_PLOCK_TYPE_COMP_SEQ: u32 = 25;
    pub const AR_FX_PLOCK_TYPE_COMP_MIX: u32 = 27;
    pub const AR_FX_PLOCK_TYPE_COMP_VOL: u32 = 28;
    pub const AR_FX_PLOCK_TYPE_LFO_SPEED: u32 = 29;
    pub const AR_FX_PLOCK_TYPE_LFO_MULTIPLY: u32 = 30;
    pub const AR_FX_PLOCK_TYPE_LFO_FADE: u32 = 31;
    pub const AR_FX_PLOCK_TYPE_LFO_DEST: u32 = 32;
    pub const AR_FX_PLOCK_TYPE_LFO_WAVEFORM: u32 = 33;
    pub const AR_FX_PLOCK_TYPE_LFO_PHASE: u32 = 34;
    pub const AR_FX_PLOCK_TYPE_LFO_MOD: u32 = 35;
    pub const AR_FX_PLOCK_TYPE_LFO_DEPTH: u32 = 36;
}
