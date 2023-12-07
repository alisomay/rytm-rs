use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;

use crate::{
    error::{ConversionError, ParameterError, RytmError},
    sound::types::FilterType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Filter {
    attack: u8,
    sustain: u8,
    decay: u8,
    release: u8,
    cutoff: u8,
    resonance: u8,
    filter_type: FilterType,
    // 0..=127 in firmware, -64..=63 in rytm
    envelope_amount: i8,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            attack: 0,
            sustain: 127,
            decay: 0,
            release: 0,
            cutoff: 127,
            resonance: 0,
            filter_type: FilterType::default(),
            envelope_amount: 0,
        }
    }
}

impl TryFrom<&ar_sound_t> for Filter {
    type Error = ConversionError;
    fn try_from(raw_sound: &ar_sound_t) -> Result<Self, Self::Error> {
        Ok(Self {
            attack: raw_sound.flt_attack,
            sustain: raw_sound.flt_sustain,
            decay: raw_sound.flt_decay,
            release: raw_sound.flt_release,
            cutoff: raw_sound.flt_cutoff,
            resonance: raw_sound.flt_res,
            filter_type: raw_sound.flt_type.try_into()?,
            envelope_amount: raw_sound.flt_env as i8 - 64,
        })
    }
}

impl Filter {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        raw_sound.flt_attack = self.attack;
        raw_sound.flt_sustain = self.sustain;
        raw_sound.flt_decay = self.decay;
        raw_sound.flt_release = self.release;
        raw_sound.flt_cutoff = self.cutoff;
        raw_sound.flt_res = self.resonance;
        raw_sound.flt_type = self.filter_type.into();
        raw_sound.flt_env = self.envelope_amount as u8 + 64;
    }

    /// Sets the attack of the filter envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "attack:0..=127")]
    pub fn set_attack(&mut self, attack: usize) -> Result<(), RytmError> {
        self.attack = attack as u8;
        Ok(())
    }

    /// Sets the sustain of the filter envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "sustain:0..=127")]
    pub fn set_sustain(&mut self, sustain: usize) -> Result<(), RytmError> {
        self.sustain = sustain as u8;
        Ok(())
    }

    /// Sets the decay of the filter envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "decay:0..=127")]
    pub fn set_decay(&mut self, decay: usize) -> Result<(), RytmError> {
        self.decay = decay as u8;
        Ok(())
    }

    /// Sets the release of the filter envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "release:0..=127")]
    pub fn set_release(&mut self, release: usize) -> Result<(), RytmError> {
        self.release = release as u8;
        Ok(())
    }

    /// Sets the cutoff of the filter.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "cutoff:0..=127")]
    pub fn set_cutoff(&mut self, cutoff: usize) -> Result<(), RytmError> {
        self.cutoff = cutoff as u8;
        Ok(())
    }

    /// Sets the resonance of the filter.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "resonance:0..=127")]
    pub fn set_resonance(&mut self, resonance: usize) -> Result<(), RytmError> {
        self.resonance = resonance as u8;
        Ok(())
    }

    /// Sets the filter type.
    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
    }

    /// Sets the envelope amount of the filter.
    #[parameter_range(range = "envelope_amount:-64..=63")]
    pub fn set_envelope_amount(&mut self, envelope_amount: isize) -> Result<(), RytmError> {
        self.envelope_amount = envelope_amount as i8;
        Ok(())
    }

    /// Returns the attack of the filter envelope.
    ///
    /// Range: `0..=127`
    pub fn attack(&self) -> usize {
        self.attack as usize
    }

    /// Returns the sustain of the filter envelope.
    ///
    /// Range: `0..=127`
    pub fn sustain(&self) -> usize {
        self.sustain as usize
    }

    /// Returns the decay of the filter envelope.
    ///
    /// Range: `0..=127`
    pub fn decay(&self) -> usize {
        self.decay as usize
    }

    /// Returns the release of the filter envelope.
    ///
    /// Range: `0..=127`
    pub fn release(&self) -> usize {
        self.release as usize
    }

    /// Returns the cutoff of the filter.
    ///
    /// Range: `0..=127`
    pub fn cutoff(&self) -> usize {
        self.cutoff as usize
    }

    /// Returns the resonance of the filter.
    ///
    /// Range: `0..=127`
    pub fn resonance(&self) -> usize {
        self.resonance as usize
    }

    /// Returns the filter type.
    pub fn filter_type(&self) -> FilterType {
        self.filter_type
    }

    /// Returns the envelope amount of the filter.
    pub fn envelope_amount(&self) -> isize {
        self.envelope_amount as isize
    }
}
