use crate::{
    error::{ConversionError, ParameterError, RytmError},
    util::{i8_to_u8_midpoint_of_u8_input_range, u8_to_i8_midpoint_of_u8_input_range},
};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_kit_t;
use serde::{Deserialize, Serialize};

/// Distortion parameters for the kit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FxDistortion {
    delay_overdrive: u8,
    delay_post: bool,
    reverb_post: bool,
    amount: u8,
    symmetry: i8,
}

impl Default for FxDistortion {
    fn default() -> Self {
        Self {
            delay_overdrive: 0,
            delay_post: true,
            reverb_post: true,
            amount: 0,
            symmetry: 0,
        }
    }
}

impl TryFrom<&ar_kit_t> for FxDistortion {
    type Error = ConversionError;
    fn try_from(raw_kit: &ar_kit_t) -> Result<Self, Self::Error> {
        Ok(Self {
            // Naming is wrong in the C API it needs to be delay_overdrive
            delay_overdrive: raw_kit.fx_dist_reverb_send,
            delay_post: raw_kit.fx_dist_delay_pre_post != 0,
            reverb_post: raw_kit.fx_dist_reverb_pre_post != 0,
            amount: raw_kit.fx_dist_amount,
            symmetry: u8_to_i8_midpoint_of_u8_input_range(raw_kit.fx_dist_sym, 0, 127),
        })
    }
}

impl FxDistortion {
    pub(crate) fn apply_to_raw_kit(self, raw_kit: &mut ar_kit_t) {
        raw_kit.fx_dist_reverb_send = self.delay_overdrive;
        raw_kit.fx_dist_delay_pre_post = self.delay_post as u8;
        raw_kit.fx_dist_reverb_pre_post = self.reverb_post as u8;
        raw_kit.fx_dist_amount = self.amount;
        raw_kit.fx_dist_sym = i8_to_u8_midpoint_of_u8_input_range(self.symmetry, 0, 127);
    }

    /// Sets the delay overdrive of the distortion.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "delay_overdrive:0..=127")]
    pub fn set_delay_overdrive(&mut self, delay_overdrive: usize) -> Result<(), RytmError> {
        self.delay_overdrive = delay_overdrive as u8;
        Ok(())
    }

    /// Sets the reverb post of the distortion.
    pub fn set_reverb_post(&mut self, reverb_post: bool) {
        self.reverb_post = reverb_post;
    }

    /// Sets the delay post of the distortion.
    pub fn set_delay_post(&mut self, delay_post: bool) {
        self.delay_post = delay_post;
    }

    /// Sets the amount of the distortion.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "amount:0..=127")]
    pub fn set_amount(&mut self, amount: usize) -> Result<(), RytmError> {
        self.amount = amount as u8;
        Ok(())
    }

    /// Sets the symmetry of the distortion.
    ///
    /// Range: `-64..=63`
    #[parameter_range(range = "symmetry:-64..=63")]
    pub fn set_symmetry(&mut self, symmetry: isize) -> Result<(), RytmError> {
        self.symmetry = symmetry as i8;
        Ok(())
    }

    /// Returns the delay overdrive of the distortion.
    ///
    /// Range: `0..=127`
    pub const fn delay_overdrive(&self) -> usize {
        self.delay_overdrive as usize
    }

    /// Returns the reverb post of the distortion.
    pub const fn reverb_post(&self) -> bool {
        self.reverb_post
    }

    /// Returns the delay post of the distortion.
    pub const fn delay_post(&self) -> bool {
        self.delay_post
    }

    /// Returns the amount of the distortion.
    ///
    /// Range: `0..=127`
    pub const fn amount(&self) -> usize {
        self.amount as usize
    }

    /// Returns the symmetry of the distortion.
    ///
    /// Range: `-64..=63`
    pub const fn symmetry(&self) -> isize {
        self.symmetry as isize
    }
}
