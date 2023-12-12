use crate::sysex::AnySysExType;

use super::ObjectQuery;

/// A permissive query type that allows for querying any object.
pub struct RawQuery {
    sysex_type: u8,
    device_id: u8,
    object_number: u16,
}

impl RawQuery {
    pub fn new(sysex_type: u8, object_number: u16) -> Self {
        Self {
            sysex_type,
            device_id: 0,
            object_number,
        }
    }

    pub fn new_with_device_id(sysex_type: u8, device_id: u8, object_number: u16) -> Self {
        Self {
            sysex_type,
            device_id,
            object_number,
        }
    }
}

impl ObjectQuery for RawQuery {
    fn sysex_type(&self) -> AnySysExType {
        self.sysex_type.into()
    }

    fn device_id(&self) -> u8 {
        self.device_id
    }

    fn obj_nr(&self) -> u16 {
        self.object_number
    }
}
