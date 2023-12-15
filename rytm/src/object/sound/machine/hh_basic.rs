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
    tun: "-64..=63" #2,
    dec: "0..=127" #3,
    ton:  "-64..=63" #4,
    trd: "0..=127" #5,
    // rst #6 
    // Unavailable #7
    // Unavailable #8
)]
/// Parameters for the `HhBasic` machine.
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct HhBasicParameters {
    lev: u8,
    tun: i8,
    dec: u8,
    ton: i8,
    trd: u8,
    rst: bool,

    #[derivative(Debug = "ignore")]
    parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for HhBasicParameters {
    fn default() -> Self {
        Self {
            lev: 110,
            tun: 0,
            dec: 29,
            ton: 0,
            trd: 68,
            rst: false,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl HhBasicParameters {
    pub(crate) fn link_parameter_lock_pool(&mut self, pool: Rc<RefCell<ParameterLockPool>>) {
        self.parameter_lock_pool = Some(pool);
    }

    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
        raw_sound.synth_param_6 = to_s_u16_t_union_a(((self.rst as u8) as u16) << 8);
    }

    /// Returns the `rst` parameter.
    pub const fn get_rst(&self) -> bool {
        self.rst
    }

    /// Sets the `rst` parameter.
    pub fn set_rst(&mut self, rst: bool) {
        self.rst = rst;
    }

    /// Sets the parameter lock for the `rst` parameter.
    pub fn plock_set_rst(&self, rst: bool, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.borrow_mut().set_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
                rst as u8,
            )?;
            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the `rst` parameter.
    pub fn plock_get_rst(&self, trig_index: usize) -> Result<Option<bool>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            let rst = pool.borrow_mut().get_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
            );
            if let Some(rst) = rst {
                return Ok(Some(rst != 0));
            }
            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the `rst` parameter if set.
    pub fn plock_clear_rst(&self, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.borrow_mut().clear_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
            );
            return Ok(());
        }
        Err(OrphanTrig)
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
                ton: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(raw_sound.synth_param_4) >> 8) as u8,
                    0,
                    127,
                ),
                trd: (from_s_u16_t(raw_sound.synth_param_5) >> 8) as u8,
                rst: from_s_u16_t(raw_sound.synth_param_6) >> 8 != 0,
            })
        }
    }
}
