use crate::error::{ParameterError, RytmError, SysexConversionError};
use crate::util::{from_s_u16_t, to_s_u16_t_union_b};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sysex_meta_t;
use rytm_sys::{
    ar_sysex_id_t_AR_TYPE_GLOBAL, ar_sysex_id_t_AR_TYPE_KIT, ar_sysex_id_t_AR_TYPE_PATTERN,
    ar_sysex_id_t_AR_TYPE_SETTINGS, ar_sysex_id_t_AR_TYPE_SONG, ar_sysex_id_t_AR_TYPE_SOUND,
};

pub trait SysexCompatible {
    fn r#type(&self) -> SysexType;
    fn as_sysex_message(&self) -> Result<Vec<u8>, RytmError>;
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
    pub fn is_targeting_work_buffer(&self) -> bool {
        self.obj_nr & 0b1000_0000_0000_0000 != 0
    }

    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn try_default_for_pattern(
        pattern_index: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: 0x0101,
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
            container_version: 0x0101,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: 0b1000_0000_0000_0000,
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
            container_version: 0x0101,
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
            container_version: 0x0101,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Sound.into(),
            obj_nr: 0b1000_0000_0000_0000,
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

pub fn decode_sysex_response_to_raw(response: &[u8]) -> Result<(Vec<u8>, SysexMeta), RytmError> {
    // This could be made smaller later, the largest reverse-engineered sysex response I've seen is 14988 bytes.
    // But for now I think it is a reasonable value.
    // It can be optimized if we know which response we're expecting if necessary.
    const LARGE_SYSEX_GUESSED_SIZE: usize = 4096 * 4;

    let meta = SysexMeta::default();
    let mut meta: rytm_sys::ar_sysex_meta_t = meta.into();
    let meta_p = &mut meta as *mut rytm_sys::ar_sysex_meta_t;

    let mut src_buf = response.as_ptr();
    let src_buf_p = &mut src_buf as *mut *const u8;
    let mut src_buf_size = response.len() as u32;
    let src_buf_size_p = &mut src_buf_size as *mut u32;

    // Will be calculated by the first call to ar_sysex_to_raw.
    let dst_buf_size = 0; // Big enough for the largest sysex message probably.
    let dest_buf_size_p = dst_buf_size as *mut u32;

    let mut dst_buf = vec![0_u8; LARGE_SYSEX_GUESSED_SIZE];
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

    dst_buf.shrink_to_fit();

    Ok((dst_buf, SysexMeta::from(&meta)))
}
