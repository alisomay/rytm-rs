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
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use std::{cell::RefCell, rc::Rc};

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
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct UtNoiseParameters {
    lev: u8,
    lpf: u8,
    dec: u8,
    hpf: u8,
    lpq: u8,
    atk: u8,
    swt: u8,
    swd: i8,

    #[derivative(Debug = "ignore")]
    parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
    assigned_track: Option<usize>,
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
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl UtNoiseParameters {
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
            })
        }
    }
}
