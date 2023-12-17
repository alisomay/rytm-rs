use super::fx_plock_types::*;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::Trig,
    util::{i8_to_u8_midpoint_of_u8_input_range, u8_to_i8_midpoint_of_u8_input_range},
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the FX distortion reverb post.
    ///
    /// `true` = post, `false` = pre
    pub fn plock_set_fx_distortion_reverb_post(&self, reverb_send: bool) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().unwrap().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_REV as u8,
                reverb_send as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX distortion delay overdrive.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "delay_overdrive:0..=127")]
    pub fn plock_set_fx_distortion_delay_overdrive(
        &self,
        delay_overdrive: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().unwrap().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_DOV as u8,
                delay_overdrive as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX distortion delay post.
    ///
    /// `true` = post, `false` = pre
    pub fn plock_set_fx_distortion_delay_post(&self, delay_post: bool) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().unwrap().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_DELAY as u8,
                delay_post as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX distortion amount.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amount:0..=127")]
    pub fn plock_set_fx_distortion_amount(&self, amount: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().unwrap().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_AMOUNT as u8,
                amount as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX distortion symmetry.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "symmetry:-64..=63")]
    pub fn plock_set_fx_distortion_symmetry(&self, symmetry: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().unwrap().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DIST_SYM as u8,
                i8_to_u8_midpoint_of_u8_input_range(symmetry as i8, 0, 127),
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX distortion reverb post.
    ///
    /// `true` = post, `false` = pre
    pub fn plock_get_fx_distortion_reverb_post(&self) -> Result<Option<bool>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .lock()
                .unwrap()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_REV as u8);

            if let Some(value) = value {
                return Ok(Some(value != 0));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX distortion delay overdrive.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_distortion_delay_overdrive(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .lock()
                .unwrap()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_DOV as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX distortion delay post.
    ///
    /// `true` = post, `false` = pre
    pub fn plock_get_fx_distortion_delay_post(&self) -> Result<Option<bool>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .lock()
                .unwrap()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_DELAY as u8);

            if let Some(value) = value {
                return Ok(Some(value != 0));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX distortion amount.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_distortion_amount(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .lock()
                .unwrap()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_AMOUNT as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX distortion symmetry.
    ///
    /// Range `-64..=63`
    pub fn plock_get_fx_distortion_symmetry(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .lock()
                .unwrap()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_SYM as u8);

            if let Some(value) = value {
                return Ok(Some(
                    u8_to_i8_midpoint_of_u8_input_range(value, 0, 127) as isize
                ));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX distortion reverb post.
    pub fn plock_clear_fx_distortion_reverb_post(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock()
                .unwrap()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_REV as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX distortion delay overdrive.
    pub fn plock_clear_fx_distortion_delay_overdrive(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock()
                .unwrap()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_DOV as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX distortion delay post.
    pub fn plock_clear_fx_distortion_delay_post(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock()
                .unwrap()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_DELAY as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX distortion amount.
    pub fn plock_clear_fx_distortion_amount(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock()
                .unwrap()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_AMOUNT as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX distortion symmetry.
    pub fn plock_clear_fx_distortion_symmetry(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock()
                .unwrap()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DIST_SYM as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
