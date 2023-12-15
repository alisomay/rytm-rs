use crate::util::scale_f32_to_u16;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::{from_s_u16_t, get_u16_min_max_from_float_range, scale_u16_to_f32, to_s_u16_t_union_a},
    RytmError::OrphanTrig,
};
use derivative::Derivative;
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use std::{cell::RefCell, rc::Rc};

#[machine_parameters(
    lev: "0..=127" #1,
    tun: "-32.0..=32.0" #2,
    dec: "0..=127" #3,
    hld: "0..=127" #4,
    swt: "0..=127" #5,
    swd: "0..=127" #6,
    wav: "0..=2" #7,
    tra: "0..=127" #8,
)]
/// Parameters for the `BdClassic` machine.
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct BdClassicParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    hld: u8,
    swt: u8,
    swd: u8,
    wav: u8,
    tra: u8,

    #[derivative(Debug = "ignore")]
    parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for BdClassicParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: -6.0,
            dec: 32,
            hld: 32,
            swt: 64,
            swd: 42,
            wav: 1,
            tra: 17,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl BdClassicParameters {
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
                hld: (from_s_u16_t(raw_sound.synth_param_4) >> 8) as u8,
                swt: (from_s_u16_t(raw_sound.synth_param_5) >> 8) as u8,
                swd: (from_s_u16_t(raw_sound.synth_param_6) >> 8) as u8,
                wav: (from_s_u16_t(raw_sound.synth_param_7) >> 8) as u8,
                tra: (from_s_u16_t(raw_sound.synth_param_8) >> 8) as u8,
            })
        }
    }
}
