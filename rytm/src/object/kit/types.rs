use crate::{error::ConversionError, object::sound::types::sound_mod_target};
use serde::{Deserialize, Serialize};

/// Targets for modulation comes from control in.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ControlInModTarget {
    /// No target.
    #[default]
    Unset,
    /// LFO multiplier.
    LfoMultiplier,
    /// LFO waveform.
    LfoWaveform,
    /// LFO trigger mode.
    LfoTrigMode,
    /// LFO speed.
    LfoSpeed,
    /// LFO fade.
    LfoFade,
    /// LFO phase.
    LfoPhase,
    /// LFO depth.
    LfoDepth,
    /// Sample tune.
    SampleTune,
    /// Sample fine tune.
    SampleFineTune,
    /// Sample slice.
    SampleSlice,
    /// Sample bit reduction.
    SampleBitReduction,
    /// Sample start.
    SampleStart,
    /// Sample end.
    SampleEnd,
    /// Sample loop.
    SampleLoop,
    /// Sample level.
    SampleLevel,
    /// Filter envelope.
    FilterEnvelope,
    /// Filter attack.
    FilterAttack,
    /// Filter decay.
    FilterDecay,
    /// Filter sustain.
    FilterSustain,
    /// Filter release.
    FilterRelease,
    /// Filter frequency.
    FilterFrequency,
    /// Filter resonance.
    FilterResonance,
    /// Amp attack.
    AmpAttack,
    /// Amp hold.
    AmpHold,
    /// Amp decay.
    AmpDecay,
    /// Amp overdrive.
    AmpOverdrive,
    /// Amp volume.
    AmpVolume,
    /// Amp pan.
    AmpPan,
    /// Amp accent.
    AmpAccent,
    /// Amp delay send.
    AmpDelaySend,
    /// Amp reverb send.
    AmpReverbSend,
}

impl TryFrom<&str> for ControlInModTarget {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "unset" => Ok(Self::Unset),
            "lfomultiplier" => Ok(Self::LfoMultiplier),
            "lfowaveform" => Ok(Self::LfoWaveform),
            "lfotrigmode" => Ok(Self::LfoTrigMode),
            "lfospeed" => Ok(Self::LfoSpeed),
            "lfofade" => Ok(Self::LfoFade),
            "lfophase" => Ok(Self::LfoPhase),
            "lfodepth" => Ok(Self::LfoDepth),
            "sampletune" => Ok(Self::SampleTune),
            "samplefinetune" => Ok(Self::SampleFineTune),
            "sampleslice" => Ok(Self::SampleSlice),
            "samplebitreduction" => Ok(Self::SampleBitReduction),
            "samplestart" => Ok(Self::SampleStart),
            "sampleend" => Ok(Self::SampleEnd),
            "sampleloop" => Ok(Self::SampleLoop),
            "samplelevel" => Ok(Self::SampleLevel),
            "filterenvelope" => Ok(Self::FilterEnvelope),
            "filterattack" => Ok(Self::FilterAttack),
            "filterdecay" => Ok(Self::FilterDecay),
            "filtersustain" => Ok(Self::FilterSustain),
            "filterrelease" => Ok(Self::FilterRelease),
            "filterfrequency" => Ok(Self::FilterFrequency),
            "filterresonance" => Ok(Self::FilterResonance),
            "ampattack" => Ok(Self::AmpAttack),
            "amphold" => Ok(Self::AmpHold),
            "ampdecay" => Ok(Self::AmpDecay),
            "ampoverdrive" => Ok(Self::AmpOverdrive),
            "ampvolume" => Ok(Self::AmpVolume),
            "amppan" => Ok(Self::AmpPan),
            "ampaccent" => Ok(Self::AmpAccent),
            "ampdelaysend" => Ok(Self::AmpDelaySend),
            "ampreverbsend" => Ok(Self::AmpReverbSend),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "ControlInModTarget".to_string(),
            }),
        }
    }
}

