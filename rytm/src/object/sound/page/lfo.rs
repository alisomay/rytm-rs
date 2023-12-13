use crate::{
    error::{ConversionError, ParameterError, RytmError},
    object::sound::types::{LfoDestination, LfoMode, LfoMultiplier, LfoWaveform},
    util::{
        from_s_u16_t, i8_to_u8_midpoint_of_u8_input_range, scale_f32_to_u16, scale_u16_to_f32,
        to_s_u16_t_union_a, u8_to_i8_midpoint_of_u8_input_range,
    },
};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;

/// Represents parameters in the lfo page of a sound.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Lfo {
    speed: i8,
    multiplier: LfoMultiplier,
    fade: i8,
    destination: LfoDestination,
    waveform: LfoWaveform,
    start_phase_or_slew: u8,
    mode: LfoMode,
    depth: f32,
}

impl Default for Lfo {
    fn default() -> Self {
        Self {
            speed: 48,
            multiplier: LfoMultiplier::default(),
            fade: 0,
            destination: LfoDestination::default(),
            waveform: LfoWaveform::default(),
            start_phase_or_slew: 0,
            mode: LfoMode::default(),
            depth: 0.0,
        }
    }
}

impl TryFrom<&ar_sound_t> for Lfo {
    type Error = ConversionError;
    fn try_from(raw_sound: &ar_sound_t) -> Result<Self, Self::Error> {
        // map range of 0..=32767 to -128.0..=127.99

        let depth = scale_u16_to_f32(
            unsafe { from_s_u16_t(&raw_sound.lfo_depth) },
            0u16,
            32767u16,
            -128f32,
            127.99f32,
        );

        Ok(Self {
            speed: u8_to_i8_midpoint_of_u8_input_range(raw_sound.lfo_speed, 0, 127),
            multiplier: raw_sound.lfo_multiplier.try_into()?,
            fade: u8_to_i8_midpoint_of_u8_input_range(raw_sound.lfo_fade, 0, 127),
            destination: raw_sound.lfo_dest.try_into()?,
            waveform: raw_sound.lfo_wav.try_into()?,
            start_phase_or_slew: raw_sound.lfo_start_phase,
            mode: raw_sound.lfo_mode.try_into()?,
            depth,
        })
    }
}

impl Lfo {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        let depth = to_s_u16_t_union_a(scale_f32_to_u16(
            self.depth, -128f32, 127.99f32, 0u16, 32767u16,
        ));

        raw_sound.lfo_speed = i8_to_u8_midpoint_of_u8_input_range(self.speed, 0, 127);
        raw_sound.lfo_multiplier = self.multiplier.into();
        raw_sound.lfo_fade = i8_to_u8_midpoint_of_u8_input_range(self.fade, 0, 127);
        raw_sound.lfo_dest = self.destination.into();
        raw_sound.lfo_wav = self.waveform.into();
        raw_sound.lfo_start_phase = self.start_phase_or_slew;
        raw_sound.lfo_mode = self.mode.into();
        raw_sound.lfo_depth = depth;
    }

    /// Sets the speed of the LFO.
    ///
    /// Range: `-64..=63`
    #[parameter_range(range = "speed:-64..=63")]
    pub fn set_speed(&mut self, speed: isize) -> Result<(), RytmError> {
        self.speed = speed as i8;
        Ok(())
    }

    /// Sets the multiplier of the LFO.
    pub fn set_multiplier(&mut self, multiplier: LfoMultiplier) -> Result<(), RytmError> {
        self.multiplier = multiplier;
        Ok(())
    }

    /// Sets the fade of the LFO.
    ///
    /// Range: `-64..=63`
    #[parameter_range(range = "fade:-64..=63")]
    pub fn set_fade(&mut self, fade: isize) -> Result<(), RytmError> {
        self.fade = fade as i8;
        Ok(())
    }

    /// Sets the destination of the LFO.
    pub fn set_destination(&mut self, destination: LfoDestination) -> Result<(), RytmError> {
        self.destination = destination;
        Ok(())
    }

    /// Sets the waveform of the LFO.
    pub fn set_waveform(&mut self, waveform: LfoWaveform) -> Result<(), RytmError> {
        self.waveform = waveform;
        Ok(())
    }

    /// Sets the depth of the LFO.
    ///
    /// Range: `-128.0..=127.99`
    #[parameter_range(range = "depth:-128.0..=127.99")]
    pub fn set_depth(&mut self, depth: f32) -> Result<(), RytmError> {
        self.depth = depth;
        Ok(())
    }

    /// Sets the start phase of the LFO.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "start_phase_or_slew:0..=127")]
    pub fn set_start_phase(&mut self, start_phase_or_slew: usize) -> Result<(), RytmError> {
        self.start_phase_or_slew = start_phase_or_slew as u8;
        Ok(())
    }

    /// Sets the mode of the LFO.
    pub fn set_mode(&mut self, mode: LfoMode) -> Result<(), RytmError> {
        self.mode = mode;
        Ok(())
    }

    /// Returns the speed of the LFO.
    pub fn speed(&self) -> isize {
        self.speed as isize
    }

    /// Returns the multiplier of the LFO.
    pub fn multiplier(&self) -> &LfoMultiplier {
        &self.multiplier
    }

    /// Returns the fade of the LFO.
    pub fn fade(&self) -> isize {
        self.fade as isize
    }

    /// Returns the destination of the LFO.
    pub fn destination(&self) -> &LfoDestination {
        &self.destination
    }

    /// Returns the waveform of the LFO.
    pub fn waveform(&self) -> &LfoWaveform {
        &self.waveform
    }

    /// Returns the start phase of the LFO.
    pub fn start_phase_or_slew(&self) -> usize {
        self.start_phase_or_slew as usize
    }

    /// Returns the mode of the LFO.
    pub fn mode(&self) -> &LfoMode {
        &self.mode
    }

    /// Returns the depth of the LFO.
    pub fn depth(&self) -> f32 {
        self.depth
    }
}
