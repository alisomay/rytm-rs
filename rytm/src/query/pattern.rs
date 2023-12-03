use super::ObjectQuery;
use crate::{
    error::{ParameterError, RytmError},
    sysex::SysexType,
};
use rytm_rs_macro::parameter_range;

/// A query for a pattern object.
pub struct PatternQuery {
    /// Pattern index
    object_number: usize,
    r#type: SysexType,
    device_id: u8,
}

impl PatternQuery {
    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn new(pattern_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: pattern_index,
            r#type: SysexType::Pattern,
            device_id: 0,
        })
    }

    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn new_with_device_id(pattern_index: usize, device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: pattern_index,
            r#type: SysexType::Pattern,
            device_id,
        })
    }

    pub fn new_targeting_work_buffer() -> Self {
        Self {
            object_number: 0b1000_0000,
            r#type: SysexType::Pattern,
            device_id: 0,
        }
    }

    pub fn new_targeting_work_buffer_with_device_id(device_id: u8) -> Self {
        Self {
            object_number: 0b1000_0000,
            r#type: SysexType::Pattern,
            device_id,
        }
    }

    pub fn object_number(&self) -> usize {
        self.object_number
    }

    pub fn r#type(&self) -> SysexType {
        self.r#type
    }

    pub fn device_id(&self) -> u8 {
        self.device_id
    }
}

impl ObjectQuery for PatternQuery {
    type SysexTypeExpression = SysexType;

    fn r#type(&self) -> Self::SysexTypeExpression {
        self.r#type()
    }

    fn device_id(&self) -> u8 {
        self.device_id()
    }

    fn obj_nr(&self) -> u16 {
        self.object_number() as u16
    }
}
