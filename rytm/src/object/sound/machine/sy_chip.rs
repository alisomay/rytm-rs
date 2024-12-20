use crate::{
    error::{ConversionError, ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
    util::{
        from_s_u16_t, get_u16_min_max_from_float_range, i8_to_u8_midpoint_of_u8_input_range,
        scale_f32_to_u16, scale_u16_to_f32, to_s_u16_t_union_a,
        u8_to_i8_midpoint_of_u8_input_range,
    },
    RytmError::OrphanTrig,
};
use derivative::Derivative;
use parking_lot::Mutex;
use rytm_rs_macro::{machine_parameters, parameter_range};
use rytm_sys::ar_sound_t;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[machine_parameters(
 lev: "0..=127" #1,
 tun: "-24.0..=24.0" #2,
 dec: "0..=127" #3,
 of2: "-24..=24" #4,
 of3: "-24..=24" #5,
 of4: "-24..=24" #6,
 // #7 wav: (manual impl) (0=sin,1=asin,2=tri,3=ssaw,4=saw,5=sqr,6=noise,7=anm1,8=anm2,9=anm3,10=anm4,11=anm5,12=pwm+,13=pwm-,14=triB,15=+tri,16=tri+,17=triX,18=sawB,19=+saw,20=saw+,21=sawX,22=sqrB,23=+sqr,24=sqr+,25=sqrX,26=tbl1,27=tbl2,28=tbl3,29=p1%..127=p99%)
 // #8 spd: (manual impl)  (0=128T,1=128,2=64T,3=128d,4=64,5=32T,6=64d,7=32,8=16T,9=32d,10=16,11=8T,12=16d,13=8,14=4T,15=8d,16=4,17=2T,18=4d,19=2,20=1T,21=2d,22=1,23=1d,24=1.0Hz,25=1.56Hz,26=1.88Hz,27=2Hz,28=3.13Hz,29=3.75Hz,30=4Hz,31=5Hz,32=6.25Hz,33=7.5Hz,34=10Hz,35=12.5Hz,36=15Hz,37=20Hz,38=25Hz,39=30Hz,40=40Hz,41=50Hz,42=60Hz,43=75Hz,44=100Hz,45=120Hz,46=150Hz,47=180Hz,48=200Hz,49=240Hz,50=250Hz,51=300Hz,52=350Hz,53=360Hz,54=400Hz,55=420Hz,56=480Hz,57=240 5Hz,58=200 5Hz,59=150 5Hz,60=120 5Hz,61=100 5Hz,62=60 5Hz,63=50 5Hz,64=30 5Hz,65=25 5Hz)
)]
/// Parameters for the `SyChip` machine.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct SyChipParameters {
    lev: u8,
    tun: f32,
    dec: u8,
    of2: i8,
    of3: i8,
    of4: i8,
    wav: SyChipWaveform,
    spd: SyChipSpeed,

    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
    assigned_track: Option<usize>,
}

impl Default for SyChipParameters {
    fn default() -> Self {
        Self {
            lev: 100,
            tun: 12.0,
            dec: 42,
            of2: 3,
            of3: 7,
            of4: 10,
            wav: SyChipWaveform::default(),
            spd: SyChipSpeed::default(),
            parameter_lock_pool: None,
            assigned_track: None,
        }
    }
}

impl SyChipParameters {
    pub(crate) fn link_parameter_lock_pool(&mut self, pool: Arc<Mutex<ParameterLockPool>>) {
        self.parameter_lock_pool = Some(pool);
    }

    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        self.apply_to_raw_sound_values(raw_sound);

        let wav: u8 = self.wav.into();
        raw_sound.synth_param_7 = to_s_u16_t_union_a((wav as u16) << 8);

