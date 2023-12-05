use crate::error::ConversionError;

/// An enum which represents a midi channel in the global menu.
///
/// It can be either a specific channel, the auto channel or off.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MidiChannel {
    Channel(usize),
    #[default]
    Auto,
    Off,
}

#[allow(clippy::from_over_into)]
impl Into<u8> for MidiChannel {
    fn into(self) -> u8 {
        match self {
            MidiChannel::Channel(channel) => channel as u8,
            // We can't do From implementations because 0xFF can mean Auto or Off depending on the context.
            MidiChannel::Auto | MidiChannel::Off => 0xFF,
        }
    }
}

/// An enum which represents a time signature in the metronome settings menu.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeSignature {
    /// 1/1
    _1B1,
    /// 2/1
    _2B1,
    /// 3/1
    _3B1,
    /// 4/1
    _4B1,
    /// 5/1
    _5B1,
    /// 6/1
    _6B1,
    /// 7/1
    _7B1,
    /// 8/1
    _8B1,
    /// 9/1
    _9B1,
    /// 10/1
    _10B1,
    /// 11/1
    _11B1,
    /// 12/1
    _12B1,
    /// 13/1
    _13B1,
    /// 14/1
    _14B1,
    /// 15/1
    _15B1,
    /// 16/1
    _16B1,
    /// 1/2
    _1B2,
    /// 2/2
    _2B2,
    /// 3/2
    _3B2,
    /// 4/2
    _4B2,
    /// 5/2
    _5B2,
    /// 6/2
    _6B2,
    /// 7/2
    _7B2,
    /// 8/2
    _8B2,
    /// 9/2
    _9B2,
    /// 10/2
    _10B2,
    /// 11/2
    _11B2,
    /// 12/2
    _12B2,
    /// 13/2
    _13B2,
    /// 14/2
    _14B2,
    /// 15/2
    _15B2,
    /// 16/2
    _16B2,
    /// 1/4
    _1B4,
    /// 2/4
    _2B4,
    /// 3/4
    _3B4,
    /// 4/4
    #[default]
    _4B4,
    /// 5/4
    _5B4,
    /// 6/4
    _6B4,
    /// 7/4
    _7B4,
    /// 8/4
    _8B4,
    /// 9/4
    _9B4,
    /// 10/4
    _10B4,
    /// 11/4
    _11B4,
    /// 12/4
    _12B4,
    /// 13/4
    _13B4,
    /// 14/4
    _14B4,
    /// 15/4
    _15B4,
    /// 16/4
    _16B4,
    /// 1/8
    _1B8,
    /// 2/8
    _2B8,
    /// 3/8
    _3B8,
    /// 4/8
    _4B8,
    /// 5/8
    _5B8,
    /// 6/8
    _6B8,
    /// 7/8
    _7B8,
    /// 8/8
    _8B8,
    /// 9/8
    _9B8,
    /// 10/8
    _10B8,
    /// 11/8
    _11B8,
    /// 12/8
    _12B8,
    /// 13/8
    _13B8,
    /// 14/8
    _14B8,
    /// 15/8
    _15B8,
    /// 16/8
    _16B8,
    /// 1/16
    _1B16,
    /// 2/16
    _2B16,
    /// 3/16
    _3B16,
    /// 4/16
    _4B16,
    /// 5/16
    _5B16,
    /// 6/16
    _6B16,
    /// 7/16
    _7B16,
    /// 8/16
    _8B16,
    /// 9/16
    _9B16,
    /// 10/16
    _10B16,
    /// 11/16
    _11B16,
    /// 12/16
    _12B16,
    /// 13/16
    _13B16,
    /// 14/16
    _14B16,
    /// 15/16
    _15B16,
    /// 16/16
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
    /// Returns the numerator of the time signature.
    pub fn numerator(&self) -> usize {
        let (numerator, _) = self.into();
        numerator as usize
    }

    /// Returns the denominator of the time signature.
    pub fn denominator(&self) -> usize {
        let (_, denominator) = self.into();
        denominator as usize
    }

    /// Returns the time signature as a tuple of `(numerator, denominator)`.
    pub fn as_tuple(&self) -> (usize, usize) {
        let (num, dem) = self.into();
        (num as usize, dem as usize)
    }
}

/// An enum which represents a midi port function in the midi config menu.
///
/// It can be either midi, din24 or din48.
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

/// An enum which represents a midi port mode in the midi config menu.
///
/// Midi data can be transported via the physical midi port, usb port or both.
///
/// It can also be disabled.
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

