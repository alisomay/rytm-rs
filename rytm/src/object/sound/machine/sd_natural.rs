use crate::{
    error::{ParameterError, RytmError},
    util::{from_s_u16_t, get_u16_min_max_from_float_range, scale_generic, to_s_u16_t_union_a},
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-32.0..=32.0" #2,
 bdy: "0..=127" #3,
 dec: "0..=127" #4,
 bal: "0..=127" #5,
 lpf: "0..=127" #6,
 hpf: "0..=127" #7,
 res: "0..=127" #8,
)]
/// Parameters for the `SdNatural` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SdNaturalParameters {
    lev: u8,
    tun: f32,
    bdy: u8,
    dec: u8,
    bal: u8,
    lpf: u8,
    hpf: u8,
    res: u8,
}

impl SdNaturalParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for SdNaturalParameters {
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
                bdy: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                dec: (from_s_u16_t(&raw_sound.synth_param_4) >> 8) as u8,
                bal: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                lpf: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                hpf: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                res: (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
            }
        }
    }
}
