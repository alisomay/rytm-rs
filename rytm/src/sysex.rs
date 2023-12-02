use crate::{error::RytmError, util::SysexType};

pub trait SysexCompatible {
    fn r#type(&self) -> SysexType;
    fn as_sysex_message(&self) -> Result<Vec<u8>, RytmError>;
}