/// An enum which represents parameter destinations in the midi config menu.
///
/// It can be either internal, internal and external or external.
///
/// - Internal means that the parameter value is not sent to the external world.
/// - External means that the parameter value is sent to the external world but not to the internal world.
/// - Internal and external means that the parameter value is sent to both the internal and external world.
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

/// An enum which represents the extra gain added to the signal when an audio signal is routed from usb audio to main.
///
/// Excerpt from the manual:
///
/// `USB TO MAIN [dB]` sets the amount of amplification of the sound that is streamed over USB to the
/// Analog Rytm MKII’s main out when used with a class compliant audio device. (0 dB–+18 dB) This parameter
/// is only available when `USB CONFIG` is set to `USB AUDIO/MIDI`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoutingUsbToMainDb {
    #[default]
    Zero,
    PlusSix,
    PlusTwelve,
    PlusEighteen,
}

impl TryFrom<u8> for RoutingUsbToMainDb {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RoutingUsbToMainDb::Zero),
            1 => Ok(RoutingUsbToMainDb::PlusSix),
            2 => Ok(RoutingUsbToMainDb::PlusTwelve),
            3 => Ok(RoutingUsbToMainDb::PlusEighteen),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "RoutingUsbToMainDb".into(),
            }),
        }
    }
}

impl From<RoutingUsbToMainDb> for u8 {
    fn from(utmd: RoutingUsbToMainDb) -> Self {
        match utmd {
            RoutingUsbToMainDb::Zero => 0,
            RoutingUsbToMainDb::PlusSix => 1,
            RoutingUsbToMainDb::PlusTwelve => 2,
            RoutingUsbToMainDb::PlusEighteen => 3,
        }
    }
}

/// An enum which represents the type of midi parameter output in the midi config menu.
///
/// It can be either `NRPN` or `CC`.
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

/// An enum which represents the channels used for midi parameter output through the midi ports.
///
/// It can be either the auto channel or the track channel.
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

/// An enum which represents the voices of the rytm coupled by pad numbers.
///
/// - Rytm has 8 voices but 12 pads.
/// - Some pads share the same voice.
/// - Also it has 8 outputs and they correspond to the voices.
///
/// This enum is used to represent the voices coupled by pad numbers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HardwareTrack {
    #[default]
    _1,
    _2,
    _3and4,
    _5,
    _6,
    _7and8,
    _9and10,
    _11and12,
}

impl TryFrom<u8> for HardwareTrack {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HardwareTrack::_1),
            1 => Ok(HardwareTrack::_2),
            2 => Ok(HardwareTrack::_3and4),
            3 => Ok(HardwareTrack::_5),
            4 => Ok(HardwareTrack::_6),
            5 => Ok(HardwareTrack::_7and8),
            6 => Ok(HardwareTrack::_9and10),
            7 => Ok(HardwareTrack::_11and12),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "HardwareTrack".into(),
            }),
        }
    }
}

impl From<HardwareTrack> for u8 {
    fn from(ht: HardwareTrack) -> Self {
        match ht {
            HardwareTrack::_1 => 0,
            HardwareTrack::_2 => 1,
            HardwareTrack::_3and4 => 2,
            HardwareTrack::_5 => 3,
            HardwareTrack::_6 => 4,
            HardwareTrack::_7and8 => 5,
            HardwareTrack::_9and10 => 6,
            HardwareTrack::_11and12 => 7,
        }
    }
}

/// An enum which represents the usb in audio routing options.
///
/// # Excerpt from the manual:
///
/// `USB IN` sets where the incoming audio from the class compliant device is routed to in the Analog Rytm
/// MKII’s signal path.
///
/// This parameter is only available when `USB CONFIG` is set to `USB AUDIO/MIDI`.
///
/// - `PRE-FX` the incoming audio is routed in before the Analog Rytm MKII’s effects and will be affected by
/// those and then be sent at the main outputs. The audio is also routed to the sampler.
/// - `POST-FX` the incoming audio is routed in after the Analog Rytm MKII’s effects and will not be affected
/// by those. The audio is then sent at the main outputs. The audio is also routed to the sampler.
/// - `TRACK 1–12, L:1–12/R:1–12` Press a `[PAD]` twice to select a single track as a destination. The select-
/// ed track’s `[PAD]` (or `[PADS]` if you selected a track that shares its voice with another track) lights up
/// white. The incoming audio is summed to mono and routed to the selected track and is affected by the
/// tracks parameters (such as filter, envelope). Press first one `[PAD]` and then another to select two separate tracks as destinations. The selected
/// tracks’ `[PADS]` lights up blue for left channel and red for right channel. The incoming audio’s left and
/// right signal is then sent to separate tracks and is affected by the tracks’ parameters.
/// The audio is also routed to the sampler.
/// - `SAMPLER ONLY` the incoming audio is only routed to the Analog Rytm MKII’s sampler and not to any
/// track or to the main out.
///
/// To be able to hear and process the incoming audio sent to one (or two) of the Analog Rytm
/// MKII’s tracks, you must place a note trig on the selected track(s) and start the sequencer.
/// This is needed to trigger and open the track’s envelope and let the audio through. For
/// continuous processing, set the trig’s LEN parameter to INF.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoutingUsbInOptions {
    #[default]
    PreFx,
    PostFx,
    /// Left channel routing, Right channel routing
    VoiceRouting((HardwareTrack, HardwareTrack)),
    SamplerOnly,
}

