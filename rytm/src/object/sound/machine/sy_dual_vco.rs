use crate::util::scale_f32_to_u16;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::parameter_lock::ParameterLockPool,
    util::{
        from_s_u16_t, get_u16_min_max_from_float_range, i8_to_u8_midpoint_of_u8_input_range,
        scale_u16_to_f32, to_s_u16_t_union_a, u8_to_i8_midpoint_of_u8_input_range,
    },
    RytmError::OrphanTrig,
};
use derivative::Derivative;
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use std::{cell::RefCell, rc::Rc};

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-32.0..=32.0" #2,
 dec1: "0..=127" #3,
 det: "-16.0..=16.0" #4,
 dec2: "0..=127" #5,
 bal: "-64..=63" #6,
 bnd: "-64..=63" #7,
 cfg: "0..=79" #8, // (0..79)
)]
/// Parameters for the `SyDualVco` machine.
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct SyDualVcoParameters {
    lev: u8,
    tun: f32,
    dec1: u8,
    det: f32,
    dec2: u8,
    bal: i8,
    bnd: i8,
    cfg: u8,

    #[derivative(Debug = "ignore")]
    parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for SyDualVcoParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: 0.0,
            dec1: 70,
            det: 0.0,
            dec2: 70,
            bal: 0,
            bnd: 0,
            cfg: 16,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl SyDualVcoParameters {
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

        let output_det_min: f32 = -16.;
        let output_det_max: f32 = 16.;
        let (input_det_min, input_det_max) =
            get_u16_min_max_from_float_range(output_det_min, output_det_max);

        unsafe {
            Ok(Self {
                parameter_lock_pool: None,
                assigned_track: track_index,
                lev: (from_s_u16_t(&raw_sound.synth_param_1) >> 8) as u8,
                tun: scale_u16_to_f32(
                    from_s_u16_t(&raw_sound.synth_param_2),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                ),
                dec1: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                det: scale_u16_to_f32(
                    from_s_u16_t(&raw_sound.synth_param_4),
                    input_det_min,
                    input_det_max,
                    output_det_min,
                    output_det_max,
                ),
                dec2: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                bal: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                    0,
                    127,
                ),
                bnd: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                    0,
                    127,
                ),
                cfg: (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
            })
        }
    }
}
