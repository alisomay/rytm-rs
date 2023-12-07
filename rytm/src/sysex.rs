use crate::{
    error::{ConversionError, ParameterError, RytmError, SysexConversionError},
    util::{from_s_u16_t, to_s_u16_t_union_b},
};
use rytm_rs_macro::parameter_range;
use rytm_sys::{
    ar_sysex_id_t_AR_TYPE_GLOBAL, ar_sysex_id_t_AR_TYPE_KIT, ar_sysex_id_t_AR_TYPE_PATTERN,
    ar_sysex_id_t_AR_TYPE_SETTINGS, ar_sysex_id_t_AR_TYPE_SONG, ar_sysex_id_t_AR_TYPE_SOUND,
    ar_sysex_meta_t,
};

/// Pattern sysex response size for FW 1.70.
pub const PATTERN_SYSEX_SIZE: usize = 2998;
/// Kit sysex response size for FW 1.70.
pub const KIT_SYSEX_SIZE: usize = 14988;
/// Sound sysex response size for FW 1.70.
pub const SOUND_SYSEX_SIZE: usize = 201;
/// Settings sysex response size for FW 1.70.
pub const SETTINGS_SYSEX_SIZE: usize = 2401;
/// Global sysex response size for FW 1.70.
pub const GLOBAL_SYSEX_SIZE: usize = 107;
/// Song sysex response size for FW 1.70.
pub const SONG_SYSEX_SIZE: usize = 1506;

const SYSEX_MESSAGE_TYPE_BYTE_INDEX: usize = 5;

pub trait SysexCompatible {
    fn r#type(&self) -> SysexType;
    fn as_sysex_message(&self) -> Result<Vec<u8>, RytmError>;
}

