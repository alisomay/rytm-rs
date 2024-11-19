use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::{
        from_s_u16_t, get_u16_min_max_from_float_range, i8_to_u8_midpoint_of_u8_input_range,
        scale_f32_to_u16, scale_u16_to_f32, to_s_u16_t_union_a,
        u8_to_i8_midpoint_of_u8_input_range,
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
 t1: "-32.0..=32.0" #2,
 dec: "0..=127" #3,
 bal: "-64..=63" #4,
 t2: "-32.0..=32.0" #5,
 sym: "-64..=63" #6,
 nol: "0..=127" #7,
 tic: "0..=127" #8,
)]
/// Parameters for the `RsClassic` machine.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct RsClassicParameters {
    lev: u8,
    t1: f32,
    dec: u8,
    bal: i8,
    t2: f32,
    sym: i8,
    nol: u8,
    tic: u8,

    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for RsClassicParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            t1: 0.0,
            dec: 64,
            bal: 0,
            t2: 0.0,
            sym: 0,
            nol: 16,
            tic: 64,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl RsClassicParameters {
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
                t1: scale_u16_to_f32(
                    from_s_u16_t(raw_sound.synth_param_2),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                ),
                dec: (from_s_u16_t(raw_sound.synth_param_3) >> 8) as u8,
                bal: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(raw_sound.synth_param_4) >> 8) as u8,
                    0,
                    127,
                ),
                t2: scale_u16_to_f32(
                    from_s_u16_t(raw_sound.synth_param_5),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                ),
                sym: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(raw_sound.synth_param_6) >> 8) as u8,
                    0,
                    127,
                ),
                nol: (from_s_u16_t(raw_sound.synth_param_7) >> 8) as u8,
                tic: (from_s_u16_t(raw_sound.synth_param_8) >> 8) as u8,
            })
        }
    }
}
