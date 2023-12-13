use super::fx_plock_types::*;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::Trig,
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the FX reverb pre delay.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "pre_delay:0..=127")]
    pub fn plock_set_fx_reverb_pre_delay(&self, pre_delay: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_PRE as u8,
                pre_delay as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb decay.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "decay:0..=127")]
    pub fn plock_set_fx_reverb_decay(&self, decay: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_DECAY as u8,
                decay as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb frequency.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "freq:0..=127")]
    pub fn plock_set_fx_reverb_freq(&self, freq: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_FREQ as u8,
                freq as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb gain.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "gain:0..=127")]
    pub fn plock_set_fx_reverb_gain(&self, gain: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_GAIN as u8,
                gain as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb high pass filter.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "hpf:0..=127")]
    pub fn plock_set_fx_reverb_hpf(&self, hpf: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_HPF as u8,
                hpf as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb low pass filter.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "lpf:0..=127")]
    pub fn plock_set_fx_reverb_lpf(&self, lpf: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_LPF as u8,
                lpf as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX reverb volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn plock_set_fx_reverb_volume(&self, volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_REVERB_VOL as u8,
                volume as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX reverb pre delay.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_reverb_pre_delay(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_PRE as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX reverb decay.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_reverb_decay(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_DECAY as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX reverb frequency.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_reverb_freq(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_FREQ as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX reverb gain.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_reverb_gain(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_GAIN as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX reverb high pass filter.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_reverb_hpf(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_HPF as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX reverb low pass filter.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_reverb_lpf(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_LPF as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX reverb volume.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_reverb_volume(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_VOL as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX reverb pre delay.
    pub fn plock_clear_fx_reverb_pre_delay(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_PRE as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX reverb decay.
    pub fn plock_clear_fx_reverb_decay(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_DECAY as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX reverb frequency.
    pub fn plock_clear_fx_reverb_freq(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_FREQ as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX reverb gain.
    pub fn plock_clear_fx_reverb_gain(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_GAIN as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX reverb high pass filter.
    pub fn plock_clear_fx_reverb_hpf(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_HPF as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX reverb low pass filter.
    pub fn plock_clear_fx_reverb_lpf(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_LPF as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX reverb volume.
    pub fn plock_clear_fx_reverb_volume(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_REVERB_VOL as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