impl From<ControlInModTarget> for &str {
    fn from(value: ControlInModTarget) -> Self {
        match value {
            ControlInModTarget::Unset => "unset",
            ControlInModTarget::LfoMultiplier => "lfomultiplier",
            ControlInModTarget::LfoWaveform => "lfowaveform",
            ControlInModTarget::LfoTrigMode => "lfotrigmode",
            ControlInModTarget::LfoSpeed => "lfospeed",
            ControlInModTarget::LfoFade => "lfofade",
            ControlInModTarget::LfoPhase => "lfophase",
            ControlInModTarget::LfoDepth => "lfodepth",
            ControlInModTarget::SampleTune => "sampletune",
            ControlInModTarget::SampleFineTune => "samplefinetune",
            ControlInModTarget::SampleSlice => "sampleslice",
            ControlInModTarget::SampleBitReduction => "samplebitreduction",
            ControlInModTarget::SampleStart => "samplestart",
            ControlInModTarget::SampleEnd => "sampleend",
            ControlInModTarget::SampleLoop => "sampleloop",
            ControlInModTarget::SampleLevel => "samplelevel",
            ControlInModTarget::FilterEnvelope => "filterenvelope",
            ControlInModTarget::FilterAttack => "filterattack",
            ControlInModTarget::FilterDecay => "filterdecay",
            ControlInModTarget::FilterSustain => "filtersustain",
            ControlInModTarget::FilterRelease => "filterrelease",
            ControlInModTarget::FilterFrequency => "filterfrequency",
            ControlInModTarget::FilterResonance => "filterresonance",
            ControlInModTarget::AmpAttack => "ampattack",
            ControlInModTarget::AmpHold => "amphold",
            ControlInModTarget::AmpDecay => "ampdecay",
            ControlInModTarget::AmpOverdrive => "ampoverdrive",
            ControlInModTarget::AmpVolume => "ampvolume",
            ControlInModTarget::AmpPan => "amppan",
            ControlInModTarget::AmpAccent => "ampaccent",
            ControlInModTarget::AmpDelaySend => "ampdelaysend",
            ControlInModTarget::AmpReverbSend => "ampreverbsend",
        }
    }
}

impl TryFrom<u8> for ControlInModTarget {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use sound_mod_target::*;
        match value {
            NONE => Ok(Self::Unset),
            LFO_MULTIPLIER => Ok(Self::LfoMultiplier),
            LFO_WAVEFORM => Ok(Self::LfoWaveform),
            LFO_TRIGMODE => Ok(Self::LfoTrigMode),
            LFO_SPEED => Ok(Self::LfoSpeed),
            LFO_FADE => Ok(Self::LfoFade),
            LFO_PHASE => Ok(Self::LfoPhase),
            LFO_DEPTH => Ok(Self::LfoDepth),
            SMP_TUN => Ok(Self::SampleTune),
            SMP_FIN => Ok(Self::SampleFineTune),
            SMP_SMP => Ok(Self::SampleSlice),
            SMP_BR => Ok(Self::SampleBitReduction),
            SMP_STA => Ok(Self::SampleStart),
            SMP_END => Ok(Self::SampleEnd),
            SMP_LOP => Ok(Self::SampleLoop),
            SMP_LEV => Ok(Self::SampleLevel),
            FLT_ENV => Ok(Self::FilterEnvelope),
            FLT_ATK => Ok(Self::FilterAttack),
            FLT_DEC => Ok(Self::FilterDecay),
            FLT_SUS => Ok(Self::FilterSustain),
            FLT_REL => Ok(Self::FilterRelease),
            FLT_FRQ => Ok(Self::FilterFrequency),
            FLT_RES => Ok(Self::FilterResonance),
            AMP_ATK => Ok(Self::AmpAttack),
            AMP_HLD => Ok(Self::AmpHold),
            AMP_DEC => Ok(Self::AmpDecay),
            AMP_OVR => Ok(Self::AmpOverdrive),
            AMP_VOL => Ok(Self::AmpVolume),
            AMP_PAN => Ok(Self::AmpPan),
            AMP_ACC => Ok(Self::AmpAccent),
            AMP_DLY => Ok(Self::AmpDelaySend),
            AMP_REV => Ok(Self::AmpReverbSend),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "ControlInModTarget".to_string(),
            }),
        }
    }
}

