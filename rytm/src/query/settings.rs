use super::ObjectQuery;
use crate::sysex::SysexType;

/// A query for the settings object.
pub struct SettingsQuery {
    r#type: SysexType,
    device_id: u8,
}

impl SettingsQuery {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            r#type: SysexType::Settings,
            device_id: 0,
        }
    }

    pub fn new_with_device_id(device_id: u8) -> Self {
        Self {
            r#type: SysexType::Settings,
            device_id,
        }
    }

    pub fn r#type(&self) -> SysexType {
        self.r#type
    }

    pub fn device_id(&self) -> u8 {
        self.device_id
    }
}

impl ObjectQuery for SettingsQuery {
    type SysexTypeExpression = SysexType;

    fn r#type(&self) -> Self::SysexTypeExpression {
        self.r#type()
    }

    fn device_id(&self) -> u8 {
        self.device_id()
    }

    fn obj_nr(&self) -> u16 {
        // Ignored for settings
        0x0000
    }
}
