use super::ObjectQuery;
use crate::{
    error::{ParameterError, RytmError},
    sysex::{AnySysExType, SysexType},
};
use rytm_rs_macro::parameter_range;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A query for a sound object.
pub struct SoundQuery {
    /// Sound index or track index if targeting work buffer
    object_number: usize,
    sysex_type: SysexType,
    device_id: u8,
}

impl SoundQuery {
    #[parameter_range(range = "sound_index:0..=127")]
    pub fn new(sound_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: sound_index,
            sysex_type: SysexType::Sound,
            device_id: 0,
        })
    }

    #[parameter_range(range = "sound_index:0..=127")]
    pub fn new_with_device_id(sound_index: usize, device_id: u8) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: sound_index,
            sysex_type: SysexType::Sound,
            device_id,
        })
    }

    #[parameter_range(range = "track_index:0..=11")]
    pub fn new_targeting_work_buffer(track_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            object_number: track_index | 0b1000_0000,
            sysex_type: SysexType::Sound,
            device_id: 0,
        })
    }

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
    fn sysex_type(&self) -> AnySysExType {
        self.sysex_type.into()
    }

    fn device_id(&self) -> u8 {
        self.device_id
    }

    fn obj_nr(&self) -> u16 {
        self.object_number as u16
    }
}
