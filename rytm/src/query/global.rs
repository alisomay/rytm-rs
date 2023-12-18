use super::ObjectQuery;
use crate::{
    error::{ParameterError, RytmError},
    sysex::{AnySysexType, SysexType},
};
use rytm_rs_macro::parameter_range;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A query to retrieve a [`Global`](crate::object::Global) object from rytm.
pub struct GlobalQuery {
    /// Global slot
    object_number: usize,
    sysex_type: SysexType,
    device_id: u8,
}

impl GlobalQuery {
    /// Creates a new global query for a global slot.
    ///
    /// Accepts a global slot in the range of `0..=3`.
    #[parameter_range(range = "global_slot:0..=3")]
    pub fn new(global_slot: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: global_slot,
            sysex_type: SysexType::Global,
            device_id: 0,
        })
    }

    /// Creates a new global query for a global slot.
    ///
    /// Accepts a global slot in the range of `0..=3`.
    ///
    /// Accepts a device id in the range of `0..=255`.
    #[parameter_range(range = "global_slot:0..=3")]
    pub fn new_with_device_id(global_slot: usize, device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: global_slot,
            sysex_type: SysexType::Global,
            device_id,
        })
    }

    /// Creates a new global query for the global in the work buffer.
    pub const fn new_targeting_work_buffer() -> Self {
        Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Global,
            device_id: 0,
        }
    }

    /// Creates a new global query for the global in the work buffer.
    ///
    /// Accepts a device id in the range of `0..=255`.
    pub const fn new_targeting_work_buffer_with_device_id(device_id: u8) -> Self {
        Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Global,
            device_id,
        }
    }
}

impl ObjectQuery for GlobalQuery {
    fn sysex_type(&self) -> AnySysexType {
        self.sysex_type.into()
    }

    fn device_id(&self) -> u8 {
        self.device_id
    }

    fn obj_nr(&self) -> u16 {
        self.object_number as u16
    }
}
