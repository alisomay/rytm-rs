use crate::{
    error::{ParameterError, RytmError},
    object::{pattern::Trig, sound::types::FilterType},
    util::{i8_to_u8_midpoint_of_u8_input_range, u8_to_i8_midpoint_of_u8_input_range},
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the filter attack.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_attack:0..=127")]
    pub fn p_lock_set_filter_attack(&self, filter_attack: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_ATTACK as u8,
                filter_attack as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter sustain.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_sustain:0..=127")]
    pub fn p_lock_set_filter_sustain(&self, filter_sustain: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_SUSTAIN as u8,
                filter_sustain as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter decay.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_decay:0..=127")]
    pub fn p_lock_set_filter_decay(&self, filter_decay: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_DECAY as u8,
                filter_decay as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter release.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_release:0..=127")]
    pub fn p_lock_set_filter_release(&self, filter_release: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_RELEASE as u8,
                filter_release as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter cutoff.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_cutoff:0..=127")]
    pub fn p_lock_set_filter_cutoff(&self, filter_cutoff: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_FREQ as u8,
                filter_cutoff as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter resonance.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "filter_resonance:0..=127")]
    pub fn p_lock_set_filter_resonance(&self, filter_resonance: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_RESO as u8,
                filter_resonance as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter type.
    pub fn p_lock_set_filter_type(&self, filter_type: FilterType) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_TYPE as u8,
                filter_type.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the filter envelope amount.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "filter_envelope_amount:-64..=63")]
    pub fn p_lock_set_filter_envelope_amount(
        &self,
        filter_envelope_amount: isize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_ENV as u8,
                i8_to_u8_midpoint_of_u8_input_range(filter_envelope_amount as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the filter attack
    ///
    /// Range `0..=127`
    pub fn p_lock_get_filter_attack(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_ATTACK as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the filter decay
    ///
    /// Range `0..=127`
    pub fn p_lock_get_filter_decay(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_DECAY as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the filter sustain
    ///
    /// Range `0..=127`
    pub fn p_lock_get_filter_sustain(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_SUSTAIN as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the filter release
    ///
    /// Range `0..=127`
    pub fn p_lock_get_filter_release(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_RELEASE as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the filter cutoff
    ///
    /// Range `0..=127`
    pub fn p_lock_get_filter_cutoff(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_FREQ as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the filter resonance
    ///
    /// Range `0..=127`
    pub fn p_lock_get_filter_resonance(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_RESO as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the filter type
    ///
    /// Range `0..=127`
    pub fn p_lock_get_filter_type(&self) -> Result<Option<FilterType>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_TYPE as u8,
            );

            if let Some(value) = value {
                return Ok(Some(FilterType::try_from(value)?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the filter envelope amount
    ///
    /// Range `-64..=63`
    pub fn p_lock_get_filter_envelope_amount(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_ENV as u8,
            );

            if let Some(value) = value {
                return Ok(Some(
                    u8_to_i8_midpoint_of_u8_input_range(value, 0, 127) as isize
                ));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the filter attack
    pub fn p_lock_clear_filter_attack(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_ATTACK as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the filter decay
    pub fn p_lock_clear_filter_decay(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_DECAY as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the filter sustain
    pub fn p_lock_clear_filter_sustain(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_SUSTAIN as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the filter release
    pub fn p_lock_clear_filter_release(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_RELEASE as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the filter cutoff
    pub fn p_lock_clear_filter_cutoff(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_FREQ as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the filter resonance
    pub fn p_lock_clear_filter_resonance(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_RESO as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the filter type
    pub fn p_lock_clear_filter_type(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_TYPE as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the filter envelope amount
    pub fn p_lock_clear_filter_envelope_amount(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_FLT_ENV as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