impl TryFrom<u8> for RoutingUsbInOptions {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let flags = value & 0b0000_0011;
        match flags {
            0 => Ok(RoutingUsbInOptions::PreFx),
            1 => Ok(RoutingUsbInOptions::PostFx),
            2 => {
                Ok(RoutingUsbInOptions::VoiceRouting((
                    // Left channel routing
                    ((value & 0b0001_1100) >> 2).try_into()?,
                    // Right channel routing
                    ((value & 0b1110_0000) >> 5).try_into()?,
                )))
            }
            3 => Ok(RoutingUsbInOptions::SamplerOnly),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "RoutingUsbInOptions".into(),
            }),
        }
    }
}

impl From<RoutingUsbInOptions> for u8 {
    fn from(ruio: RoutingUsbInOptions) -> Self {
        match ruio {
            RoutingUsbInOptions::PreFx => 0,
            RoutingUsbInOptions::PostFx => 1,
            RoutingUsbInOptions::VoiceRouting((left_channel_routing, right_channel_routing)) => {
                let mut flags = 0b0000_0011;
                flags |= (left_channel_routing as u8) << 2;
                flags |= (right_channel_routing as u8) << 5;
                flags
            }
            RoutingUsbInOptions::SamplerOnly => 3,
        }
    }
}

/// An enum which represents the usb out audio routing options.
///
/// # Excerpt from the manual:
///
/// `USB OUT` sets from where in the Analog Rytm MKII’s signal path, the outgoing audio is routed to the
/// class compliant device. This parameter is only available when `USB CONFIG` is set to `USB AUDIO/MIDI`.
///
/// - `MAIN OUT` the outgoing audio is routed from the Analog Rytm MKII’s main out at the end of the signal
/// path.
/// - `TRACK 1–12, L:1–12/R:1–12` Press a `[PAD]` twice to select a single track as a source The selected
/// track’s `[PAD]` (or [PADS] if you selected a track that shares its voice with another track) lights up white.
/// Press first one `[PAD]` and then another to select two separate tracks as sources. The selected tracks’
/// `[PADS]` lights up blue for left channel and red for right channel. The audio from the tracks is routed
/// out and sent separately on left and right channel.
/// - `AUDIO IN` the outgoing audio is routed straight from the Analog Rytm MKII’s audio inputs to the class
/// compliant device.
/// `OFF` no audio is sent to the class compliant device.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoutingUsbOutOptions {
    #[default]
    MainOut,
    /// Left channel routing, Right channel routing
    VoiceRouting((HardwareTrack, HardwareTrack)),
    AudioIn,
    Off,
}

impl TryFrom<u8> for RoutingUsbOutOptions {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let flags = value & 0b0000_0001;
        match flags {
            0 => Ok(RoutingUsbOutOptions::MainOut),
            1 => {
                Ok(RoutingUsbOutOptions::VoiceRouting((
                    // Left channel routing
                    ((value & 0b0001_1100) >> 2).try_into()?,
                    // Right channel routing
                    ((value & 0b1110_0000) >> 5).try_into()?,
                )))
            }
            2 => Ok(RoutingUsbOutOptions::AudioIn),
            3 => Ok(RoutingUsbOutOptions::Off),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "RoutingUsbOutOptions".into(),
            }),
        }
    }
}

impl From<RoutingUsbOutOptions> for u8 {
    fn from(ruoo: RoutingUsbOutOptions) -> Self {
        match ruoo {
            RoutingUsbOutOptions::MainOut => 0,
            RoutingUsbOutOptions::VoiceRouting((left_channel_routing, right_channel_routing)) => {
                let mut flags = 0b0000_0001;
                flags |= (left_channel_routing as u8) << 2;
                flags |= (right_channel_routing as u8) << 5;
                flags
            }
            RoutingUsbOutOptions::AudioIn => 2,
            RoutingUsbOutOptions::Off => 3,
        }
    }
}
