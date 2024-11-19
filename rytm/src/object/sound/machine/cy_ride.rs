use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::{
        from_s_u16_t, i8_to_u8_midpoint_of_u8_input_range, to_s_u16_t_union_a,
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
    tun: "-64..=63" #2,
    dec: "0..=127" #3,
    // (0..=3 = A..=D)
    typ: "0..=3" #4,
    hit: "0..=127" #5,
    c1: "0..=127" #6,
    c2: "0..=127" #7,
    c3: "0..=127" #8,
)]
/// Parameters for the `CyRide` machine.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct CyRideParameters {
    lev: u8,
    tun: i8,
    dec: u8,
    typ: u8,
    hit: u8,
    c1: u8,
    c2: u8,
    c3: u8,

    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for CyRideParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: 0,
            dec: 64,
            typ: 0,
            hit: 64,
            c1: 64,
            c2: 64,
            c3: 32,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl CyRideParameters {
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
        unsafe {
            Ok(Self {
                parameter_lock_pool: None,
                assigned_track: track_index,
                lev: (from_s_u16_t(raw_sound.synth_param_1) >> 8) as u8,
                tun: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(raw_sound.synth_param_2) >> 8) as u8,
                    0,
                    127,
                ),
                dec: (from_s_u16_t(raw_sound.synth_param_3) >> 8) as u8,
                typ: (from_s_u16_t(raw_sound.synth_param_4) >> 8) as u8,
                hit: (from_s_u16_t(raw_sound.synth_param_5) >> 8) as u8,
                c1: (from_s_u16_t(raw_sound.synth_param_6) >> 8) as u8,
                c2: (from_s_u16_t(raw_sound.synth_param_7) >> 8) as u8,
                c3: (from_s_u16_t(raw_sound.synth_param_8) >> 8) as u8,
            })
        }
    }
}
