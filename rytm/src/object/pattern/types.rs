use crate::error::ConversionError;
use rytm_sys::{
    AR_SPEED_1B2X, AR_SPEED_1B4X, AR_SPEED_1B8X, AR_SPEED_1X, AR_SPEED_2X, AR_SPEED_3B2X,
    AR_SPEED_3B4X,
};
use serde::{Deserialize, Serialize};

/// The speed of a pattern.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Speed {
    #[default]
    /// 1x
    X1,
    /// 2x
    X2,
    /// 3/2x
    X3B2,
    /// 3/4x
    X3B4,
    /// 1/2x
    X1B2,
    /// 1/4x
    X1B4,
    /// 1/8x
    X1B8,
}

impl TryFrom<&str> for Speed {
    type Error = ConversionError;
    fn try_from(speed: &str) -> Result<Self, Self::Error> {
        match speed {
            "1x" => Ok(Self::X1),
            "2x" => Ok(Self::X2),
            "3/2x" => Ok(Self::X3B2),
            "3/4x" => Ok(Self::X3B4),
            "1/2x" => Ok(Self::X1B2),
            "1/4x" => Ok(Self::X1B4),
            "1/8x" => Ok(Self::X1B8),
            _ => Err(ConversionError::Range {
                value: speed.to_string(),
                type_name: "Speed".into(),
            }),
        }
    }
}

impl From<Speed> for &str {
    fn from(speed: Speed) -> Self {
        match speed {
            Speed::X1 => "1x",
            Speed::X2 => "2x",
            Speed::X3B2 => "3/2x",
            Speed::X3B4 => "3/4x",
            Speed::X1B2 => "1/2x",
            Speed::X1B4 => "1/4x",
            Speed::X1B8 => "1/8x",
        }
    }
}

impl From<Speed> for u8 {
    fn from(speed: Speed) -> Self {
        let speed = match speed {
            Speed::X1 => AR_SPEED_1X,
            Speed::X2 => AR_SPEED_2X,
            Speed::X3B2 => AR_SPEED_3B2X,
            Speed::X3B4 => AR_SPEED_3B4X,
            Speed::X1B2 => AR_SPEED_1B2X,
            Speed::X1B4 => AR_SPEED_1B4X,
            Speed::X1B8 => AR_SPEED_1B8X,
        };
        speed as Self
    }
}

impl TryFrom<u8> for Speed {
    type Error = ConversionError;
    fn try_from(speed: u8) -> Result<Self, Self::Error> {
        match speed as u32 {
            AR_SPEED_1X => Ok(Self::X1),
            AR_SPEED_2X => Ok(Self::X2),
            AR_SPEED_3B2X => Ok(Self::X3B2),
            AR_SPEED_3B4X => Ok(Self::X3B4),
            AR_SPEED_1B2X => Ok(Self::X1B2),
            AR_SPEED_1B4X => Ok(Self::X1B4),
            AR_SPEED_1B8X => Ok(Self::X1B8),
            _ => Err(ConversionError::Range {
                value: speed.to_string(),
                type_name: "Speed".into(),
            }),
        }
    }
}

/// The time mode of a pattern.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum TimeMode {
    #[default]
    Normal,
    Advanced,
}

impl TryFrom<&str> for TimeMode {
    type Error = ConversionError;
    fn try_from(mode: &str) -> Result<Self, Self::Error> {
        match mode {
            "normal" => Ok(Self::Normal),
            "advanced" => Ok(Self::Advanced),
            _ => Err(ConversionError::Range {
                value: mode.to_string(),
                type_name: "TimeMode".into(),
            }),
        }
    }
}

impl From<TimeMode> for &str {
    fn from(mode: TimeMode) -> Self {
        match mode {
            TimeMode::Normal => "normal",
            TimeMode::Advanced => "advanced",
        }
    }
}

impl From<TimeMode> for u8 {
    fn from(mode: TimeMode) -> Self {
        match mode {
            TimeMode::Normal => 0,
            TimeMode::Advanced => 1,
        }
    }
}

impl TryFrom<u8> for TimeMode {
    type Error = ConversionError;
    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Advanced),
            _ => Err(ConversionError::Range {
                value: mode.to_string(),
                type_name: "TimeMode".into(),
            }),
        }
    }
}
