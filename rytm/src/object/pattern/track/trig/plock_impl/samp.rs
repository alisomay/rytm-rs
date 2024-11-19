use crate::{
    error::{ParameterError, RytmError},
    object::pattern::Trig,
    util::{
        i8_to_u8_midpoint_of_u8_input_range, scale_f32_to_u16, scale_u16_to_f32,
        u8_to_i8_midpoint_of_u8_input_range,
    },
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the sample tune.
    ///
    /// Range `-24..=24`
    #[parameter_range(range = "sample_tune:-24..=24")]
    pub fn plock_set_sample_tune(&self, sample_tune: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_TUNE as u8,
                i8_to_u8_midpoint_of_u8_input_range(sample_tune as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample fine tune.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "sample_fine_tune:-64..=63")]
    pub fn plock_set_sample_fine_tune(&self, sample_fine_tune: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_FINE as u8,
                i8_to_u8_midpoint_of_u8_input_range(sample_fine_tune as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample number.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "sample_number:0..=127")]
    pub fn plock_set_sample_number(&self, sample_number: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_NR as u8,
                sample_number as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample bit reduction.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "sample_bit_reduction:0..=127")]
    pub fn plock_set_sample_bit_reduction(
        &self,
        sample_bit_reduction: usize,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_BITRDC as u8,
                sample_bit_reduction as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample start.
    ///
    /// Range `0.0..=120.0`
    #[parameter_range(range = "sample_start:0.0..=120.0")]
    pub fn plock_set_sample_start(&self, sample_start: f32) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let start = scale_f32_to_u16(sample_start, 0f32, 120.0f32, 0u16, 30720u16);

            pool.lock().set_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_START as u8,
                start,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample end.
    ///
    /// Range `0.0..=120.0`
    #[parameter_range(range = "sample_end:0.0..=120.0")]
    pub fn plock_set_sample_end(&self, sample_end: f32) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let end = scale_f32_to_u16(sample_end, 0f32, 120.0f32, 0u16, 30720u16);

            pool.lock().set_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_END as u8,
                end,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample loop flag.
    pub fn plock_set_sample_loop_flag(&self, sample_loop_flag: bool) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_LOOPSW as u8,
                sample_loop_flag as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the sample volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "sample_volume:0..=127")]
    pub fn plock_set_sample_volume(&self, sample_volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().set_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_LEVEL as u8,
                sample_volume as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the sample tune.
    ///
    /// Range `-24..=24`
    pub fn plock_get_sample_tune(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_TUNE as u8,
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

    /// Gets the parameter lock for the sample fine tune.
    ///
    /// Range `-64..=63`
    pub fn plock_get_sample_fine_tune(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_FINE as u8,
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

    /// Gets the parameter lock for the sample number.
    ///
    /// Range `0..=127`
    pub fn plock_get_sample_number(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_NR as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the sample bit reduction.
    ///
    /// Range `0..=127`
    pub fn plock_get_sample_bit_reduction(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_BITRDC as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the sample start.
    ///
    /// Range `0.0..=120.0`
    pub fn plock_get_sample_start(&self) -> Result<Option<f32>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_START as u8,
            );

            if let Some(value) = value {
                return Ok(Some(scale_u16_to_f32(value, 0u16, 30720u16, 0f32, 120f32)));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the sample end.
    ///
    /// Range `0.0..=120.0`
    pub fn plock_get_sample_end(&self) -> Result<Option<f32>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_END as u8,
            );

            if let Some(value) = value {
                return Ok(Some(scale_u16_to_f32(value, 0u16, 30720u16, 0f32, 120f32)));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the sample loop flag.
    pub fn plock_get_sample_loop_flag(&self) -> Result<Option<bool>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_LOOPSW as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value != 0));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the sample volume.
    ///
    /// Range `0..=127`
    pub fn plock_get_sample_volume(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool.lock().get_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_LEVEL as u8,
            );

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the sample tune.
    pub fn plock_clear_sample_tune(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_TUNE as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the sample fine tune.
    pub fn plock_clear_sample_fine_tune(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_FINE as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the sample number.
    pub fn plock_clear_sample_number(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_NR as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the sample bit reduction.
    pub fn plock_clear_sample_bit_reduction(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_BITRDC as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the sample start.
    pub fn plock_clear_sample_start(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_START as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the sample end.
    pub fn plock_clear_sample_end(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_compound_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_END as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the sample loop flag.
    pub fn plock_clear_sample_loop_flag(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_LOOPSW as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the sample volume.
    pub fn plock_clear_sample_volume(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.lock().clear_basic_plock(
                self.index,
                self.track_index as u8,
                rytm_sys::AR_PLOCK_TYPE_SMP_LEVEL as u8,
            );

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