impl From<ControlInModTarget> for u8 {
    fn from(value: ControlInModTarget) -> Self {
        use sound_mod_target::*;
        match value {
            ControlInModTarget::Unset => NONE,
            ControlInModTarget::LfoMultiplier => LFO_MULTIPLIER,
            ControlInModTarget::LfoWaveform => LFO_WAVEFORM,
            ControlInModTarget::LfoTrigMode => LFO_TRIGMODE,
            ControlInModTarget::LfoSpeed => LFO_SPEED,
            ControlInModTarget::LfoFade => LFO_FADE,
            ControlInModTarget::LfoPhase => LFO_PHASE,
            ControlInModTarget::LfoDepth => LFO_DEPTH,
            ControlInModTarget::SampleTune => SMP_TUN,
            ControlInModTarget::SampleFineTune => SMP_FIN,
            ControlInModTarget::SampleSlice => SMP_SMP,
            ControlInModTarget::SampleBitReduction => SMP_BR,
            ControlInModTarget::SampleStart => SMP_STA,
            ControlInModTarget::SampleEnd => SMP_END,
            ControlInModTarget::SampleLoop => SMP_LOP,
            ControlInModTarget::SampleLevel => SMP_LEV,
            ControlInModTarget::FilterEnvelope => FLT_ENV,
            ControlInModTarget::FilterAttack => FLT_ATK,
            ControlInModTarget::FilterDecay => FLT_DEC,
            ControlInModTarget::FilterSustain => FLT_SUS,
            ControlInModTarget::FilterRelease => FLT_REL,
            ControlInModTarget::FilterFrequency => FLT_FRQ,
            ControlInModTarget::FilterResonance => FLT_RES,
            ControlInModTarget::AmpAttack => AMP_ATK,
            ControlInModTarget::AmpHold => AMP_HLD,
            ControlInModTarget::AmpDecay => AMP_DEC,
            ControlInModTarget::AmpOverdrive => AMP_OVR,
            ControlInModTarget::AmpVolume => AMP_VOL,
            ControlInModTarget::AmpPan => AMP_PAN,
            ControlInModTarget::AmpAccent => AMP_ACC,
            ControlInModTarget::AmpDelaySend => AMP_DLY,
            ControlInModTarget::AmpReverbSend => AMP_REV,
        }
    }
}

/// Destination of an LFO.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum FxLfoDestination {
    #[default]
    Unset,
    DelayTime,
    DelayPingPong,
    DelayStereoWidth,
    DelayFeedback,
    DelayHpFilter,
    DelayLpFilter,
    DelayReverbSend,
    DelayMixVolume,
    DelayOverdrive,
    ReverbPreDelay,
    ReverbDecay,
    ReverbShelvingFreq,
    ReverbShelvingGain,
    ReverbHpFilter,
    ReverbLpFilter,
    ReverbMixVolume,
    DistortionAmount,
    DistortionSymmetry,
    CompressorThreshold,
    CompressorAttack,
    CompressorRelease,
    CompressorRatio,
    CompressorSideChainEq,
    CompressorMakeUpGain,
    CompressorDryWetMix,
    CompressorVolume,
}

impl TryFrom<&str> for FxLfoDestination {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "unset" => Ok(Self::Unset),
            "delaytime" => Ok(Self::DelayTime),
            "delaypingpong" => Ok(Self::DelayPingPong),
            "delaystereowidth" => Ok(Self::DelayStereoWidth),
            "delayfeedback" => Ok(Self::DelayFeedback),
            "delayhpfilter" => Ok(Self::DelayHpFilter),
            "delaylpfilter" => Ok(Self::DelayLpFilter),
            "delayreverbsend" => Ok(Self::DelayReverbSend),
            "delaymixvolume" => Ok(Self::DelayMixVolume),
            "delayoverdrive" => Ok(Self::DelayOverdrive),
            "reverbpredelay" => Ok(Self::ReverbPreDelay),
            "reverbdecay" => Ok(Self::ReverbDecay),
            "reverbshelvingfreq" => Ok(Self::ReverbShelvingFreq),
            "reverbshelvinggain" => Ok(Self::ReverbShelvingGain),
            "reverbhpfilter" => Ok(Self::ReverbHpFilter),
            "reverblpfilter" => Ok(Self::ReverbLpFilter),
            "reverbmixvolume" => Ok(Self::ReverbMixVolume),
            "distortionamount" => Ok(Self::DistortionAmount),
            "distortionsymmetry" => Ok(Self::DistortionSymmetry),
            "compressorthreshold" => Ok(Self::CompressorThreshold),
            "compressorattack" => Ok(Self::CompressorAttack),
            "compressorrelease" => Ok(Self::CompressorRelease),
            "compressorratio" => Ok(Self::CompressorRatio),
            "compressorsidechaineq" => Ok(Self::CompressorSideChainEq),
            "compressormakeupgain" => Ok(Self::CompressorMakeUpGain),
            "compressordrywetmix" => Ok(Self::CompressorDryWetMix),
            "compressorvolume" => Ok(Self::CompressorVolume),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxLfoDestination".to_string(),
            }),
        }
    }
}

