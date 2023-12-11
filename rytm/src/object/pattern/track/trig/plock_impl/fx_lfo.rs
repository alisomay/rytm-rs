use super::fx_plock_types::*;
use crate::{
    error::{ParameterError, RytmError},
    object::{
        kit::types::FxLfoDestination,
        pattern::Trig,
        sound::types::{LfoMode, LfoMultiplier, LfoWaveform},
    },
    util::{
        i8_to_u8_midpoint_of_u8_input_range, scale_generic, u8_to_i8_midpoint_of_u8_input_range,
    },
    RytmError::OrphanTrig,
};
use rytm_rs_macro::parameter_range;

impl Trig {
    /// Sets a parameter lock for the FX LFO speed.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "speed:-64..=63")]
    pub fn p_lock_set_fx_lfo_speed(&self, speed: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_SPEED as u8,
                i8_to_u8_midpoint_of_u8_input_range(speed as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO multiplier.
    pub fn p_lock_set_fx_lfo_multiplier(&self, multiplier: LfoMultiplier) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_MULTIPLY as u8,
                multiplier.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO fade.
    ///
    /// Range `-64..=63`
    #[parameter_range(range = "fade:-64..=63")]
    pub fn p_lock_set_fx_lfo_fade(&self, fade: isize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_FADE as u8,
                i8_to_u8_midpoint_of_u8_input_range(fade as i8, 0, 127),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO destination.
    pub fn p_lock_set_fx_lfo_destination(
        &self,
        destination: FxLfoDestination,
    ) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_DEST as u8,
                destination.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO waveform.
    pub fn p_lock_set_fx_lfo_waveform(&self, waveform: LfoWaveform) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_WAVEFORM as u8,
                waveform.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO start phase.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "start_phase:0..=127")]
    pub fn p_lock_set_fx_lfo_start_phase(&self, start_phase: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_PHASE as u8,
                start_phase as u8,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO mode.
    pub fn p_lock_set_fx_lfo_mode(&self, mode: LfoMode) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut().set_fx_basic_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_MOD as u8,
                mode.into(),
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets a parameter lock for the FX LFO depth.
    ///
    /// Range `-128.0..=127.99`
    #[parameter_range(range = "depth:-128.0..=127.99")]
    pub fn p_lock_set_fx_lfo_depth(&self, depth: f32) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let depth = scale_generic(depth, -128f32, 127.99f32, 0u16, 32767u16, |x| {
                x.round() as u16
            });

            pool.borrow_mut().set_fx_compound_plock(
                self.index,
                AR_FX_PLOCK_TYPE_LFO_DEPTH as u8,
                depth,
            )?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX LFO speed.
    ///
    /// Range `-64..=63`
    pub fn p_lock_get_fx_lfo_speed(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_SPEED as u8);

            if let Some(value) = value {
                return Ok(Some(
                    u8_to_i8_midpoint_of_u8_input_range(value, 0, 127) as isize
                ));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX LFO multiplier.
    pub fn p_lock_get_fx_lfo_multiplier(&self) -> Result<Option<LfoMultiplier>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_MULTIPLY as u8);

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX LFO fade.
    ///
    /// Range `-64..=63`
    pub fn p_lock_get_fx_lfo_fade(&self) -> Result<Option<isize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_FADE as u8);

            if let Some(value) = value {
                return Ok(Some(
                    u8_to_i8_midpoint_of_u8_input_range(value, 0, 127) as isize
                ));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX LFO destination.
    ///
    /// Range `0..=127`
    pub fn p_lock_get_fx_lfo_destination(&self) -> Result<Option<FxLfoDestination>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_DEST as u8);

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX LFO waveform.
    pub fn p_lock_get_fx_lfo_waveform(&self) -> Result<Option<LfoWaveform>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_WAVEFORM as u8);

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX LFO start phase.
    ///
    /// Range `0..=127`
    pub fn p_lock_get_fx_lfo_start_phase(&self) -> Result<Option<usize>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_PHASE as u8);

            if let Some(value) = value {
                return Ok(Some(value as usize));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX LFO mode.
    pub fn p_lock_get_fx_lfo_mode(&self) -> Result<Option<LfoMode>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_MOD as u8);

            if let Some(value) = value {
                return Ok(Some(value.try_into()?));
            }

            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the FX LFO depth.
    ///
    /// Range `-128.0..=127.99`
    pub fn p_lock_get_fx_lfo_depth(&self) -> Result<Option<f32>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let value = pool
                .borrow_mut()
                .get_fx_compound_plock(self.index, AR_FX_PLOCK_TYPE_LFO_DEPTH as u8);

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

    /// Clears the parameter lock for the FX LFO speed.
    pub fn p_lock_clear_fx_lfo_speed(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_SPEED as u8)?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX LFO multiplier.
    pub fn p_lock_clear_fx_lfo_multiplier(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_MULTIPLY as u8)?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX LFO fade.
    pub fn p_lock_clear_fx_lfo_fade(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_FADE as u8)?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX LFO destination.
    pub fn p_lock_clear_fx_lfo_destination(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_DEST as u8)?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX LFO waveform.
    pub fn p_lock_clear_fx_lfo_waveform(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_WAVEFORM as u8)?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX LFO start phase.
    pub fn p_lock_clear_fx_lfo_start_phase(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_PHASE as u8)?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX LFO mode.
    pub fn p_lock_clear_fx_lfo_mode(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_basic_plock(self.index, AR_FX_PLOCK_TYPE_LFO_MOD as u8)?;

            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the FX LFO depth.
    pub fn p_lock_clear_fx_lfo_depth(&self) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            pool.borrow_mut()
                .clear_fx_compound_plock(self.index, AR_FX_PLOCK_TYPE_LFO_DEPTH as u8)?;

            return Ok(());
        }
        Err(OrphanTrig)
    }
}
