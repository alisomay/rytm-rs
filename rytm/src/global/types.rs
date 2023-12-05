use crate::error::ConversionError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MidiChannel {
    Channel(usize),
    #[default]
    Auto,
    Off,
}

impl Into<u8> for MidiChannel {
    fn into(self) -> u8 {
        match self {
            MidiChannel::Channel(channel) => channel as u8,
            MidiChannel::Auto | MidiChannel::Off => 0xFF,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeSignature {
    _1B1,
    _2B1,
    _3B1,
    _4B1,
    _5B1,
    _6B1,
    _7B1,
    _8B1,
    _9B1,
    _10B1,
    _11B1,
    _12B1,
    _13B1,
    _14B1,
    _15B1,
    _16B1,
    _1B2,
    _2B2,
    _3B2,
    _4B2,
    _5B2,
    _6B2,
    _7B2,
    _8B2,
    _9B2,
    _10B2,
    _11B2,
    _12B2,
    _13B2,
    _14B2,
    _15B2,
    _16B2,
    _1B4,
    _2B4,
    _3B4,
    #[default]
    _4B4,
    _5B4,
    _6B4,
    _7B4,
    _8B4,
    _9B4,
    _10B4,
    _11B4,
    _12B4,
    _13B4,
    _14B4,
    _15B4,
    _16B4,
    _1B8,
    _2B8,
    _3B8,
    _4B8,
    _5B8,
    _6B8,
    _7B8,
    _8B8,
    _9B8,
    _10B8,
    _11B8,
    _12B8,
    _13B8,
    _14B8,
    _15B8,
    _16B8,
    _1B16,
    _2B16,
    _3B16,
    _4B16,
    _5B16,
    _6B16,
    _7B16,
    _8B16,
    _9B16,
    _10B16,
    _11B16,
    _12B16,
    _13B16,
    _14B16,
    _15B16,
    _16B16,
}

impl TryFrom<(u8, u8)> for TimeSignature {
    type Error = ConversionError;

    fn try_from((num, den): (u8, u8)) -> Result<Self, Self::Error> {
        match (num, den) {
            (1, 1) => Ok(TimeSignature::_1B1),
            (2, 1) => Ok(TimeSignature::_2B1),
            (3, 1) => Ok(TimeSignature::_3B1),
            (4, 1) => Ok(TimeSignature::_4B1),
            (5, 1) => Ok(TimeSignature::_5B1),
            (6, 1) => Ok(TimeSignature::_6B1),
            (7, 1) => Ok(TimeSignature::_7B1),
            (8, 1) => Ok(TimeSignature::_8B1),
            (9, 1) => Ok(TimeSignature::_9B1),
            (10, 1) => Ok(TimeSignature::_10B1),
            (11, 1) => Ok(TimeSignature::_11B1),
            (12, 1) => Ok(TimeSignature::_12B1),
            (13, 1) => Ok(TimeSignature::_13B1),
            (14, 1) => Ok(TimeSignature::_14B1),
            (15, 1) => Ok(TimeSignature::_15B1),
            (16, 1) => Ok(TimeSignature::_16B1),
            (1, 2) => Ok(TimeSignature::_1B2),
            (2, 2) => Ok(TimeSignature::_2B2),
            (3, 2) => Ok(TimeSignature::_3B2),
            (4, 2) => Ok(TimeSignature::_4B2),
            (5, 2) => Ok(TimeSignature::_5B2),
            (6, 2) => Ok(TimeSignature::_6B2),
            (7, 2) => Ok(TimeSignature::_7B2),
            (8, 2) => Ok(TimeSignature::_8B2),
            (9, 2) => Ok(TimeSignature::_9B2),
            (10, 2) => Ok(TimeSignature::_10B2),
            (11, 2) => Ok(TimeSignature::_11B2),
            (12, 2) => Ok(TimeSignature::_12B2),
            (13, 2) => Ok(TimeSignature::_13B2),
            (14, 2) => Ok(TimeSignature::_14B2),
            (15, 2) => Ok(TimeSignature::_15B2),
            (16, 2) => Ok(TimeSignature::_16B2),
            (1, 4) => Ok(TimeSignature::_1B4),
            (2, 4) => Ok(TimeSignature::_2B4),
            (3, 4) => Ok(TimeSignature::_3B4),
            (4, 4) => Ok(TimeSignature::_4B4),
            (5, 4) => Ok(TimeSignature::_5B4),
            (6, 4) => Ok(TimeSignature::_6B4),
            (7, 4) => Ok(TimeSignature::_7B4),
            (8, 4) => Ok(TimeSignature::_8B4),
            (9, 4) => Ok(TimeSignature::_9B4),
            (10, 4) => Ok(TimeSignature::_10B4),
            (11, 4) => Ok(TimeSignature::_11B4),
            (12, 4) => Ok(TimeSignature::_12B4),
            (13, 4) => Ok(TimeSignature::_13B4),
            (14, 4) => Ok(TimeSignature::_14B4),
            (15, 4) => Ok(TimeSignature::_15B4),
            (16, 4) => Ok(TimeSignature::_16B4),
            (1, 8) => Ok(TimeSignature::_1B8),
            (2, 8) => Ok(TimeSignature::_2B8),
            (3, 8) => Ok(TimeSignature::_3B8),
            (4, 8) => Ok(TimeSignature::_4B8),
            (5, 8) => Ok(TimeSignature::_5B8),
            (6, 8) => Ok(TimeSignature::_6B8),
            (7, 8) => Ok(TimeSignature::_7B8),
            (8, 8) => Ok(TimeSignature::_8B8),
            (9, 8) => Ok(TimeSignature::_9B8),
            (10, 8) => Ok(TimeSignature::_10B8),
            (11, 8) => Ok(TimeSignature::_11B8),
            (12, 8) => Ok(TimeSignature::_12B8),
            (13, 8) => Ok(TimeSignature::_13B8),
            (14, 8) => Ok(TimeSignature::_14B8),
            (15, 8) => Ok(TimeSignature::_15B8),
            (16, 8) => Ok(TimeSignature::_16B8),
            (1, 16) => Ok(TimeSignature::_1B16),
            (2, 16) => Ok(TimeSignature::_2B16),
            (3, 16) => Ok(TimeSignature::_3B16),
            (4, 16) => Ok(TimeSignature::_4B16),
            (5, 16) => Ok(TimeSignature::_5B16),
            (6, 16) => Ok(TimeSignature::_6B16),
            (7, 16) => Ok(TimeSignature::_7B16),
            (8, 16) => Ok(TimeSignature::_8B16),
            (9, 16) => Ok(TimeSignature::_9B16),
            (10, 16) => Ok(TimeSignature::_10B16),
            (11, 16) => Ok(TimeSignature::_11B16),
            (12, 16) => Ok(TimeSignature::_12B16),
            (13, 16) => Ok(TimeSignature::_13B16),
            (14, 16) => Ok(TimeSignature::_14B16),
            (15, 16) => Ok(TimeSignature::_15B16),
            (16, 16) => Ok(TimeSignature::_16B16),
            _ => Err(ConversionError::Range {
                value: format!("{}/{}", num, den),
                type_name: "TimeSignature".into(),
            }),
        }
    }
}

impl From<&TimeSignature> for (u8, u8) {
    fn from(ts: &TimeSignature) -> Self {
        match ts {
            TimeSignature::_1B1 => (1, 1),
            TimeSignature::_2B1 => (2, 1),
            TimeSignature::_3B1 => (3, 1),
            TimeSignature::_4B1 => (4, 1),
            TimeSignature::_5B1 => (5, 1),
            TimeSignature::_6B1 => (6, 1),
            TimeSignature::_7B1 => (7, 1),
            TimeSignature::_8B1 => (8, 1),
            TimeSignature::_9B1 => (9, 1),
            TimeSignature::_10B1 => (10, 1),
            TimeSignature::_11B1 => (11, 1),
            TimeSignature::_12B1 => (12, 1),
            TimeSignature::_13B1 => (13, 1),
            TimeSignature::_14B1 => (14, 1),
            TimeSignature::_15B1 => (15, 1),
            TimeSignature::_16B1 => (16, 1),
            TimeSignature::_1B2 => (1, 2),
            TimeSignature::_2B2 => (2, 2),
            TimeSignature::_3B2 => (3, 2),
            TimeSignature::_4B2 => (4, 2),
            TimeSignature::_5B2 => (5, 2),
            TimeSignature::_6B2 => (6, 2),
            TimeSignature::_7B2 => (7, 2),
            TimeSignature::_8B2 => (8, 2),
            TimeSignature::_9B2 => (9, 2),
            TimeSignature::_10B2 => (10, 2),
            TimeSignature::_11B2 => (11, 2),
            TimeSignature::_12B2 => (12, 2),
            TimeSignature::_13B2 => (13, 2),
            TimeSignature::_14B2 => (14, 2),
            TimeSignature::_15B2 => (15, 2),
            TimeSignature::_16B2 => (16, 2),
            TimeSignature::_1B4 => (1, 4),
            TimeSignature::_2B4 => (2, 4),
            TimeSignature::_3B4 => (3, 4),
            TimeSignature::_4B4 => (4, 4),
            TimeSignature::_5B4 => (5, 4),
            TimeSignature::_6B4 => (6, 4),
            TimeSignature::_7B4 => (7, 4),
            TimeSignature::_8B4 => (8, 4),
            TimeSignature::_9B4 => (9, 4),
            TimeSignature::_10B4 => (10, 4),
            TimeSignature::_11B4 => (11, 4),
            TimeSignature::_12B4 => (12, 4),
            TimeSignature::_13B4 => (13, 4),
            TimeSignature::_14B4 => (14, 4),
            TimeSignature::_15B4 => (15, 4),
            TimeSignature::_16B4 => (16, 4),
            TimeSignature::_1B8 => (1, 8),
            TimeSignature::_2B8 => (2, 8),
            TimeSignature::_3B8 => (3, 8),
            TimeSignature::_4B8 => (4, 8),
            TimeSignature::_5B8 => (5, 8),
            TimeSignature::_6B8 => (6, 8),
            TimeSignature::_7B8 => (7, 8),
            TimeSignature::_8B8 => (8, 8),
            TimeSignature::_9B8 => (9, 8),
            TimeSignature::_10B8 => (10, 8),
            TimeSignature::_11B8 => (11, 8),
            TimeSignature::_12B8 => (12, 8),
            TimeSignature::_13B8 => (13, 8),
            TimeSignature::_14B8 => (14, 8),
            TimeSignature::_15B8 => (15, 8),
            TimeSignature::_16B8 => (16, 8),
            TimeSignature::_1B16 => (1, 16),
            TimeSignature::_2B16 => (2, 16),
            TimeSignature::_3B16 => (3, 16),
            TimeSignature::_4B16 => (4, 16),
            TimeSignature::_5B16 => (5, 16),
            TimeSignature::_6B16 => (6, 16),
            TimeSignature::_7B16 => (7, 16),
            TimeSignature::_8B16 => (8, 16),
            TimeSignature::_9B16 => (9, 16),
            TimeSignature::_10B16 => (10, 16),
            TimeSignature::_11B16 => (11, 16),
            TimeSignature::_12B16 => (12, 16),
            TimeSignature::_13B16 => (13, 16),
            TimeSignature::_14B16 => (14, 16),
            TimeSignature::_15B16 => (15, 16),
            TimeSignature::_16B16 => (16, 16),
        }
    }
}

impl TimeSignature {
    pub fn numerator(&self) -> usize {
        let (numerator, _) = self.into();
        numerator as usize
    }
    pub fn denominator(&self) -> usize {
        let (_, denominator) = self.into();
        denominator as usize
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MidiPortFunction {
    #[default]
    Midi,
    Din24,
    Din48,
}

impl TryFrom<u8> for MidiPortFunction {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MidiPortFunction::Midi),
            1 => Ok(MidiPortFunction::Din24),
            2 => Ok(MidiPortFunction::Din48),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "MidiPortFunction".into(),
            }),
        }
    }
}