impl From<FxLfoDestination> for &str {
    fn from(value: FxLfoDestination) -> Self {
        match value {
            FxLfoDestination::Unset => "unset",
            FxLfoDestination::DelayTime => "delaytime",
            FxLfoDestination::DelayPingPong => "delaypingpong",
            FxLfoDestination::DelayStereoWidth => "delaystereowidth",
            FxLfoDestination::DelayFeedback => "delayfeedback",
            FxLfoDestination::DelayHpFilter => "delayhpfilter",
            FxLfoDestination::DelayLpFilter => "delaylpfilter",
            FxLfoDestination::DelayReverbSend => "delayreverbsend",
            FxLfoDestination::DelayMixVolume => "delaymixvolume",
            FxLfoDestination::DelayOverdrive => "delayoverdrive",
            FxLfoDestination::ReverbPreDelay => "reverbpredelay",
            FxLfoDestination::ReverbDecay => "reverbdecay",
            FxLfoDestination::ReverbShelvingFreq => "reverbshelvingfreq",
            FxLfoDestination::ReverbShelvingGain => "reverbshelvinggain",
            FxLfoDestination::ReverbHpFilter => "reverbhpfilter",
            FxLfoDestination::ReverbLpFilter => "reverblpfilter",
            FxLfoDestination::ReverbMixVolume => "reverbmixvolume",
            FxLfoDestination::DistortionAmount => "distortionamount",
            FxLfoDestination::DistortionSymmetry => "distortionsymmetry",
            FxLfoDestination::CompressorThreshold => "compressorthreshold",
            FxLfoDestination::CompressorAttack => "compressorattack",
            FxLfoDestination::CompressorRelease => "compressorrelease",
            FxLfoDestination::CompressorRatio => "compressorratio",
            FxLfoDestination::CompressorSideChainEq => "compressorsidechaineq",
            FxLfoDestination::CompressorMakeUpGain => "compressormakeupgain",
            FxLfoDestination::CompressorDryWetMix => "compressordrywetmix",
            FxLfoDestination::CompressorVolume => "compressorvolume",
        }
    }
}

impl TryFrom<u8> for FxLfoDestination {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // with numbers
        match value {
            37 => Ok(Self::Unset),
            0 => Ok(Self::DelayTime),
            1 => Ok(Self::DelayPingPong),
            2 => Ok(Self::DelayStereoWidth),
            3 => Ok(Self::DelayFeedback),
            4 => Ok(Self::DelayHpFilter),
            5 => Ok(Self::DelayLpFilter),
            6 => Ok(Self::DelayReverbSend),
            7 => Ok(Self::DelayMixVolume),
            8 => Ok(Self::DelayOverdrive),
            10 => Ok(Self::ReverbPreDelay),
            11 => Ok(Self::ReverbDecay),
            12 => Ok(Self::ReverbShelvingFreq),
            13 => Ok(Self::ReverbShelvingGain),
            14 => Ok(Self::ReverbHpFilter),
            15 => Ok(Self::ReverbLpFilter),
            16 => Ok(Self::ReverbMixVolume),
            18 => Ok(Self::DistortionAmount),
            19 => Ok(Self::DistortionSymmetry),
            21 => Ok(Self::CompressorThreshold),
            22 => Ok(Self::CompressorAttack),
            23 => Ok(Self::CompressorRelease),
            24 => Ok(Self::CompressorRatio),
            25 => Ok(Self::CompressorSideChainEq),
            26 => Ok(Self::CompressorMakeUpGain),
            27 => Ok(Self::CompressorDryWetMix),
            28 => Ok(Self::CompressorVolume),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxLfoDestination".to_string(),
            }),
        }
    }
}

