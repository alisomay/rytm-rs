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
    typ: "0..=3" #4, // (0..3=A..D)
    hit: "0..=127" #5,
    c1: "0..=127" #6,
    c2: "0..=127" #7,
    c3: "0..=127" #8,
)]
/// Parameters for the `CyRide` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CyRideParameters {
    lev: u8,
    tun: i8,
    dec: u8,
    typ: u8,
    hit: u8,
    c1: u8,
    c2: u8,
    c3: u8,
}

impl CyRideParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for CyRideParameters {
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
                typ: (from_s_u16_t(&raw_sound.synth_param_4) >> 8) as u8,
                hit: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                c1: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                c2: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                c3: (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
            }
        }
    }
}