        let spd: u8 = self.spd.into();
        raw_sound.synth_param_8 = to_s_u16_t_union_a((spd as u16) << 8);
    }

    /// Sets the `wav` parameter.
    pub fn set_wav(&mut self, wav: SyChipWaveform) {
        self.wav = wav;
    }

    /// Returns the `wav` parameter.
    pub const fn get_wav(&self) -> SyChipWaveform {
        self.wav
    }

    /// Sets the parameter lock for the `wav` parameter.
    pub fn plock_set_wav(&self, wav: SyChipWaveform, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.lock().set_basic_plock(
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
    pub fn plock_get_wav(&self, trig_index: usize) -> Result<Option<SyChipWaveform>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            let wav = pool.lock().get_basic_plock(
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
            pool.lock().clear_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP6 as u8,
            );
            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Sets the `spd` parameter.
    pub fn set_spd(&mut self, spd: SyChipSpeed) -> Result<(), RytmError> {
        self.spd = spd;
        Ok(())
    }

    /// Returns the `spd` parameter.
    pub const fn get_spd(&self) -> SyChipSpeed {
        self.spd
    }

    /// Sets the parameter lock for the `spd` parameter.
    pub fn plock_set_spd(&self, spd: SyChipSpeed, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.lock().set_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP7 as u8,
                spd.into(),
            )?;
            return Ok(());
        }
        Err(OrphanTrig)
    }

    /// Gets the parameter lock for the `spd` parameter.
    pub fn plock_get_spd(&self, trig_index: usize) -> Result<Option<SyChipSpeed>, RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            let spd = pool.lock().get_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP7 as u8,
            );
            if let Some(spd) = spd {
                return Ok(Some(spd.try_into()?));
            }
            return Ok(None);
        }
        Err(OrphanTrig)
    }

    /// Clears the parameter lock for the `spd` parameter if set.
    pub fn plock_clear_spd(&self, trig_index: usize) -> Result<(), RytmError> {
        if let Some(ref pool) = self.parameter_lock_pool {
            let assigned_track = self.assigned_track.ok_or(OrphanTrig)?;
            pool.lock().clear_basic_plock(
                trig_index,
                assigned_track as u8,
                rytm_sys::AR_PLOCK_TYPE_MP7 as u8,
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
                dec: (from_s_u16_t(raw_sound.synth_param_3) >> 8) as u8,
                of2: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(raw_sound.synth_param_4) >> 8) as u8,
                    0,
                    127,
                ),
                of3: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(raw_sound.synth_param_5) >> 8) as u8,
                    0,
                    127,
                ),
                of4: u8_to_i8_midpoint_of_u8_input_range(
                    (from_s_u16_t(raw_sound.synth_param_6) >> 8) as u8,
                    0,
                    127,
                ),
                wav: SyChipWaveform::try_from((from_s_u16_t(raw_sound.synth_param_7) >> 8) as u8)?,
                spd: SyChipSpeed::try_from((from_s_u16_t(raw_sound.synth_param_8) >> 8) as u8)?,
            })
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum SyChipWaveform {
    Sin,
    Asin,
    #[default]
    Tri,
    Ssaw,
    Saw,
    Sqr,
    Noise,
    Anm1,
    Anm2,
    Anm3,
    Anm4,
    Anm5,
    PwmPlus,
    PwmMinus,
    TriB,
    TriPlus,
    TriPlusPlus,
    TriX,
    SawB,
    SawPlus,
    SawPlusPlus,
    SawX,
    SqrB,
    SqrPlus,
    SqrPlusPlus,
    SqrX,
    Tbl1,
    Tbl2,
    Tbl3,
    /// Pulse width is a percentage from 1 to 99.
    Percentage(usize),
}
impl TryFrom<&str> for SyChipWaveform {
    type Error = ConversionError;
    fn try_from(wav: &str) -> Result<Self, Self::Error> {
        match wav {
            "sin" => Ok(Self::Sin),
            "asin" => Ok(Self::Asin),
            "tri" => Ok(Self::Tri),
            "ssaw" => Ok(Self::Ssaw),
            "saw" => Ok(Self::Saw),
            "sqr" => Ok(Self::Sqr),
            "noise" => Ok(Self::Noise),
            "anm1" => Ok(Self::Anm1),
            "anm2" => Ok(Self::Anm2),
            "anm3" => Ok(Self::Anm3),
            "anm4" => Ok(Self::Anm4),
            "anm5" => Ok(Self::Anm5),
            "pwm+" => Ok(Self::PwmPlus),
            "pwm-" => Ok(Self::PwmMinus),
            "trib" => Ok(Self::TriB),
            "tri+" => Ok(Self::TriPlus),
            "tri++" => Ok(Self::TriPlusPlus),
            "trix" => Ok(Self::TriX),
            "sawb" => Ok(Self::SawB),
            "saw+" => Ok(Self::SawPlus),
            "saw++" => Ok(Self::SawPlusPlus),
            "sawx" => Ok(Self::SawX),
            "sqrb" => Ok(Self::SqrB),
            "sqr+" => Ok(Self::SqrPlus),
            "sqr++" => Ok(Self::SqrPlusPlus),
            "sqrx" => Ok(Self::SqrX),
            "tbl1" => Ok(Self::Tbl1),
            "tbl2" => Ok(Self::Tbl2),
            "tbl3" => Ok(Self::Tbl3),
            wav if !wav.is_empty() => {
                let (value, _) = wav.split_once('%').ok_or_else(|| ConversionError::Range {
                    value: wav.to_string(),
                    type_name: "SyChipWaveform".into(),
                })?;
                let wav = value.parse::<usize>()?;
                if !(1..=99).contains(&wav) {
                    return Err(ConversionError::Range {
                        value: wav.to_string(),
                        type_name: "SyChipWaveform".into(),
                    });
                }
                Ok(Self::Percentage(wav))
            }
            _ => Err(ConversionError::Range {
                value: wav.to_string(),
                type_name: "SyChipWaveform".into(),
            }),
        }
    }
}

