use super::ObjectQuery;
use crate::sysex::{AnySysexType, SysexType};

/// A query to retrieve a [`Settings`](crate::object::Settings) object from rytm.
pub struct SettingsQuery {
    sysex_type: SysexType,
    device_id: u8,
}

impl SettingsQuery {
    /// Creates a new settings query.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            sysex_type: SysexType::Settings,
            device_id: 0,
        }
    }

    /// Creates a new settings query with a device id.
    pub const fn new_with_device_id(device_id: u8) -> Self {
        Self {
            sysex_type: SysexType::Settings,
            device_id,
        }
    }
}

impl ObjectQuery for SettingsQuery {
    fn sysex_type(&self) -> AnySysexType {
        self.sysex_type.into()
    }

    fn device_id(&self) -> u8 {
        self.device_id
    }

    fn obj_nr(&self) -> u16 {
        0x0000
    }
}
