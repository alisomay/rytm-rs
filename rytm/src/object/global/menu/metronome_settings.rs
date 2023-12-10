use super::super::types::TimeSignature;
use crate::error::{ConversionError, ParameterError, RytmError};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_global_t;

/// Represents the metronome settings menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetronomeSettings {
    active: bool,
    time_signature: TimeSignature,
    pre_roll_bars: u8,
    volume: u8,
}

impl Default for MetronomeSettings {
    fn default() -> Self {
        Self {
            active: false,
            time_signature: TimeSignature::default(),
            pre_roll_bars: 0,
            volume: 64,
        }
    }
}

impl TryFrom<&ar_global_t> for MetronomeSettings {
    type Error = ConversionError;
    fn try_from(raw_global: &ar_global_t) -> Result<Self, Self::Error> {
        Ok(Self {
            active: raw_global.click_active != 0,
            time_signature: TimeSignature::try_from((
                raw_global.click_time_sig_num,
                raw_global.click_time_sig_den,
            ))?,
            pre_roll_bars: raw_global.pre_roll,
            volume: raw_global.volume,
        })
    }
}

impl MetronomeSettings {
    pub(crate) fn apply_to_raw_global(&self, raw_global: &mut ar_global_t) {
        raw_global.click_active = self.active as u8;
        raw_global.click_time_sig_num = self.time_signature.numerator() as u8;
        raw_global.click_time_sig_den = self.time_signature.denominator() as u8;
        raw_global.pre_roll = self.pre_roll_bars;
        raw_global.volume = self.volume;
    }

    /// Turns metronome on or off.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Sets the time signature of the metronome.
    pub fn set_time_signature(&mut self, time_signature: TimeSignature) {
        self.time_signature = time_signature;
    }

    // TODO: Double check range
    /// Sets the number of bars to pre-roll.
    ///
    /// Range: `0..=16`
    ///
    /// Numbers represent bars, `0` bars means no pre-roll.
    #[parameter_range(range = "pre_roll_bars:0..=16")]
    pub fn set_pre_roll_bars(&mut self, pre_roll_bars: usize) -> Result<(), RytmError> {
        self.pre_roll_bars = pre_roll_bars as u8;
        Ok(())
    }

    /// Sets the volume of the metronome.
    #[parameter_range(range = "volume:0..=127")]
    pub fn set_volume(&mut self, volume: usize) -> Result<(), RytmError> {
        self.volume = volume as u8;
        Ok(())
    }

    /// Returns `true` if the metronome is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Returns the time signature of the metronome.
    pub fn time_signature(&self) -> TimeSignature {
        self.time_signature
    }

    /// Returns the number of bars to pre-roll.
    ///
    /// Range: `0..=16`
    ///
    /// Numbers represent bars, `0` bars means no pre-roll.
    pub fn pre_roll_bars(&self) -> usize {
        self.pre_roll_bars as usize
    }

    /// Returns the volume of the metronome.
    pub fn volume(&self) -> usize {
        self.volume as usize
    }
}
