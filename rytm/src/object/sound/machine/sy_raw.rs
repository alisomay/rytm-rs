use crate::{
    error::{ParameterError, RytmError},
    util::{
        from_s_u16_t, get_u16_min_max_from_float_range, i8_to_u8_midpoint_of_u8_input_range,
        scale_generic, to_s_u16_t_union_a, u8_to_i8_midpoint_of_u8_input_range,
    },
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum SyRawWaveform1 {
    Sin,
    Asin,
    Tri,
    Ssaw,
    #[default]
    Asaw,
    Saw,
    Ring,
}

impl From<u8> for SyRawWaveform1 {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Sin,
            1 => Self::Asin,
            2 => Self::Tri,
            3 => Self::Ssaw,
            4 => Self::Asaw,
            5 => Self::Saw,
            6 => Self::Ring,
            _ => panic!("Invalid SyRawWaveform1 value: {}", value),
        }
    }
}

impl From<SyRawWaveform1> for u8 {
    fn from(value: SyRawWaveform1) -> Self {
        match value {
            SyRawWaveform1::Sin => 0,
            SyRawWaveform1::Asin => 1,
            SyRawWaveform1::Tri => 2,
            SyRawWaveform1::Ssaw => 3,
            SyRawWaveform1::Asaw => 4,
            SyRawWaveform1::Saw => 5,
            SyRawWaveform1::Ring => 6,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum SyRawWaveform2 {
    #[default]
    SineA,
    SsawA,
    SineB,
    SsawB,
}

impl From<u8> for SyRawWaveform2 {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::SineA,
            1 => Self::SsawA,
            2 => Self::SineB,
            3 => Self::SsawB,
            _ => panic!("Invalid SyRawWaveform2 value: {}", value),
        }
    }
}

impl From<SyRawWaveform2> for u8 {
    fn from(value: SyRawWaveform2) -> Self {
        match value {
            SyRawWaveform2::SineA => 0,
            SyRawWaveform2::SsawA => 1,
            SyRawWaveform2::SineB => 2,
            SyRawWaveform2::SsawB => 3,
        }
    }
}

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-24.0..=24.0" #2,
 dec2: "0..=127" #3,
 det: "-24.0..=24.0" #4,
 nlev: "0..=127" #5,
 // wav1 manual impl (0=sin,1=asin,2=tri,3=ssaw,4=asaw,5=saw,6=ring)
 // wav2 manual impl (0=sineA,1=ssawA,2=sineB,3=ssawB)
 bal: "-64..=63" #8,
)]
/// Parameters for the `SyRaw` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SyRawParameters {
    lev: u8,
    tun: f32,
    dec2: u8,
    det: f32,
    nlev: u8,
    wav1: SyRawWaveform1,
    wav2: SyRawWaveform2,
    bal: i8,
}

impl Default for SyRawParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: 0.0,
            dec2: 127,
            det: -12.0,
            nlev: 15,
            wav1: SyRawWaveform1::default(),
            wav2: SyRawWaveform2::default(),
            bal: 0,
        }
    }
}

impl SyRawParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
        let wav1: u8 = self.wav1.into();
        raw_sound.synth_param_6 = to_s_u16_t_union_a((wav1 as u16) << 8);

        let wav2: u8 = self.wav2.into();
        raw_sound.synth_param_7 = to_s_u16_t_union_a((wav2 as u16) << 8);
    }

    // Sets the `wav1` parameter.
    pub fn set_wav1(&mut self, wav1: SyRawWaveform1) -> Result<(), ParameterError> {
        self.wav1 = wav1;
        Ok(())
    }

    // Sets the `wav2` parameter.
    pub fn set_wav2(&mut self, wav2: SyRawWaveform2) -> Result<(), ParameterError> {
        self.wav2 = wav2;
        Ok(())
    }

    // Returns the `wav1` parameter.
    pub fn get_wav1(&self) -> SyRawWaveform1 {
        self.wav1
    }

    // Returns the `wav2` parameter.
    pub fn get_wav2(&self) -> SyRawWaveform2 {
        self.wav2
    }
}

impl From<&ar_sound_t> for SyRawParameters {
    fn from(raw_sound: &ar_sound_t) -> Self {
        let output_tun_min: f32 = -24.;
        let output_tun_max: f32 = 24.;
        let (input_tun_min, input_tun_max) =
            get_u16_min_max_from_float_range(output_tun_min, output_tun_max);
        unsafe {
            Self {
                lev: (from_s_u16_t(&raw_sound.synth_param_1) >> 8) as u8,
                tun: scale_generic(
                    from_s_u16_t(&raw_sound.synth_param_2),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                    |tun: u16| tun as f32,
                ),
                dec2: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                det: scale_generic(
                    from_s_u16_t(&raw_sound.synth_param_4),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                    |tun: u16| tun as f32,
                ),
                nlev: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                wav1: ((from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8).into(),
                wav2: ((from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8).into(),
                bal: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
                    0,
                    127,
                ),
            }
        }
    }
}
