use crate::error::ConversionError;

/// Destination of an LFO.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

// // 0..=127 device (from 0-127 in order) 1/128 1/164 1/64 1/32 5 1/32. 7 1/16 9 10 11 1/16. 13 14 15 1/8 17..=23 1/8. 25..=31 1/4 33..=47 1/4. 49..=63 1/2 65..=79 1/2. 81..=95 1/2. 97..=127 1/1

/// On the grid delay time.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FxCompRelease {
    #[default]
    /// 0.1
    _0_1,
    /// 0.2
    _0_2,
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FxCompSideChainEq {
    #[default]
    Off,
    Lpf,
    Hpf,
    Hit,
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
