use crate::{
    error::{ParameterError, RytmError},
    util::{from_s_u16_t, to_s_u16_t_union_a},
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[machine_parameters(
 lev: "0..=127" #1,
 atk:  "0..=127" #2,
 dec: "0..=127" #3,
 // Unavailable #4
 // Unavailable #5
 // Unavailable #6
 // Unavailable #7
 pol: "0..=1" #8,
)]
/// Parameters for the `UtImpulse` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UtImpulseParameters {
    lev: u8,
    atk: u8,
    dec: u8,
    pol: u8,
}

impl UtImpulseParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for UtImpulseParameters {
    fn from(raw_sound: &ar_sound_t) -> Self {
        unsafe {
            Self {
                lev: (from_s_u16_t(&raw_sound.synth_param_1) >> 8) as u8,
                atk: (from_s_u16_t(&raw_sound.synth_param_2) >> 8) as u8,
                dec: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                pol: (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
            }
        }
    }
}