impl From<MidiPortFunction> for u8 {
    fn from(pf: MidiPortFunction) -> Self {
        match pf {
            MidiPortFunction::Midi => 0,
            MidiPortFunction::Din24 => 1,
            MidiPortFunction::Din48 => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MidiTransportLayer {
    Disabled,
    Midi,
    Usb,
    #[default]
    MidiAndUsb,
}

impl TryFrom<u8> for MidiTransportLayer {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MidiTransportLayer::Disabled),
            1 => Ok(MidiTransportLayer::Midi),
            2 => Ok(MidiTransportLayer::Usb),
            3 => Ok(MidiTransportLayer::MidiAndUsb),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "MidiTransportLayer".into(),
            }),
        }
    }
}

impl From<MidiTransportLayer> for u8 {
    fn from(mtl: MidiTransportLayer) -> Self {
        match mtl {
            MidiTransportLayer::Disabled => 0,
            MidiTransportLayer::Midi => 1,
            MidiTransportLayer::Usb => 2,
            MidiTransportLayer::MidiAndUsb => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParameterDestination {
    #[default]
    Internal,
    InternalAndExternal,
    External,
}

impl TryFrom<u8> for ParameterDestination {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParameterDestination::Internal),
            1 => Ok(ParameterDestination::InternalAndExternal),
            2 => Ok(ParameterDestination::External),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "ParameterDestination".into(),
            }),
        }
    }
}

