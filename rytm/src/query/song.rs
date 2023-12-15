use super::ObjectQuery;
use crate::{
    error::{ParameterError, RytmError},
    sysex::{AnySysexType, SysexType},
};
use rytm_rs_macro::parameter_range;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A query to retrieve a song object from rytm.
///
/// Currently the song object is not supported by rytm-rs.
pub struct SongQuery {
    /// Song index or track index if targeting work buffer
    object_number: usize,
    sysex_type: SysexType,
    device_id: u8,
}

impl SongQuery {
    /// Creates a new song query.
    ///
    /// Accepts a song index in the range of `0..=15`.
    #[parameter_range(range = "song_index:0..=15")]
    pub fn new(song_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: song_index,
            sysex_type: SysexType::Song,
            device_id: 0,
        })
    }

    /// Creates a new song query.
    ///
    /// Accepts a song index in the range of `0..=15`.
    ///
    /// Accepts a device id in the range of `0..=255`.
    #[parameter_range(range = "song_index:0..=15")]
    pub fn new_with_device_id(song_index: usize, device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: song_index,
            sysex_type: SysexType::Song,
            device_id,
        })
    }

    /// Creates a new song query for the song in the work buffer.
    pub const fn new_targeting_work_buffer() -> Self {
        Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Song,
            device_id: 0,
        }
    }

    /// Creates a new song query for the song in the work buffer.
    ///
    /// Accepts a device id in the range of `0..=255`.
    pub const fn new_targeting_work_buffer_with_device_id(device_id: u8) -> Self {
        Self {
            object_number: 0b1000_0000,
            sysex_type: SysexType::Song,
            device_id,
        }
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
