use serde::{Deserialize, Serialize};

use crate::error::ConversionError;

/// The six sequential square buttons on the right side of the Analog Rytm MKII. Fx mode off.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ParameterMenuItem {
    #[default]
    Trig,
    Src,
    Smpl,
    Fltr,
    Amp,
    Lfo,
}

impl TryFrom<u8> for ParameterMenuItem {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Trig),
            1 => Ok(Self::Src),
            2 => Ok(Self::Smpl),
            3 => Ok(Self::Fltr),
            4 => Ok(Self::Amp),
            5 => Ok(Self::Lfo),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "ParameterMenuItem".into(),
            }),
        }
    }
}

impl From<ParameterMenuItem> for u8 {
    fn from(parameter_menu_item: ParameterMenuItem) -> Self {
        match parameter_menu_item {
            ParameterMenuItem::Trig => 0,
            ParameterMenuItem::Src => 1,
            ParameterMenuItem::Smpl => 2,
            ParameterMenuItem::Fltr => 3,
            ParameterMenuItem::Amp => 4,
            ParameterMenuItem::Lfo => 5,
        }
    }
}

/// The six sequential square buttons on the right side of the Analog Rytm MKII. Fx mode on.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum FxParameterMenuItem {
    #[default]
    Trig,
    Delay,
    Reverb,
    Dist,
    Comp,
    Lfo,
}

impl TryFrom<u8> for FxParameterMenuItem {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Trig),
            1 => Ok(Self::Delay),
            2 => Ok(Self::Reverb),
            3 => Ok(Self::Dist),
            4 => Ok(Self::Comp),
            5 => Ok(Self::Lfo),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FxParameterMenuItem".into(),
            }),
        }
    }
}

impl From<FxParameterMenuItem> for u8 {
    fn from(fx_parameter_menu_item: FxParameterMenuItem) -> Self {
        match fx_parameter_menu_item {
            FxParameterMenuItem::Trig => 0,
            FxParameterMenuItem::Delay => 1,
            FxParameterMenuItem::Reverb => 2,
            FxParameterMenuItem::Dist => 3,
            FxParameterMenuItem::Comp => 4,
            FxParameterMenuItem::Lfo => 5,
        }
    }
}

/// - Normal operation mode is the default mode. In this mode, the sequencer plays the selected pattern in a loop.
/// - Chain mode is used to chain patterns together into a chain. The chain is played in a loop.
/// - Song mode is used to chain patterns together into a song. The song is played once from start to finish.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SequencerMode {
    #[default]
    Normal,
    Chain,
    Song,
}

impl TryFrom<u8> for SequencerMode {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Chain),
            2 => Ok(Self::Song),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SequencerMode".into(),
            }),
        }
    }
}

impl From<SequencerMode> for u8 {
    fn from(sequencer_mode: SequencerMode) -> Self {
        match sequencer_mode {
            SequencerMode::Normal => 0,
            SequencerMode::Chain => 1,
            SequencerMode::Song => 2,
        }
    }
}

/// # Excerpt from the manual
///
/// When changing patterns, different modes affecting the way the active pattern will be changed exist.
///
/// Press `[FUNC]` + `[BANK A–D]` to select PATTERN mode. The <PATTERN MODE> LEDs indicate which mode is
/// selected.
///
/// There are four PATTERN modes.
///
/// - SEQUENTIAL changes patterns after the currently playing pattern reaches its end. This mode is the
/// default mode.
/// - DIRECT START immediately changes patterns. The new pattern will start playing from the beginning.
/// - DIRECT JUMP immediately changes patterns. The new pattern will start playing from the position where
/// the previous pattern left off.
/// - TEMP JUMP works a little bit differently from the other PATTERN modes. It works like this:
///
/// 1. Press `[FUNC]` + `[BANK D]` to arm TEMP JUMP PATTERN mode. The Temp Jump LED starts to flash
/// (if the sequencer is running) to indicate that Temp Jump mode is armed.
///
/// 2. Select a new pattern. The Temp Jump LED is now firmly lit to indicate that Temp Jump mode is active
/// The pattern changes immediately and the new pattern starts playing from the position where the pre-
/// vious pattern left off. It plays the new pattern once to the end and then return to the pattern that was
/// playing before the change. Once the sequencer has returned to the earlier pattern, then TEMP JUMP
/// mode is no longer active.
///
/// You can also use TEMP JUMP mode when you are in CHAIN mode, but then the pattern you change to
/// instead replaces the current pattern in the chain. For example, say that you have a chain set up like this:
/// A01 > A03 > A04 > A02. When the chain is playing, and you are in TEMP JUMP mode, change pattern to
/// A16 while pattern A03 is playing. The pattern will immediately change to A16 and once A16 has ended
/// then the chain will continue to play from pattern A04.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum PatternMode {
    #[default]
    Sequential,
    DirectStart,
    DirectJump,
    TempJump,
}

impl TryFrom<u8> for PatternMode {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sequential),
            1 => Ok(Self::DirectStart),
            2 => Ok(Self::DirectJump),
            3 => Ok(Self::TempJump),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "PatternMode".into(),
            }),
        }
    }
}

