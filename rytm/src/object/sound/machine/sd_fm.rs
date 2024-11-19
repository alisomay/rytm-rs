use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::{
        from_s_u16_t, get_u16_min_max_from_float_range, scale_f32_to_u16, scale_u16_to_f32,
        to_s_u16_t_union_a,
    },
    RytmError::OrphanTrig,
};
use derivative::Derivative;
use parking_lot::Mutex;
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-32.0..=32.0" #2,
 dec: "0..=127" #3,
 fmt:  "-32.0..=32.0" #4,
 fmd: "0..=127" #5,
 nod: "0..=127" #6,
 nol: "0..=127" #7,
 fma: "0..=127" #8,
)]
/// Parameters for the `SdFm` machine.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct SdFmParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    fmt: f32,
    fmd: u8,
    nod: u8,
    nol: u8,
    fma: u8,

    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for SdFmParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: -6.0,
            dec: 64,
            fmt: 12.0,
            fmd: 64,
            nod: 32,
            nol: 64,
            fma: 127,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl SdFmParameters {
    pub(crate) fn link_parameter_lock_pool(&mut self, pool: Arc<Mutex<ParameterLockPool>>) {
        self.parameter_lock_pool = Some(pool);
    }

    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
    }

    #[parameter_range(range = "track_index[opt]:0..=11")]
    pub(crate) fn from_raw_sound(
        raw_sound: &ar_sound_t,
        track_index: Option<usize>,
    ) -> Result<Self, RytmError> {
        let output_tun_min: f32 = -32.;
        let output_tun_max: f32 = 32.;
        let (input_tun_min, input_tun_max) =
            get_u16_min_max_from_float_range(output_tun_min, output_tun_max);
        unsafe {
            Ok(Self {
                parameter_lock_pool: None,
                assigned_track: track_index,
                lev: (from_s_u16_t(raw_sound.synth_param_1) >> 8) as u8,
                tun: scale_u16_to_f32(
                    from_s_u16_t(raw_sound.synth_param_2),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                ),
                dec: (from_s_u16_t(raw_sound.synth_param_3) >> 8) as u8,
                fmt: scale_u16_to_f32(
                    from_s_u16_t(raw_sound.synth_param_4),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                ),
                fmd: (from_s_u16_t(raw_sound.synth_param_5) >> 8) as u8,
                nod: (from_s_u16_t(raw_sound.synth_param_6) >> 8) as u8,
                nol: (from_s_u16_t(raw_sound.synth_param_7) >> 8) as u8,
                fma: (from_s_u16_t(raw_sound.synth_param_8) >> 8) as u8,
            })
        }
    }
}
