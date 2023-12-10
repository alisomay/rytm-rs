#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ConversionError {
    #[error("Conversion error: {value} is out of range for {type_name}")]
    Range { value: String, type_name: String },
    #[error("Conversion error: The object name you have provided \"{0}\" is {1} characters and too long. The maximum length is 15 characters.")]
    ObjectNameTooLong(String, usize),
    #[error("Conversion error: The object name you have provided \"{0}\" contains non-ascii characters.")]
    ObjectNameNotAscii(String),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ParameterError {
    #[error("Parameter error: {value} is out of range for {parameter_name}")]
    Range {
        value: String,
        parameter_name: String,
    },

    #[error("Parameter error: {value} is not compatible with {parameter_name}. {reason:?}")]
    Compatibility {
        value: String,
        parameter_name: String,
        reason: Option<String>,
    },
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum SysexConversionError {
    #[error("Not a sysex message")]
    NotASysexMsg,
    #[error("Short read on sysex message")]
    ShortRead,
    #[error("End of message too soon")]
    EndOfMessage,
    #[error("Abort")]
    Abort,
    #[error("Invalid manufacturer ID")]
    InvalidManufacturerId,
    #[error("Invalid product ID")]
    InvalidProductId,
    #[error("Invalid dump message ID")]
    InvalidDumpMsgId,
    #[error("Invalid object type")]
    InvalidObjType,
    #[error("Checksum error")]
    Chksum,
    #[error("Null pointer")]
    Nullptr,
    #[error("Invalid object number")]
    InvalidObjNr,
    #[error("Not a pattern")]
    NotAPattern,
    #[error("Not a kit")]
    NotAKit,
    #[error("Not a sound")]
    NotASound,
    #[error("The type of the sysex message does not match the size of the message. Expected {0} got {1}")]
    InvalidSize(usize, usize),
    #[error("Unknown error code: {0}")]
    Unknown(u8),
}

impl From<u8> for SysexConversionError {
    fn from(code: u8) -> Self {
        match code {
            1 => SysexConversionError::NotASysexMsg,
            2 => SysexConversionError::ShortRead,
            3 => SysexConversionError::EndOfMessage,
            4 => SysexConversionError::Abort,
            5 => SysexConversionError::InvalidManufacturerId,
            6 => SysexConversionError::InvalidProductId,
            7 => SysexConversionError::InvalidDumpMsgId,
            8 => SysexConversionError::InvalidObjType,
            9 => SysexConversionError::Chksum,
            10 => SysexConversionError::Nullptr,
            11 => SysexConversionError::InvalidObjNr,
            12 => SysexConversionError::NotAPattern,
            13 => SysexConversionError::NotAKit,
            14 => SysexConversionError::NotASound,
            _ => SysexConversionError::Unknown(code),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum RytmError {
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    Conversion(#[from] ConversionError),
    #[error("{0}")]
    Parameter(#[from] ParameterError),
    #[error("{0}")]
    SysexConversion(#[from] SysexConversionError),

    #[error("Parameter lock memory full.")]
    ParameterLockMemoryFull,
    #[error("Parameter lock pool is not set for this trig thus it is not connected to a pattern and orphan. This function can not be called on an orphan trig.")]
    OrphanTrig,
}
