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
 lpf: "0..=127" #2,
 dec: "0..=127" #3,
 hpf: "0..=127" #4,
 lpq: "0..=127" #5,
 atk: "0..=127" #6,
 swt: "0..=127" #7,
 swd: "-64..=63" #8,
)]
/// Parameters for the `UtNoise` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UtNoiseParameters {
    lev: u8,
    lpf: u8,
    dec: u8,
    hpf: u8,
    lpq: u8,
    atk: u8,
    swt: u8,
    swd: i8,
}

impl Default for UtNoiseParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            lpf: 127,
            dec: 40,
            hpf: 0,
            lpq: 0,
            atk: 0,
            swt: 64,
            swd: 0,
        }
    }
}

impl UtNoiseParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for UtNoiseParameters {
    fn from(raw_sound: &ar_sound_t) -> Self {
        unsafe {
            Self {
                lev: (from_s_u16_t(&raw_sound.synth_param_1) >> 8) as u8,
                lpf: (from_s_u16_t(&raw_sound.synth_param_2) >> 8) as u8,
                dec: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                hpf: (from_s_u16_t(&raw_sound.synth_param_4) >> 8) as u8,
                lpq: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                atk: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                swt: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                swd: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
                    0,
                    127,
                ),
            }
        }
    }
}