impl From<FxLfoDestination> for u8 {
    fn from(value: FxLfoDestination) -> Self {
        match value {
            FxLfoDestination::Unset => 37,
            FxLfoDestination::DelayTime => 0,
            FxLfoDestination::DelayPingPong => 1,
            FxLfoDestination::DelayStereoWidth => 2,
            FxLfoDestination::DelayFeedback => 3,
            FxLfoDestination::DelayHpFilter => 4,
            FxLfoDestination::DelayLpFilter => 5,
            FxLfoDestination::DelayReverbSend => 6,
            FxLfoDestination::DelayMixVolume => 7,
            FxLfoDestination::DelayOverdrive => 8,
            FxLfoDestination::ReverbPreDelay => 10,
            FxLfoDestination::ReverbDecay => 11,
            FxLfoDestination::ReverbShelvingFreq => 12,
            FxLfoDestination::ReverbShelvingGain => 13,
            FxLfoDestination::ReverbHpFilter => 14,
            FxLfoDestination::ReverbLpFilter => 15,
            FxLfoDestination::ReverbMixVolume => 16,
            FxLfoDestination::DistortionAmount => 18,
            FxLfoDestination::DistortionSymmetry => 19,
            FxLfoDestination::CompressorThreshold => 21,
            FxLfoDestination::CompressorAttack => 22,
            FxLfoDestination::CompressorRelease => 23,
            FxLfoDestination::CompressorRatio => 24,
            FxLfoDestination::CompressorSideChainEq => 25,
            FxLfoDestination::CompressorMakeUpGain => 26,
            FxLfoDestination::CompressorDryWetMix => 27,
            FxLfoDestination::CompressorVolume => 28,
        }
    }
}

/// On the grid delay time.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum FxDelayTimeOnTheGrid {
    _128th,
    _64th,
    _64thDotted,
    _32nd,
    _32ndDotted,
    _16th,
    _16thDotted,
    _8th,
    _8thDotted,
    #[default]
    Quarter,
    QuarterDotted,
    Half,
    HalfDotted,
    Whole,
    NotOnTheGrid(u8),
}

impl TryFrom<&str> for FxDelayTimeOnTheGrid {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Ignore not on the grid.
        match value {
            "128th" => Ok(Self::_128th),
            "64th" => Ok(Self::_64th),
            "64thdotted" => Ok(Self::_64thDotted),
            "32nd" => Ok(Self::_32nd),
            "32nddotted" => Ok(Self::_32ndDotted),
            "16th" => Ok(Self::_16th),
            "16thdotted" => Ok(Self::_16thDotted),
            "8th" => Ok(Self::_8th),
            "8thdotted" => Ok(Self::_8thDotted),
            "quarter" => Ok(Self::Quarter),
            "quarterdotted" => Ok(Self::QuarterDotted),
            "half" => Ok(Self::Half),
            "halfdotted" => Ok(Self::HalfDotted),
            "whole" => Ok(Self::Whole),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxDelayTimeOnTheGrid".to_string(),
            }),
        }
    }
}

impl From<FxDelayTimeOnTheGrid> for &str {
    fn from(value: FxDelayTimeOnTheGrid) -> Self {
        match value {
            FxDelayTimeOnTheGrid::_128th => "128th",
            FxDelayTimeOnTheGrid::_64th => "64th",
            FxDelayTimeOnTheGrid::_64thDotted => "64thdotted",
            FxDelayTimeOnTheGrid::_32nd => "32nd",
            FxDelayTimeOnTheGrid::_32ndDotted => "32nddotted",
            FxDelayTimeOnTheGrid::_16th => "16th",
            FxDelayTimeOnTheGrid::_16thDotted => "16thdotted",
            FxDelayTimeOnTheGrid::_8th => "8th",
            FxDelayTimeOnTheGrid::_8thDotted => "8thdotted",
            FxDelayTimeOnTheGrid::Quarter => "quarter",
            FxDelayTimeOnTheGrid::QuarterDotted => "quarterdotted",
            FxDelayTimeOnTheGrid::Half => "half",
            FxDelayTimeOnTheGrid::HalfDotted => "halfdotted",
            FxDelayTimeOnTheGrid::Whole => "whole",
            FxDelayTimeOnTheGrid::NotOnTheGrid(val) => {
                panic!("Not on the grid delay time: {}", val)
            }
        }
    }
}

