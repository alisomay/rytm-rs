use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;

use crate::{
    error::{ConversionError, ParameterError, RytmError},
    util::{from_s_u16_t, to_s_u16_t_union_a},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sample {
    tune: i8,
    fine_tune: i8,
    number: u8,
    bit_reduction: u8,
    start: f64,
    end: f64,
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
            end: 0.0,
            loop_flag: false,
            volume: 0,
        }
    }
}

impl TryFrom<&ar_sound_t> for Sample {
    type Error = ConversionError;
    fn try_from(raw_sound: &ar_sound_t) -> Result<Self, Self::Error> {
        // // map range of u16 0..=30720 to 0.0..=120.0
        let start = (unsafe { from_s_u16_t(&raw_sound.sample_start) } as f64 / 30720.0) * 120.0;
        let end = (unsafe { from_s_u16_t(&raw_sound.sample_end) } as f64 / 30720.0) * 120.0;

        Ok(Self {
            // 40..=88 device -24..=24
            // TODO: Double check
            tune: raw_sound.sample_tune as i8 - 64,
            fine_tune: raw_sound.sample_fine_tune as i8 - 64,
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
        raw_sound.sample_tune = self.tune as u8 + 64;
        raw_sound.sample_fine_tune = self.fine_tune as u8 + 64;
        raw_sound.sample_nr = self.number;
        raw_sound.sample_br = self.bit_reduction;
        // // map range of 0.0..=120.0 to u16 0..=30720
        raw_sound.sample_start = to_s_u16_t_union_a(((self.start / 120.0) * 30720.0) as u16);
        raw_sound.sample_end = to_s_u16_t_union_a(((self.end / 120.0) * 30720.0) as u16);
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
    /// Synonym with `SMP OFF``.
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
    #[parameter_range(range = "start:0.0..=120.0:f64")]
    pub fn set_start(&mut self, start: f64) -> Result<(), RytmError> {
        self.start = start;
        Ok(())
    }

    /// Sets the end of the sample.
    ///
    /// Range: `0.0..=120.0`
    #[parameter_range(range = "end:0.0..=120.0:f64")]
    pub fn set_end(&mut self, end: f64) -> Result<(), RytmError> {
        self.end = end;
        Ok(())
    }

    /// Sets the loop flag of the sample.
    pub fn set_loop_flag(&mut self, loop_flag: bool) -> Result<(), RytmError> {
        self.loop_flag = loop_flag;
        Ok(())
    }

    /// Sets the volume of the sample.
    #[parameter_range(range = "volume:0..=127")]
    pub fn set_volume(&mut self, volume: usize) -> Result<(), RytmError> {
        self.volume = volume as u8;
        Ok(())
    }

    // Returns the coarse tune of the sample.
    pub fn tune(&self) -> isize {
        self.tune as isize
    }

    // Returns the fine tune of the sample.
    pub fn fine_tune(&self) -> isize {
        self.fine_tune as isize
    }

    // Returns the slice number of the sample.
    pub fn slice_number(&self) -> usize {
        self.number as usize
    }

    // Returns the bit reduction of the sample.
    pub fn bit_reduction(&self) -> usize {
        self.bit_reduction as usize
    }

    // Returns the start of the sample.
    pub fn start(&self) -> f64 {
        self.start
    }

    // Returns the end of the sample.
    pub fn end(&self) -> f64 {
        self.end
    }

    // Returns the loop flag of the sample.
    pub fn loop_flag(&self) -> bool {
        self.loop_flag
    }

    // Returns the volume of the sample.
    pub fn volume(&self) -> usize {
        self.volume as usize
    }
}
