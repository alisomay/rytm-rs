use super::ObjectQuery;
use crate::{
    error::{ParameterError, RytmError},
    sysex::{AnySysExType, SysexType},
};
use rytm_rs_macro::parameter_range;

/// A query for a kit object.
pub struct KitQuery {
    /// Kit index
    object_number: usize,
    sysex_type: SysexType,
    device_id: u8,
}

impl KitQuery {
    #[parameter_range(range = "kit_index:0..=127")]
    pub fn new(kit_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: kit_index,
            sysex_type: SysexType::Kit,
            device_id: 0,
        })
    }

    #[parameter_range(range = "kit_index:0..=127")]
    pub fn new_with_device_id(kit_index: usize, device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: kit_index,
            sysex_type: SysexType::Kit,
            device_id,
        })
    }

    pub fn new_targeting_work_buffer() -> Self {
        Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Kit,
            device_id: 0,
        }
    }

    pub fn new_targeting_work_buffer_with_device_id(device_id: u8) -> Self {
        Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Kit,
            device_id,
        }
    }
}

impl ObjectQuery for KitQuery {
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
