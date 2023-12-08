use crate::error::{ConversionError, ParameterError, RytmError};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;

/// Represents parameters in the amp page of a sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amplitude {
    attack: u8,
    hold: u8,
    decay: u8,
    overdrive: u8,
    delay_send: u8,
    reverb_send: u8,
    pan: i8,
    volume: u8,
}

impl Default for Amplitude {
    fn default() -> Self {
        Self {
            attack: 0,
            hold: 64,
            decay: 0,
            overdrive: 0,
            delay_send: 0,
            reverb_send: 0,
            pan: 0,
            volume: 100,
        }
    }
}

impl TryFrom<&ar_sound_t> for Amplitude {
    type Error = ConversionError;
    fn try_from(raw_sound: &ar_sound_t) -> Result<Self, Self::Error> {
        Ok(Self {
            attack: raw_sound.amp_attack,
            hold: raw_sound.amp_hold,
            decay: raw_sound.amp_decay,
            overdrive: raw_sound.amp_overdrive,
            delay_send: raw_sound.amp_delay_send,
            reverb_send: raw_sound.amp_reverb_send,
            pan: raw_sound.amp_pan as i8 - 64,
            volume: raw_sound.amp_volume,
        })
    }
}

impl Amplitude {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        raw_sound.amp_attack = self.attack;
        raw_sound.amp_hold = self.hold;
        raw_sound.amp_decay = self.decay;
        raw_sound.amp_overdrive = self.overdrive;
        raw_sound.amp_delay_send = self.delay_send;
        raw_sound.amp_reverb_send = self.reverb_send;
        raw_sound.amp_pan = self.pan as u8 + 64;
        raw_sound.amp_volume = self.volume;
    }

    /// Sets the attack of the amplitude envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "attack:0..=127")]
    pub fn set_attack(&mut self, attack: usize) -> Result<(), RytmError> {
        self.attack = attack as u8;
        Ok(())
    }

    /// Sets the hold of the amplitude envelope.
    ///
    /// Range: `0..=127`
    ///
    /// `0` is no hold, `127` is infinite hold.
    #[parameter_range(range = "hold:0..=127")]
    pub fn set_hold(&mut self, hold: usize) -> Result<(), RytmError> {
        self.hold = hold as u8;
        Ok(())
    }

    /// Sets the decay of the amplitude envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "decay:0..=127")]
    pub fn set_decay(&mut self, decay: usize) -> Result<(), RytmError> {
        self.decay = decay as u8;
        Ok(())
    }

    /// Sets the overdrive amount applied to the amplitude.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "overdrive:0..=127")]
    pub fn set_overdrive(&mut self, overdrive: usize) -> Result<(), RytmError> {
        self.overdrive = overdrive as u8;
        Ok(())
    }

    /// Sets the delay send of the amplitude envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "delay_send:0..=127")]
    pub fn set_delay_send(&mut self, delay_send: usize) -> Result<(), RytmError> {
        self.delay_send = delay_send as u8;
        Ok(())
    }

    /// Sets the reverb send of the amplitude envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "reverb_send:0..=127")]
    pub fn set_reverb_send(&mut self, reverb_send: usize) -> Result<(), RytmError> {
        self.reverb_send = reverb_send as u8;
        Ok(())
    }

    /// Sets the pan of the amplitude envelope.
    ///
    /// Range: `-64..=63`
    #[parameter_range(range = "pan:-64..=63")]
    pub fn set_pan(&mut self, pan: isize) -> Result<(), RytmError> {
        self.pan = pan as i8;
        Ok(())
    }

    /// Sets the volume of the amplitude envelope.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn set_volume(&mut self, volume: usize) -> Result<(), RytmError> {
        self.volume = volume as u8;
        Ok(())
    }

    /// Returns the attack of the amplitude envelope.
    ///
    /// Range: `0..=127`
    pub fn attack(&self) -> usize {
        self.attack as usize
    }

    /// Returns the hold of the amplitude envelope.
    ///
    /// Range: `0..=127`
    ///
    /// `0` is no hold, `127` is infinite hold.
    pub fn hold(&self) -> usize {
        self.hold as usize
    }

    /// Returns the decay of the amplitude envelope.
    ///
    ///
    pub fn decay(&self) -> usize {
        self.decay as usize
    }

    /// Returns the overdrive amount applied to the amplitude.
    ///
    /// Range: `0..=127`
    pub fn overdrive(&self) -> usize {
        self.overdrive as usize
    }

    /// Returns the delay send of the amplitude envelope.
    ///
    /// Range: `0..=127`
    pub fn delay_send(&self) -> usize {
        self.delay_send as usize
    }

    /// Returns the reverb send of the amplitude envelope.
    ///
    /// Range: `0..=127`
    pub fn reverb_send(&self) -> usize {
        self.reverb_send as usize
    }

    /// Returns the pan of the amplitude envelope.
    ///
    /// Range: `-64..=63`
    ///
    /// `0` is center, `-64` is hard left, `63` is hard right.
    pub fn pan(&self) -> usize {
        self.pan as usize
    }

    /// Returns the volume of the amplitude envelope.
    ///
    /// Range: `0..=127`
    pub fn volume(&self) -> usize {
        self.volume as usize
    }
}
