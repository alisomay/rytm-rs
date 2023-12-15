use super::ObjectQuery;
use crate::{
    error::{ParameterError, RytmError},
    sysex::{AnySysexType, SysexType},
};
use rytm_rs_macro::parameter_range;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A query to retrieve a [`Sound`](crate::object::Sound) object from rytm.
pub struct SoundQuery {
    /// Sound index or track index if targeting work buffer
    object_number: usize,
    sysex_type: SysexType,
    device_id: u8,
}

impl SoundQuery {
    /// Creates a new sound query for a pool sound.
    ///
    /// Accepts a sound index in the range of `0..=127`.
    #[parameter_range(range = "sound_index:0..=127")]
    pub fn new(sound_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: sound_index,
            sysex_type: SysexType::Sound,
            device_id: 0,
        })
    }

    /// Creates a new sound query for a pool sound.
    ///
    /// Accepts a sound index in the range of `0..=127`.
    ///
    /// Accepts a device id in the range of `0..=255`.
    #[parameter_range(range = "sound_index:0..=127")]
    pub fn new_with_device_id(sound_index: usize, device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: sound_index,
            sysex_type: SysexType::Sound,
            device_id,
        })
    }

    /// Creates a new sound query for a sound in the work buffer.
    ///
    /// Accepts a track index in the range of `0..=11`.
    #[parameter_range(range = "track_index:0..=11")]
    pub fn new_targeting_work_buffer(track_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: track_index | 0b1000_0000,
            sysex_type: SysexType::Sound,
            device_id: 0,
        })
    }

    /// Creates a new sound query for a sound in the work buffer.
    ///
    /// Accepts a track index in the range of `0..=11`.
    ///
    /// Accepts a device id in the range of `0..=255`.
    #[parameter_range(range = "track_index:0..=11")]
    pub fn new_targeting_work_buffer_with_device_id(
        track_index: usize,
        device_id: u8,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: track_index | 0b1000_0000,
            sysex_type: SysexType::Sound,
            device_id,
        })
    }
}

impl ObjectQuery for SoundQuery {
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