impl From<PatternMode> for u8 {
    fn from(pattern_mode: PatternMode) -> Self {
        match pattern_mode {
            PatternMode::Sequential => 0,
            PatternMode::DirectStart => 1,
            PatternMode::DirectJump => 2,
            PatternMode::TempJump => 3,
        }
    }
}

/// # Excerpt from the manual
///
/// Source selects between different audio sources to sample from.
/// - AUD L+R sets the input source to sample external audio through the AUDIO IN L+R inputs. The
/// audio is summed together to mono.
/// - AUD L sets the input source to AUDIO IN L.
/// - AUD R sets the input source to AUDIO IN R.
/// - BD, SD, RS/CP, BT, LT, MT/HT, CH/OH, CY/CB sets the input source to the internal audio from the
/// separate drum tracks.
/// - MAIN sets the input source to the internal MAIN L+R channels. The audio is summed together to
/// mono.
/// - USB L+R sets the input source to sample external audio (from both left and right channel) through
/// the USB input. The audio is summed together to mono.
/// - USB L sets the input source to sample external audio from only the left channel of the incoming
/// audio from the USB input.
/// - USB R sets the input source to sample external audio from only the right channel of the incoming
/// audio from the USB input.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SampleRecorderSource {
    #[default]
    AudLPlusR,
    AudL,
    AudR,
    Bd,
    Sd,
    RsCp,
    Bt,
    Lt,
    MtHt,
    ChOh,
    CyCb,
    Main,
    UsbL,
    UsbR,
    UsbLPlusR,
}

impl TryFrom<u8> for SampleRecorderSource {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AudLPlusR),
            1 => Ok(Self::AudL),
            2 => Ok(Self::AudR),
            3 => Ok(Self::Bd),
            4 => Ok(Self::Sd),
            5 => Ok(Self::RsCp),
            6 => Ok(Self::Bt),
            7 => Ok(Self::Lt),
            8 => Ok(Self::MtHt),
            9 => Ok(Self::ChOh),
            10 => Ok(Self::CyCb),
            11 => Ok(Self::Main),
            12 => Ok(Self::UsbL),
            13 => Ok(Self::UsbR),
            14 => Ok(Self::UsbLPlusR),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SampleRecorderSource".into(),
            }),
        }
    }
}

impl From<SampleRecorderSource> for u8 {
    fn from(sample_recorder_source: SampleRecorderSource) -> Self {
        match sample_recorder_source {
            SampleRecorderSource::AudLPlusR => 0,
            SampleRecorderSource::AudL => 1,
            SampleRecorderSource::AudR => 2,
            SampleRecorderSource::Bd => 3,
            SampleRecorderSource::Sd => 4,
            SampleRecorderSource::RsCp => 5,
            SampleRecorderSource::Bt => 6,
            SampleRecorderSource::Lt => 7,
            SampleRecorderSource::MtHt => 8,
            SampleRecorderSource::ChOh => 9,
            SampleRecorderSource::CyCb => 10,
            SampleRecorderSource::Main => 11,
            SampleRecorderSource::UsbL => 12,
            SampleRecorderSource::UsbR => 13,
            SampleRecorderSource::UsbLPlusR => 14,
        }
    }
}

/// # Excerpt from the manual
///
/// Record Length sets the length of the sampling. With a setting of 1–128 steps, the sampling length is
/// decided by the time that it takes the sequencer to advance the set number of steps at the current BPM.
/// With the MAX setting, the sampling continues until max sampling time (33 seconds) is reached or until
/// you press `[YES]` to stop sampling.
///
/// With a BPM set lower than 59, recording the complete 128 steps is not possible because
/// it will take longer than the maximum sampling time. If the device cannot record the entire
/// set length, this is shown by two exclamation marks next to the sample memory time on the
/// screen.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SampleRecorderRecordingLength {
    _1Step,
    _2Steps,
    _4Steps,
    _8Steps,
    _16Steps,
    _32Steps,
    _64Steps,
    _128Steps,
    #[default]
    Max,
}

impl TryFrom<u8> for SampleRecorderRecordingLength {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::_1Step),
            1 => Ok(Self::_2Steps),
            2 => Ok(Self::_4Steps),
            3 => Ok(Self::_8Steps),
            4 => Ok(Self::_16Steps),
            5 => Ok(Self::_32Steps),
            6 => Ok(Self::_64Steps),
            7 => Ok(Self::_128Steps),
            8 => Ok(Self::Max),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SampleRecorderRecordingLength".into(),
            }),
        }
    }
}

impl From<SampleRecorderRecordingLength> for u8 {
    fn from(sample_recorder_recording_length: SampleRecorderRecordingLength) -> Self {
        match sample_recorder_recording_length {
            SampleRecorderRecordingLength::_1Step => 0,
            SampleRecorderRecordingLength::_2Steps => 1,
            SampleRecorderRecordingLength::_4Steps => 2,
            SampleRecorderRecordingLength::_8Steps => 3,
            SampleRecorderRecordingLength::_16Steps => 4,
            SampleRecorderRecordingLength::_32Steps => 5,
            SampleRecorderRecordingLength::_64Steps => 6,
            SampleRecorderRecordingLength::_128Steps => 7,
            SampleRecorderRecordingLength::Max => 8,
        }
    }
}
