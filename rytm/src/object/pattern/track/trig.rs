// TODO: Document PL_SW and PL_EN flags.

pub mod flags;
pub mod types;

use self::types::{Length, MicroTime, RetrigRate, TrigCondition};
use crate::{
    error::{ParameterError, RytmError},
    object::{
        kit::types::{
            FxCompAttack, FxCompRatio, FxCompRelease, FxCompSideChainEq, FxLfoDestination,
        },
        pattern::parameter_lock::{self, ParameterLockPool},
        sound::types::{LfoDestination, LfoMode, LfoMultiplier, LfoWaveform},
    },
    util::{decode_micro_timing_byte, i8_to_u8_midpoint_of_u8_input_range, scale_generic},
    RytmError::OrphanTrig,
};
use derivative::Derivative;
use flags::*;
use rytm_rs_macro::parameter_range;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

mod fx_plock_types {
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

use fx_plock_types::*;

pub trait HoldsTrigFlags {
    /// Returns the raw flags value.
    fn raw_trig_flags(&self) -> u16;

    /// Returns a mutable reference to the raw flags value.
    fn raw_trig_flags_mut(&mut self) -> &mut u16;

    /// Enables or disables a trig.
    fn set_trig_enable(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= ENABLE;
        } else {
            *(self.raw_trig_flags_mut()) &= !(ENABLE);
        }
    }

    /// Enables or disables a trig's retrig option.
    fn set_retrig(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= RETRIG;
        } else {
            *(self.raw_trig_flags_mut()) &= !(RETRIG);
        }
    }

    /// Enables or disables a trig's mute option.
    fn set_mute(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= MUTE;
        } else {
            *(self.raw_trig_flags_mut()) &= !(MUTE);
        }
    }

    /// Enables or disables a trig's accent option.
    fn set_accent(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= ACCENT;
        } else {
            *(self.raw_trig_flags_mut()) &= !(ACCENT);
        }
    }

    /// Enables or disables a trig's swing option.
    fn set_swing(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= SWING;
        } else {
            *(self.raw_trig_flags_mut()) &= !(SWING);
        }
    }

    /// Enables or disables a trig's slide option.
    fn set_slide(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= SLIDE;
        } else {
            *(self.raw_trig_flags_mut()) &= !(SLIDE);
        }
    }

    fn set_parameter_lock_lfo_switch(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= LFO_PL_SW;
        } else {
            *(self.raw_trig_flags_mut()) &= !(LFO_PL_SW);
        }
    }

    fn set_parameter_lock_lfo(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= LFO_PL_EN;
        } else {
            *(self.raw_trig_flags_mut()) &= !(LFO_PL_EN);
        }
    }

    fn set_parameter_lock_synth_switch(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= SYN_PL_SW;
        } else {
            *(self.raw_trig_flags_mut()) &= !(SYN_PL_SW);
        }
    }

    fn set_parameter_lock_synth(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= SYN_PL_EN;
        } else {
            *(self.raw_trig_flags_mut()) &= !(SYN_PL_EN);
        }
    }

    fn set_parameter_lock_sample_switch(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= SMP_PL_SW;
        } else {
            *(self.raw_trig_flags_mut()) &= !(SMP_PL_SW);
        }
    }

    fn set_parameter_lock_sample(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= SMP_PL_EN;
        } else {
            *(self.raw_trig_flags_mut()) &= !(SMP_PL_EN);
        }
    }

    fn set_parameter_lock_env_switch(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= ENV_PL_SW;
        } else {
            *(self.raw_trig_flags_mut()) &= !(ENV_PL_SW);
        }
    }

    fn set_parameter_lock_env(&mut self, enable: bool) {
        if enable {
            *(self.raw_trig_flags_mut()) |= ENV_PL_EN;
        } else {
            *(self.raw_trig_flags_mut()) &= !(ENV_PL_EN);
        }
    }

    /// Returns `true` if the trig is enabled.
    fn enabled_trig(&self) -> bool {
        self.raw_trig_flags() & ENABLE == ENABLE
    }

    /// Returns `true` if the trig's retrig option is enabled.
    fn enabled_retrig(&self) -> bool {
        self.raw_trig_flags() & RETRIG == RETRIG
    }

    /// Returns `true` if the trig's mute option is enabled.
    fn enabled_mute(&self) -> bool {
        self.raw_trig_flags() & MUTE == MUTE
    }

    /// Returns `true` if the trig's accent option is enabled.
    fn enabled_accent(&self) -> bool {
        self.raw_trig_flags() & ACCENT == ACCENT
    }

    /// Returns `true` if the trig's swing option is enabled.
    fn enabled_swing(&self) -> bool {
        self.raw_trig_flags() & SWING == SWING
    }

    /// Returns `true` if the trig's slide option is enabled.
    fn enabled_slide(&self) -> bool {
        self.raw_trig_flags() & SLIDE == SLIDE
    }
    fn enabled_parameter_lock_lfo_switch(&self) -> bool {
        self.raw_trig_flags() & LFO_PL_SW == LFO_PL_SW
    }
    fn enabled_parameter_lock_lfo(&self) -> bool {
        self.raw_trig_flags() & LFO_PL_EN == LFO_PL_EN
    }
    fn enabled_parameter_lock_synth_switch(&self) -> bool {
        self.raw_trig_flags() & SYN_PL_SW == SYN_PL_SW
    }
    fn enabled_parameter_lock_synth(&self) -> bool {
        self.raw_trig_flags() & SYN_PL_EN == SYN_PL_EN
    }
    fn enabled_parameter_lock_sample_switch(&self) -> bool {
        self.raw_trig_flags() & SMP_PL_SW == SMP_PL_SW
    }
    fn enabled_parameter_lock_sample(&self) -> bool {
        self.raw_trig_flags() & SMP_PL_EN == SMP_PL_EN
    }
    fn enabled_parameter_lock_env_switch(&self) -> bool {
        self.raw_trig_flags() & ENV_PL_SW == ENV_PL_SW
    }
    fn enabled_parameter_lock_env(&self) -> bool {
        self.raw_trig_flags() & ENV_PL_EN == ENV_PL_EN
    }

    /// Sets all flags to the given value.
    fn swap_all_flags(&mut self, flags: &impl HoldsTrigFlags) {
        *(self.raw_trig_flags_mut()) = flags.raw_trig_flags();
    }

    /// Sets all flags to `0`.
    fn unset_all_flags(&mut self) {
        *(self.raw_trig_flags_mut()) = 0;
    }

    /// Flips all flags.
    fn flip_all_flags(&mut self, flags: &impl HoldsTrigFlags) {
        *(self.raw_trig_flags_mut()) ^= flags.raw_trig_flags();
    }

    /// Sets the current flags to the intersection between the current flags and the given flags.
    fn set_difference_from(&mut self, other: &impl HoldsTrigFlags) {
        *(self.raw_trig_flags_mut()) ^= other.raw_trig_flags();
    }

    /// Sets the current flags to the union between the current flags and the given flags.
    fn set_intersection_with(&mut self, other: &impl HoldsTrigFlags) {
        *(self.raw_trig_flags_mut()) &= other.raw_trig_flags();
    }

    /// Sets the current flags to the union between the current flags and the given flags.
    fn set_union_with(&mut self, other: &impl HoldsTrigFlags) {
        *(self.raw_trig_flags_mut()) |= other.raw_trig_flags();
    }

    /// Returns the difference between the current flags and the given flags.
    fn get_difference_from(&self, other: &impl HoldsTrigFlags) -> u16 {
        self.raw_trig_flags() ^ other.raw_trig_flags()
    }

    /// Returns the intersection between the current flags and the given flags.
    fn get_intersection_with(&self, other: &impl HoldsTrigFlags) -> u16 {
        self.raw_trig_flags() & other.raw_trig_flags()
    }

    /// Returns the union between the current flags and the given flags.
    fn get_union_with(&self, other: &impl HoldsTrigFlags) -> u16 {
        self.raw_trig_flags() | other.raw_trig_flags()
    }
}

