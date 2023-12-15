use crate::sysex::AnySysexType;

use super::ObjectQuery;

/// A permissive query type that allows for querying any object.
pub struct RawQuery {
    sysex_type: u8,
    device_id: u8,
    object_number: u16,
}

impl RawQuery {
    /// Creates a new raw query.
    pub const fn new(sysex_type: u8, object_number: u16) -> Self {
        Self {
            sysex_type,
            device_id: 0,
            object_number,
        }
    }

    /// Creates a new raw query with a device id.
    pub const fn new_with_device_id(sysex_type: u8, device_id: u8, object_number: u16) -> Self {
        Self {
            sysex_type,
            device_id,
            object_number,
        }
    }
}

impl ObjectQuery for RawQuery {
    fn sysex_type(&self) -> AnySysexType {
        self.sysex_type.into()
    }

    fn device_id(&self) -> u8 {
        self.device_id
    }

    fn obj_nr(&self) -> u16 {
        self.object_number
    }
}
