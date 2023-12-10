use crate::{
    error::{ParameterError, RytmError},
    util::{from_s_u16_t, get_u16_min_max_from_float_range, scale_generic, to_s_u16_t_union_a},
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[machine_parameters(
    lev: "0..=127" #1,
    tun: "-32.0..=32.0" #2,
    dec: "0..=127" #3,
    fma: "0..=127" #4,
    swt: "0..=127" #5,
    fms: "0..=127" #6,
    fmd: "0..=127" #7,
    fmt:  "-32.0..=32.0" #8,
)]
/// Parameters for the `BdFm` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BdFmParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    fma: u8,
    swt: u8,
    fms: u8,
    fmd: u8,
    fmt: f32,
}

impl Default for BdFmParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: -6.0,
            dec: 32,
            fma: 32,
            swt: 48,
            fms: 32,
            fmd: 48,
            fmt: -16.0,
        }
    }
}

impl BdFmParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for BdFmParameters {
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
                fma: (from_s_u16_t(&raw_sound.synth_param_4) >> 8) as u8,
                swt: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                fms: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                fmd: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                fmt: scale_generic(
                    from_s_u16_t(&raw_sound.synth_param_8),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                    |fmt: u16| fmt as f32,
                ),
            }
        }
    }
}
