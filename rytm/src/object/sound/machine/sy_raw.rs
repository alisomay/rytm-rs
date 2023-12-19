use crate::error::ConversionError;
use crate::util::scale_f32_to_u16;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::{
        from_s_u16_t, get_u16_min_max_from_float_range, i8_to_u8_midpoint_of_u8_input_range,
        scale_u16_to_f32, to_s_u16_t_union_a, u8_to_i8_midpoint_of_u8_input_range,
    },
    RytmError::OrphanTrig,
};
use derivative::Derivative;
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum SyRawWaveform1 {
    Sin,
    Asin,
    Tri,
    Ssaw,
    #[default]
    Asaw,
    Saw,
    Ring,
}

impl TryFrom<&str> for SyRawWaveform1 {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "sin" => Ok(Self::Sin),
            "asin" => Ok(Self::Asin),
            "tri" => Ok(Self::Tri),
            "ssaw" => Ok(Self::Ssaw),
            "asaw" => Ok(Self::Asaw),
            "saw" => Ok(Self::Saw),
            "ring" => Ok(Self::Ring),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SyRawWaveform1".into(),
            }),
        }
    }
}

impl TryFrom<u8> for SyRawWaveform1 {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sin),
            1 => Ok(Self::Asin),
            2 => Ok(Self::Tri),
            3 => Ok(Self::Ssaw),
            4 => Ok(Self::Asaw),
            5 => Ok(Self::Saw),
            6 => Ok(Self::Ring),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SyRawWaveform1".into(),
            }),
        }
    }
}

impl From<SyRawWaveform1> for u8 {
    fn from(value: SyRawWaveform1) -> Self {
        match value {
            SyRawWaveform1::Sin => 0,
            SyRawWaveform1::Asin => 1,
            SyRawWaveform1::Tri => 2,
            SyRawWaveform1::Ssaw => 3,
            SyRawWaveform1::Asaw => 4,
            SyRawWaveform1::Saw => 5,
            SyRawWaveform1::Ring => 6,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum SyRawWaveform2 {
    #[default]
    SineA,
    SsawA,
    SineB,
    SsawB,
}

impl TryFrom<&str> for SyRawWaveform2 {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "sineA" => Ok(Self::SineA),
            "ssawA" => Ok(Self::SsawA),
            "sineB" => Ok(Self::SineB),
            "ssawB" => Ok(Self::SsawB),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SyRawWaveform2".into(),
            }),
        }
    }
}

impl TryFrom<u8> for SyRawWaveform2 {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SineA),
            1 => Ok(Self::SsawA),
            2 => Ok(Self::SineB),
            3 => Ok(Self::SsawB),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SyRawWaveform2".into(),
            }),
        }
    }
}

