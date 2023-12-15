use super::fx_plock_types::*;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::Trig,
    util::{i8_to_u8_midpoint_of_u8_input_range, u8_to_i8_midpoint_of_u8_input_range},
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the FX delay time.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "time:0..=127")]
    pub fn plock_set_fx_delay_time(&self, time: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_TIME as u8,
                time as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay ping pong.
    pub fn plock_set_fx_delay_ping_pong(&self, ping_pong: bool) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_PING_PONG as u8,
                ping_pong as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay stereo width.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "stereo_width:-64..=63")]
    pub fn plock_set_fx_delay_stereo_width(&self, stereo_width: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_WIDTH as u8,
                i8_to_u8_midpoint_of_u8_input_range(stereo_width as i8, 0, 127),
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay feedback.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "feedback:0..=127")]
    pub fn plock_set_fx_delay_feedback(&self, feedback: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_FEEDBACK as u8,
                feedback as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay high pass filter.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "hpf:0..=127")]
    pub fn plock_set_fx_delay_hpf(&self, hpf: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_HPF as u8,
                hpf as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay low pass filter.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "lpf:0..=127")]
    pub fn plock_set_fx_delay_lpf(&self, lpf: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_LPF as u8,
                lpf as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay reverb send.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "reverb_send:0..=127")]
    pub fn plock_set_fx_delay_reverb_send(&self, reverb_send: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_REV as u8,
                reverb_send as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX delay volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn plock_set_fx_delay_volume(&self, volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_DELAY_VOL as u8,
                volume as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX delay time.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_delay_time(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_TIME as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX delay ping pong.
    pub fn plock_get_fx_delay_ping_pong(&self) -> Result<Option<bool>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_PING_PONG as u8);

            if let Some(value) = value {
                return Ok(Some(value != 0));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX delay stereo width.
    ///
    /// Range `-64..=63`
    pub fn plock_get_fx_delay_stereo_width(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_WIDTH as u8);

            if let Some(value) = value {
                return Ok(Some(
                    u8_to_i8_midpoint_of_u8_input_range(value, 0, 127) as isize
                ));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX delay feedback.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_delay_feedback(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_FEEDBACK as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX delay high pass filter.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_delay_hpf(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_HPF as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX delay low pass filter.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_delay_lpf(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_LPF as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX delay reverb send.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_delay_reverb_send(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_REV as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX delay volume.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_delay_volume(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_VOL as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX delay time.
    pub fn plock_clear_fx_delay_time(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_TIME as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX delay ping pong.
    pub fn plock_clear_fx_delay_ping_pong(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_PING_PONG as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX delay stereo width.
    pub fn plock_clear_fx_delay_stereo_width(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_WIDTH as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX delay feedback.
    pub fn plock_clear_fx_delay_feedback(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_FEEDBACK as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX delay high pass filter.
    pub fn plock_clear_fx_delay_hpf(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_HPF as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX delay low pass filter.
    pub fn plock_clear_fx_delay_lpf(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_LPF as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX delay reverb send.
    pub fn plock_clear_fx_delay_reverb_send(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_REV as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX delay volume.
    pub fn plock_clear_fx_delay_volume(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_DELAY_VOL as u8);
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
