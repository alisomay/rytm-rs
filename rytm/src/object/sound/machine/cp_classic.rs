use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::{from_s_u16_t, to_s_u16_t_union_a},
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
    ton: "0..=127" #2,
    nod: "0..=127" #3,
    num: "0..=127" #4,
    rat: "0..=127" #5,
    nol: "0..=127" #6,
    rnd: "0..=127" #7,
    cpt: "0..=127" #8,
)]
/// Parameters for the `CpClassic` machine.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct CpClassicParameters {
    lev: u8,
    ton: u8,
    nod: u8,
    num: u8,
    rat: u8,
    nol: u8,
    rnd: u8,
    cpt: u8,

    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for CpClassicParameters {
    fn default() -> Self {
        Self {
            lev: 115,
            ton: 64,
            nod: 55,
            num: 2,
            rat: 85,
            nol: 90,
            rnd: 32,
            cpt: 90,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl CpClassicParameters {
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
                ton: (from_s_u16_t(raw_sound.synth_param_2) >> 8) as u8,
                nod: (from_s_u16_t(raw_sound.synth_param_3) >> 8) as u8,
                num: (from_s_u16_t(raw_sound.synth_param_4) >> 8) as u8,
                rat: (from_s_u16_t(raw_sound.synth_param_5) >> 8) as u8,
                nol: (from_s_u16_t(raw_sound.synth_param_6) >> 8) as u8,
                rnd: (from_s_u16_t(raw_sound.synth_param_7) >> 8) as u8,
                cpt: (from_s_u16_t(raw_sound.synth_param_8) >> 8) as u8,
            })
        }
    }
}
