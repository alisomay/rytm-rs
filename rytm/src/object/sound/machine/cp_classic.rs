use crate::{
    error::{ParameterError, RytmError},
    util::{from_s_u16_t, to_s_u16_t_union_a},
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[machine_parameters(
    lev: "0..=127" #1,
    ton: "0..=127" #2,
    nod: "0..=127" #3,
    num: "0..=127" #4,
    rat: "0..=127" #5,
    nol: "0..=127" #6,
    rnd: "0..=127" #7,
    cpt: "0..=127" #8,
)]

/// Parameters for the `CpClassic` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CpClassicParameters {
    lev: u8,
    ton: u8,
    nod: u8,
    num: u8,
    rat: u8,
    nol: u8,
    rnd: u8,
    cpt: u8,
}

impl CpClassicParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for CpClassicParameters {
    fn from(raw_sound: &ar_sound_t) -> Self {
        unsafe {
            Self {
                lev: (from_s_u16_t(&raw_sound.synth_param_1) >> 8) as u8,
                ton: (from_s_u16_t(&raw_sound.synth_param_2) >> 8) as u8,
                nod: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                num: (from_s_u16_t(&raw_sound.synth_param_4) >> 8) as u8,
                rat: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                nol: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                rnd: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                cpt: (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
            }
        }
    }
}