impl From<ParameterDestination> for u8 {
    fn from(ppemd: ParameterDestination) -> Self {
        match ppemd {
            ParameterDestination::Internal => 0,
            ParameterDestination::InternalAndExternal => 1,
            ParameterDestination::External => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UsbToMainDb {
    #[default]
    Zero,
    PlusSix,
    PlusTwelve,
    PlusEighteen,
}

impl TryFrom<u8> for UsbToMainDb {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UsbToMainDb::Zero),
            1 => Ok(UsbToMainDb::PlusSix),
            2 => Ok(UsbToMainDb::PlusTwelve),
            3 => Ok(UsbToMainDb::PlusEighteen),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "UsbToMainDb".into(),
            }),
        }
    }
}

impl From<UsbToMainDb> for u8 {
    fn from(utmd: UsbToMainDb) -> Self {
        match utmd {
            UsbToMainDb::Zero => 0,
            UsbToMainDb::PlusSix => 1,
            UsbToMainDb::PlusTwelve => 2,
            UsbToMainDb::PlusEighteen => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MidiParameterOutput {
    Nrpn,
    #[default]
    Cc,
}

impl TryFrom<u8> for MidiParameterOutput {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MidiParameterOutput::Nrpn),
            1 => Ok(MidiParameterOutput::Cc),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "MidiParameterOutput".into(),
            }),
        }
    }
}

impl From<MidiParameterOutput> for u8 {
    fn from(mpo: MidiParameterOutput) -> Self {
        match mpo {
            MidiParameterOutput::Nrpn => 0,
            MidiParameterOutput::Cc => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MidiPortsOutputChannel {
    #[default]
    AutoChannel,
    TrackChannel,
}

impl TryFrom<u8> for MidiPortsOutputChannel {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MidiPortsOutputChannel::AutoChannel),
            1 => Ok(MidiPortsOutputChannel::TrackChannel),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "MidiPortsOutputChannel".into(),
            }),
        }
    }
}

impl From<MidiPortsOutputChannel> for u8 {
    fn from(mpoc: MidiPortsOutputChannel) -> Self {
        match mpoc {
            MidiPortsOutputChannel::AutoChannel => 0,
            MidiPortsOutputChannel::TrackChannel => 1,
        }
    }
}