// Double check.
impl From<FxDelayTimeOnTheGrid> for u8 {
    fn from(value: FxDelayTimeOnTheGrid) -> Self {
        match value {
            FxDelayTimeOnTheGrid::_128th => 0,
            FxDelayTimeOnTheGrid::_64th => 1,
            FxDelayTimeOnTheGrid::_64thDotted => 2,
            FxDelayTimeOnTheGrid::_32nd => 3,
            FxDelayTimeOnTheGrid::_32ndDotted => 5,
            FxDelayTimeOnTheGrid::_16th => 7,
            FxDelayTimeOnTheGrid::_16thDotted => 11,
            FxDelayTimeOnTheGrid::_8th => 15,
            FxDelayTimeOnTheGrid::_8thDotted => 24,
            FxDelayTimeOnTheGrid::Quarter => 32,
            FxDelayTimeOnTheGrid::QuarterDotted => 48,
            FxDelayTimeOnTheGrid::Half => 64,
            FxDelayTimeOnTheGrid::HalfDotted => 80,
            FxDelayTimeOnTheGrid::Whole => 128,
            FxDelayTimeOnTheGrid::NotOnTheGrid(val) => val,
        }
    }
}

/// Compressor attack type
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum FxCompAttack {
    /// 0.03
    #[default]
    _0_03,
    /// 0.1
    _0_1,
    /// 0.3
    _0_3,
    /// 1
    _1,
    /// 3
    _3,
    /// 10
    _10,
    /// 30
    _30,
}

impl TryFrom<&str> for FxCompAttack {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0.03" => Ok(Self::_0_03),
            "0.1" => Ok(Self::_0_1),
            "0.3" => Ok(Self::_0_3),
            "1" => Ok(Self::_1),
            "3" => Ok(Self::_3),
            "10" => Ok(Self::_10),
            "30" => Ok(Self::_30),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxCompAttack".to_string(),
            }),
        }
    }
}

impl From<FxCompAttack> for &str {
    fn from(value: FxCompAttack) -> Self {
        match value {
            FxCompAttack::_0_03 => "0.03",
            FxCompAttack::_0_1 => "0.1",
            FxCompAttack::_0_3 => "0.3",
            FxCompAttack::_1 => "1",
            FxCompAttack::_3 => "3",
            FxCompAttack::_10 => "10",
            FxCompAttack::_30 => "30",
        }
    }
}

impl From<FxCompAttack> for u8 {
    fn from(value: FxCompAttack) -> Self {
        match value {
            FxCompAttack::_0_03 => 0,
            FxCompAttack::_0_1 => 1,
            FxCompAttack::_0_3 => 2,
            FxCompAttack::_1 => 3,
            FxCompAttack::_3 => 4,
            FxCompAttack::_10 => 5,
            FxCompAttack::_30 => 6,
        }
    }
}

impl TryFrom<u8> for FxCompAttack {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::_0_03),
            1 => Ok(Self::_0_1),
            2 => Ok(Self::_0_3),
            3 => Ok(Self::_1),
            4 => Ok(Self::_3),
            5 => Ok(Self::_10),
            6 => Ok(Self::_30),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxCompAttack".to_string(),
            }),
        }
    }
}

/// Compressor release type
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum FxCompRelease {
    /// 0.1
    _0_1,
    /// 0.2
    _0_2,
    #[default]
    /// 0.4
    _0_4,
    /// 0.6
    _0_6,
    /// 1
    _1,
    /// 2
    _2,
    /// A1
    A1,
    /// A2
    A2,
}

impl TryFrom<&str> for FxCompRelease {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0.1" => Ok(Self::_0_1),
            "0.2" => Ok(Self::_0_2),
            "0.4" => Ok(Self::_0_4),
            "0.6" => Ok(Self::_0_6),
            "1" => Ok(Self::_1),
            "2" => Ok(Self::_2),
            "A1" => Ok(Self::A1),
            "A2" => Ok(Self::A2),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxCompRelease".to_string(),
            }),
        }
    }
}

