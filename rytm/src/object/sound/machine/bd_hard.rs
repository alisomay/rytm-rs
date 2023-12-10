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
 hld: "0..=127" #4,
 swt: "0..=127" #5,
 snp: "0..=127" #6,
 wav: "0..=2" #7,
 tic: "0..=127" #8,
)]

/// Parameters for the `BdHard` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BdHardParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    hld: u8,
    swt: u8,
    snp: u8,
    wav: u8,
    tic: u8,
}

impl Default for BdHardParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: -3.0,
            dec: 47,
            hld: 40,
            swt: 90,
            snp: 6,
            wav: 0,
            tic: 64,
        }
    }
}

impl BdHardParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for BdHardParameters {
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
                snp: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                wav: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                tic: (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
            }
        }
    }
}