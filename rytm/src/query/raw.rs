use super::ObjectQuery;

/// A permissive query type that allows for querying any object.
pub struct RawQuery {
    r#type: u8,
    device_id: u8,
    object_number: u16,
}

impl RawQuery {
    pub fn new(r#type: u8, object_number: u16) -> Self {
        Self {
            r#type,
            device_id: 0,
            object_number,
        }
    }

    pub fn new_with_device_id(r#type: u8, device_id: u8, object_number: u16) -> Self {
        Self {
            r#type,
            device_id,
            object_number,
        }
    }

    pub fn r#type(&self) -> u8 {
        self.r#type
    }

    pub fn device_id(&self) -> u8 {
        self.device_id
    }

    pub fn object_number(&self) -> u16 {
        self.object_number
    }
}

impl ObjectQuery for RawQuery {
    type SysexTypeExpression = u8;

    fn r#type(&self) -> Self::SysexTypeExpression {
        self.r#type()
    }

    fn device_id(&self) -> u8 {
        self.device_id()
    }

    fn obj_nr(&self) -> u16 {
        self.object_number()
    }
}
