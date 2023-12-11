use crate::{
    error::{ParameterError, RytmError},
    object::pattern::parameter_lock::ParameterLockPool,
    util::{
        from_s_u16_t, get_u16_min_max_from_float_range, i8_to_u8_midpoint_of_u8_input_range,
        scale_generic, to_s_u16_t_union_a, u8_to_i8_midpoint_of_u8_input_range,
    },
};
use derivative::Derivative;
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use std::{cell::RefCell, rc::Rc};

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-32.0..=32.0" #2,
 dec: "0..=127" #3,
 det:  "-32.0..=32.0" #4,
 snp: "0..=127" #5,
 nod: "0..=127" #6,
 nol: "0..=127" #7,
 bal: "-64..=63" #8,
)]

/// Parameters for the `SdClassic` machine.
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct SdClassicParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    det: f32,
    snp: u8,
    nod: u8,
    nol: u8,
    bal: i8,

    #[derivative(Debug = "ignore")]
    parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for SdClassicParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: -6.0,
            dec: 48,
            det: 52.0,
            snp: 64,
            nod: 64,
            nol: 64,
            bal: -32,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl SdClassicParameters {
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
                det: scale_generic(
                    from_s_u16_t(&raw_sound.synth_param_4),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                    |tun: u16| tun as f32,
                ),
                snp: (from_s_u16_t(&raw_sound.synth_param_5) >> 8) as u8,
                nod: (from_s_u16_t(&raw_sound.synth_param_6) >> 8) as u8,
                nol: (from_s_u16_t(&raw_sound.synth_param_7) >> 8) as u8,
                bal: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(&raw_sound.synth_param_8) >> 8) as u8,
                    0,
                    127,
                ),
            })
        }
    }
}
