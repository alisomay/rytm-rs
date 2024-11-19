use crate::{
    error::{ParameterError, RytmError},
    object::pattern::Trig,
    util::{i8_to_u8_midpoint_of_u8_input_range, u8_to_i8_midpoint_of_u8_input_range},
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the amplitude attack.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_attack:0..=127")]
    pub fn plock_set_amplitude_attack(&self, amplitude_attack: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_ATTACK as u8,
                amplitude_attack as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude hold.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_hold:0..=127")]
    pub fn plock_set_amplitude_hold(&self, amplitude_hold: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_HOLD as u8,
                amplitude_hold as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude decay.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_decay:0..=127")]
    pub fn plock_set_amplitude_decay(&self, amplitude_decay: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DECAY as u8,
                amplitude_decay as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude overdrive.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_overdrive:0..=127")]
    pub fn plock_set_amplitude_overdrive(
        &self,
        amplitude_overdrive: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DRIVE as u8,
                amplitude_overdrive as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude delay send.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_delay_send:0..=127")]
    pub fn plock_set_amplitude_delay_send(
        &self,
        amplitude_delay_send: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DELAY as u8,
                amplitude_delay_send as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude reverb send.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_reverb_send:0..=127")]
    pub fn plock_set_amplitude_reverb_send(
        &self,
        amplitude_reverb_send: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_REVERB as u8,
                amplitude_reverb_send as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude pan.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "amplitude_pan:-64..=63")]
    pub fn plock_set_amplitude_pan(&self, amplitude_pan: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_PAN as u8,
                i8_to_u8_midpoint_of_u8_input_range(amplitude_pan as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the amplitude volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "amplitude_volume:0..=127")]
    pub fn plock_set_amplitude_volume(&self, amplitude_volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_VOLUME as u8,
                amplitude_volume as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the amplitude attack.
    ///
    /// Range `0..=127`
    pub fn plock_get_amplitude_attack(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_ATTACK as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the amplitude hold.
    ///
    /// Range `0..=127`
    pub fn plock_get_amplitude_hold(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_HOLD as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the amplitude decay.
    ///
    /// Range `0..=127`
    pub fn plock_get_amplitude_decay(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DECAY as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the amplitude overdrive.
    ///
    /// Range `0..=127`
    pub fn plock_get_amplitude_overdrive(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DRIVE as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the amplitude delay send.
    ///
    /// Range `0..=127`
    pub fn plock_get_amplitude_delay_send(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DELAY as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the amplitude reverb send.
    ///
    /// Range `0..=127`
    pub fn plock_get_amplitude_reverb_send(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_REVERB as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the amplitude pan.
    ///
    /// Range `-64..=63`
    pub fn plock_get_amplitude_pan(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_PAN as u8,
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

    /// Gets the parameter lock for the amplitude volume.
    ///
    /// Range `0..=127`
    pub fn plock_get_amplitude_volume(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_VOLUME as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the amplitude attack.
    pub fn plock_clear_amplitude_attack(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_ATTACK as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the amplitude hold.
    pub fn plock_clear_amplitude_hold(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_HOLD as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the amplitude decay.
    pub fn plock_clear_amplitude_decay(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DECAY as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the amplitude overdrive.
    pub fn plock_clear_amplitude_overdrive(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DRIVE as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the amplitude delay send.
    pub fn plock_clear_amplitude_delay_send(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_DELAY as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the amplitude reverb send.
    pub fn plock_clear_amplitude_reverb_send(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_REVERB as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the amplitude pan.
    pub fn plock_clear_amplitude_pan(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_PAN as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the amplitude volume.
    pub fn plock_clear_amplitude_volume(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_AMP_VOLUME as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
