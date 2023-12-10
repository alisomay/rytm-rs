use crate::{
    error::{ParameterError, RytmError},
    util::{from_s_u16_t, get_u16_min_max_from_float_range, scale_generic, to_s_u16_t_union_a},
};
use rytm_rs_macro::machine_parameters;
use rytm_sys::ar_sound_t;

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-24.0..=24.0" #2,
 bdy: "0..=127" #3,
 nod: "0..=127" #4,
 nol: "0..=127" #5,
 hld: "0..=127" #6,
 swd: "0..=127" #7,
 imp: "0..=127" #8,
)]

/// Parameters for the `SdAcoustic` machine.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SdAcousticParameters {
    lev: u8,
    tun: f32,
    bdy: u8,
    nod: u8,
    nol: u8,
    hld: u8,
    swd: u8,
    imp: u8,
}

impl Default for SdAcousticParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: 6.0,
            bdy: 64,
            nod: 64,
            nol: 64,
            hld: 64,
            swd: 32,
            imp: 64,
        }
    }
}

impl SdAcousticParameters {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }
}

impl From<&ar_sound_t> for SdAcousticParameters {
    fn from(raw_sound: &ar_sound_t) -> Self {
        let output_tun_min: f32 = -24.;
        let output_tun_max: f32 = 24.;
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
                nod: (from_s_u16_t(&raw_sound.synth_param_4) >> 8) as u8,
                nol: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                hld: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                swd: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                imp: (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
            }
        }
    }
}
