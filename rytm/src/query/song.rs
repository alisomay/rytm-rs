use super::ObjectQuery;
use crate::{
    error::{ParameterError, RytmError},
    sysex::{AnySysexType, SysexType},
};
use rytm_rs_macro::parameter_range;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A query for a sound object.
pub struct SongQuery {
    /// Song index or track index if targeting work buffer
    object_number: usize,
    sysex_type: SysexType,
    device_id: u8,
}

impl SongQuery {
    #[parameter_range(range = "song_index:0..=15")]
    pub fn new(song_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: song_index,
            sysex_type: SysexType::Song,
            device_id: 0,
        })
    }

    #[parameter_range(range = "song_index:0..=15")]
    pub fn new_with_device_id(song_index: usize, device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: song_index,
            sysex_type: SysexType::Song,
            device_id,
        })
    }

    pub fn new_targeting_work_buffer() -> Result<Self, RytmError> {
        Ok(Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Song,
            device_id: 0,
        })
    }

    pub fn new_targeting_work_buffer_with_device_id(device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Song,
            device_id,
        })
    }
}

impl ObjectQuery for SongQuery {
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
