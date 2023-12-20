use crate::error::{ConversionError, ParameterError, RytmError};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_kit_t;
use serde::{Deserialize, Serialize};

/// Reverb parameters for the kit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FxReverb {
    pre_delay: u8,
    decay: u8,
    freq: u8,
    gain: u8,
    hpf: u8,
    lpf: u8,
    volume: u8,
}

impl Default for FxReverb {
    fn default() -> Self {
        Self {
            pre_delay: 8,
            decay: 40,
            freq: 64,
            gain: 32,
            hpf: 32,
            lpf: 96,
            volume: 110,
        }
    }
}

impl TryFrom<&ar_kit_t> for FxReverb {
    type Error = ConversionError;
    fn try_from(raw_kit: &ar_kit_t) -> Result<Self, Self::Error> {
        Ok(Self {
            pre_delay: raw_kit.fx_reverb_pre,
            decay: raw_kit.fx_reverb_decay,
            freq: raw_kit.fx_reverb_freq,
            gain: raw_kit.fx_reverb_gain,
            hpf: raw_kit.fx_reverb_hpf,
            lpf: raw_kit.fx_reverb_lpf,
            volume: raw_kit.fx_reverb_volume,
        })
    }
}

impl FxReverb {
    pub(crate) fn apply_to_raw_kit(self, raw_kit: &mut ar_kit_t) {
        raw_kit.fx_reverb_pre = self.pre_delay;
        raw_kit.fx_reverb_decay = self.decay;
        raw_kit.fx_reverb_freq = self.freq;
        raw_kit.fx_reverb_gain = self.gain;
        raw_kit.fx_reverb_hpf = self.hpf;
        raw_kit.fx_reverb_lpf = self.lpf;
        raw_kit.fx_reverb_volume = self.volume;
    }

    /// Sets the pre delay of the reverb.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "pre_delay:0..=127")]
    pub fn set_pre_delay(&mut self, pre_delay: usize) -> Result<(), RytmError> {
        self.pre_delay = pre_delay as u8;
        Ok(())
    }

    /// Sets the decay of the reverb.
    ///
    /// Range: `0..=127`
    ///
    /// `127` is infinite decay.
    #[parameter_range(range = "decay:0..=127")]
    pub fn set_decay(&mut self, decay: usize) -> Result<(), RytmError> {
        self.decay = decay as u8;
        Ok(())
    }

    /// Sets the frequency of the reverb.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "freq:0..=127")]
    pub fn set_freq(&mut self, freq: usize) -> Result<(), RytmError> {
        self.freq = freq as u8;
        Ok(())
    }

    /// Sets the gain of the reverb.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "gain:0..=127")]
    pub fn set_gain(&mut self, gain: usize) -> Result<(), RytmError> {
        self.gain = gain as u8;
        Ok(())
    }

    /// Sets the high-pass filter of the reverb.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "hpf:0..=127")]
    pub fn set_hpf(&mut self, hpf: usize) -> Result<(), RytmError> {
        self.hpf = hpf as u8;
        Ok(())
    }

    /// Sets the low-pass filter of the reverb.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "lpf:0..=127")]
    pub fn set_lpf(&mut self, lpf: usize) -> Result<(), RytmError> {
        self.lpf = lpf as u8;
        Ok(())
    }
    /// Sets the volume of the reverb.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn set_volume(&mut self, volume: usize) -> Result<(), RytmError> {
        self.volume = volume as u8;
        Ok(())
    }

    /// Returns the pre delay of the reverb.
    ///
    /// Range: `0..=127`
    pub const fn pre_delay(&self) -> usize {
        self.pre_delay as usize
    }

    /// Returns the decay of the reverb.
    ///
    /// Range: `0..=127`
    pub const fn decay(&self) -> usize {
        self.decay as usize
    }

    /// Returns the frequency of the reverb.
    ///
    /// Range: `0..=127`
    pub const fn freq(&self) -> usize {
        self.freq as usize
    }

    /// Returns the gain of the reverb.
    ///
    /// Range: `0..=127`
    pub const fn gain(&self) -> usize {
        self.gain as usize
    }

    /// Returns the high-pass filter of the reverb.
    ///
    /// Range: `0..=127`
    pub const fn hpf(&self) -> usize {
        self.hpf as usize
    }

    /// Returns the low-pass filter of the reverb.
    ///
    /// Range: `0..=127`
    pub const fn lpf(&self) -> usize {
        self.lpf as usize
    }

    /// Returns the volume of the reverb.
    ///
    /// Range: `0..=127`
    pub const fn volume(&self) -> usize {
        self.volume as usize
    }
}
