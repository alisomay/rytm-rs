use crate::{
    error::{ParameterError, RytmError},
    util::{from_s_u16_t, get_u16_min_max_from_float_range, scale_generic, to_s_u16_t_union_a},
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum BdSharpWaveform {
    #[default]
    SinA,
    SinB,
    AsinA,
    AsinB,
    TriA,
    TriB,
    SsawA,
    SsawB,
    SawA,
    SawB,
    SqrA,
    SqrB,
}

impl From<u8> for BdSharpWaveform {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::SinA,
            1 => Self::SinB,
            2 => Self::AsinA,
            3 => Self::AsinB,
            4 => Self::TriA,
            5 => Self::TriB,
            6 => Self::SsawA,
            7 => Self::SsawB,
            8 => Self::SawA,
            9 => Self::SawB,
            10 => Self::SqrA,
            11 => Self::SqrB,
            _ => panic!("Invalid BdSharpWaveform value: {}", value),
        }
    }
}

impl From<BdSharpWaveform> for u8 {
    fn from(value: BdSharpWaveform) -> Self {
        match value {
            BdSharpWaveform::SinA => 0,
            BdSharpWaveform::SinB => 1,
            BdSharpWaveform::AsinA => 2,
            BdSharpWaveform::AsinB => 3,
            BdSharpWaveform::TriA => 4,
            BdSharpWaveform::TriB => 5,
            BdSharpWaveform::SsawA => 6,
            BdSharpWaveform::SsawB => 7,
            BdSharpWaveform::SawA => 8,
            BdSharpWaveform::SawB => 9,
            BdSharpWaveform::SqrA => 10,
            BdSharpWaveform::SqrB => 11,
        }
    }
}

#[machine_parameters(
    lev: "0..=127" #1,
    tun: "-32.0..=32.0" #2,
    dec: "0..=127" #3,
    hld: "0..=127" #4,
    swt: "0..=127" #5,
    swd: "0..=127" #6,
    // wav: (0=sinA,1=sinB,2=asinA,3=asinB,4=triA,5=triB,6=ssawA,7=ssawB,8=sawA,9=sawB,10=sqrA,11=sqrB)
    tic: "0..=127" #8,
)]

/// Parameters for the `BdSharp` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BdSharpParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    hld: u8,
    swt: u8,
    swd: u8,
    wav: BdSharpWaveform,
    tic: u8,
}

impl Default for BdSharpParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: -6.0,
            dec: 80,
            hld: 32,
            swt: 100,
            swd: 100,
            wav: BdSharpWaveform::default(),
            tic: 64,
        }
    }
}

impl BdSharpParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);

        let wav: u8 = self.wav.into();
        let wav = to_s_u16_t_union_a((wav as u16) << 8);
        raw_sound.synth_param_7 = wav;
    }

    /// Sets the `wav` parameter.
    pub fn set_wav(&mut self, wav: BdSharpWaveform) -> Result<(), RytmError> {
        self.wav = wav;
        Ok(())
    }

    /// Returns the `wav` parameter.
    pub fn get_wav(&self) -> BdSharpWaveform {
        self.wav
    }
}

impl From<&ar_sound_t> for BdSharpParameters {
    fn from(raw_sound: &ar_sound_t) -> Self {
        let output_tun_min: f32 = -32.;
        let output_tun_max: f32 = 32.;
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
                dec: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                hld: (from_s_u16_t(&raw_sound.synth_param_4) >> 8) as u8,
                swt: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                swd: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                wav: ((from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8).into(),
                tic: (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
            }
        }
    }
}