impl TryFrom<u8> for SyChipWaveform {
    type Error = ConversionError;
    fn try_from(wav: u8) -> Result<Self, Self::Error> {
        match wav {
            0 => Ok(Self::Sin),
            1 => Ok(Self::Asin),
            2 => Ok(Self::Tri),
            3 => Ok(Self::Ssaw),
            4 => Ok(Self::Saw),
            5 => Ok(Self::Sqr),
            6 => Ok(Self::Noise),
            7 => Ok(Self::Anm1),
            8 => Ok(Self::Anm2),
            9 => Ok(Self::Anm3),
            10 => Ok(Self::Anm4),
            11 => Ok(Self::Anm5),
            12 => Ok(Self::PwmPlus),
            13 => Ok(Self::PwmMinus),
            14 => Ok(Self::TriB),
            15 => Ok(Self::TriPlus),
            16 => Ok(Self::TriPlusPlus),
            17 => Ok(Self::TriX),
            18 => Ok(Self::SawB),
            19 => Ok(Self::SawPlus),
            20 => Ok(Self::SawPlusPlus),
            21 => Ok(Self::SawX),
            22 => Ok(Self::SqrB),
            23 => Ok(Self::SqrPlus),
            24 => Ok(Self::SqrPlusPlus),
            25 => Ok(Self::SqrX),
            26 => Ok(Self::Tbl1),
            27 => Ok(Self::Tbl2),
            28 => Ok(Self::Tbl3),
            29..=127 => Ok(Self::Percentage(wav as usize - 28)),
            _ => Err(ConversionError::Range {
                value: wav.to_string(),
                type_name: "SyChipWaveform".into(),
            }),
        }
    }
}