#[macro_export]
macro_rules! impl_sysex_compatible {
    ($object_type:ty, $object_raw_type:ty, $object_encoder_function:ident, $object_sysex_type:expr, $object_sysex_size:expr) => {
        impl SysexCompatible for $object_type {
            fn as_sysex_message(&self) -> Result<Vec<u8>, RytmError> {
                let (sysex_meta, raw_object) = self.to_raw_parts();

                let raw_size = std::mem::size_of::<$object_raw_type>();
                let mut raw_buffer: Vec<u8> = Vec::with_capacity(raw_size);

                unsafe {
                    let raw: *const u8 = &raw_object as *const $object_raw_type as *const u8;
                    for i in 0..raw_size {
                        raw_buffer.push(*raw.add(i));
                    }
                }

                let mut encoded_buffer_length: u32 = 0;
                let mut encoded_buf = vec![0; $object_sysex_size];

                let mut meta = sysex_meta.into();
                let meta_ptr = &mut meta as *mut ar_sysex_meta_t;

                unsafe {
                    let return_code = $object_encoder_function(
                        encoded_buf.as_mut_ptr(),
                        raw_buffer.as_ptr(),
                        std::mem::size_of::<$object_raw_type>() as u32,
                        &mut encoded_buffer_length as *mut u32,
                        meta_ptr,
                    ) as u8;

                    if return_code != 0 {
                        return Err(SysexConversionError::from(return_code).into());
                    }

                    Ok(encoded_buf)
                }
            }

            fn r#type(&self) -> SysexType {
                $object_sysex_type
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SysexType {
    Pattern,
    Kit,
    Sound,
    Song,
    #[default]
    Settings,
    Global,
}

impl From<SysexType> for u8 {
    fn from(sysex_type: SysexType) -> Self {
        let sysex_type = match sysex_type {
            SysexType::Pattern => ar_sysex_id_t_AR_TYPE_PATTERN,
            SysexType::Kit => ar_sysex_id_t_AR_TYPE_KIT,
            SysexType::Sound => ar_sysex_id_t_AR_TYPE_SOUND,
            SysexType::Song => ar_sysex_id_t_AR_TYPE_SONG,
            SysexType::Settings => ar_sysex_id_t_AR_TYPE_SETTINGS,
            SysexType::Global => ar_sysex_id_t_AR_TYPE_GLOBAL,
        };

        sysex_type as u8
    }
}

#[allow(non_upper_case_globals)]
impl TryFrom<u8> for SysexType {
    type Error = ConversionError;
    fn try_from(sysex_type: u8) -> Result<Self, Self::Error> {
        match sysex_type as u32 {
            ar_sysex_id_t_AR_TYPE_PATTERN => Ok(SysexType::Pattern),
            ar_sysex_id_t_AR_TYPE_KIT => Ok(SysexType::Kit),
            ar_sysex_id_t_AR_TYPE_SOUND => Ok(SysexType::Sound),
            ar_sysex_id_t_AR_TYPE_SONG => Ok(SysexType::Song),
            ar_sysex_id_t_AR_TYPE_SETTINGS => Ok(SysexType::Settings),
            ar_sysex_id_t_AR_TYPE_GLOBAL => Ok(SysexType::Global),
            _ => Err(ConversionError::Range {
                value: sysex_type.to_string(),
                type_name: "SysexType".into(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SysexMeta {
    pub container_version: u16,
    pub dev_id: u8,
    pub obj_type: u8,
    pub obj_nr: u16,
    pub chksum: u16,
    pub data_size: u16,
}

impl SysexMeta {
    const SYSEX_META_CONTAINER_VERSION: u16 = 0x0101;

    pub fn is_targeting_work_buffer(&self) -> bool {
        self.obj_nr >= 128
    }

    pub fn object_type(&self) -> Result<SysexType, RytmError> {
        let r#type: SysexType = self.obj_type.try_into()?;
        Ok(r#type)
    }

    pub fn default_for_settings(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Settings.into(),
            obj_nr: 0b0000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    #[parameter_range(range = "global_slot:0..=3")]
    pub fn try_default_for_global(
        global_slot: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Global.into(),
            obj_nr: global_slot as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_global_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Global.into(),
            obj_nr: 0b1000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn try_default_for_pattern(
        pattern_index: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: pattern_index as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_pattern_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: 0b1000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    #[parameter_range(range = "kit_index:0..=127")]
    pub fn try_default_for_kit(kit_index: usize, dev_id: Option<usize>) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Kit.into(),
            obj_nr: kit_index as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_kit_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Kit.into(),
            obj_nr: 0b1000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    #[parameter_range(range = "sound_index:0..=127")]
    pub fn try_default_for_sound(
        sound_index: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Sound.into(),
            obj_nr: sound_index as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_sound_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Sound.into(),
            obj_nr: 0b1000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }
}

impl From<SysexMeta> for ar_sysex_meta_t {
    fn from(meta: SysexMeta) -> Self {
        ar_sysex_meta_t {
            container_version: to_s_u16_t_union_b(meta.container_version),
            dev_id: meta.dev_id,
            obj_type: meta.obj_type,
            obj_nr: meta.obj_nr,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: to_s_u16_t_union_b(meta.chksum),
            data_size: to_s_u16_t_union_b(meta.data_size),
        }
    }
}

impl From<&ar_sysex_meta_t> for SysexMeta {
    fn from(meta: &ar_sysex_meta_t) -> Self {
        SysexMeta {
            container_version: unsafe { from_s_u16_t(&meta.container_version) },
            dev_id: meta.dev_id,
            obj_type: meta.obj_type,
            obj_nr: meta.obj_nr,
            chksum: unsafe { from_s_u16_t(&meta.chksum) },
            data_size: unsafe { from_s_u16_t(&meta.data_size) },
        }
    }
}

/// This function assumes that the response is a valid sysex response.
///
/// It should be used in a context where this case is true and validity check is not necessary.
pub fn decode_sysex_response_to_raw(response: &[u8]) -> Result<(Vec<u8>, SysexMeta), RytmError> {
    let response_type: SysexType = response[SYSEX_MESSAGE_TYPE_BYTE_INDEX].try_into()?;
    let response_size = match response_type {
        SysexType::Pattern => PATTERN_SYSEX_SIZE,
        SysexType::Kit => KIT_SYSEX_SIZE,
        SysexType::Sound => SOUND_SYSEX_SIZE,
        SysexType::Settings => SETTINGS_SYSEX_SIZE,
        SysexType::Global => GLOBAL_SYSEX_SIZE,
        SysexType::Song => SONG_SYSEX_SIZE,
    };

    if response.len() != response_size {
        return Err(SysexConversionError::InvalidSize.into());
    }

    // Make a default meta struct to fill.
    let meta = SysexMeta::default();
    let mut meta: rytm_sys::ar_sysex_meta_t = meta.into();
    let meta_p = &mut meta as *mut rytm_sys::ar_sysex_meta_t;

    // The response buffer.
    let mut src_buf = response.as_ptr();
    let src_buf_p = &mut src_buf as *mut *const u8;
    let mut src_buf_size = response.len() as u32;
    let src_buf_size_p = &mut src_buf_size as *mut u32;

    // Will be calculated by the first call to ar_sysex_to_raw.
    let dst_buf_size = 0; // Big enough for the largest sysex message probably.
    let dest_buf_size_p = dst_buf_size as *mut u32;

    // The destination buffer, raw buffer.
    let mut dst_buf = vec![0_u8; response_size];
    let dst_buf_p = dst_buf.as_mut_slice().as_mut_ptr();

    unsafe {
        let return_code = rytm_sys::ar_sysex_to_raw(
            dst_buf_p,
            src_buf_p,
            src_buf_size_p,
            dest_buf_size_p,
            meta_p,
        ) as u8;

        if return_code != 0 {
            return Err(SysexConversionError::from(return_code).into());
        }
    }

    Ok((dst_buf, SysexMeta::from(&meta)))
}
