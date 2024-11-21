use crate::{
    error::{ConversionError, ParameterError, RytmError, SysexConversionError},
    util::{from_s_u16_t, to_s_u16_t_union_b},
};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sysex_meta_t;
use serde::{Deserialize, Serialize};

mod sysex_id {
    #![allow(clippy::cast_possible_truncation)]
    use rytm_sys::{
        ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_GLOBAL, ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_KIT,
        ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_PATTERN,
        ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_SETTINGS, ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_SONG,
        ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_SOUND, ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_GLOBAL,
        ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_KIT, ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_PATTERN,
        ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_SETTINGS, ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_SONG,
        ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_SOUND, ar_sysex_id_t_AR_TYPE_GLOBAL,
        ar_sysex_id_t_AR_TYPE_KIT, ar_sysex_id_t_AR_TYPE_PATTERN, ar_sysex_id_t_AR_TYPE_SETTINGS,
        ar_sysex_id_t_AR_TYPE_SONG, ar_sysex_id_t_AR_TYPE_SOUND,
    };

    pub const DUMP_ID_PATTERN: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_PATTERN as u8;
    pub const DUMP_ID_KIT: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_KIT as u8;
    pub const DUMP_ID_SOUND: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_SOUND as u8;
    pub const DUMP_ID_SONG: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_SONG as u8;
    pub const DUMP_ID_SETTINGS: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_SETTINGS as u8;
    pub const DUMP_ID_GLOBAL: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMP_ID_GLOBAL as u8;

    pub const DUMPX_ID_PATTERN: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_PATTERN as u8;
    pub const DUMPX_ID_KIT: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_KIT as u8;
    pub const DUMPX_ID_SOUND: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_SOUND as u8;
    pub const DUMPX_ID_SONG: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_SONG as u8;
    pub const DUMPX_ID_SETTINGS: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_SETTINGS as u8;
    pub const DUMPX_ID_GLOBAL: u8 = ar_sysex_dump_id_t_AR_SYSEX_DUMPX_ID_GLOBAL as u8;

    pub const ID_PATTERN: u8 = ar_sysex_id_t_AR_TYPE_PATTERN as u8;
    pub const ID_KIT: u8 = ar_sysex_id_t_AR_TYPE_KIT as u8;
    pub const ID_SOUND: u8 = ar_sysex_id_t_AR_TYPE_SOUND as u8;
    pub const ID_SONG: u8 = ar_sysex_id_t_AR_TYPE_SONG as u8;
    pub const ID_SETTINGS: u8 = ar_sysex_id_t_AR_TYPE_SETTINGS as u8;
    pub const ID_GLOBAL: u8 = ar_sysex_id_t_AR_TYPE_GLOBAL as u8;
}

/// The type of a sysex message.
///
/// Can represent known sysex types.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SysexType {
    Pattern,
    Kit,
    Sound,
    Song,
    #[default]
    Settings,
    Global,
}

impl SysexType {
    /// Dump ids are different compared to query ids.
    ///
    /// # Errors
    ///
    /// Returns an error if the dump id is invalid.
    pub fn try_from_dump_id(dump_id: u8) -> Result<Self, RytmError> {
        use sysex_id::*;
        match dump_id {
            DUMP_ID_PATTERN | DUMPX_ID_PATTERN => Ok(Self::Pattern),
            DUMP_ID_KIT | DUMPX_ID_KIT => Ok(Self::Kit),
            DUMP_ID_SOUND | DUMPX_ID_SOUND => Ok(Self::Sound),
            DUMP_ID_SONG | DUMPX_ID_SONG => Ok(Self::Song),
            DUMP_ID_SETTINGS | DUMPX_ID_SETTINGS => Ok(Self::Settings),
            DUMP_ID_GLOBAL | DUMPX_ID_GLOBAL => Ok(Self::Global),
            _ => Err(SysexConversionError::InvalidDumpMsgId.into()),
        }
    }
}

impl From<SysexType> for u8 {
    fn from(sysex_type: SysexType) -> Self {
        use sysex_id::*;
        match sysex_type {
            SysexType::Pattern => ID_PATTERN,
            SysexType::Kit => ID_KIT,
            SysexType::Sound => ID_SOUND,
            SysexType::Song => ID_SONG,
            SysexType::Settings => ID_SETTINGS,
            SysexType::Global => ID_GLOBAL,
        }
    }
}

