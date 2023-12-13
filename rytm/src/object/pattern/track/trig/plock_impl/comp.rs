use super::fx_plock_types::*;
use crate::{
    error::{ParameterError, RytmError},
    object::{
        kit::types::{FxCompAttack, FxCompRatio, FxCompRelease, FxCompSideChainEq},
        pattern::Trig,
    },
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the FX compressor threshold.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "threshold:0..=127")]
    pub fn plock_set_fx_compressor_threshold(&self, threshold: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_THRESHOLD as u8,
                threshold as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor attack.
    pub fn plock_set_fx_compressor_attack(&self, attack: FxCompAttack) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_ATTACK as u8,
                attack.into(),
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor release.
    pub fn plock_set_fx_compressor_release(&self, release: FxCompRelease) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_RELEASE as u8,
                release.into(),
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor ratio.
    pub fn plock_set_fx_compressor_ratio(&self, ratio: FxCompRatio) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_RATIO as u8,
                ratio.into(),
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor side chain eq.
    pub fn plock_set_fx_compressor_side_chain_eq(
        &self,
        seq: FxCompSideChainEq,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_SEQ as u8,
                seq.into(),
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor gain.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "gain:0..=127")]
    pub fn plock_set_fx_compressor_gain(&self, gain: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_MAKEUP as u8,
                gain as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor mix.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "mix:0..=127")]
    pub fn plock_set_fx_compressor_mix(&self, mix: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_MIX as u8,
                mix as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX compressor volume.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn plock_set_fx_compressor_volume(&self, volume: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_COMP_VOL as u8,
                volume as u8,
            )?;

            self.enable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the FX compressor threshold parameter lock.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_compressor_threshold(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_THRESHOLD as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the FX compressor attack parameter lock.
    pub fn plock_get_fx_compressor_attack(&self) -> Result<Option<FxCompAttack>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_ATTACK as u8);

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the FX compressor release parameter lock.
    pub fn plock_get_fx_compressor_release(&self) -> Result<Option<FxCompRelease>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_RELEASE as u8);

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the FX compressor ratio parameter lock.
    pub fn plock_get_fx_compressor_ratio(&self) -> Result<Option<FxCompRatio>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_RATIO as u8);

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the FX compressor side chain eq parameter lock.
    pub fn plock_get_fx_compressor_side_chain_eq(
        &self,
    ) -> Result<Option<FxCompSideChainEq>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_SEQ as u8);

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the FX compressor gain parameter lock.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_compressor_gain(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_MAKEUP as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the FX compressor mix parameter lock.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_compressor_mix(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_MIX as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the FX compressor volume parameter lock.
    ///
    /// Range `0..=127`
    pub fn plock_get_fx_compressor_volume(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_VOL as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the FX compressor threshold parameter lock.
    pub fn plock_clear_fx_compressor_threshold(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_THRESHOLD as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the FX compressor attack parameter lock.
    pub fn plock_clear_fx_compressor_attack(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_ATTACK as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the FX compressor release parameter lock.
    pub fn plock_clear_fx_compressor_release(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_RELEASE as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the FX compressor ratio parameter lock.
    pub fn plock_clear_fx_compressor_ratio(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_RATIO as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the FX compressor side chain eq parameter lock.
    pub fn plock_clear_fx_compressor_side_chain_eq(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_SEQ as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the FX compressor gain parameter lock.
    pub fn plock_clear_fx_compressor_gain(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_MAKEUP as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the FX compressor mix parameter lock.
    pub fn plock_clear_fx_compressor_mix(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_MIX as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the FX compressor volume parameter lock.
    pub fn plock_clear_fx_compressor_volume(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_COMP_VOL as u8)?;
            self.disable_fx_trig_if_necessary();

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
