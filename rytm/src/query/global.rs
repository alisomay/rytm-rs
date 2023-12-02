use super::ObjectQuery;
use crate::util::SysexType;

/// A query for a global object.
pub struct GlobalQuery {
    /// Global slot
    object_number: usize,
    r#type: SysexType,
    device_id: u8,
}

impl GlobalQuery {
    pub fn new(global_slot: usize) -> Self {
        Self {
            object_number: global_slot,
            r#type: SysexType::Global,
            device_id: 0,
        }
    }

    pub fn new_with_device_id(global_slot: usize, device_id: u8) -> Self {
        Self {
            object_number: global_slot,
            r#type: SysexType::Global,
            device_id,
        }
    }

    pub fn new_targeting_work_buffer() -> Self {
        Self {
            object_number: 0xFF,
            r#type: SysexType::Global,
            device_id: 0,
        }
    }

    pub fn new_targeting_work_buffer_with_device_id(device_id: u8) -> Self {
        Self {
            object_number: 0xFF,
            r#type: SysexType::Global,
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

impl ObjectQuery for GlobalQuery {
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