impl From<SyRawWaveform2> for u8 {
    fn from(value: SyRawWaveform2) -> Self {
        match value {
            SyRawWaveform2::SineA => 0,
            SyRawWaveform2::SsawA => 1,
            SyRawWaveform2::SineB => 2,
            SyRawWaveform2::SsawB => 3,
        }
    }
}

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-24.0..=24.0" #2,
 dec2: "0..=127" #3,
 det: "-24.0..=24.0" #4,
 nlev: "0..=127" #5,
 // wav1 manual impl (0=sin,1=asin,2=tri,3=ssaw,4=asaw,5=saw,6=ring)
 // wav2 manual impl (0=sineA,1=ssawA,2=sineB,3=ssawB)
 bal: "-64..=63" #8,
)]
/// Parameters for the `SyRaw` machine.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct SyRawParameters {
    lev: u8,
    tun: f32,
    dec2: u8,
    det: f32,
    nlev: u8,
    wav1: SyRawWaveform1,
    wav2: SyRawWaveform2,
    bal: i8,

    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for SyRawParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: 0.0,
            dec2: 127,
            det: -12.0,
            nlev: 15,
            wav1: SyRawWaveform1::default(),
            wav2: SyRawWaveform2::default(),
            bal: 0,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl SyRawParameters {
    pub(crate) fn link_parameter_lock_pool(&mut self, pool: Arc<Mutex<ParameterLockPool>>) {
        self.parameter_lock_pool = Some(pool);
    }

    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);
        let wav1: u8 = self.wav1.into();
        raw_sound.synth_param_6 = to_s_u16_t_union_a((wav1 as u16) << 8);

        let wav2: u8 = self.wav2.into();
        raw_sound.synth_param_7 = to_s_u16_t_union_a((wav2 as u16) << 8);
    }

    // Sets the `wav1` parameter.
    pub fn set_wav1(&mut self, wav1: SyRawWaveform1) {
        self.wav1 = wav1;
    }

    // Sets the `wav2` parameter.
    pub fn set_wav2(&mut self, wav2: SyRawWaveform2) {
        self.wav2 = wav2;
    }

    // Returns the `wav1` parameter.
    pub const fn get_wav1(&self) -> SyRawWaveform1 {
        self.wav1
    }

    // Returns the `wav2` parameter.
    pub const fn get_wav2(&self) -> SyRawWaveform2 {
        self.wav2
    }

    /// Sets the parameter lock for the `wav1` parameter.
    pub fn plock_set_wav1(&self, wav1: SyRawWaveform1, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.lock().unwrap().set_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
                wav1.into(),
            )?;
            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the `wav1` parameter.
    pub fn plock_get_wav1(&self, trig_index: usize) -> Result<Option<SyRawWaveform1>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            let wav1 = pool.lock().unwrap().get_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
            );
            if let Some(wav1) = wav1 {
                return Ok(Some(wav1.try_into()?));
            }
            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the `wav1` parameter if set.
    pub fn plock_clear_wav1(&self, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.lock().unwrap().clear_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
            );
            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets the parameter lock for the `wav2` parameter.
    pub fn plock_set_wav2(&self, wav2: SyRawWaveform2, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.lock().unwrap().set_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
                wav2.into(),
            )?;
            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the `wav2` parameter.
    pub fn plock_get_wav2(&self, trig_index: usize) -> Result<Option<SyRawWaveform2>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            let wav2 = pool.lock().unwrap().get_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
            );
            if let Some(wav2) = wav2 {
                return Ok(Some(wav2.try_into()?));
            }
            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the `wav2` parameter if set.
    pub fn plock_clear_wav2(&self, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.lock().unwrap().clear_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
            );
            return Ok(());
        }
        Err(OrphanTrig)
    }

    #[parameter_range(range = "track_index[opt]:0..=11")]
    pub(crate) fn from_raw_sound(
        raw_sound: &ar_sound_t,
        track_index: Option<usize>,
    ) -> Result<Self, RytmError> {
        let output_tun_min: f32 = -24.;
        let output_tun_max: f32 = 24.;
        let (input_tun_min, input_tun_max) =
            get_u16_min_max_from_float_range(output_tun_min, output_tun_max);
        unsafe {
            Ok(Self {
                parameter_lock_pool: None,
                assigned_track: track_index,
                lev: (from_s_u16_t(raw_sound.synth_param_1) >> 8) as u8,
                tun: scale_u16_to_f32(
                    from_s_u16_t(raw_sound.synth_param_2),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                ),
                dec2: (from_s_u16_t(raw_sound.synth_param_3) >> 8) as u8,
                det: scale_u16_to_f32(
                    from_s_u16_t(raw_sound.synth_param_4),
                    input_tun_min,
                    input_tun_max,
                    output_tun_min,
                    output_tun_max,
                ),
                nlev: (from_s_u16_t(raw_sound.synth_param_5) >> 8) as u8,
                wav1: ((from_s_u16_t(raw_sound.synth_param_6) >> 8) as u8).try_into()?,
                wav2: ((from_s_u16_t(raw_sound.synth_param_7) >> 8) as u8).try_into()?,
                bal: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(raw_sound.synth_param_8) >> 8) as u8,
                    0,
                    127,
                ),
            })
        }
    }
}
