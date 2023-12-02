use rytm_rs_macro::parameter_range;
use rytm_sys::{
    ar_sysex_id_t_AR_TYPE_GLOBAL, ar_sysex_id_t_AR_TYPE_KIT, ar_sysex_id_t_AR_TYPE_PATTERN,
    ar_sysex_id_t_AR_TYPE_SETTINGS, ar_sysex_id_t_AR_TYPE_SONG, ar_sysex_id_t_AR_TYPE_SOUND,
};
use rytm_sys::{ar_sysex_meta_t, s_u16_t, s_u16_t__bindgen_ty_1};

use super::error::RytmError;
use crate::error::ParameterError;
use crate::error::{ConversionError, SysexConversionError};
use crate::pattern::MicroTime;

#[allow(unused)]
pub fn to_s_u16_t_union_a(value: u16) -> s_u16_t {
    let msb = (value >> 8) as u8;
    let lsb = (value & 0xFF) as u8;
    s_u16_t { a: [msb, lsb] }
}

#[allow(unused)]
pub fn to_s_u16_t_union_v(value: u16) -> s_u16_t {
    s_u16_t { v: value }
}

pub fn to_s_u16_t_union_b(value: u16) -> s_u16_t {
    let msb = (value >> 8) as u8;
    let lsb = (value & 0xFF) as u8;
    s_u16_t {
        b: s_u16_t__bindgen_ty_1 { hi: msb, lo: lsb },
    }
}

pub unsafe fn from_s_u16_t(value: &s_u16_t) -> u16 {
    let msb = value.b.hi as u16;
    let lsb = value.b.lo as u16;
    (msb << 8) | lsb
}

#[derive(Clone, Copy, Debug)]
pub enum SysexType {
    Pattern,
    Kit,
    Sound,
    Song,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct SysexMeta {
    pub container_version: u16,
    pub dev_id: u8,
    pub obj_type: u8,
    pub obj_nr: u16,
    pub chksum: u16,
    pub data_size: u16,
}

impl SysexMeta {
    #[parameter_range(range = "object_index:0..=127")]
    pub fn try_default_for_pattern(
        object_index: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: 0x0101,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: object_index as u16,
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
            obj_nr: 0xFF,
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

pub(crate) fn decode_micro_timing_byte(
    micro_timing_value: i8,
) -> Result<MicroTime, ConversionError> {
    match micro_timing_value {
        -23 => Ok(MicroTime::M23B384),
        -22 => Ok(MicroTime::M11B192),
        -21 => Ok(MicroTime::M7B128),
        -20 => Ok(MicroTime::M5B96),
        -19 => Ok(MicroTime::M19B384),
        -18 => Ok(MicroTime::M3B64),
        -17 => Ok(MicroTime::M17B384),
        -16 => Ok(MicroTime::M1B24),
        -15 => Ok(MicroTime::M5B128),
        -14 => Ok(MicroTime::M7B192),
        -13 => Ok(MicroTime::M13B384),
        -12 => Ok(MicroTime::M1B32),
        -11 => Ok(MicroTime::M11B384),
        -10 => Ok(MicroTime::M5B192),
        -9 => Ok(MicroTime::M3B128),
        -8 => Ok(MicroTime::M1B48),
        -7 => Ok(MicroTime::M7B384),
        -6 => Ok(MicroTime::M1B64),
        -5 => Ok(MicroTime::M5B384),
        -4 => Ok(MicroTime::M1B96),
        -3 => Ok(MicroTime::M1B128),
        -2 => Ok(MicroTime::M1B192),
        -1 => Ok(MicroTime::M1B384),
        0 => Ok(MicroTime::OnGrid),
        1..=23 => Err(ConversionError::Range {
            value: micro_timing_value.to_string(),
            type_name: "MicroTime".into(),
        }),
        -64 => Ok(MicroTime::P1B384),
        -63 => Ok(MicroTime::P1B192),
        -62 => Ok(MicroTime::P1B128),
        -61 => Ok(MicroTime::P1B96),
        -60 => Ok(MicroTime::P5B384),
        -59 => Ok(MicroTime::P1B64),
        -58 => Ok(MicroTime::P7B384),
        -57 => Ok(MicroTime::P1B48),
        -56 => Ok(MicroTime::P3B128),
        -55 => Ok(MicroTime::P5B192),
        -54 => Ok(MicroTime::P11B384),
        -53 => Ok(MicroTime::P1B32),
        -52 => Ok(MicroTime::P13B384),
        -51 => Ok(MicroTime::P7B192),
        -50 => Ok(MicroTime::P5B128),
        -49 => Ok(MicroTime::P1B24),
        -48 => Ok(MicroTime::P17B384),
        -47 => Ok(MicroTime::P3B64),
        -46 => Ok(MicroTime::P19B384),
        -45 => Ok(MicroTime::P5B96),
        -44 => Ok(MicroTime::P7B128),
        -43 => Ok(MicroTime::P11B192),
        -42 => Ok(MicroTime::P23B384),
        _ => Err(ConversionError::Range {
            value: micro_timing_value.to_string(),
            type_name: "MicroTime".into(),
        }),
    }
}

pub(crate) fn encode_micro_timing_byte(micro_timing: &MicroTime) -> i8 {
    match micro_timing {
        MicroTime::M23B384 => -23,
        MicroTime::M11B192 => -22,
        MicroTime::M7B128 => -21,
        MicroTime::M5B96 => -20,
        MicroTime::M19B384 => -19,
        MicroTime::M3B64 => -18,
        MicroTime::M17B384 => -17,
        MicroTime::M1B24 => -16,
        MicroTime::M5B128 => -15,
        MicroTime::M7B192 => -14,
        MicroTime::M13B384 => -13,
        MicroTime::M1B32 => -12,
        MicroTime::M11B384 => -11,
        MicroTime::M5B192 => -10,
        MicroTime::M3B128 => -9,
        MicroTime::M1B48 => -8,
        MicroTime::M7B384 => -7,
        MicroTime::M1B64 => -6,
        MicroTime::M5B384 => -5,
        MicroTime::M1B96 => -4,
        MicroTime::M1B128 => -3,
        MicroTime::M1B192 => -2,
        MicroTime::M1B384 => -1,
        MicroTime::OnGrid => 0,
        MicroTime::P1B384 => -64,
        MicroTime::P1B192 => -63,
        MicroTime::P1B128 => -62,
        MicroTime::P1B96 => -61,
        MicroTime::P5B384 => -60,
        MicroTime::P1B64 => -59,
        MicroTime::P7B384 => -58,
        MicroTime::P1B48 => -57,
        MicroTime::P3B128 => -56,
        MicroTime::P5B192 => -55,
        MicroTime::P11B384 => -54,
        MicroTime::P1B32 => -53,
        MicroTime::P13B384 => -52,
        MicroTime::P7B192 => -51,
        MicroTime::P5B128 => -50,
        MicroTime::P1B24 => -49,
        MicroTime::P17B384 => -48,
        MicroTime::P3B64 => -47,
        MicroTime::P19B384 => -46,
        MicroTime::P5B96 => -45,
        MicroTime::P7B128 => -44,
        MicroTime::P11B192 => -43,
        MicroTime::P23B384 => -42,
    }
}
