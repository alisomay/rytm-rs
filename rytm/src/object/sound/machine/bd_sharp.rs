use crate::error::ConversionError;
use crate::util::scale_f32_to_u16;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::{from_s_u16_t, get_u16_min_max_from_float_range, scale_u16_to_f32, to_s_u16_t_union_a},
    RytmError::OrphanTrig,
};
use derivative::Derivative;
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum BdSharpWaveform {
    #[default]
    SinA,
    SinB,
    AsinA,
    AsinB,
    TriA,
    TriB,
    SsawA,
    SsawB,
    SawA,
    SawB,
    SqrA,
    SqrB,
}

impl TryFrom<u8> for BdSharpWaveform {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SinA),
            1 => Ok(Self::SinB),
            2 => Ok(Self::AsinA),
            3 => Ok(Self::AsinB),
            4 => Ok(Self::TriA),
            5 => Ok(Self::TriB),
            6 => Ok(Self::SsawA),
            7 => Ok(Self::SsawB),
            8 => Ok(Self::SawA),
            9 => Ok(Self::SawB),
            10 => Ok(Self::SqrA),
            11 => Ok(Self::SqrB),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "BdSharpWaveform".into(),
            }),
        }
    }
}

impl From<BdSharpWaveform> for u8 {
    fn from(value: BdSharpWaveform) -> Self {
        match value {
            BdSharpWaveform::SinA => 0,
            BdSharpWaveform::SinB => 1,
            BdSharpWaveform::AsinA => 2,
            BdSharpWaveform::AsinB => 3,
            BdSharpWaveform::TriA => 4,
            BdSharpWaveform::TriB => 5,
            BdSharpWaveform::SsawA => 6,
            BdSharpWaveform::SsawB => 7,
            BdSharpWaveform::SawA => 8,
            BdSharpWaveform::SawB => 9,
            BdSharpWaveform::SqrA => 10,
            BdSharpWaveform::SqrB => 11,
        }
    }
}

#[machine_parameters(
    lev: "0..=127" #1,
    tun: "-32.0..=32.0" #2,
    dec: "0..=127" #3,
    hld: "0..=127" #4,
    swt: "0..=127" #5,
    swd: "0..=127" #6,
    // wav #7
    tic: "0..=127" #8,
)]
/// Parameters for the `BdSharp` machine.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct BdSharpParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    hld: u8,
    swt: u8,
    swd: u8,
    wav: BdSharpWaveform,
    tic: u8,

    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for BdSharpParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: -6.0,
            dec: 80,
            hld: 32,
            swt: 100,
            swd: 100,
            wav: BdSharpWaveform::default(),
            tic: 64,
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl BdSharpParameters {
    pub(crate) fn link_parameter_lock_pool(&mut self, pool: Arc<Mutex<ParameterLockPool>>) {
        self.parameter_lock_pool = Some(pool);
    }

    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);

        let wav: u8 = self.wav.into();
        let wav = to_s_u16_t_union_a((wav as u16) << 8);
        raw_sound.synth_param_7 = wav;
    }

    /// Sets the `wav` parameter.
    pub fn set_wav(&mut self, wav: BdSharpWaveform) {
        self.wav = wav;
    }

    /// Returns the `wav` parameter.
    pub const fn get_wav(&self) -> BdSharpWaveform {
        self.wav
    }

    /// Sets the parameter lock for the `wav` parameter.
    pub fn plock_set_wav(&self, wav: BdSharpWaveform, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.lock().unwrap().set_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
                wav.into(),
            )?;
            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the `wav` parameter.
    pub fn plock_get_wav(&self, trig_index: usize) -> Result<Option<BdSharpWaveform>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            let wav = pool.lock().unwrap().get_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
            );
            if let Some(wav) = wav {
                return Ok(Some(wav.try_into()?));
            }
            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the `wav` parameter if set.
    pub fn plock_clear_wav(&self, trig_index: usize) -> Result<(), RytmError> {
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
        let output_tun_min: f32 = -32.;
        let output_tun_max: f32 = 32.;
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
                dec: (from_s_u16_t(raw_sound.synth_param_3) >> 8) as u8,
                hld: (from_s_u16_t(raw_sound.synth_param_4) >> 8) as u8,
                swt: (from_s_u16_t(raw_sound.synth_param_5) >> 8) as u8,
                swd: (from_s_u16_t(raw_sound.synth_param_6) >> 8) as u8,
                wav: ((from_s_u16_t(raw_sound.synth_param_7) >> 8) as u8).try_into()?,
                tic: (from_s_u16_t(raw_sound.synth_param_8) >> 8) as u8,
            })
        }
    }
}
