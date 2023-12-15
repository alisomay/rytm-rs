// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use crate::{
    error::{ConversionError, ParameterError, RytmError},
    util::{
        from_s_u16_t, i8_to_u8_midpoint_of_u8_input_range, scale_f32_to_u16, scale_u16_to_f32,
        to_s_u16_t_union_a, u8_to_i8_midpoint_of_u8_input_range,
    },
};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;

/// Represents parameters in the sample page of a sound.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sample {
    tune: i8,
    fine_tune: i8,
    number: u8,
    bit_reduction: u8,
    start: f32,
    end: f32,
    loop_flag: bool,
    volume: u8,
}

impl Default for Sample {
    fn default() -> Self {
        Self {
            tune: 0,
            fine_tune: 0,
            number: 0,
            bit_reduction: 0,
            start: 0.0,
            end: 120.0,
            loop_flag: false,
            volume: 100,
        }
    }
}

impl TryFrom<&ar_sound_t> for Sample {
    type Error = ConversionError;
    fn try_from(raw_sound: &ar_sound_t) -> Result<Self, Self::Error> {
        let start = scale_u16_to_f32(
            unsafe { from_s_u16_t(raw_sound.sample_start) },
            0u16,
            30720u16,
            0f32,
            120.0f32,
        );

        let end = scale_u16_to_f32(
            unsafe { from_s_u16_t(raw_sound.sample_end) },
            0u16,
            30720u16,
            0f32,
            120.0f32,
        );

        Ok(Self {
            tune: u8_to_i8_midpoint_of_u8_input_range(raw_sound.sample_tune, 127, 0),
            fine_tune: u8_to_i8_midpoint_of_u8_input_range(raw_sound.sample_fine_tune, 127, 0),
            number: raw_sound.sample_nr,
            bit_reduction: raw_sound.sample_br,
            start,
            end,
            loop_flag: raw_sound.sample_loop_flag != 0,
            volume: raw_sound.sample_volume,
        })
    }
}

impl Sample {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        // // map range of 0.0..=120.0 to u16 0..=30720
        let start = scale_f32_to_u16(self.start, 0f32, 120.0f32, 0u16, 30720u16);
        let end = scale_f32_to_u16(self.end, 0f32, 120.0f32, 0u16, 30720u16);

        raw_sound.sample_tune = i8_to_u8_midpoint_of_u8_input_range(self.tune, 127, 0);
        raw_sound.sample_fine_tune = i8_to_u8_midpoint_of_u8_input_range(self.fine_tune, 127, 0);
        raw_sound.sample_nr = self.number;
        raw_sound.sample_br = self.bit_reduction;

        raw_sound.sample_start = to_s_u16_t_union_a(start);
        raw_sound.sample_end = to_s_u16_t_union_a(end);
        raw_sound.sample_loop_flag = self.loop_flag as u8;
        raw_sound.sample_volume = self.volume;
    }

    /// Sets the coarse tune of the sample.
    ///
    /// Range: `-24..=24`
    #[parameter_range(range = "tune:-24..=24")]
    pub fn set_tune(&mut self, tune: isize) -> Result<(), RytmError> {
        self.tune = tune as i8;
        Ok(())
    }

    /// Sets the fine tune of the sample.
    #[parameter_range(range = "fine_tune:-64..=63")]
    pub fn set_fine_tune(&mut self, fine_tune: isize) -> Result<(), RytmError> {
        self.fine_tune = fine_tune as i8;
        Ok(())
    }

    /// Sets the slice number of the sample.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "number:0..=127")]
    pub fn set_slice_number(&mut self, number: usize) -> Result<(), RytmError> {
        self.number = number as u8;
        Ok(())
    }

    /// Unsets the sample slice.
    ///
    /// Synonym with `SMP OFF`.
    pub fn unset_slice(&mut self) {
        // TODO: Double check
        self.number = 0xFF;
    }

    /// Sets the bit reduction of the sample.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "bit_reduction:0..=127")]
    pub fn set_bit_reduction(&mut self, bit_reduction: usize) -> Result<(), RytmError> {
        self.bit_reduction = bit_reduction as u8;
        Ok(())
    }

    /// Sets the start of the sample.
    ///
    /// Range: `0.0..=120.0`
    #[parameter_range(range = "start:0.0..=120.0")]
    pub fn set_start(&mut self, start: f32) -> Result<(), RytmError> {
        self.start = start;
        Ok(())
    }

    /// Sets the end of the sample.
    ///
    /// Range: `0.0..=120.0`
    #[parameter_range(range = "end:0.0..=120.0")]
    pub fn set_end(&mut self, end: f32) -> Result<(), RytmError> {
        self.end = end;
        Ok(())
    }

    /// Sets the loop flag of the sample.
    pub fn set_loop_flag(&mut self, loop_flag: bool) {
        self.loop_flag = loop_flag;
    }

    /// Sets the volume of the sample.
    #[parameter_range(range = "volume:0..=127")]
    pub fn set_volume(&mut self, volume: usize) -> Result<(), RytmError> {
        self.volume = volume as u8;
        Ok(())
    }

    // Returns the coarse tune of the sample.
    pub const fn tune(&self) -> isize {
        self.tune as isize
    }

    // Returns the fine tune of the sample.
    pub const fn fine_tune(&self) -> isize {
        self.fine_tune as isize
    }

    // Returns the slice number of the sample.
    pub const fn slice_number(&self) -> usize {
        self.number as usize
    }

    // Returns the bit reduction of the sample.
    pub const fn bit_reduction(&self) -> usize {
        self.bit_reduction as usize
    }

    // Returns the start of the sample.
    pub const fn start(&self) -> f32 {
        self.start
    }

    // Returns the end of the sample.
    pub const fn end(&self) -> f32 {
        self.end
    }

    // Returns the loop flag of the sample.
    pub const fn loop_flag(&self) -> bool {
        self.loop_flag
    }

    // Returns the volume of the sample.
    pub const fn volume(&self) -> usize {
        self.volume as usize
    }
}
