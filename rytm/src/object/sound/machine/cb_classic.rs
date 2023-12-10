use crate::{
    error::{ParameterError, RytmError},
    util::{
        from_s_u16_t, i8_to_u8_midpoint_of_u8_input_range, to_s_u16_t_union_a,
        u8_to_i8_midpoint_of_u8_input_range,
    },
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-64..=63" #2,
 dec: "0..=127" #3,
 det: "0..=127" #4,
 pw1: "-64..=63" #5,
 pw2: "-64..=63" #6,
 // Unavailable #7
 // Unavailable #8
)]
/// Parameters for the `CbClassic` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CbClassicParameters {
    lev: u8,
    tun: i8,
    dec: u8,
    det: u8,
    pw1: i8,
    pw2: i8,
}

impl CbClassicParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for CbClassicParameters {
    fn from(raw_sound: &ar_sound_t) -> Self {
        unsafe {
            Self {
                lev: (from_s_u16_t(&raw_sound.synth_param_1) >> 8) as u8,
                tun: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_2) >> 8) as u8,
                    0,
                    127,
                ),
                dec: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                det: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                pw1: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                    0,
                    127,
                ),
                pw2: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                    0,
                    127,
                ),
            }
        }
    }
}
