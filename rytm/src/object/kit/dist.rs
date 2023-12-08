use crate::error::{ConversionError, ParameterError, RytmError};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_kit_t;

/// Distortion parameters for the kit.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FxDistortion {
    reverb_send: u8,
    delay_overdrive: u8,
    reverb_post: bool,
    amount: u8,
    symmetry: i8,
}

impl Default for FxDistortion {
    fn default() -> Self {
        Self {
            reverb_send: 0,
            delay_overdrive: 0,
            reverb_post: false,
            amount: 0,
            symmetry: 0,
        }
    }
}

impl TryFrom<&ar_kit_t> for FxDistortion {
    type Error = ConversionError;
    fn try_from(raw_kit: &ar_kit_t) -> Result<Self, Self::Error> {
        Ok(Self {
            reverb_send: raw_kit.fx_dist_reverb_send,
            // TODO: fx_dist_delay_pre_post naming is wrong in libanalogrytm make a PR there to change it.
            delay_overdrive: raw_kit.fx_dist_delay_pre_post,
            reverb_post: raw_kit.fx_dist_reverb_pre_post != 0,
            amount: raw_kit.fx_dist_amount,
            symmetry: raw_kit.fx_dist_sym as i8 - 64,
        })
    }
}

impl FxDistortion {
    pub(crate) fn apply_to_raw_kit(&self, raw_kit: &mut ar_kit_t) {
        raw_kit.fx_dist_reverb_send = self.reverb_send;
        raw_kit.fx_dist_delay_pre_post = self.delay_overdrive;
        raw_kit.fx_dist_reverb_pre_post = self.reverb_post as u8;
        raw_kit.fx_dist_amount = self.amount;
        raw_kit.fx_dist_sym = self.symmetry as u8 + 64;
    }

    /// Sets the reverb send of the distortion.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "reverb_send:0..=127")]
    pub fn set_reverb_send(&mut self, reverb_send: usize) -> Result<(), RytmError> {
        self.reverb_send = reverb_send as u8;
        Ok(())
    }

    /// Sets the delay overdrive of the distortion.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "delay_overdrive:0..=127")]
    pub fn set_delay_overdrive(&mut self, delay_overdrive: usize) -> Result<(), RytmError> {
        self.delay_overdrive = delay_overdrive as u8;
        Ok(())
    }

    // TODO: Update doc
    /// Sets the reverb post of the distortion.
    pub fn set_reverb_post(&mut self, reverb_post: bool) -> Result<(), RytmError> {
        self.reverb_post = reverb_post;
        Ok(())
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

    /// Returns the reverb send of the distortion.
    ///
    /// Range: `0..=127`
    pub fn reverb_send(&self) -> usize {
        self.reverb_send as usize
    }

    /// Returns the delay overdrive of the distortion.
    ///
    /// Range: `0..=127`
    pub fn delay_overdrive(&self) -> usize {
        self.delay_overdrive as usize
    }

    /// Returns the reverb post of the distortion.

    pub fn reverb_post(&self) -> bool {
        self.reverb_post
    }

    /// Returns the amount of the distortion.
    ///
    /// Range: `0..=127`
    pub fn amount(&self) -> usize {
        self.amount as usize
    }

    /// Returns the symmetry of the distortion.
    ///
    /// Range: `-64..=63`
    pub fn symmetry(&self) -> isize {
        self.symmetry as isize
    }
}
