use crate::{
    error::{ParameterError, RytmError},
    object::{
        pattern::Trig,
        sound::types::{LfoDestination, LfoMode, LfoMultiplier, LfoWaveform},
    },
    util::{
        i8_to_u8_midpoint_of_u8_input_range, scale_generic, u8_to_i8_midpoint_of_u8_input_range,
    },
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the LFO speed.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "lfo_speed:-64..=63")]
    pub fn p_lock_set_lfo_speed(&self, lfo_speed: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_SPEED as u8,
                i8_to_u8_midpoint_of_u8_input_range(lfo_speed as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO multiplier.
    pub fn p_lock_set_lfo_multiplier(
        &self,
        lfo_multiplier: LfoMultiplier,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_MULTIPLY as u8,
                lfo_multiplier.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO fade.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "lfo_fade:-64..=63")]
    pub fn p_lock_set_lfo_fade(&self, lfo_fade: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_FADE as u8,
                i8_to_u8_midpoint_of_u8_input_range(lfo_fade as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO destination.
    pub fn p_lock_set_lfo_destination(
        &self,
        lfo_destination: LfoDestination,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_DEST as u8,
                lfo_destination.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO waveform.
    pub fn p_lock_set_lfo_waveform(&self, lfo_waveform: LfoWaveform) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_WAVEFORM as u8,
                lfo_waveform.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO start phase.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "lfo_start_phase:0..=127")]
    pub fn p_lock_set_lfo_start_phase(&self, lfo_start_phase: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_PHASE as u8,
                lfo_start_phase as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO mode.
    pub fn p_lock_set_lfo_mode(&self, lfo_mode: LfoMode) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_TRIGMODE as u8,
                lfo_mode.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the LFO depth.
    ///
    /// Range `-128.0..=127.99`
    #[parameter_range(range = "lfo_depth:-128.0..=127.99")]
    pub fn p_lock_set_lfo_depth(&self, lfo_depth: f32) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let depth = scale_generic(lfo_depth, -128f32, 127.99f32, 0u16, 32767u16, |x| {
                x.round() as u16
            });

            pool.borrow_mut().set_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_DEPTH as u8,
                depth,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the LFO speed.
    ///
    /// Range `-64..=63`
    pub fn p_lock_get_lfo_speed(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_SPEED as u8,
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

    /// Gets the parameter lock for the LFO multiplier.
    pub fn p_lock_get_lfo_multiplier(&self) -> Result<Option<LfoMultiplier>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_MULTIPLY as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the LFO fade.
    ///
    /// Range `-64..=63`
    pub fn p_lock_get_lfo_fade(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_FADE as u8,
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

    /// Gets the parameter lock for the LFO destination.
    pub fn p_lock_get_lfo_destination(&self) -> Result<Option<LfoDestination>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_DEST as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the LFO waveform.
    pub fn p_lock_get_lfo_waveform(&self) -> Result<Option<LfoWaveform>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_WAVEFORM as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the LFO start phase.
    ///
    /// Range `0..=127`
    pub fn p_lock_get_lfo_start_phase(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_PHASE as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the LFO mode.
    pub fn p_lock_get_lfo_mode(&self) -> Result<Option<LfoMode>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_TRIGMODE as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the LFO depth.
    ///
    /// Range `-128.0..=127.99`
    pub fn p_lock_get_lfo_depth(&self) -> Result<Option<f32>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.borrow_mut().get_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_DEPTH as u8,
            );

            if let Some(value) = value {
                return Ok(Some(scale_generic(
                    value,
                    0u16,
                    32767u16,
                    -128f32,
                    127.99f32,
                    |x| x as f32,
                )));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the LFO speed.
    pub fn p_lock_clear_lfo_speed(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_SPEED as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the LFO multiplier.
    pub fn p_lock_clear_lfo_multiplier(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_MULTIPLY as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the LFO fade.
    pub fn p_lock_clear_lfo_fade(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_FADE as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the LFO destination.
    pub fn p_lock_clear_lfo_destination(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_DEST as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the LFO waveform.
    pub fn p_lock_clear_lfo_waveform(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_WAVEFORM as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the LFO start phase.
    pub fn p_lock_clear_lfo_start_phase(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_PHASE as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the LFO mode.
    pub fn p_lock_clear_lfo_mode(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_TRIGMODE as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the LFO depth.
    pub fn p_lock_clear_lfo_depth(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().clear_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_LFO_DEPTH as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
