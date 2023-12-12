use crate::util::scale_f32_to_u16;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::parameter_lock::ParameterLockPool,
    util::{
        from_s_u16_t, i8_to_u8_midpoint_of_u8_input_range, to_s_u16_t_union_a,
        u8_to_i8_midpoint_of_u8_input_range,
    },
    RytmError::OrphanTrig,
};
use derivative::Derivative;
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use std::{cell::RefCell, rc::Rc};

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-64..=63" #2,
 dec: "0..=127" #3,
 col: "-64..=63" #4,
 // Unavailable #5
 // Unavailable #6
 // Unavailable #7
 // Unavailable #8
)]
/// Parameters for the `OhClassic` machine.
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct OhClassicParameters {
    lev: u8,
    tun: i8,
    dec: u8,
    col: i8,

    #[derivative(Debug = "ignore")]
    parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for OhClassicParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: 0,
            dec: 32,
            col: 0,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl OhClassicParameters {
    pub(crate) fn link_parameter_lock_pool(&mut self, pool: Rc<RefCell<ParameterLockPool>>) {
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
                lev: (from_s_u16_t(&raw_sound.synth_param_1) >> 8) as u8,
                tun: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_2) >> 8) as u8,
                    0,
                    127,
                ),
                dec: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                col: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_4) >> 8) as u8,
                    0,
                    127,
                ),
            })
        }
    }
}