#[allow(non_upper_case_globals)]
impl TryFrom<u8> for SysexType {
    type Error = ConversionError;
    fn try_from(sysex_type: u8) -> Result<Self, Self::Error> {
        use sysex_id::*;
        match sysex_type {
            ID_PATTERN => Ok(Self::Pattern),
            ID_KIT => Ok(Self::Kit),
            ID_SOUND => Ok(Self::Sound),
            ID_SONG => Ok(Self::Song),
            ID_SETTINGS => Ok(Self::Settings),
            ID_GLOBAL => Ok(Self::Global),
            _ => Err(ConversionError::Range {
                value: sysex_type.to_string(),
                type_name: "SysexType".into(),
            }),
        }
    }
}

/// Contains the metadata of a sysex message.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct SysexMeta {
    pub container_version: u16,
    pub dev_id: u8,
    pub obj_type: u8,
    pub obj_nr: u16,
    // The rest is not calculated in queries. Only calculated in responses and the calculation is done by [libanalogrytm](https://github.com/bsp2/libanalogrytm).
    pub chksum: u16,
    pub data_size: u16,
}

impl SysexMeta {
    // Found in all sysex messages comes from rytm.
    const SYSEX_META_CONTAINER_VERSION: u16 = 0x0101;

    pub const fn is_targeting_work_buffer(&self) -> bool {
        self.obj_nr >= 128
    }

    pub const fn get_normalized_object_index(&self) -> usize {
        if self.is_targeting_work_buffer() {
            return (self.obj_nr & 0b0111_1111) as usize;
        }
        self.obj_nr as usize
    }

    /// Returns the object type of the sysex message.
    pub fn object_type(&self) -> Result<SysexType, RytmError> {
        Ok(self.obj_type.try_into()?)
    }

    pub fn default_for_settings(dev_id: Option<u8>) -> Self {
        Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Settings.into(),
            obj_nr: 0b0000_0000,
            ..Default::default()
        }
    }

    #[parameter_range(range = "global_slot:0..=3")]
    pub fn try_default_for_global(
        global_slot: usize,
        dev_id: Option<u8>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Global.into(),
            obj_nr: global_slot as u16,
            ..Default::default()
        })
    }

    pub fn default_for_global_in_work_buffer(dev_id: Option<u8>) -> Self {
        Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Global.into(),
            obj_nr: 0b1000_0000,
            ..Default::default()
        }
    }

    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn try_default_for_pattern(
        pattern_index: usize,
        dev_id: Option<u8>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Pattern.into(),
            obj_nr: pattern_index as u16,
            ..Default::default()
        })
    }

    pub fn default_for_pattern_in_work_buffer(dev_id: Option<u8>) -> Self {
        Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Pattern.into(),
            obj_nr: 0b1000_0000,
            ..Default::default()
        }
    }

    #[parameter_range(range = "kit_index:0..=127")]
    pub fn try_default_for_kit(kit_index: usize, dev_id: Option<u8>) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Kit.into(),
            obj_nr: kit_index as u16,
            ..Default::default()
        })
    }

    pub fn default_for_kit_in_work_buffer(dev_id: Option<u8>) -> Self {
        Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Kit.into(),
            obj_nr: 0b1000_0000,
            ..Default::default()
        }
    }

    #[parameter_range(range = "sound_index:0..=127")]
    pub fn try_default_for_sound(
        sound_index: usize,
        dev_id: Option<u8>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Sound.into(),
            obj_nr: sound_index as u16,
            ..Default::default()
        })
    }

    pub fn default_for_sound_in_work_buffer(track_index: usize, dev_id: Option<u8>) -> Self {
        Self {
            container_version: Self::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0),
            obj_type: SysexType::Sound.into(),
            obj_nr: (0b1000_0000_u8 | track_index as u8) as u16,
            ..Default::default()
        }
    }

    pub fn set_device_id(&mut self, dev_id: u8) {
        self.dev_id = dev_id;
    }
}

impl From<SysexMeta> for ar_sysex_meta_t {
    fn from(meta: SysexMeta) -> Self {
        Self {
            container_version: to_s_u16_t_union_b(meta.container_version),
            dev_id: meta.dev_id,
            obj_type: meta.obj_type,
            obj_nr: meta.obj_nr,
            ..Default::default()
        }
    }
}

impl From<&ar_sysex_meta_t> for SysexMeta {
    fn from(meta: &ar_sysex_meta_t) -> Self {
        Self {
            container_version: unsafe { from_s_u16_t(meta.container_version) },
            dev_id: meta.dev_id,
            obj_type: meta.obj_type,
            obj_nr: meta.obj_nr,
            chksum: unsafe { from_s_u16_t(meta.chksum) },
            data_size: unsafe { from_s_u16_t(meta.data_size) },
        }
    }
}
