use crate::{
    error::{ParameterError, RytmError},
    util::{
        from_s_u16_t, get_u16_min_max_from_float_range, i8_to_u8_midpoint_of_u8_input_range,
        scale_generic, to_s_u16_t_union_a, u8_to_i8_midpoint_of_u8_input_range,
    },
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-32.0..=32.0" #2,
 dec: "0..=127" #3,
 det:  "-32.0..=32.0" #4,
 snp: "0..=127" #5,
 nod: "0..=127" #6,
 nol: "0..=127" #7,
 bal: "-64..=63" #8,
)]

/// Parameters for the `SdClassic` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SdClassicParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    det: f32,
    snp: u8,
    nod: u8,
    nol: u8,
    bal: i8,
}

impl SdClassicParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for SdClassicParameters {
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
                det: scale_generic(
                    from_s_u16_t(&raw_sound.synth_param_4),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                    |tun: u16| tun as f32,
                ),
                snp: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                nod: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                nol: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                bal: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
                    0,
                    127,
                ),
            }
        }
    }
}