impl From<SyChipWaveform> for u8 {
    fn from(wav: SyChipWaveform) -> Self {
        match wav {
            SyChipWaveform::Sin => 0,
            SyChipWaveform::Asin => 1,
            SyChipWaveform::Tri => 2,
            SyChipWaveform::Ssaw => 3,
            SyChipWaveform::Saw => 4,
            SyChipWaveform::Sqr => 5,
            SyChipWaveform::Noise => 6,
            SyChipWaveform::Anm1 => 7,
            SyChipWaveform::Anm2 => 8,
            SyChipWaveform::Anm3 => 9,
            SyChipWaveform::Anm4 => 10,
            SyChipWaveform::Anm5 => 11,
            SyChipWaveform::PwmPlus => 12,
            SyChipWaveform::PwmMinus => 13,
            SyChipWaveform::TriB => 14,
            SyChipWaveform::TriPlus => 15,
            SyChipWaveform::TriPlusPlus => 16,
            SyChipWaveform::TriX => 17,
            SyChipWaveform::SawB => 18,
            SyChipWaveform::SawPlus => 19,
            SyChipWaveform::SawPlusPlus => 20,
            SyChipWaveform::SawX => 21,
            SyChipWaveform::SqrB => 22,
            SyChipWaveform::SqrPlus => 23,
            SyChipWaveform::SqrPlusPlus => 24,
            SyChipWaveform::SqrX => 25,
            SyChipWaveform::Tbl1 => 26,
            SyChipWaveform::Tbl2 => 27,
            SyChipWaveform::Tbl3 => 28,
            SyChipWaveform::Percentage(wav) => wav as Self + 28,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum SyChipSpeed {
    _128T,
    _128,
    _64T,
    _128D,
    _64,
    _32T,
    _64D,
    _32,
    _16T,
    _32D,
    _16,
    _8T,
    _16D,
    _8,
    _4T,
    _8D,
    _4,
    _2T,
    _4D,
    _2,
    _1T,
    _2D,
    _1,
    _1D,
    _1_0Hz,
    _1_56Hz,
    _1_88Hz,
    _2Hz,
    _3_13Hz,
    _3_75Hz,
    _4Hz,
    _5Hz,
    _6_25Hz,
    _7_5Hz,
    _10Hz,
    _12_5Hz,
    _15Hz,
    _20Hz,
    _25Hz,
    _30Hz,
    _40Hz,
    #[default]
    _50Hz,
    _60Hz,
    _75Hz,
    _100Hz,
    _120Hz,
    _150Hz,
    _180Hz,
    _200Hz,
    _240Hz,
    _250Hz,
    _300Hz,
    _350Hz,
    _360Hz,
    _400Hz,
    _420Hz,
    _480Hz,
    _240S,
    _200S,
    _150S,
    _120S,
    _100S,
    _60S,
    _50S,
    _30S,
    _25S,
}

impl TryFrom<&str> for SyChipSpeed {
    type Error = ConversionError;
    fn try_from(spd: &str) -> Result<Self, Self::Error> {
        match spd {
            "128t" => Ok(Self::_128T),
            "128" => Ok(Self::_128),
            "64t" => Ok(Self::_64T),
            "128d" => Ok(Self::_128D),
            "64" => Ok(Self::_64),
            "32t" => Ok(Self::_32T),
            "64d" => Ok(Self::_64D),
            "32" => Ok(Self::_32),
            "16t" => Ok(Self::_16T),
            "32d" => Ok(Self::_32D),
            "16" => Ok(Self::_16),
            "8t" => Ok(Self::_8T),
            "16d" => Ok(Self::_16D),
            "8" => Ok(Self::_8),
            "4t" => Ok(Self::_4T),
            "8d" => Ok(Self::_8D),
            "4" => Ok(Self::_4),
            "2t" => Ok(Self::_2T),
            "4d" => Ok(Self::_4D),
            "2" => Ok(Self::_2),
            "1t" => Ok(Self::_1T),
            "2d" => Ok(Self::_2D),
            "1" => Ok(Self::_1),
            "1d" => Ok(Self::_1D),
            "1.0hz" => Ok(Self::_1_0Hz),
            "1.56hz" => Ok(Self::_1_56Hz),
            "1.88hz" => Ok(Self::_1_88Hz),
            "2hz" => Ok(Self::_2Hz),
            "3.13hz" => Ok(Self::_3_13Hz),
            "3.75hz" => Ok(Self::_3_75Hz),
            "4hz" => Ok(Self::_4Hz),
            "5hz" => Ok(Self::_5Hz),
            "6.25hz" => Ok(Self::_6_25Hz),
            "7.5hz" => Ok(Self::_7_5Hz),
            "10hz" => Ok(Self::_10Hz),
            "12.5hz" => Ok(Self::_12_5Hz),
            "15hz" => Ok(Self::_15Hz),
            "20hz" => Ok(Self::_20Hz),
            "25hz" => Ok(Self::_25Hz),
            "30hz" => Ok(Self::_30Hz),
            "40hz" => Ok(Self::_40Hz),
            "50hz" => Ok(Self::_50Hz),
            "60hz" => Ok(Self::_60Hz),
            "75hz" => Ok(Self::_75Hz),
            "100hz" => Ok(Self::_100Hz),
            "120hz" => Ok(Self::_120Hz),
            "150hz" => Ok(Self::_150Hz),
            "180hz" => Ok(Self::_180Hz),
            "200hz" => Ok(Self::_200Hz),
            "240hz" => Ok(Self::_240Hz),
            "250hz" => Ok(Self::_250Hz),
            "300hz" => Ok(Self::_300Hz),
            "350hz" => Ok(Self::_350Hz),
            "360hz" => Ok(Self::_360Hz),
            "400hz" => Ok(Self::_400Hz),
            "420hz" => Ok(Self::_420Hz),
            "480hz" => Ok(Self::_480Hz),
            "240s" => Ok(Self::_240S),
            "200s" => Ok(Self::_200S),
            "150s" => Ok(Self::_150S),
            "120s" => Ok(Self::_120S),
            "100s" => Ok(Self::_100S),
            "60s" => Ok(Self::_60S),
            "50s" => Ok(Self::_50S),
            "30s" => Ok(Self::_30S),
            "25s" => Ok(Self::_25S),
            _ => Err(ConversionError::Range {
                value: spd.to_string(),
                type_name: "SyChipSpeed".into(),
            }),
        }
    }
}

impl TryFrom<u8> for SyChipSpeed {
    type Error = ConversionError;
    fn try_from(spd: u8) -> Result<Self, Self::Error> {
        match spd {
            0 => Ok(Self::_128T),
            1 => Ok(Self::_128),
            2 => Ok(Self::_64T),
            3 => Ok(Self::_128D),
            4 => Ok(Self::_64),
            5 => Ok(Self::_32T),
            6 => Ok(Self::_64D),
            7 => Ok(Self::_32),
            8 => Ok(Self::_16T),
            9 => Ok(Self::_32D),
            10 => Ok(Self::_16),
            11 => Ok(Self::_8T),
            12 => Ok(Self::_16D),
            13 => Ok(Self::_8),
            14 => Ok(Self::_4T),
            15 => Ok(Self::_8D),
            16 => Ok(Self::_4),
            17 => Ok(Self::_2T),
            18 => Ok(Self::_4D),
            19 => Ok(Self::_2),
            20 => Ok(Self::_1T),
            21 => Ok(Self::_2D),
            22 => Ok(Self::_1),
            23 => Ok(Self::_1D),
            24 => Ok(Self::_1_0Hz),
            25 => Ok(Self::_1_56Hz),
            26 => Ok(Self::_1_88Hz),
            27 => Ok(Self::_2Hz),
            28 => Ok(Self::_3_13Hz),
            29 => Ok(Self::_3_75Hz),
            30 => Ok(Self::_4Hz),
            31 => Ok(Self::_5Hz),
            32 => Ok(Self::_6_25Hz),
            33 => Ok(Self::_7_5Hz),
            34 => Ok(Self::_10Hz),
            35 => Ok(Self::_12_5Hz),
            36 => Ok(Self::_15Hz),
            37 => Ok(Self::_20Hz),
            38 => Ok(Self::_25Hz),
            39 => Ok(Self::_30Hz),
            40 => Ok(Self::_40Hz),
            41 => Ok(Self::_50Hz),
            42 => Ok(Self::_60Hz),
            43 => Ok(Self::_75Hz),
            44 => Ok(Self::_100Hz),
            45 => Ok(Self::_120Hz),
            46 => Ok(Self::_150Hz),
            47 => Ok(Self::_180Hz),
            48 => Ok(Self::_200Hz),
            49 => Ok(Self::_240Hz),
            50 => Ok(Self::_250Hz),
            51 => Ok(Self::_300Hz),
            52 => Ok(Self::_350Hz),
            53 => Ok(Self::_360Hz),
            54 => Ok(Self::_400Hz),
            55 => Ok(Self::_420Hz),
            56 => Ok(Self::_480Hz),
            57 => Ok(Self::_240S),
            58 => Ok(Self::_200S),
            59 => Ok(Self::_150S),
            60 => Ok(Self::_120S),
            61 => Ok(Self::_100S),
            62 => Ok(Self::_60S),
            63 => Ok(Self::_50S),
            64 => Ok(Self::_30S),
            65 => Ok(Self::_25S),
            _ => Err(ConversionError::Range {
                value: spd.to_string(),
                type_name: "SyChipSpeed".into(),
            }),
        }
    }
}

impl From<SyChipSpeed> for u8 {
    fn from(spd: SyChipSpeed) -> Self {
        match spd {
            SyChipSpeed::_128T => 0,
            SyChipSpeed::_128 => 1,
            SyChipSpeed::_64T => 2,
            SyChipSpeed::_128D => 3,
            SyChipSpeed::_64 => 4,
            SyChipSpeed::_32T => 5,
            SyChipSpeed::_64D => 6,
            SyChipSpeed::_32 => 7,
            SyChipSpeed::_16T => 8,
            SyChipSpeed::_32D => 9,
            SyChipSpeed::_16 => 10,
            SyChipSpeed::_8T => 11,
            SyChipSpeed::_16D => 12,
            SyChipSpeed::_8 => 13,
            SyChipSpeed::_4T => 14,
            SyChipSpeed::_8D => 15,
            SyChipSpeed::_4 => 16,
            SyChipSpeed::_2T => 17,
            SyChipSpeed::_4D => 18,
            SyChipSpeed::_2 => 19,
            SyChipSpeed::_1T => 20,
            SyChipSpeed::_2D => 21,
            SyChipSpeed::_1 => 22,
            SyChipSpeed::_1D => 23,
            SyChipSpeed::_1_0Hz => 24,
            SyChipSpeed::_1_56Hz => 25,
            SyChipSpeed::_1_88Hz => 26,
            SyChipSpeed::_2Hz => 27,
            SyChipSpeed::_3_13Hz => 28,
            SyChipSpeed::_3_75Hz => 29,
            SyChipSpeed::_4Hz => 30,
            SyChipSpeed::_5Hz => 31,
            SyChipSpeed::_6_25Hz => 32,
            SyChipSpeed::_7_5Hz => 33,
            SyChipSpeed::_10Hz => 34,
            SyChipSpeed::_12_5Hz => 35,
            SyChipSpeed::_15Hz => 36,
            SyChipSpeed::_20Hz => 37,
            SyChipSpeed::_25Hz => 38,
            SyChipSpeed::_30Hz => 39,
            SyChipSpeed::_40Hz => 40,
            SyChipSpeed::_50Hz => 41,
            SyChipSpeed::_60Hz => 42,
            SyChipSpeed::_75Hz => 43,
            SyChipSpeed::_100Hz => 44,
            SyChipSpeed::_120Hz => 45,
            SyChipSpeed::_150Hz => 46,
            SyChipSpeed::_180Hz => 47,
            SyChipSpeed::_200Hz => 48,
            SyChipSpeed::_240Hz => 49,
            SyChipSpeed::_250Hz => 50,
            SyChipSpeed::_300Hz => 51,
            SyChipSpeed::_350Hz => 52,
            SyChipSpeed::_360Hz => 53,
            SyChipSpeed::_400Hz => 54,
            SyChipSpeed::_420Hz => 55,
            SyChipSpeed::_480Hz => 56,
            SyChipSpeed::_240S => 57,
            SyChipSpeed::_200S => 58,
            SyChipSpeed::_150S => 59,
            SyChipSpeed::_120S => 60,
            SyChipSpeed::_100S => 61,
            SyChipSpeed::_60S => 62,
            SyChipSpeed::_50S => 63,
            SyChipSpeed::_30S => 64,
            SyChipSpeed::_25S => 65,
        }
    }
}
