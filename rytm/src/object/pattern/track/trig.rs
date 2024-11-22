// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

// TODO: Document PL_SW and PL_EN flags.
// TODO: Maybe builder..

/// Holds the raw trig flag types
pub mod flags;
mod plock_impl;
/// Holds the trig types.
pub mod types;

use self::types::{Length, MicroTime, RetrigRate, TrigCondition};
use super::Track;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::decode_micro_timing_byte,
};
use derivative::Derivative;
use flags::*;
use parking_lot::Mutex;
use rytm_rs_macro::parameter_range;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

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

#[derive(Clone, Copy, Serialize, Deserialize)]
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

#[derive(Derivative, Clone, Serialize, Deserialize)]
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
    /// Range `36..=84`
    ///
    /// Follows the midi note convention. C-4 is `0x3C`.
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
    #[serde(skip)]
    pub(crate) parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,

    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    pub(crate) fx_track_ref: Option<Arc<Mutex<Track>>>,
}

impl Trig {
    /// Makes a new trig complying to project defaults.
    ///
    /// The trig index and track index are required.
    ///
    /// Range `0..=63` for trig index.
    /// Range `0..=12` for track index.
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
            note: 60, // 36 to 84 is valid on device.
            trig_condition: TrigCondition::default(),
            velocity: 0xFF,
            note_length: Length::Unset,
            micro_timing: MicroTime::default(),
            retrig_rate: RetrigRate::default(),
            retrig_length: Length::Quarter,
            retrig_velocity_offset: 0,
            sound_lock: 0xFF,
            parameter_lock_pool: None,
            fx_track_ref: None,
        })
    }

    /// Makes a new trig from raw values.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
        parameter_lock_pool: Arc<Mutex<ParameterLockPool>>,
        fx_track_ref: Option<Arc<Mutex<Track>>>,
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

        // When the trig condition is unset in the device this is how the raw data looks like:
        // Although this is taken from a new untouched project so everything is in their device defaults.

        // note: 0xFF          // All bits 1
        // micro_timing: 0xC0  // Top 2 bits 1 (0b11xxxxxx)
        // retrig_length: 0x80 // Top bit 1  (0b1xxxxxxx)
        // retrig_rate: 0xE0   // Top 3 bits 1 (0b111xxxxx)

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
            fx_track_ref,
        })
    }

    pub(crate) const fn encode_note(&self) -> u8 {
        // Always use standard encoding, even for Unset
        // This preserves note value in lower 7 bits (0-127 range)
        // and uses MSB for trig condition AGR_6 bit
        ((self.trig_condition as u8) & 0b0_1000000) << 1 | self.note
    }

    #[allow(overflowing_literals)]
    pub(crate) fn encode_micro_timing(&self) -> u8 {
        let encoded_byte = crate::util::encode_micro_timing_byte(self.micro_timing);
        if self.trig_condition == TrigCondition::Unset {
            // When unset, set top 2 bits (0b1100_0000) and keep the micro timing value
            0xC0 | ((encoded_byte >> 2) as u8)
        } else {
            // Shift the micro timing 2 bits to the right to leave space for 2 bits which is a part of encoded trig condition.
            // Then fill those two bits with the trig condition's most significant mid bits.
            //
            // Since we're just setting bits, fabricating values and not doing any arithmetic we can use the literal values.
            // Overflowing literals are safe in this case.
            ((encoded_byte >> 2) | ((self.trig_condition as i8 & 0b0_0110000) << 2)) as u8
        }
    }

    pub(crate) fn encode_retrig_length(&self) -> u8 {
        if self.trig_condition == TrigCondition::Unset {
            // When unset, set high bit (0b1000_0000) and keep the length value
            0x80 | (self.retrig_length as u8)
        } else {
            // Apply the trig condition's least significant mid bit to the retrig length's most significant bit.
            ((self.trig_condition as u8) & 0b0_0001000) << 4 | self.retrig_length as u8
        }
    }

    pub(crate) fn encode_retrig_rate(&self) -> u8 {
        if self.trig_condition == TrigCondition::Unset {
            // When unset, set top 3 bits (0b1110_0000) and keep the rate value
            0xE0 | (self.retrig_rate as u8)
        } else {
            // Apply the trig condition's least significant 3 bits to the retrig rate's most significant 3 bits.
            ((self.trig_condition as u8) & 0b0_0000111) << 5 | self.retrig_rate as u8
        }
    }

    /// Returns the index of the trig.
    ///
    /// Range `0..=63`
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the index of the track which this trig belongs to.
    ///
    /// Range `0..=12`
    pub const fn track_index(&self) -> usize {
        self.track_index
    }

    // TODO: Can we try to exceed the range how does the device respond?
    /// Sets the note value.
    ///
    /// Range `36..=84`
    ///
    /// Follows the midi note convention. C-4 is `0x3C`.
    #[parameter_range(range = "note:36..=84")]
    pub fn set_note(&mut self, note: usize) -> Result<(), RytmError> {
        self.note = note as u8;
        Ok(())
    }

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
    pub fn set_micro_timing(&mut self, micro_timing: MicroTime) {
        self.micro_timing = micro_timing;
    }

    /// Sets the note length value.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "note_length:0..=127")]
    pub fn set_note_length_by_value(&mut self, note_length: usize) -> Result<(), RytmError> {
        self.note_length = (note_length as u8).try_into()?;
        Ok(())
    }

    #[allow(clippy::doc_markdown)]
    /// Sets the note_length.
    pub fn set_note_length(&mut self, note_length: Length) {
        self.note_length = note_length;
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
    pub fn set_retrig_length(&mut self, retrig_length: Length) {
        self.retrig_length = retrig_length;
    }

    /// Sets the retrig rate.
    pub fn set_retrig_rate(&mut self, retrig_rate: RetrigRate) {
        self.retrig_rate = retrig_rate;
    }

    /// Sets the trig condition.
    pub fn set_trig_condition(&mut self, trig_condition: TrigCondition) {
        self.trig_condition = trig_condition;
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
    pub const fn note(&self) -> usize {
        self.note as usize
    }

    /// Returns the value of the trig condition.
    pub const fn trig_condition(&self) -> TrigCondition {
        self.trig_condition
    }

    /// Returns the velocity value.
    ///
    /// Range `1..=127`
    pub const fn velocity(&self) -> usize {
        self.velocity as usize
    }

    /// Returns the micro timing value.
    ///
    /// Range `-23..=23`
    pub const fn micro_timing_value(&self) -> isize {
        self.micro_timing as isize
    }

    /// Returns the micro timing.
    pub const fn micro_timing(&self) -> MicroTime {
        self.micro_timing
    }

    /// Returns the note length value.
    ///
    /// Range `0..=127`
    pub const fn note_length_value(&self) -> usize {
        self.note_length as usize
    }

    /// Returns the note length.
    pub const fn note_length(&self) -> Length {
        self.note_length
    }

    /// Returns the retrig length value.
    ///
    /// Range `0..=127`
    pub const fn retrig_length_value(&self) -> usize {
        self.retrig_length as usize
    }

    /// Returns the retrig length.
    pub const fn retrig_length(&self) -> Length {
        self.retrig_length
    }

    /// Returns the retrig rate.
    pub const fn retrig_rate(&self) -> RetrigRate {
        self.retrig_rate
    }

    /// Returns the retrig velocity offset.
    ///
    /// Range `-128..=127`
    pub const fn retrig_velocity_offset(&self) -> isize {
        self.retrig_velocity_offset as isize
    }

    /// Returns the sound lock value.
    ///
    /// Range `0..=127`
    pub const fn sound_lock(&self) -> usize {
        self.sound_lock as usize
    }

    /// Returns if this trig is a trig for the FX track.
    pub const fn is_fx_trig(&self) -> bool {
        self.track_index == 12
    }

    /// Utility method to be called in fx plock setter.
    ///
    /// When a parameter lock is set regarding fx, it is necessary to sync the fx track's trigs.
    pub(crate) fn enable_fx_trig_if_necessary(&self) {
        // Enable fx trig if this method is called on a non-fx trig.
        if let Some(ref fx_track_ref) = self.fx_track_ref {
            let mut borrow = fx_track_ref.lock();
            let fx_trig = &mut borrow.trigs_mut()[self.index];
            fx_trig.set_trig_enable(true);
            drop(borrow);
        }
    }

    /// Utility method to be called in fx plock clearer.
    ///
    /// When a parameter lock is cleared regarding fx, it is necessary to sync the fx track's trigs.
    pub(crate) fn disable_fx_trig_if_necessary(&self) {
        // Disable fx trig if this method is called on a non-fx trig.
        if let Some(ref fx_track_ref) = self.fx_track_ref {
            let mut borrow = fx_track_ref.lock();
            let fx_trig = &mut borrow.trigs_mut()[self.index];
            fx_trig.set_trig_enable(false);
            drop(borrow);
        }
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
