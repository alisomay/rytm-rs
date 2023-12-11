use crate::{
    error::{ParameterError, RytmError},
    object::pattern::parameter_lock::ParameterLockPool,
    util::{from_s_u16_t, to_s_u16_t_union_a},
};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;
use std::{cell::RefCell, rc::Rc};

/// Parameters for the `HhLab` machine.
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct HhLabParameters {
    lev: u8,
    osc1: u16, // 0..=16256
    dec: u8,
    osc2: u16,
    osc3: u16,
    osc4: u16,
    osc5: u16,
    osc6: u16,

    #[derivative(Debug = "ignore")]
    parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for HhLabParameters {
    fn default() -> Self {
        Self {
            lev: 110,
            osc1: 512,
            dec: 29,
            osc2: 768,
            osc3: 1024,
            osc4: 1280,
            osc5: 1536,
            osc6: 1792,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl HhLabParameters {
    pub(crate) fn link_parameter_lock_pool(&mut self, pool: Rc<RefCell<ParameterLockPool>>) {
        self.parameter_lock_pool = Some(pool);
    }

    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        raw_sound.synth_param_1 = to_s_u16_t_union_a((self.lev as u16) << 8);
        raw_sound.synth_param_2 = to_s_u16_t_union_a(self.osc1);
        raw_sound.synth_param_3 = to_s_u16_t_union_a((self.dec as u16) << 8);
        raw_sound.synth_param_4 = to_s_u16_t_union_a(self.osc2);
        raw_sound.synth_param_5 = to_s_u16_t_union_a(self.osc3);
        raw_sound.synth_param_6 = to_s_u16_t_union_a(self.osc4);
        raw_sound.synth_param_7 = to_s_u16_t_union_a(self.osc5);
        raw_sound.synth_param_8 = to_s_u16_t_union_a(self.osc6);
    }

    /// Sets the `lev` parameter.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "lev:0..=127")]
    pub fn set_lev(&mut self, lev: usize) -> Result<(), RytmError> {
        self.lev = lev as u8;
        Ok(())
    }

    /// Sets the `osc1` parameter.
    ///
    /// Range: `0..=16256`
    #[parameter_range(range = "osc1:0..=16256")]
    pub fn set_osc1(&mut self, osc1: usize) -> Result<(), RytmError> {
        self.osc1 = osc1 as u16;
        Ok(())
    }

    /// Sets the `dec` parameter.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "dec:0..=127")]
    pub fn set_dec(&mut self, dec: usize) -> Result<(), RytmError> {
        self.dec = dec as u8;
        Ok(())
    }

    /// Sets the `osc2` parameter.
    ///
    /// Range: `0..=16256`
    #[parameter_range(range = "osc2:0..=16256")]
    pub fn set_osc2(&mut self, osc2: usize) -> Result<(), RytmError> {
        self.osc2 = osc2 as u16;
        Ok(())
    }

    /// Sets the `osc3` parameter.
    ///
    /// Range: `0..=16256`
    #[parameter_range(range = "osc3:0..=16256")]
    pub fn set_osc3(&mut self, osc3: usize) -> Result<(), RytmError> {
        self.osc3 = osc3 as u16;
        Ok(())
    }

    /// Sets the `osc4` parameter.
    ///
    /// Range: `0..=16256`
    #[parameter_range(range = "osc4:0..=16256")]
    pub fn set_osc4(&mut self, osc4: usize) -> Result<(), RytmError> {
        self.osc4 = osc4 as u16;
        Ok(())
    }

    /// Sets the `osc5` parameter.
    ///
    /// Range: `0..=16256`
    #[parameter_range(range = "osc5:0..=16256")]
    pub fn set_osc5(&mut self, osc5: usize) -> Result<(), RytmError> {
        self.osc5 = osc5 as u16;
        Ok(())
    }

    /// Sets the `osc6` parameter.
    ///
    /// Range: `0..=16256`
    #[parameter_range(range = "osc6:0..=16256")]
    pub fn set_osc6(&mut self, osc6: usize) -> Result<(), RytmError> {
        self.osc6 = osc6 as u16;
        Ok(())
    }

    /// Returns the `lev` parameter.
    ///
    /// Range: `0..=127`
    pub fn lev(&self) -> usize {
        self.lev as usize
    }

    /// Returns the `osc1` parameter.
    ///
    /// Range: `0..=16256`
    pub fn osc1(&self) -> usize {
        self.osc1 as usize
    }

    /// Returns the `dec` parameter.
    ///
    /// Range: `0..=127`
    pub fn dec(&self) -> usize {
        self.dec as usize
    }

    /// Returns the `osc2` parameter.
    ///
    /// Range: `0..=16256`
    pub fn osc2(&self) -> usize {
        self.osc2 as usize
    }

    /// Returns the `osc3` parameter.
    ///
    /// Range: `0..=16256`
    pub fn osc3(&self) -> usize {
        self.osc3 as usize
    }

    /// Returns the `osc4` parameter.
    ///
    /// Range: `0..=16256`
    pub fn osc4(&self) -> usize {
        self.osc4 as usize
    }

    /// Returns the `osc5` parameter.
    ///
    /// Range: `0..=16256`
    pub fn osc5(&self) -> usize {
        self.osc5 as usize
    }

    /// Returns the `osc6` parameter.
    ///
    /// Range: `0..=16256`
    pub fn osc6(&self) -> usize {
        self.osc6 as usize
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
                osc1: from_s_u16_t(&raw_sound.synth_param_2),
                dec: (from_s_u16_t(&raw_sound.synth_param_3) >> 8) as u8,
                osc2: from_s_u16_t(&raw_sound.synth_param_4),
                osc3: from_s_u16_t(&raw_sound.synth_param_5),
                osc4: from_s_u16_t(&raw_sound.synth_param_6),
                osc5: from_s_u16_t(&raw_sound.synth_param_7),
                osc6: from_s_u16_t(&raw_sound.synth_param_8),
            })
        }
    }
}