impl From<FxCompRelease> for &str {
    fn from(value: FxCompRelease) -> Self {
        match value {
            FxCompRelease::_0_1 => "0.1",
            FxCompRelease::_0_2 => "0.2",
            FxCompRelease::_0_4 => "0.4",
            FxCompRelease::_0_6 => "0.6",
            FxCompRelease::_1 => "1",
            FxCompRelease::_2 => "2",
            FxCompRelease::A1 => "A1",
            FxCompRelease::A2 => "A2",
        }
    }
}

impl From<FxCompRelease> for u8 {
    fn from(value: FxCompRelease) -> Self {
        match value {
            FxCompRelease::_0_1 => 0,
            FxCompRelease::_0_2 => 1,
            FxCompRelease::_0_4 => 2,
            FxCompRelease::_0_6 => 3,
            FxCompRelease::_1 => 4,
            FxCompRelease::_2 => 5,
            FxCompRelease::A1 => 6,
            FxCompRelease::A2 => 7,
        }
    }
}

impl TryFrom<u8> for FxCompRelease {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::_0_1),
            1 => Ok(Self::_0_2),
            2 => Ok(Self::_0_4),
            3 => Ok(Self::_0_6),
            4 => Ok(Self::_1),
            5 => Ok(Self::_2),
            6 => Ok(Self::A1),
            7 => Ok(Self::A2),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxCompRelease".to_string(),
            }),
        }
    }
}

/// Compressor ratio type
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum FxCompRatio {
    #[default]
    /// 1:2
    _1B2,
    /// 1:4
    _1B4,
    /// 1:8
    _1B8,
    /// MAX
    Max,
}

impl TryFrom<&str> for FxCompRatio {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1:2" => Ok(Self::_1B2),
            "1:4" => Ok(Self::_1B4),
            "1:8" => Ok(Self::_1B8),
            "max" => Ok(Self::Max),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxCompRatio".to_string(),
            }),
        }
    }
}

impl From<FxCompRatio> for &str {
    fn from(value: FxCompRatio) -> Self {
        match value {
            FxCompRatio::_1B2 => "1:2",
            FxCompRatio::_1B4 => "1:4",
            FxCompRatio::_1B8 => "1:8",
            FxCompRatio::Max => "max",
        }
    }
}

impl From<FxCompRatio> for u8 {
    fn from(value: FxCompRatio) -> Self {
        match value {
            FxCompRatio::_1B2 => 0,
            FxCompRatio::_1B4 => 1,
            FxCompRatio::_1B8 => 2,
            FxCompRatio::Max => 3,
        }
    }
}

impl TryFrom<u8> for FxCompRatio {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::_1B2),
            1 => Ok(Self::_1B4),
            2 => Ok(Self::_1B8),
            3 => Ok(Self::Max),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxCompRatio".to_string(),
            }),
        }
    }
}

/// Compressor side chain eq type
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum FxCompSideChainEq {
    Off,
    #[default]
    Lpf,
    Hpf,
    Hit,
}

impl TryFrom<&str> for FxCompSideChainEq {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "off" => Ok(Self::Off),
            "lpf" => Ok(Self::Lpf),
            "hpf" => Ok(Self::Hpf),
            "hit" => Ok(Self::Hit),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxCompSideChainEq".to_string(),
            }),
        }
    }
}

impl From<FxCompSideChainEq> for &str {
    fn from(value: FxCompSideChainEq) -> Self {
        match value {
            FxCompSideChainEq::Off => "off",
            FxCompSideChainEq::Lpf => "lpf",
            FxCompSideChainEq::Hpf => "hpf",
            FxCompSideChainEq::Hit => "hit",
        }
    }
}

impl From<FxCompSideChainEq> for u8 {
    fn from(value: FxCompSideChainEq) -> Self {
        match value {
            FxCompSideChainEq::Off => 0,
            FxCompSideChainEq::Lpf => 1,
            FxCompSideChainEq::Hpf => 2,
            FxCompSideChainEq::Hit => 3,
        }
    }
}

impl TryFrom<u8> for FxCompSideChainEq {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::Lpf),
            2 => Ok(Self::Hpf),
            3 => Ok(Self::Hit),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxCompSideChainEq".to_string(),
            }),
        }
    }
}
