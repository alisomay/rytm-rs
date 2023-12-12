use super::ObjectQuery;
use crate::{
    error::{ParameterError, RytmError},
    sysex::{AnySysExType, SysexType},
};
use rytm_rs_macro::parameter_range;

/// A query for a global object.
pub struct GlobalQuery {
    /// Global slot
    object_number: usize,
    sysex_type: SysexType,
    device_id: u8,
}

impl GlobalQuery {
    #[parameter_range(range = "global_slot:0..=3")]
    pub fn new(global_slot: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: global_slot,
            sysex_type: SysexType::Global,
            device_id: 0,
        })
    }

    #[parameter_range(range = "global_slot:0..=3")]
    pub fn new_with_device_id(global_slot: usize, device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: global_slot,
            sysex_type: SysexType::Global,
            device_id,
        })
    }

    pub fn new_targeting_work_buffer() -> Self {
        Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Global,
            device_id: 0,
        }
    }

    pub fn new_targeting_work_buffer_with_device_id(device_id: u8) -> Self {
        Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Global,
            device_id,
        }
    }
}

impl ObjectQuery for GlobalQuery {
    fn sysex_type(&self) -> AnySysExType {
        self.sysex_type.into()
    }

    fn device_id(&self) -> u8 {
        self.device_id
    }

    fn obj_nr(&self) -> u16 {
        self.object_number as u16
    }
}