#[derive(Clone, Copy)]
/// A struct that holds the trig flags.
pub struct TrigFlags(u16);

impl Default for TrigFlags {
    fn default() -> Self {
        // SYN_PL_SW, SMP_PL_SW, ENV_PL_SW,
        Self(0b0000_0011_1000_0000)
    }
}

impl DerefMut for TrigFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for TrigFlags {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u16> for TrigFlags {
    fn from(flags: u16) -> Self {
        Self(flags)
    }
}

impl From<&u16> for TrigFlags {
    fn from(flags: &u16) -> Self {
        Self(*flags)
    }
}

impl From<TrigFlags> for u16 {
    fn from(flags: TrigFlags) -> Self {
        flags.0
    }
}

impl From<&TrigFlags> for u16 {
    fn from(flags: &TrigFlags) -> Self {
        flags.0
    }
}

impl HoldsTrigFlags for TrigFlags {
    fn raw_trig_flags(&self) -> u16 {
        self.0
    }

    fn raw_trig_flags_mut(&mut self) -> &mut u16 {
        &mut self.0
    }
}

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Trig {
    track_index: usize,
    index: usize,
    /// The raw flags value.
    ///
    /// Getters and setters are provided for each flag.
    ///
    /// Default trig flags are inherited if no parameter locks are set.
    flags: TrigFlags,
    /// The note value.
    ///
    /// Follows the midi note convention. C-4 is `0x3C``.
    note: u8,
    /// Stores the state of the trig condition.
    trig_condition: TrigCondition,
    /// The velocity value for the trig.
    velocity: u8,

    note_length: Length,

    micro_timing: MicroTime,

    retrig_rate: RetrigRate,

    retrig_length: Length,

    /// The velocity offset for the retrig.
    retrig_velocity_offset: i8,

    sound_lock: u8,

    #[derivative(Debug = "ignore")]
    parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
}

// TODO: Maybe builder..
impl Trig {
    #[parameter_range(range = "trig_index:0..=63", range = "track_index:0..=12")]
    pub fn try_default(trig_index: usize, track_index: usize) -> Result<Self, RytmError> {
        let flags: u16 = if trig_index % 2 == 0 {
            // No flags
            0b0000_0000_0000_0000
        } else {
            // SWING
            0b0000_0000_0001_0000
        };

        Ok(Self {
            track_index,
            index: trig_index,
            flags: flags.into(),
            note: 127,
            trig_condition: TrigCondition::default(),
            velocity: 0xFF,
            note_length: Length::Unset,
            micro_timing: MicroTime::default(),
            retrig_rate: RetrigRate::default(),
            retrig_length: Length::Quarter,
            retrig_velocity_offset: 0,
            sound_lock: 0xFF,
            // Inefficient but it will be dropped as soon as the trig is created so it sh
            parameter_lock_pool: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        index: usize,
        track_index: usize,
        flags: u16,
        note: u8,
        note_length: u8,
        velocity: u8,
        micro_timing: u8,
        retrig_rate: u8,
        retrig_length: u8,
        retrig_velocity_offset: i8,
        sound_lock: u8,
        parameter_lock_pool: Rc<RefCell<ParameterLockPool>>,
    ) -> Result<Self, RytmError> {
        let trig_condition_msb = note & 0b1000_0000;
        let note = note & 0b0111_1111;

        let trig_condition_most_significant_mid_bits = micro_timing & 0b1100_0000;
        // Shift the micro timing 2 bits to the right so it reads as a relevant signed value. -92..=92 for every value increments and decrements by 4.
        let micro_timing = (micro_timing & 0b0011_1111) << 2;

        let trig_condition_least_significant_mid_bit = retrig_length & 0b1000_0000;
        let retrig_length = retrig_length & 0b0111_1111;

        let trig_condition_least_significant_3_bits = retrig_rate & 0b1110_0000;
        let retrig_rate = retrig_rate & 0b0001_1111;

        let mut trig_condition_value = 0_u8;

        trig_condition_value |= (trig_condition_msb >> 1)
            | (trig_condition_most_significant_mid_bits >> 2)
            | (trig_condition_least_significant_mid_bit >> 4)
            | (trig_condition_least_significant_3_bits >> 5);

        Ok(Self {
            track_index,
            index,
            flags: flags.into(),
            note,
            trig_condition: trig_condition_value.try_into()?,
            velocity,
            note_length: note_length.try_into()?,
            micro_timing: decode_micro_timing_byte(micro_timing as i8)?,
            retrig_rate: retrig_rate.try_into()?,
            retrig_length: retrig_length.try_into()?,
            retrig_velocity_offset,
            sound_lock,
            parameter_lock_pool: Some(parameter_lock_pool),
        })
    }

    pub(crate) fn encode_note(&self) -> u8 {
        (self.trig_condition as u8) & 0b1000_0000 | self.note
    }

    #[allow(overflowing_literals)]
    pub(crate) fn encode_micro_timing(&self) -> u8 {
        let encoded_byte = crate::util::encode_micro_timing_byte(&self.micro_timing);
        // Shift the micro timing 2 bits to the right to leave space for 2 bits which is a part of encoded trig condition.
        // Then fill those two bits with the trig condition's most significant mid bits.
        //
        // Since we're just setting bits, fabricating values and not doing any arithmetic we can use the literal values.
        // Overflowing literals are safe in this case.
        ((encoded_byte >> 2) | (((self.trig_condition as i8) & 0b0110_0000) << 1)) as u8
    }

    pub(crate) fn encode_retrig_length(&self) -> u8 {
        // Apply the trig condition's least significant mid bit to the retrig length's most significant bit.
        (((self.trig_condition as u8) & 0b0000_1000) << 4) | self.retrig_length as u8
    }

    pub(crate) fn encode_retrig_rate(&self) -> u8 {
        // Apply the trig condition's least significant 3 bits to the retrig rate's most significant 3 bits.
        (((self.trig_condition as u8) & 0b0000_0111) << 5) | self.retrig_rate as u8
    }

    /// Returns the index of the trig.
    pub fn index(&self) -> usize {
        self.index
    }

    // TODO: On device 36..=84 is valid.
    // Are values set from here valid?
    /// Sets the note value.
    ///
    /// Range `0..=127`
    ///
    /// Follows the midi note convention. C-4 is `0x3C`.
    #[parameter_range(range = "note:0..=127")]
    pub fn set_note(&mut self, note: usize) -> Result<(), RytmError> {
        self.note = note as u8;
        Ok(())
    }

    /// Sets the trig condition state.
    // pub fn set_trig_condition(&mut self, enable: bool) {
    //     self.trig_condition = enable;
    // }

    /// Sets the velocity value.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "velocity:1..=127")]
    pub fn set_velocity(&mut self, velocity: usize) -> Result<(), RytmError> {
        self.velocity = velocity as u8;
        Ok(())
    }

    /// Sets the micro timing by value.
    ///
    /// Range `-23..=23`
    #[parameter_range(range = "micro_timing_value:-23..=23")]
    pub fn set_micro_timing_by_value(
        &mut self,
        micro_timing_value: isize,
    ) -> Result<(), RytmError> {
        self.micro_timing = micro_timing_value.try_into()?;
        Ok(())
    }

    /// Sets the micro timing.
    pub fn set_micro_timing(&mut self, micro_timing: MicroTime) -> Result<(), RytmError> {
        self.micro_timing = micro_timing;
        Ok(())
    }

    /// Sets the note length value.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "note_length:0..=127")]
    pub fn set_note_length_by_value(&mut self, note_length: usize) -> Result<(), RytmError> {
        self.note_length = (note_length as u8).try_into()?;
        Ok(())
    }

    /// Sets the note_length
    pub fn set_note_length(&mut self, note_length: Length) -> Result<(), RytmError> {
        self.note_length = note_length;
        Ok(())
    }

    /// Sets the retrig length value.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "retrig_length:0..=127")]
    pub fn set_retrig_length_by_value(&mut self, retrig_length: usize) -> Result<(), RytmError> {
        self.retrig_length = (retrig_length as u8).try_into()?;
        Ok(())
    }

    /// Sets the retrig length.
    pub fn set_retrig_length(&mut self, retrig_length: Length) -> Result<(), RytmError> {
        self.retrig_length = retrig_length;
        Ok(())
    }

    /// Sets the retrig rate.
    pub fn set_retrig_rate(&mut self, retrig_rate: RetrigRate) -> Result<(), RytmError> {
        self.retrig_rate = retrig_rate;
        Ok(())
    }

    /// Sets the trig condition.
    pub fn set_trig_condition(&mut self, trig_condition: TrigCondition) -> Result<(), RytmError> {
        self.trig_condition = trig_condition;
        Ok(())
    }

    /// Sets retrig velocity offset.
    ///
    /// Range `-128..=127`
    #[parameter_range(range = "retrig_velocity_offset:-128..=127")]
    pub fn set_retrig_velocity_offset(
        &mut self,
        retrig_velocity_offset: isize,
    ) -> Result<(), RytmError> {
        self.retrig_velocity_offset = retrig_velocity_offset as i8;
        Ok(())
    }

    /// Sets the sound lock value.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "sound_lock:0..=127")]
    pub fn set_sound_lock(&mut self, sound_lock: usize) -> Result<(), RytmError> {
        self.sound_lock = sound_lock as u8;
        Ok(())
    }

    /// Returns the note value.
    ///
    /// Follows the midi note convention. C-4 is `0x3C`.
    pub fn note(&self) -> usize {
        self.note as usize
    }

    /// Returns the value of the trig condition.
    pub fn trig_condition(&self) -> TrigCondition {
        self.trig_condition
    }

    /// Returns the velocity value.
    ///
    /// Range `1..=127`
    pub fn velocity(&self) -> usize {
        self.velocity as usize
    }

    /// Returns the micro timing value.
    ///
    /// Range `-23..=23`
    pub fn micro_timing_value(&self) -> isize {
        self.micro_timing as isize
    }

    /// Returns the micro timing.
    pub fn micro_timing(&self) -> MicroTime {
        self.micro_timing
    }

    /// Returns the note length value.
    ///
    /// Range `0..=127`
    pub fn note_length_value(&self) -> usize {
        self.note_length as usize
    }

    /// Returns the note length.
    pub fn note_length(&self) -> Length {
        self.note_length
    }

    /// Returns the retrig length value.
    ///
    /// Range `0..=127`
    pub fn retrig_length_value(&self) -> usize {
        self.retrig_length as usize
    }

    /// Returns the retrig length.
    pub fn retrig_length(&self) -> Length {
        self.retrig_length
    }

    /// Returns the retrig rate.
    pub fn retrig_rate(&self) -> RetrigRate {
        self.retrig_rate
    }

    /// Returns the retrig velocity offset.
    ///
    /// Range `-128..=127`
    pub fn retrig_velocity_offset(&self) -> isize {
        self.retrig_velocity_offset as isize
    }

    /// Returns the sound lock value.
    ///
    /// Range `0..=127`
    pub fn sound_lock(&self) -> usize {
        self.sound_lock as usize
    }

    /********* Parameter Lock Setters *********/

    // Filter

    /// Sets a parameter lock for the filter attack.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_attack:0..=127")]
    pub fn p_lock_set_filter_attack(&self, filter_attack: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_ATTACK as u8,
                filter_attack as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter sustain.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_sustain:0..=127")]
    pub fn p_lock_set_filter_sustain(&self, filter_sustain: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_SUSTAIN as u8,
                filter_sustain as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter decay.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_decay:0..=127")]
    pub fn p_lock_set_filter_decay(&self, filter_decay: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_DECAY as u8,
                filter_decay as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter release.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_release:0..=127")]
    pub fn p_lock_set_filter_release(&self, filter_release: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_RELEASE as u8,
                filter_release as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter cutoff.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_cutoff:0..=127")]
    pub fn p_lock_set_filter_cutoff(&self, filter_cutoff: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_FREQ as u8,
                filter_cutoff as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter resonance.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_resonance:0..=127")]
    pub fn p_lock_set_filter_resonance(&self, filter_resonance: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_RESO as u8,
                filter_resonance as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter envelope amount.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "filter_envelope_amount:-64..=63")]
    pub fn p_lock_set_filter_envelope_amount(
        &self,
        filter_envelope_amount: isize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_ENV as u8,
                i8_to_u8_midpoint_of_u8_input_range(filter_envelope_amount as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    // Amplitude

    /// Sets a parameter lock for the amplitude attack.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_attack:0..=127")]
    pub fn p_lock_set_amplitude_attack(&self, amplitude_attack: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_ATTACK as u8,
                amplitude_attack as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude hold.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_hold:0..=127")]
    pub fn p_lock_set_amplitude_hold(&self, amplitude_hold: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_HOLD as u8,
                amplitude_hold as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude decay.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_decay:0..=127")]
    pub fn p_lock_set_amplitude_decay(&self, amplitude_decay: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DECAY as u8,
                amplitude_decay as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude overdrive.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_overdrive:0..=127")]
    pub fn p_lock_set_amplitude_overdrive(
        &self,
        amplitude_overdrive: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DRIVE as u8,
                amplitude_overdrive as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude delay send.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_delay_send:0..=127")]
    pub fn p_lock_set_amplitude_delay_send(
        &self,
        amplitude_delay_send: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DELAY as u8,
                amplitude_delay_send as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude reverb send.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_reverb_send:0..=127")]
    pub fn p_lock_set_amplitude_reverb_send(
        &self,
        amplitude_reverb_send: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_REVERB as u8,
                amplitude_reverb_send as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude pan.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "amplitude_pan:-64..=63")]
    pub fn p_lock_set_amplitude_pan(&self, amplitude_pan: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_PAN as u8,
                i8_to_u8_midpoint_of_u8_input_range(amplitude_pan as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_volume:0..=127")]
    pub fn p_lock_set_amplitude_volume(&self, amplitude_volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_VOLUME as u8,
                amplitude_volume as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    // Sample

    /// Sets a parameter lock for the sample tune.
    ///
    /// Range `-24..=24`
    #[parameter_range(range = "sample_tune:-24..=24")]
    pub fn p_lock_set_sample_tune(&self, sample_tune: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_TUNE as u8,
                i8_to_u8_midpoint_of_u8_input_range(sample_tune as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample fine tune.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "sample_fine_tune:-64..=63")]
    pub fn p_lock_set_sample_fine_tune(&self, sample_fine_tune: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_FINE as u8,
                i8_to_u8_midpoint_of_u8_input_range(sample_fine_tune as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample number.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "sample_number:0..=127")]
    pub fn p_lock_set_sample_number(&self, sample_number: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_NR as u8,
                sample_number as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample bit reduction.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "sample_bit_reduction:0..=127")]
    pub fn p_lock_set_sample_bit_reduction(
        &self,
        sample_bit_reduction: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_BITRDC as u8,
                sample_bit_reduction as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample start.
    ///
    /// Range `0.0..=120.0`
    #[parameter_range(range = "sample_start:0.0..=120.0")]
    pub fn p_lock_set_sample_start(&self, sample_start: f32) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let start = scale_generic(sample_start, 0f32, 120.0f32, 0u16, 30720u16, |x| {
                x.round() as u16
            });

            pool.borrow_mut().set_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_START as u8,
                start,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample end.
    ///
    /// Range `0.0..=120.0`
    #[parameter_range(range = "sample_end:0.0..=120.0")]
    pub fn p_lock_set_sample_end(&self, sample_end: f32) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let end = scale_generic(sample_end, 0f32, 120.0f32, 0u16, 30720u16, |x| {
                x.round() as u16
            });

            pool.borrow_mut().set_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_END as u8,
                end,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample loop flag.
    pub fn p_lock_set_sample_loop_flag(&self, sample_loop_flag: bool) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_LOOPSW as u8,
                sample_loop_flag as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "sample_volume:0..=127")]
    pub fn p_lock_set_sample_volume(&self, sample_volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_LEVEL as u8,
                sample_volume as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    // LFO

    /// Sets a parameter lock for the LFO speed.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "lfo_speed:-64..=63")]
    pub fn p_lock_set_lfo_speed(&self, lfo_speed: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_SPEED as u8,
                i8_to_u8_midpoint_of_u8_input_range(lfo_speed as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO multiplier.
    pub fn p_lock_set_lfo_multiplier(
        &self,
        lfo_multiplier: LfoMultiplier,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_MULTIPLY as u8,
                lfo_multiplier.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO fade.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "lfo_fade:-64..=63")]
    pub fn p_lock_set_lfo_fade(&self, lfo_fade: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_FADE as u8,
                i8_to_u8_midpoint_of_u8_input_range(lfo_fade as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO destination.
    pub fn p_lock_set_lfo_destination(
        &self,
        lfo_destination: LfoDestination,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_DEST as u8,
                lfo_destination.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO waveform.
    pub fn p_lock_set_lfo_waveform(&self, lfo_waveform: LfoWaveform) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_WAVEFORM as u8,
                lfo_waveform.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO start phase.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "lfo_start_phase:0..=127")]
    pub fn p_lock_set_lfo_start_phase(&self, lfo_start_phase: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_PHASE as u8,
                lfo_start_phase as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO mode.
    pub fn p_lock_set_lfo_mode(&self, lfo_mode: LfoMode) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_TRIGMODE as u8,
                lfo_mode.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO depth.
    ///
    /// Range `-128.0..=127.99`
    #[parameter_range(range = "lfo_depth:-128.0..=127.99")]
    pub fn p_lock_set_lfo_depth(&self, lfo_depth: f32) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let depth = scale_generic(lfo_depth, -128f32, 127.99f32, 0u16, 32767u16, |x| {
                x.round() as u16
            });

            pool.borrow_mut().set_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_DEPTH as u8,
                depth,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    // TODO: Syn param setters

    // FX Comp

    /// Sets a parameter lock for the FX compressor threshold.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "threshold:0..=127")]
    pub fn p_lock_set_fx_compressor_threshold(&self, threshold: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_THRESHOLD as u8,
                threshold as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor attack.
    pub fn p_lock_set_fx_compressor_attack(&self, attack: FxCompAttack) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_ATTACK as u8,
                attack.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor release.
    pub fn p_lock_set_fx_compressor_release(
        &self,
        release: FxCompRelease,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_RELEASE as u8,
                release.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor ratio.
    pub fn p_lock_set_fx_compressor_ratio(&self, ratio: FxCompRatio) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_RATIO as u8,
                ratio.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor side chain eq.
    pub fn p_lock_set_fx_compressor_side_chain_eq(
        &self,
        seq: FxCompSideChainEq,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_SEQ as u8,
                seq.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor gain.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "gain:0..=127")]
    pub fn p_lock_set_fx_compressor_gain(&self, gain: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_MAKEUP as u8,
                gain as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor mix.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "mix:0..=127")]
    pub fn p_lock_set_fx_compressor_mix(&self, mix: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_MIX as u8,
                mix as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn p_lock_set_fx_compressor_volume(&self, volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_VOL as u8,
                volume as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    // FX Reverb

    /// Sets a parameter lock for the FX reverb pre delay.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "pre_delay:0..=127")]
    pub fn p_lock_set_fx_reverb_pre_delay(&self, pre_delay: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_PRE as u8,
                pre_delay as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb decay.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "decay:0..=127")]
    pub fn p_lock_set_fx_reverb_decay(&self, decay: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_DECAY as u8,
                decay as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb frequency.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "freq:0..=127")]
    pub fn p_lock_set_fx_reverb_freq(&self, freq: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_FREQ as u8,
                freq as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb gain.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "gain:0..=127")]
    pub fn p_lock_set_fx_reverb_gain(&self, gain: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_GAIN as u8,
                gain as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb high pass filter.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "hpf:0..=127")]
    pub fn p_lock_set_fx_reverb_hpf(&self, hpf: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_HPF as u8,
                hpf as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb low pass filter.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "lpf:0..=127")]
    pub fn p_lock_set_fx_reverb_lpf(&self, lpf: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_LPF as u8,
                lpf as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn p_lock_set_fx_reverb_volume(&self, volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_VOL as u8,
                volume as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    // FX Delay

    /// Sets a parameter lock for the FX delay time.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "time:0..=127")]
    pub fn p_lock_set_fx_delay_time(&self, time: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_TIME as u8,
                time as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay ping pong.
    pub fn p_lock_set_fx_delay_ping_pong(&self, ping_pong: bool) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_PING_PONG as u8,
                ping_pong as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay stereo width.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "stereo_width:-64..=63")]
    pub fn p_lock_set_fx_delay_stereo_width(&self, stereo_width: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_WIDTH as u8,
                i8_to_u8_midpoint_of_u8_input_range(stereo_width as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay feedback.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "feedback:0..=127")]
    pub fn p_lock_set_fx_delay_feedback(&self, feedback: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_FEEDBACK as u8,
                feedback as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay high pass filter.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "hpf:0..=127")]
    pub fn p_lock_set_fx_delay_hpf(&self, hpf: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_HPF as u8,
                hpf as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay low pass filter.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "lpf:0..=127")]
    pub fn p_lock_set_fx_delay_lpf(&self, lpf: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_LPF as u8,
                lpf as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay reverb send.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "reverb_send:0..=127")]
    pub fn p_lock_set_fx_delay_reverb_send(&self, reverb_send: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_REV as u8,
                reverb_send as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn p_lock_set_fx_delay_volume(&self, volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_VOL as u8,
                volume as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    // Fx Distortion

    /// Sets a parameter lock for the FX distortion reverb send.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "reverb_send:0..=127")]
    pub fn p_lock_set_fx_distortion_reverb_send(
        &self,
        reverb_send: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_REV as u8,
                reverb_send as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX distortion delay overdrive.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "delay_overdrive:0..=127")]
    pub fn p_lock_set_fx_distortion_delay_overdrive(
        &self,
        delay_overdrive: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_DOV as u8,
                delay_overdrive as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX distortion reverb post.
    ///
    /// `true` = post, `false` = pre
    pub fn p_lock_set_fx_distortion_reverb_post(&self, reverb_post: bool) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_REV as u8,
                reverb_post as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX distortion amount.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amount:0..=127")]
    pub fn p_lock_set_fx_distortion_amount(&self, amount: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_AMOUNT as u8,
                amount as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX distortion symmetry.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "symmetry:-64..=63")]
    pub fn p_lock_set_fx_distortion_symmetry(&self, symmetry: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_SYM as u8,
                i8_to_u8_midpoint_of_u8_input_range(symmetry as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    // Fx LFO

    /// Sets a parameter lock for the FX LFO speed.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "speed:-64..=63")]
    pub fn p_lock_set_fx_lfo_speed(&self, speed: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_SPEED as u8,
                i8_to_u8_midpoint_of_u8_input_range(speed as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO multiplier.
    pub fn p_lock_set_fx_lfo_multiplier(&self, multiplier: LfoMultiplier) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_MULTIPLY as u8,
                multiplier.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO fade.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "fade:-64..=63")]
    pub fn p_lock_set_fx_lfo_fade(&self, fade: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_FADE as u8,
                i8_to_u8_midpoint_of_u8_input_range(fade as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO destination.
    pub fn p_lock_set_fx_lfo_destination(
        &self,
        destination: FxLfoDestination,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_DEST as u8,
                destination.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO waveform.
    pub fn p_lock_set_fx_lfo_waveform(&self, waveform: LfoWaveform) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_WAVEFORM as u8,
                waveform.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO start phase.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "start_phase:0..=127")]
    pub fn p_lock_set_fx_lfo_start_phase(&self, start_phase: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_PHASE as u8,
                start_phase as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO mode.
    pub fn p_lock_set_fx_lfo_mode(&self, mode: LfoMode) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_MOD as u8,
                mode.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO depth.
    ///
    /// Range `-128.0..=127.99`
    #[parameter_range(range = "depth:-128.0..=127.99")]
    pub fn p_lock_set_fx_lfo_depth(&self, depth: f32) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let depth = scale_generic(depth, -128f32, 127.99f32, 0u16, 32767u16, |x| {
                x.round() as u16
            });

            pool.borrow_mut().set_fx_compound_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_DEPTH as u8,
                depth,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }
}

impl HoldsTrigFlags for Trig {
    fn raw_trig_flags(&self) -> u16 {
        self.flags.raw_trig_flags()
    }

    fn raw_trig_flags_mut(&mut self) -> &mut u16 {
        self.flags.raw_trig_flags_mut()
    }
}

impl std::fmt::Display for TrigFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04b}_{:04b}_{:04b}_{:04b} - {}",
            (self.raw_trig_flags() >> 12) & 0b1111,
            (self.raw_trig_flags() >> 8) & 0b1111,
            (self.raw_trig_flags() >> 4) & 0b1111,
            self.raw_trig_flags() & 0b1111,
            format_trig_flags(self).join(", ")
        )
    }
}

impl std::fmt::Debug for TrigFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04b}_{:04b}_{:04b}_{:04b} - {}",
            (self.raw_trig_flags() >> 12) & 0b1111,
            (self.raw_trig_flags() >> 8) & 0b1111,
            (self.raw_trig_flags() >> 4) & 0b1111,
            self.raw_trig_flags() & 0b1111,
            format_trig_flags(self).join(", ")
        )
    }
}

fn format_trig_flags(trig: &impl HoldsTrigFlags) -> Vec<&str> {
    let mut flags = vec![];

    if trig.enabled_trig() {
        flags.push("ENABLE");
    }

    if trig.enabled_retrig() {
        flags.push("RETRIG");
    }

    if trig.enabled_mute() {
        flags.push("MUTE");
    }

    if trig.enabled_accent() {
        flags.push("ACCENT");
    }

    if trig.enabled_swing() {
        flags.push("SWING");
    }

    if trig.enabled_slide() {
        flags.push("SLIDE");
    }

    if trig.enabled_parameter_lock_lfo_switch() {
        flags.push("LFO_PL_SW");
    }

    if trig.enabled_parameter_lock_lfo() {
        flags.push("LFO_PL_EN");
    }

    if trig.enabled_parameter_lock_synth_switch() {
        flags.push("SYN_PL_SW");
    }

    if trig.enabled_parameter_lock_synth() {
        flags.push("SYN_PL_EN");
    }

    if trig.enabled_parameter_lock_sample_switch() {
        flags.push("SMP_PL_SW");
    }

    if trig.enabled_parameter_lock_sample() {
        flags.push("SMP_PL_EN");
    }

    if trig.enabled_parameter_lock_env_switch() {
        flags.push("ENV_PL_SW");
    }

    if trig.enabled_parameter_lock_env() {
        flags.push("ENV_PL_EN");
    }

    flags
}

impl Trig {
    pub(crate) fn default_trig_array(track_index: usize) -> [Trig; 64] {
        [
            Trig::try_default(0, track_index).unwrap(),
            Trig::try_default(1, track_index).unwrap(),
            Trig::try_default(2, track_index).unwrap(),
            Trig::try_default(3, track_index).unwrap(),
            Trig::try_default(4, track_index).unwrap(),
            Trig::try_default(5, track_index).unwrap(),
            Trig::try_default(6, track_index).unwrap(),
            Trig::try_default(7, track_index).unwrap(),
            Trig::try_default(8, track_index).unwrap(),
            Trig::try_default(9, track_index).unwrap(),
            Trig::try_default(10, track_index).unwrap(),
            Trig::try_default(11, track_index).unwrap(),
            Trig::try_default(12, track_index).unwrap(),
            Trig::try_default(13, track_index).unwrap(),
            Trig::try_default(14, track_index).unwrap(),
            Trig::try_default(15, track_index).unwrap(),
            Trig::try_default(16, track_index).unwrap(),
            Trig::try_default(17, track_index).unwrap(),
            Trig::try_default(18, track_index).unwrap(),
            Trig::try_default(19, track_index).unwrap(),
            Trig::try_default(20, track_index).unwrap(),
            Trig::try_default(21, track_index).unwrap(),
            Trig::try_default(22, track_index).unwrap(),
            Trig::try_default(23, track_index).unwrap(),
            Trig::try_default(24, track_index).unwrap(),
            Trig::try_default(25, track_index).unwrap(),
            Trig::try_default(26, track_index).unwrap(),
            Trig::try_default(27, track_index).unwrap(),
            Trig::try_default(28, track_index).unwrap(),
            Trig::try_default(29, track_index).unwrap(),
            Trig::try_default(30, track_index).unwrap(),
            Trig::try_default(31, track_index).unwrap(),
            Trig::try_default(32, track_index).unwrap(),
            Trig::try_default(33, track_index).unwrap(),
            Trig::try_default(34, track_index).unwrap(),
            Trig::try_default(35, track_index).unwrap(),
            Trig::try_default(36, track_index).unwrap(),
            Trig::try_default(37, track_index).unwrap(),
            Trig::try_default(38, track_index).unwrap(),
            Trig::try_default(39, track_index).unwrap(),
            Trig::try_default(40, track_index).unwrap(),
            Trig::try_default(41, track_index).unwrap(),
            Trig::try_default(42, track_index).unwrap(),
            Trig::try_default(43, track_index).unwrap(),
            Trig::try_default(44, track_index).unwrap(),
            Trig::try_default(45, track_index).unwrap(),
            Trig::try_default(46, track_index).unwrap(),
            Trig::try_default(47, track_index).unwrap(),
            Trig::try_default(48, track_index).unwrap(),
            Trig::try_default(49, track_index).unwrap(),
            Trig::try_default(50, track_index).unwrap(),
            Trig::try_default(51, track_index).unwrap(),
            Trig::try_default(52, track_index).unwrap(),
            Trig::try_default(53, track_index).unwrap(),
            Trig::try_default(54, track_index).unwrap(),
            Trig::try_default(55, track_index).unwrap(),
            Trig::try_default(56, track_index).unwrap(),
            Trig::try_default(57, track_index).unwrap(),
            Trig::try_default(58, track_index).unwrap(),
            Trig::try_default(59, track_index).unwrap(),
            Trig::try_default(60, track_index).unwrap(),
            Trig::try_default(61, track_index).unwrap(),
            Trig::try_default(62, track_index).unwrap(),
            Trig::try_default(63, track_index).unwrap(),
        ]
    }
}
