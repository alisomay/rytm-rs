use super::types::FxLfoDestination;
use crate::{
    error::{ConversionError, ParameterError, RytmError},
    object::sound::types::{LfoMode, LfoMultiplier, LfoWaveform},
    util::{
        i8_to_u8_midpoint_of_u8_input_range, scale_f32_to_u16, scale_u16_to_f32,
        u8_to_i8_midpoint_of_u8_input_range,
    },
};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_kit_t;

/// Lfo parameters for the fx.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FxLfo {
    speed: i8,
    multiplier: LfoMultiplier,
    fade: i8,
    destination: FxLfoDestination,
    waveform: LfoWaveform,
    start_phase_or_slew: u8,
    mode: LfoMode,
    depth: f32,
}

impl Default for FxLfo {
    fn default() -> Self {
        Self {
            speed: 48,
            multiplier: LfoMultiplier::default(),
            fade: 0,
            destination: FxLfoDestination::default(),
            waveform: LfoWaveform::default(),
            start_phase_or_slew: 0,
            mode: LfoMode::default(),
            depth: 0.0,
        }
    }
}

impl TryFrom<&ar_kit_t> for FxLfo {
    type Error = ConversionError;
    fn try_from(raw_kit: &ar_kit_t) -> Result<Self, Self::Error> {
        // map range of 0..=32767 to -128.0..=127.99
        let depth_u16 = ((raw_kit.fx_lfo_depth_msb as u16) << 8) + raw_kit.fx_lfo_depth_lsb as u16;
        let depth = scale_u16_to_f32(depth_u16, 0u16, 32767u16, -128f32, 127.99f32);

        Ok(Self {
            speed: u8_to_i8_midpoint_of_u8_input_range(raw_kit.fx_lfo_speed, 0, 127),
            multiplier: raw_kit.fx_lfo_multiplier.try_into()?,
            fade: u8_to_i8_midpoint_of_u8_input_range(raw_kit.fx_lfo_fade, 0, 127),
            destination: raw_kit.fx_lfo_dest.try_into()?,
            waveform: raw_kit.fx_lfo_wave.try_into()?,
            start_phase_or_slew: raw_kit.fx_lfo_start_phase,
            mode: raw_kit.fx_lfo_mode.try_into()?,
            depth,
        })
    }
}

impl FxLfo {
    pub(crate) fn apply_to_raw_kit(&self, raw_kit: &mut ar_kit_t) {
        // map range of -128.0..=127.99 to 0..=32767
        let depth = scale_f32_to_u16(self.depth, -128f32, 127.99f32, 0u16, 32767u16);

        raw_kit.fx_lfo_speed = i8_to_u8_midpoint_of_u8_input_range(self.speed, 0, 127);
        raw_kit.fx_lfo_multiplier = self.multiplier.into();
        raw_kit.fx_lfo_fade = i8_to_u8_midpoint_of_u8_input_range(self.fade, 0, 127);
        raw_kit.fx_lfo_dest = self.destination.into();
        raw_kit.fx_lfo_wave = self.waveform.into();
        raw_kit.fx_lfo_start_phase = self.start_phase_or_slew;
        raw_kit.fx_lfo_mode = self.mode.into();
        raw_kit.fx_lfo_depth_msb = ((depth & 0xFF00) >> 8) as u8;
        raw_kit.fx_lfo_depth_lsb = depth as u8;
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
    pub fn set_multiplier(&mut self, multiplier: LfoMultiplier) {
        self.multiplier = multiplier;
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
    pub fn set_destination(&mut self, destination: FxLfoDestination) {
        self.destination = destination;
    }

    /// Sets the waveform of the LFO.
    pub fn set_waveform(&mut self, waveform: LfoWaveform) {
        self.waveform = waveform;
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
    pub fn set_mode(&mut self, mode: LfoMode) {
        self.mode = mode;
    }

    /// Returns the speed of the LFO.
    pub const fn speed(&self) -> isize {
        self.speed as isize
    }

    /// Returns the multiplier of the LFO.
    pub const fn multiplier(&self) -> &LfoMultiplier {
        &self.multiplier
    }

    /// Returns the fade of the LFO.
    pub const fn fade(&self) -> isize {
        self.fade as isize
    }

    /// Returns the destination of the LFO.
    pub const fn destination(&self) -> &FxLfoDestination {
        &self.destination
    }

    /// Returns the waveform of the LFO.
    pub const fn waveform(&self) -> &LfoWaveform {
        &self.waveform
    }

    /// Returns the start phase of the LFO.
    pub const fn start_phase_or_slew(&self) -> usize {
        self.start_phase_or_slew as usize
    }

    /// Returns the mode of the LFO.
    pub const fn mode(&self) -> &LfoMode {
        &self.mode
    }

    /// Returns the depth of the LFO.
    pub const fn depth(&self) -> f32 {
        self.depth
    }
}
